//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::env::*;
use crate::error::*;
use crate::tree::*;
use crate::value::*;

#[derive(Clone, Debug)]
pub struct Interp
{
    stack_trace: Vec<(Option<Value>, Pos)>,
    ret_value: Value,
}

impl Interp
{
    pub fn new() -> Self
    { Interp { stack_trace: Vec::new(), ret_value: Value::None, } }
    
    pub fn stack_trace(&self) -> &[(Option<Value>, Pos)]
    { self.stack_trace.as_slice() }

    pub fn clear_stack_trace(&mut self)
    { self.stack_trace.clear(); }
    
    pub fn interpret(&mut self, env: &mut Env, tree: &Tree) -> Result<()>
    { Ok(()) }

    pub fn apply_fun(&mut self, env: &mut Env, fun_value: &Value, arg_values: &[Value]) -> Result<Value>
    {
        match fun_value {
            Value::Object(fun_object) => {
                match &**fun_object {
                    Object::Fun(_, _, fun) => {
                        match &**fun {
                            Fun(args, stats) => {
                                env.push_local_vars(args, arg_values);
                                let res = match self.interpret_stats(env, stats.as_slice()) {
                                    Ok(()) => Ok(self.ret_value.clone()),
                                    Err(Error::Stop(Stop::Break)) => Err(Error::Interp(String::from("break isn't in loop"))),
                                    Err(Error::Stop(Stop::Continue)) => Err(Error::Interp(String::from("continue isn't in loop"))),
                                    Err(Error::Stop(Stop::Break)) => {
                                        self.stack_trace.clear();
                                        Ok(self.ret_value.clone())
                                    },
                                    Err(err) => Err(err),
                                };
                                env.pop_local_vars();
                                match res {
                                    Ok(value) => Ok(value),
                                    Err(err) => {
                                        match self.stack_trace.pop() {
                                            Some((_, pos)) => {
                                                self.stack_trace.push((Some(fun_value.clone()), pos));
                                            },
                                            None => (),
                                        }
                                        Err(err)
                                    },
                                }
                            },
                        }
                    },
                    Object::BuiltinFun(_, f) => f(self, env, arg_values),
                    _ => {
                        self.ret_value = Value::None;
                        Err(Error::Interp(format!("value isn't function")))
                    },
                }
            },
            _ => {
                self.ret_value = Value::None;
                Err(Error::Interp(format!("value isn't function")))
            },
        }
    }

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
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(Error::Interp(format!("undefined module for variable {}", name)));
                            },
                            Err(err) => {
                                self.stack_trace.push((None, pos.clone()));
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
                                self.stack_trace.push((None, pos.clone()));
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
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                    _ => {
                        self.stack_trace.push((None, expr.pos().clone()));
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
                                            self.stack_trace.push((None, pos.clone()));
                                            self.ret_value = Value::None;
                                            return Err(Error::Interp(format!("undefined module for variable {}", ident)));
                                        },
                                        Err(err) => {
                                            self.stack_trace.push((None, pos.clone()));
                                            self.ret_value = Value::None;
                                            return Err(err);
                                        },
                                    }
                                },
                                Some(Err(err)) => {
                                    self.stack_trace.push((None, pos.clone()));
                                    self.ret_value = Value::None;
                                    return Err(err);
                                },
                                None => break,
                            }
                            match self.interpret_stats(env, stats.as_slice()) {
                                Ok(()) => (),
                                Err(Error::Stop(Stop::Break)) => {
                                    self.stack_trace.clear();
                                    break;
                                },
                                Err(Error::Stop(Stop::Continue)) => {
                                    self.stack_trace.clear();
                                    continue;
                                },
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
                        Err(Error::Stop(Stop::Break)) => {
                            self.stack_trace.clear();
                            break;
                        },
                        Err(Error::Stop(Stop::Continue)) => {
                            self.stack_trace.clear();
                            continue;
                        },
                        Err(err) => return Err(err),                                
                    }
                }
            },
            Stat::Break(pos) => {
                self.stack_trace.push((None, pos.clone()));
                self.ret_value = Value::None;
                return Err(Error::Stop(Stop::Break));
            },
            Stat::Continue(pos) => {
                self.stack_trace.push((None, pos.clone()));
                self.ret_value = Value::None;
                return Err(Error::Stop(Stop::Continue));
            },
            Stat::Return(expr, pos) => {
                match expr {
                    Some(expr) => self.ret_value = self.interpret_expr(env, &**expr)?,
                    None => self.ret_value = Value::None,
                }
                self.stack_trace.push((None, pos.clone()));
                return Err(Error::Stop(Stop::Return));
            },
            Stat::Quit(pos) => {
                self.stack_trace.push((None, pos.clone()));
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
    {
        match expr {
            Expr::Lit(lit, pos) => self.interpret_lit(env, lit, pos),
            Expr::Var(name, pos) => {
                match env.var(name) {
                    Ok(Some(value)) => Ok(value),
                    Ok(None) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(Error::Interp(format!("variable {} isn't set", name)))
                    },
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Expr::App(expr2, exprs, pos) => {
                let fun_value = self.interpret_expr(env, &**expr2)?;
                let mut arg_values: Vec<Value> = Vec::new();
                for expr3 in exprs {
                    arg_values.push(self.interpret_expr(env, &**expr2)?);
                }
                match self.apply_fun(env, &fun_value, arg_values.as_slice()) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Expr::UnaryOp(op, expr2, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match value2.unary_op(*op) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Expr::BinOp(op, expr2, expr3, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                let value3 = self.interpret_expr(env, &**expr3)?;
                match value2.bin_op(*op, &value3) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Expr::And(expr2, expr3, _) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                if value2.to_bool() {
                    self.interpret_expr(env, &**expr3)
                } else {
                    Ok(value2)
                }
            },
            Expr::Or(expr2, expr3, _) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                if value2.to_bool() {
                    Ok(value2)
                } else {
                    self.interpret_expr(env, &**expr3)
                }
            },
            Expr::Field(expr2, ident, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match value2.field(ident) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Expr::Range(expr2, expr3, expr4, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                let value3 = self.interpret_expr(env, &**expr3)?;
                let value4 = match expr4 {
                    Some(expr4) => Some(self.interpret_expr(env, &**expr4)?),
                    None => None,
                };
                match (&value2, &value3, &value4) {
                    (Value::Int(a), Value::Int(b), None) => Ok(Value::Object(Arc::new(Object::IntRange(*a, *b, 1)))),
                    (Value::Int(a), Value::Int(b), Some(Value::Int(c))) => Ok(Value::Object(Arc::new(Object::IntRange(*a, *b, *c)))),
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_), None) => Ok(Value::Object(Arc::new(Object::FloatRange(value2.to_f32(), value3.to_f32(), 1.0)))),
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_), Some(value4 @ (Value::Int(_) | Value::Float(_)))) => Ok(Value::Object(Arc::new(Object::FloatRange(value2.to_f32(), value3.to_f32(), value4.to_f32())))),
                    (_, _, _) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(Error::Interp(String::from("unsupported types for range creation")))
                    },
                }
            },
            Expr::PropagateError(expr2, pos) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match &value2 {
                    Value::None => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = value2.clone();
                        Err(Error::Stop(Stop::Return))
                    },
                    Value::Object(object2) => {
                        match &**object2 {
                            Object::Error(_, _) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = value2.clone();
                                Err(Error::Stop(Stop::Return))
                            },
                            _ => Ok(value2),
                        }
                    },
                    _ => Ok(value2),
                }
            },
        }
    }

    fn interpret_lit(&mut self, env: &mut Env, lit: &Lit, pos: &Pos) -> Result<Value>
    { Ok(Value::None) }
}
