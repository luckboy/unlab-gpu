//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::RwLock;
use crate::env::*;
use crate::error::*;
use crate::private::*;
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
    { 
        match tree {
            Tree(nodes) => {
                let res = match self.interpret_nodes(env, nodes) {
                    Ok(()) => Ok(()),
                    Err(Error::Stop(Stop::Break)) => Err(Error::Interp(String::from("break isn't in loop"))),
                    Err(Error::Stop(Stop::Continue)) => Err(Error::Interp(String::from("continue isn't in loop"))),
                    Err(Error::Stop(Stop::Return)) => Err(Error::Interp(String::from("return or error propagation isn't in function"))),
                    Err(err) => Err(err),
                };
                match res {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        env.reset()?;
                        Err(err)
                    },
                }
            },
        }
    }

    pub fn apply_fun(&mut self, env: &mut Env, fun_value: &Value, arg_values: &[Value]) -> Result<Value>
    {
        match fun_value {
            Value::Object(fun_object) => {
                match &**fun_object {
                    Object::Fun(fun_mod_idents, _, fun) => {
                        match &**fun {
                            Fun(args, stats) => {
                                match env.push_fun_mod_and_local_vars(fun_mod_idents.as_slice(), args, arg_values) {
                                    Ok(true) => (),
                                    Ok(false) => return Err(Error::Interp(String::from("invalid number of arguments"))),
                                    Err(err) => return Err(err),
                                }
                                let res = match self.interpret_stats(env, stats.as_slice()) {
                                    Ok(()) => Ok(self.ret_value.clone()),
                                    Err(Error::Stop(Stop::Break)) => Err(Error::Interp(String::from("break isn't in loop"))),
                                    Err(Error::Stop(Stop::Continue)) => Err(Error::Interp(String::from("continue isn't in loop"))),
                                    Err(Error::Stop(Stop::Return)) => {
                                        self.stack_trace.clear();
                                        Ok(self.ret_value.clone())
                                    },
                                    Err(err) => Err(err),
                                };
                                env.pop_fun_mod_and_local_vars();
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

    fn interpret_node(&mut self, env: &mut Env, node: &Node) -> Result<()>
    {
        match node {
            Node::Def(def) => self.interpret_def(env, &**def),
            Node::Stat(stat) => self.interpret_stat(env, &**stat),
        }
    }

    fn interpret_nodes(&mut self, env: &mut Env, nodes: &[Node]) -> Result<()>
    {
        self.ret_value = Value::None;
        for node in nodes {
            let res = self.interpret_node(env, node);
            self.ret_value = Value::None;
            res?;
        }
        Ok(())
    }
    
    fn interpret_def(&mut self, env: &mut Env, def: &Def) -> Result<()>
    {
        match def {
            Def::Mod(ident, mod1, pos) => {
                match &**mod1 {
                    Mod(nodes) => {
                        match env.add_and_push_mod(ident.clone()) {
                            Ok(true) => {
                                self.interpret_nodes(env, nodes.as_slice())?;
                                match env.pop_mod() {
                                    Ok(true) => (),
                                    Ok(false) => {
                                        self.stack_trace.push((None, pos.clone()));
                                        self.ret_value = Value::None;
                                        return Err(Error::Interp(format!("can't pop module {}", ident)));
                                    },
                                    Err(err) => {
                                        self.stack_trace.push((None, pos.clone()));
                                        self.ret_value = Value::None;
                                        return Err(err);
                                    },
                                }
                            },
                            Ok(false) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(Error::Interp(format!("already defined module {}", ident)));
                            },
                            Err(err) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                }
            },
            Def::Fun(ident, fun, pos) => {
                match &**fun {
                    Fun(args, _) => {
                        let mut idents: BTreeSet<&String> = BTreeSet::new();
                        for arg in args {
                            match arg {
                                Arg(ident, pos2) => {
                                    if idents.contains(&ident) {
                                        self.stack_trace.push((None, pos2.clone()));
                                        self.ret_value = Value::None;
                                        return Err(Error::Interp(format!("already defined argument {}", ident)));
                                    }
                                    idents.insert(ident);
                                },
                            }
                        }
                        match env.add_fun(ident.clone(), fun.clone()) {
                            Ok(true) => (),
                            Ok(false) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(Error::Interp(format!("already variable {} is set", ident)));
                            },
                            Err(err) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(err);
                            },
                        }
                    },
                }
            },
        }
        Ok(())
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
                    arg_values.push(self.interpret_expr(env, &**expr3)?);
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

    fn interpret_matrix_row(&mut self, env: &mut Env, matrix_row: &MatrixRow) -> Result<Vec<f32>>
    {
        match matrix_row {
            MatrixRow::Row(exprs) => {
                let mut xs: Vec<f32> = Vec::new();
                for expr in exprs {
                    let value = self.interpret_expr(env, &**expr)?;
                    match value.to_opt_f32() {
                        Some(x) => xs.push(x),
                        None => {
                            self.stack_trace.push((None, expr.pos().clone()));
                            self.ret_value = Value::None;
                            return Err(Error::Interp(String::from("can't convert value to floating-point number")));
                        },
                    }
                }
                Ok(xs)
            },
            MatrixRow::FilledRow(expr, expr2) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match value2.to_opt_i64() {
                    Some(n) => {
                        let mut xs: Vec<f32> = Vec::new();
                        for _ in 0..n {
                            let value = self.interpret_expr(env, &**expr)?;
                            match value.to_opt_f32() {
                                Some(x) => xs.push(x),
                                None => {
                                    self.stack_trace.push((None, expr.pos().clone()));
                                    self.ret_value = Value::None;
                                    return Err(Error::Interp(String::from("can't convert value to floating-point number")));
                                },
                            }
                        }
                        Ok(xs)
                    },
                    None => {
                        self.stack_trace.push((None, expr2.pos().clone()));
                        self.ret_value = Value::None;
                        Err(Error::Interp(String::from("can't convert value to integer")))
                    },
                }
            },
        }
    }

    fn interpret_lit(&mut self, env: &mut Env, lit: &Lit, pos: &Pos) -> Result<Value>
    {
        match lit {
            Lit::None => Ok(Value::None),
            Lit::Bool(b) => Ok(Value::Bool(*b)),
            Lit::Int(n) => Ok(Value::Int(*n)),
            Lit::Float(n) => Ok(Value::Float(*n)),
            Lit::String(s) => Ok(Value::Object(Arc::new(Object::String(s.clone())))),
            Lit::Matrix(matrix_rows) => {
                let mut xs: Vec<f32> = Vec::new();
                let mut row_count = 0usize;
                let mut col_count: Option<usize> = None;
                for matrix_row in matrix_rows {
                    let ys = self.interpret_matrix_row(env, matrix_row)?;
                    if col_count.map(|n| n == ys.len()).unwrap_or(true) {
                        xs.extend_from_slice(ys.as_slice());
                        col_count = Some(ys.len());
                    } else {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        return Err(Error::Interp(String::from("numbers of columns of matrix rows aren't equal")));
                    }
                    match row_count.checked_add(1) {
                        Some(new_row_count) => row_count = new_row_count,
                        None => {
                            self.stack_trace.push((None, pos.clone()));
                            self.ret_value = Value::None;
                            return Err(Error::Interp(String::from("too many matrix rows")));
                        },
                    }
                }
                match matrix_create_and_set_elems(row_count, col_count.unwrap_or(0), xs.as_slice()) {
                    Ok(a) => Ok(Value::Object(Arc::new(Object::Matrix(a)))),
                    Err(err) => {
                        self.stack_trace.push((None, pos.clone()));
                        self.ret_value = Value::None;
                        Err(err)
                    },
                }
            },
            Lit::FilledMatrix(matrix_row, expr) => {
                let value = self.interpret_expr(env, &**expr)?;
                match value.to_opt_i64() {
                    Some(n) => {
                        let mut  xs: Vec<f32> = Vec::new();
                        let mut row_count = 0usize;
                        let mut col_count: Option<usize> = None;
                        for _ in 0..n {
                            let ys = self.interpret_matrix_row(env, matrix_row)?;
                            if col_count.map(|n| n == ys.len()).unwrap_or(true) {
                                xs.extend_from_slice(ys.as_slice());
                                col_count = Some(ys.len());
                            } else {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                return Err(Error::Interp(String::from("numbers of columns of matrix rows aren't equal")));
                            }
                            match row_count.checked_add(1) {
                                Some(new_row_count) => row_count = new_row_count,
                                None => {
                                    self.stack_trace.push((None, pos.clone()));
                                    self.ret_value = Value::None;
                                    return Err(Error::Interp(String::from("too many matrix rows")));
                                },
                            }
                        }
                        match matrix_create_and_set_elems(row_count, col_count.unwrap_or(0), xs.as_slice()) {
                            Ok(a) => Ok(Value::Object(Arc::new(Object::Matrix(a)))),
                            Err(err) => {
                                self.stack_trace.push((None, pos.clone()));
                                self.ret_value = Value::None;
                                Err(err)
                            },
                        }
                    },
                    None => {
                        self.stack_trace.push((None, expr.pos().clone()));
                        self.ret_value = Value::None;
                        Err(Error::Interp(String::from("can't convert value to integer")))
                    },
                }
            },
            Lit::Array(exprs) => {
                let mut elems: Vec<Value> = Vec::new();
                for expr in exprs {
                    elems.push(self.interpret_expr(env, &**expr)?);
                }
                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems)))))
            },
            Lit::FilledArray(expr, expr2) => {
                let value2 = self.interpret_expr(env, &**expr2)?;
                match value2.to_opt_i64() {
                    Some(n) => {
                        let mut elems: Vec<Value> = Vec::new();
                        for _ in 0..n {
                            elems.push(self.interpret_expr(env, &**expr)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems)))))
                    },
                    None => {
                        self.stack_trace.push((None, expr2.pos().clone()));
                        self.ret_value = Value::None;
                        Err(Error::Interp(String::from("can't convert value to integer")))
                    },
                }
            },
            Lit::Struct(field_pairs) => {
                let mut fields: BTreeMap<String, Value> = BTreeMap::new();
                for field_pair in field_pairs {
                    match field_pair {
                        FieldPair(ident, expr, pos2) => {
                            if fields.contains_key(ident) {
                                self.stack_trace.push((None, pos2.clone()));
                                self.ret_value = Value::None;
                                return Err(Error::Interp(format!("already defined field {}", ident)));
                            }
                            fields.insert(ident.clone(), self.interpret_expr(env, &**expr)?);
                        },
                    }
                }
                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields)))))
            },
        }
    }
}
