//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::env::*;
use crate::error::*;
use crate::tree::*;
use crate::value::*;

#[derive(Clone, Debug)]
pub struct Interp
{
    stack_trace: Vec<Pos>,
    ret_value: Value,
}

impl Interp
{
    pub fn new() -> Self
    { Interp { stack_trace: Vec::new(), ret_value: Value::None, } }
    
    pub fn stack_trace(&self) -> &[Pos]
    { self.stack_trace.as_slice() }

    pub fn clear_stack_trace(&mut self)
    { self.stack_trace.clear(); }
    
    pub fn interpret(&mut self, env: &mut Env, tree: &Tree) -> Result<()>
    { Ok(()) }

    pub fn apply_fun(&mut self, env: &mut Env, fun_value: &Value) -> Result<Value>
    { Ok(Value::None) }

    fn interpret_stat(&mut self, env: &mut Env, stat: &Stat) -> Result<()>
    {
        match stat {
            Stat::Expr(expr, _) => self.ret_value = self.interpret_expr(env, &**expr)?,
            Stat::Assign(expr, expr2, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match &**expr {
                    Expr::Var(name, _) => {
                        match env.set_var(name, value2) {
                            Ok(true) => (),
                            Ok(false) => {
                                self.stack_trace.push(pos.clone());
                                self.ret_value = Value::None;
                                return Err(Error::Interp(format!("undefined module for variable {}", name)));
                            },
                            Err(err) => {
                                self.stack_trace.push(pos.clone());
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                    Expr::BinOp(BinOp::Index, expr3, expr4, _) => {
                        let value = self.interpret_expr(env, expr3)?;
                        let idx_value = self.interpret_expr(env, expr4)?;
                        match value.set_elem(&idx_value, value2) {
                            Ok(()) => (),
                            Err(err) => {
                                self.stack_trace.push(pos.clone());
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                    Expr::Field(expr3, ident, _) => {
                        let value = self.interpret_expr(env, expr3)?;
                        match value.set_field(ident.clone(), value2) {
                            Ok(()) => (),
                            Err(err) => {
                                self.stack_trace.push(pos.clone());
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                    _ => {
                        self.stack_trace.push(expr.pos().clone());
                        self.ret_value = Value::None;
                        return Err(Error::Interp(String::from("expression isn't assignable")));
                    },
                }
                self.ret_value = Value::None;
            },
            Stat::If(expr, stats, else_if_pairs, else_stats, _) => {
                if self.interpret_expr(env, &**expr)?.to_bool() {
                    self.interpret_stats(env, stats.as_slice())?;
                } else {
                    let mut is_else_if = false;
                    for (else_if_expr, else_if_stats) in else_if_pairs {
                        if self.interpret_expr(env, &**else_if_expr)?.to_bool() {
                            self.interpret_stats(env, else_if_stats.as_slice())?;
                            is_else_if = true;
                            break;
                        }
                    }
                    if !is_else_if {
                        match else_stats {
                            Some(else_stats) => self.interpret_stats(env, else_stats.as_slice())?,
                            None => (),
                        }
                    }
                }
            },
            Stat::For(ident, expr, stats, pos) => {
                let value = self.interpret_expr(env, &**expr)?;
                match value.iter() {
                    Ok(Some(mut iter)) => {
                        loop {
                            match iter.next() {
                                Some(Ok(elem)) => {
                                    match env.set_var(&Name::Var(ident.clone()), elem.clone()) {
                                        Ok(true) => (),
                                        Ok(false) => {
                                            self.stack_trace.push(pos.clone());
                                            self.ret_value = Value::None;
                                            return Err(Error::Interp(format!("undefined module for variable {}", ident)));
                                        },
                                        Err(err) => {
                                            self.stack_trace.push(pos.clone());
                                            self.ret_value = Value::None;
                                            return Err(err);
                                        },
                                    }
                                },
                                Some(Err(err)) => {
                                    self.stack_trace.push(pos.clone());
                                    self.ret_value = Value::None;
                                    return Err(err);
                                },
                                None => break,
                            }
                            match self.interpret_stats(env, stats.as_slice()) {
                                Ok(()) => (),
                                Err(Error::Stop(Stop::Continue)) => continue,
                                Err(Error::Stop(Stop::Break)) => break,
                                Err(err) => return Err(err),                                
                            }
                        }
                    },
                    _ => (),
                }
            },
            Stat::While(expr, stats, _) => {
                while self.interpret_expr(env, &**expr)?.to_bool() {
                    match self.interpret_stats(env, stats.as_slice()) {
                        Ok(()) => (),
                        Err(Error::Stop(Stop::Continue)) => continue,
                        Err(Error::Stop(Stop::Break)) => break,
                        Err(err) => return Err(err),                                
                    }
                }
            },
            Stat::Break(pos) => {
                self.stack_trace.push(pos.clone());
                self.ret_value = Value::None;
                return Err(Error::Stop(Stop::Break));
            },
            Stat::Continue(pos) => {
                self.stack_trace.push(pos.clone());
                self.ret_value = Value::None;
                return Err(Error::Stop(Stop::Continue));
            },
            Stat::Return(expr, pos) => {
                match expr {
                    Some(expr) => self.ret_value = self.interpret_expr(env, &**expr)?,
                    None => self.ret_value = Value::None,
                }
                self.stack_trace.push(pos.clone());
                return Err(Error::Stop(Stop::Return));
            },
            Stat::Quit(pos) => {
                self.stack_trace.push(pos.clone());
                self.ret_value = Value::None;
                return Err(Error::Stop(Stop::Quit));
            },
        }
        Ok(())
    }

    fn interpret_stats(&mut self, env: &mut Env, stats: &[Box<Stat>]) -> Result<()>
    {
        self.ret_value = Value::None;
        for stat in stats {
            self.interpret_stat(env, &**stat)?;
        }
        Ok(())
    }
    
    fn interpret_expr(&mut self, env: &mut Env, expr: &Expr) -> Result<Value>
    { Ok(Value::None) }
}
