//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::mem::size_of;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::matrix::Matrix;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::mod_node::*;
use crate::utils::*;
use crate::value::*;

fn fun1<F>(arg_values: &[Value], f: F) -> Result<Value>
    where F: FnOnce(&Value) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value) => f(value),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

fn fun2<F>(arg_values: &[Value], f: F) -> Result<Value>
    where F: FnOnce(&Value, &Value) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(value), Some(value2)) => f(value, value2),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

fn fun1_for_f32<F>(arg_values: &[Value], err_msg: &str, f: F) -> Result<Value>
    where F: FnOnce(f32) -> f32
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value @ (Value::Int(_) | Value::Float(_))) => Ok(Value::Float(f(value.to_f32()))),
        Some(_) => Err(Error::Interp(String::from(err_msg))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

fn fun2_for_f32<F>(arg_values: &[Value], err_msg: &str, f: F) -> Result<Value>
    where F: FnOnce(f32, f32) -> f32
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(value @ (Value::Int(_) | Value::Float(_))), Some(value2 @ (Value::Int(_) | Value::Float(_)))) => Ok(Value::Float(f(value.to_f32(), value2.to_f32()))),
        (Some(_), Some(_)) => Err(Error::Interp(String::from(err_msg))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

fn fun1_for_f32_and_matrix_with_fun_refs<F, G>(arg_values: &[Value], err_msg: &str, f: &mut F, g: &mut G) -> Result<Value>
    where F: FnMut(f32) -> f32,
        G: FnMut(&Matrix) -> Result<Matrix>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value @ (Value::Int(_) | Value::Float(_))) => Ok(Value::Float(f(value.to_f32()))),
        Some(Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(g(a)?)))),
                _ => Err(Error::Interp(String::from(err_msg))),
            }
        },
        Some(value) => value.dot1(err_msg, |a| fun1_for_f32_and_matrix_with_fun_refs(&[a.clone()], err_msg, f, g)),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

fn fun1_for_f32_and_matrix<F, G>(arg_values: &[Value], err_msg: &str, mut f: F, mut g: G) -> Result<Value>
    where F: FnMut(f32) -> f32,
        G: FnMut(&Matrix) -> Result<Matrix>
{ fun1_for_f32_and_matrix_with_fun_refs(arg_values, err_msg, &mut f, &mut g) }

pub fn typ(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::None) => Ok(Value::Object(Arc::new(Object::String(String::from("none"))))),
        Some(Value::Bool(_)) => Ok(Value::Object(Arc::new(Object::String(String::from("bool"))))),
        Some(Value::Int(_)) => Ok(Value::Object(Arc::new(Object::String(String::from("int"))))),
        Some(Value::Float(_)) => Ok(Value::Object(Arc::new(Object::String(String::from("float"))))),
        Some(Value::Object(object)) => {
            match &**object {
                Object::String(_) => Ok(Value::Object(Arc::new(Object::String(String::from("string"))))),
                Object::IntRange(_, _, _) => Ok(Value::Object(Arc::new(Object::String(String::from("intrange"))))),
                Object::FloatRange(_, _, _) => Ok(Value::Object(Arc::new(Object::String(String::from("floatrange"))))),
                Object::Matrix(_) => Ok(Value::Object(Arc::new(Object::String(String::from("matrix"))))),
                Object::Fun(_, _, _) | Object::BuiltinFun(_, _) => Ok(Value::Object(Arc::new(Object::String(String::from("function"))))),
                Object::MatrixArray(_, _, _, _) => Ok(Value::Object(Arc::new(Object::String(String::from("matrixarray"))))),
                Object::MatrixRowSlice(_, _) => Ok(Value::Object(Arc::new(Object::String(String::from("matrixrowslice"))))),
                Object::Error(_, _) => Ok(Value::Object(Arc::new(Object::String(String::from("error"))))),
            }
        },
        Some(Value::Ref(object)) => {
            let object_g = rw_lock_read(object)?;
            match &*object_g {
                MutObject::Array(_) => Ok(Value::Object(Arc::new(Object::String(String::from("array"))))),
                MutObject::Struct(_) => Ok(Value::Object(Arc::new(Object::String(String::from("struct"))))),
            }
        },
        Some(Value::Weak(_)) => Ok(Value::Object(Arc::new(Object::String(String::from("weak"))))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn boolean(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1(arg_values, |a| Ok(Value::Bool(a.to_bool()))) }

pub fn int(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1(arg_values, |a| Ok(Value::Int(a.to_i64()))) }

pub fn float(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1(arg_values, |a| Ok(Value::Float(a.to_f32()))) }

pub fn string(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1(arg_values, |a| Ok(Value::Object(Arc::new(Object::String(format!("{}", a)))))) }

fn checked_mul_row_count_and_col_count(row_count: i64, col_count: i64) -> Result<usize>
{
    if row_count < 0 {
        return Err(Error::Interp(String::from("number of rows is negative")));
    }
    if col_count < 0 {
        return Err(Error::Interp(String::from("number of columns is negative")));
    }
    if row_count > (isize::MAX as i64) {
        return Err(Error::Interp(String::from("too large number of rows")));
    }
    if col_count > (isize::MAX as i64) {
        return Err(Error::Interp(String::from("too large number of columns")));
    }
    match row_count.checked_mul(col_count) {
        Some(len) => {
            if len > (isize::MAX as i64) {
                return Err(Error::Interp(String::from("too large number of matrix elements")));
            }
            match (len as isize).checked_mul(size_of::<f32>() as isize) {
                Some(_) => Ok(len as usize),
                None => Err(Error::Interp(String::from("too large number of matrix elements"))),
            }
        },
        None => Err(Error::Interp(String::from("too large number of matrix elements"))),
    }
}

pub fn zeros(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(n_value @ (Value::Int(_) | Value::Float(_))), Some(m_value @ (Value::Int(_) | Value::Float(_)))) => {
            let n = n_value.to_i64();
            let m = m_value.to_i64();
            match checked_mul_row_count_and_col_count(n, m) {
                Ok(_) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_zeros(n as usize, m as usize)?)))),
                Err(err) => Err(err), 
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function zeros"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn ones(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(n_value @ (Value::Int(_) | Value::Float(_))), Some(m_value @ (Value::Int(_) | Value::Float(_)))) => {
            let n = n_value.to_i64();
            let m = m_value.to_i64();
            match checked_mul_row_count_and_col_count(n, m) {
                Ok(len) => {
                    let xs = vec![1.0f32; len];
                    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(n as usize, m as usize, xs.as_slice())?))))
                },
                Err(err) => Err(err), 
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function ones"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn eye(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(n_value @ (Value::Int(_) | Value::Float(_))) => {
            let n = n_value.to_i64();
            match checked_mul_row_count_and_col_count(n, n) {
                Ok(len) => {
                    let mut xs = vec![0.0f32; len];
                    for i in 0..(n as usize) {
                        xs[i * (n as usize) + i] = 1.0;
                    }
                    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(n as usize, n as usize, xs.as_slice())?))))
                },
                Err(err) => Err(err), 
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function eye"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn init(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 4 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2), arg_values.get(3)) {
        (Some(n_value @ (Value::Int(_) | Value::Float(_))), Some(m_value @ (Value::Int(_) | Value::Float(_))), Some(data_value), Some(fun_value)) => {
            let n = n_value.to_i64();
            let m = m_value.to_i64();
            match checked_mul_row_count_and_col_count(n, m) {
                Ok(len) => {
                    let mut xs = vec![0.0f32; len];
                    for i in 0..(n as usize) {
                        for j in 0..(m as usize) {
                            match fun_value.apply(interp, env, &[data_value.clone(), Value::Int(i as i64), Value::Int(j as i64)])?.to_opt_f32() {
                                Some(x) => xs[i * (m as usize) + j] = x,
                                None => return Err(Error::Interp(String::from("can't convert value to floating-point number"))),
                            }
                        }
                    }
                    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(n as usize, m as usize, xs.as_slice())?))))
                },
                Err(err) => Err(err), 
            }
        },
        (Some(_), Some(_), Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function init"))),
        (_, _, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn initdiag(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(n_value @ (Value::Int(_) | Value::Float(_))), Some(data_value), Some(fun_value)) => {
            let n = n_value.to_i64();
            match checked_mul_row_count_and_col_count(n, n) {
                Ok(len) => {
                    let mut xs = vec![0.0f32; len];
                    for i in 0..(n as usize) {
                        match fun_value.apply(interp, env, &[data_value.clone(), Value::Int(i as i64)])?.to_opt_f32() {
                            Some(x) => xs[i * (n as usize) + i] = x,
                            None => return Err(Error::Interp(String::from("can't convert value to floating-point number"))),
                        }
                    }
                    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(n as usize, n as usize, xs.as_slice())?))))
                },
                Err(err) => Err(err), 
            }
        },
        (Some(_), Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function init"))),
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

fn to_row_or_column(value: &Value) -> Result<Vec<f32>>
{
    match value.iter()? {
        Some(mut iter) => {
            let mut xs: Vec<f32> = Vec::new();
            loop {
                match iter.next() {
                    Some(Ok(elem)) => {
                        match elem.to_opt_f32() {
                            Some(x) => xs.push(x),
                            None => return Err(Error::Interp(String::from("can't convert value to floating-point number"))),
                        }
                    },
                    Some(Err(err)) => return Err(err),
                    None => break,
                }
            }
            Ok(xs)
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

pub fn matrix(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let value = match arg_values.get(0) {
        Some(tmp_value @ Value::Object(object)) => {
            match &**object {
                Object::Matrix(_) => return Ok(tmp_value.clone()),
                _ => tmp_value,
            }
        },
        Some(tmp_value) => tmp_value,
        None => return Err(Error::Interp(String::from("no argument"))),
    };
    match value.iter()? {
        Some(mut iter) => {
            let mut xs: Vec<f32> = Vec::new();
            let mut row_count = 0usize;
            let mut col_count: Option<usize> = None;
            loop {
                match iter.next() {
                    Some(Ok(row_value)) => {
                        let ys = to_row_or_column(&row_value)?;
                        if col_count.map(|n| n == ys.len()).unwrap_or(true) {
                            xs.extend_from_slice(ys.as_slice());
                            col_count = Some(ys.len());
                        } else {
                            return Err(Error::Interp(String::from("numbers of columns of matrix rows aren't equal")));
                        }
                        match row_count.checked_add(1) {
                            Some(new_row_count) => row_count = new_row_count,
                            None => return Err(Error::Interp(String::from("too many matrix rows"))),
                        }
                    },
                    Some(Err(err)) => return Err(err),
                    None => break,
                }
            }
            Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(row_count, col_count.unwrap_or(0), xs.as_slice())?))))
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

pub fn rowvector(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let value = match arg_values.get(0) {
        Some(tmp_value @ Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => {
                    if a.row_count() == 1 {
                        return Ok(tmp_value.clone());
                    } else {
                        return Err(Error::Interp(String::from("number of rows isn't one")));
                    }
                },
                _ => tmp_value,
            }
        },
        Some(tmp_value) => tmp_value,
        None => return Err(Error::Interp(String::from("no argument"))),
    };
    let xs = to_row_or_column(&value)?;
    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(1, xs.len(), xs.as_slice())?))))
}

pub fn colvector(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let value = match arg_values.get(0) {
        Some(tmp_value @ Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => {
                    if a.col_count() == 1 {
                        return Ok(tmp_value.clone());
                    } else {
                        return Err(Error::Interp(String::from("number of columns isn't one")));
                    }
                },
                _ => tmp_value,
            }
        },
        Some(tmp_value) => tmp_value,
        None => return Err(Error::Interp(String::from("no argument"))),
    };
    let xs = to_row_or_column(&value)?;
    Ok(Value::Object(Arc::new(Object::Matrix(matrix_create_and_set_elems(xs.len(), 1, xs.as_slice())?))))
}

pub fn matrixarray(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1(arg_values, Value::to_matrix_array) }

pub fn error(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Object(kind_object)), Some(Value::Object(msg_object))) => {
            match (&**kind_object, &**msg_object) {
                (Object::String(kind), Object::String(msg)) => Ok(Value::Object(Arc::new(Object::Error(kind.clone(), msg.clone())))),
                (_, _) => Err(Error::Interp(String::from("unsupported types for function error"))),
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function error"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn array(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let value = match arg_values.get(0) {
        Some(tmp_value @ Value::Ref(object)) => {
            let object_g = rw_lock_read(object)?;
            match &*object_g {
                MutObject::Array(_) => return Ok(tmp_value.clone()),
                _ => tmp_value,
            }
        },
        Some(tmp_value) => tmp_value,
        None => return Err(Error::Interp(String::from("no argument"))),
    };
    match value.iter()? {
        Some(mut iter) => {
            let mut elems: Vec<Value> = Vec::new();
            loop {
                match iter.next() {
                    Some(Ok(elem)) => elems.push(elem),
                    Some(Err(err)) => return Err(err),
                    None => break,
                }
            }
            Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems)))))
        },
        None => Err(Error::Interp(String::from("value isn't iterable"))),
    }
}

pub fn strong(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value @ Value::Ref(_)) => Ok(value.clone()),
        Some(Value::Weak(object)) => {
            match object.upgrade() {
                Some(object) => Ok(Value::Ref(object)),
                None => Ok(Value::None),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported types for function strong"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn weak(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() > 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Ref(object)) => Ok(Value::Weak(Arc::downgrade(object))),
        Some(value @ Value::Weak(_)) => Ok(value.clone()),
        Some(_) => Err(Error::Interp(String::from("unsupported types for function weak"))),
        None => Ok(Value::Weak(Weak::new())),
    }
}

pub fn length(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::String(s) => Ok(Value::Int(s.chars().count() as i64)),
                Object::MatrixArray(row_count, _, _, _) => Ok(Value::Int(*row_count as i64)),
                Object::MatrixRowSlice(matrix_array, _) => {
                    match &**matrix_array {
                        Object::MatrixArray(_, col_count, _, _) => Ok(Value::Int(*col_count as i64)),
                        _ => Err(Error::Interp(String::from("invalid matrix array type"))),
                    }
                },
                _ => Err(Error::Interp(String::from("unsupported types for function length"))),
            }
        },
        Some(Value::Ref(object)) => {
            let object_g = rw_lock_read(object)?;
            match &*object_g {
                MutObject::Array(elems) => Ok(Value::Int(elems.len() as i64)),
                _ => Err(Error::Interp(String::from("unsupported types for function length"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported types for function length"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn rows(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => Ok(Value::Int(a.row_count() as i64)),
                Object::MatrixArray(row_count, _, _, _) => Ok(Value::Int(*row_count as i64)),
                _ => Err(Error::Interp(String::from("unsupported types for function rows"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported types for function rows"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn columns(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => Ok(Value::Int(a.col_count() as i64)),
                Object::MatrixArray(_, col_count, _, _) => Ok(Value::Int(*col_count as i64)),
                Object::MatrixRowSlice(matrix_array, _) => {
                    match &**matrix_array {
                        Object::MatrixArray(_, col_count, _, _) => Ok(Value::Int(*col_count as i64)),
                        _ => Err(Error::Interp(String::from("invalid matrix array type"))),
                    }
                },
                _ => Err(Error::Interp(String::from("unsupported types for function columns"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported types for function columns"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn get(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() < 2 || arg_values.len() > 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(Value::Object(object)), Some(i_value @ (Value::Int(_) | Value::Float(_))), None)  => {
            match &**object {
                Object::String(s) => {
                    let i = i_value.to_i64();
                    if i < 1 || i > (s.chars().count() as i64) {
                        return Ok(Value::None);
                    }
                    match s.chars().nth((i - 1) as usize) {
                        Some(c) => {
                            let mut t = String::new();
                            t.push(c);
                            Ok(Value::Object(Arc::new(Object::String(t))))
                        }
                        None => Ok(Value::None),
                    }
                },
                Object::MatrixArray(row_count, _, _, _) => {
                    let i = i_value.to_i64();
                    if i < 1 || i > (*row_count as i64) {
                        return Ok(Value::None);
                    }
                    Ok(Value::Object(Arc::new(Object::MatrixRowSlice(object.clone(), (i - 1) as usize))))
                },
                Object::MatrixRowSlice(matrix_array, i) => {
                    let j = i_value.to_i64();
                    match &**matrix_array {
                        Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                            if j < 1 || j > (*col_count as i64) {
                                return Ok(Value::None);
                            }
                            let k = match transpose_flag {
                                TransposeFlag::NoTranspose => i * (*col_count) + ((j - 1) as usize),
                                TransposeFlag::Transpose => ((j - 1) as usize) * (*row_count) + i,
                            };
                            Ok(xs.get(k).map(|x| Value::Float(*x)).unwrap_or(Value::None))
                        },
                        _ => Err(Error::Interp(String::from("invalid matrix array type"))),
                    }
                },
                _ => Err(Error::Interp(String::from("unsupported type for function get"))),
            }
        },
        (Some(Value::Ref(object)), Some(i_value @ (Value::Int(_) | Value::Float(_))), None)  => {
            let object_g = rw_lock_read(&**object)?;
            match &*object_g {
                MutObject::Array(elems) => {
                    match i_value {
                        Value::Int(_) | Value::Float(_) => {
                            let i = i_value.to_i64();
                            if i < 1 || i > (elems.len() as i64) {
                                return Ok(Value::None);
                            }
                            Ok(elems.get((i - 1) as usize).map(|x| x.clone()).unwrap_or(Value::None))
                        },
                        _ => Err(Error::Interp(String::from("unsupported types for function get"))),
                    }
                },
                MutObject::Struct(fields) => {
                    match i_value {
                        Value::Object(i_object) => {
                            match &**i_object {
                                Object::String(ident) => Ok(fields.get(ident).map(|x| x.clone()).unwrap_or(Value::None)),
                                _ => Err(Error::Interp(String::from("unsupported types for function get"))),
                            }
                        },
                        _ => Err(Error::Interp(String::from("unsupported types for function get"))),
                    }
                },
            }
        },
        (Some(Value::Object(object)), Some(i_value @ (Value::Int(_) | Value::Float(_))), Some(j_value @ (Value::Int(_) | Value::Float(_))))  => {
            match &**object {
                Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                    let i = i_value.to_i64();
                    let j = j_value.to_i64();
                    if i < 1 || i > (*row_count as i64) {
                        return Ok(Value::None);
                    }
                    if j < 1 || j > (*col_count as i64) {
                        return Ok(Value::None);
                    }
                    let k = match transpose_flag {
                        TransposeFlag::NoTranspose => ((i - 1) as usize) * (*col_count) + ((j - 1) as usize),
                        TransposeFlag::Transpose => ((j - 1) as usize) * (*row_count) + ((i - 1) as usize),
                    };
                    Ok(xs.get(k).map(|x| Value::Float(*x)).unwrap_or(Value::None))
                },
                _ => Err(Error::Interp(String::from("unsupported type for function get"))),
            }
        },
        (Some(_), Some(_), _)  => Err(Error::Interp(String::from("unsupported types for function get"))),
        (_, _, _) => Err(Error::Interp(String::from("no argument")))
    }
}

pub fn getdiag(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Object(object)), Some(i_value @ (Value::Int(_) | Value::Float(_))))  => {
            match &**object {
                Object::MatrixArray(row_count, col_count, _, xs) => {
                    if *row_count != *col_count {
                        return Err(Error::Interp(String::from("number of rows isn't equal to number of columns")));
                    }
                    let i = i_value.to_i64();
                    if i < 1 || i > (*row_count as i64) {
                        return Ok(Value::None);
                    }
                    let k = ((i - 1) as usize) * (*col_count) + ((i - 1) as usize);
                    Ok(xs.get(k).map(|x| Value::Float(*x)).unwrap_or(Value::None))
                },
                _ => Err(Error::Interp(String::from("unsupported type for function getdiag"))),
            }
        },
        (Some(_), Some(_))  => Err(Error::Interp(String::from("unsupported types for function getdiag"))),
        (_, _) => Err(Error::Interp(String::from("no argument")))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SortType
{
    Bool,
    Number,
    String,
    Incomparable,
}

pub fn sort(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Ref(object)) => {
            let mut object_g = rw_lock_write(object)?;
            match &mut *object_g {
                MutObject::Array(elems) => {
                    let mut sort_type: Option<SortType> = None;
                    for elem in &*elems {
                        let new_sort_type = match elem {
                            Value::Bool(_) => SortType::Bool,
                            Value::Int(_) => SortType::Number,
                            Value::Float(n) => if !n.is_nan() { SortType::Number } else { SortType::Incomparable },
                            Value::Object(elem_object) => {
                                match &**elem_object {
                                    Object::String(_) => SortType::String,
                                    _ => SortType::Incomparable,
                                }
                            },
                            _ => SortType::Incomparable,
                       };
                       if sort_type.map(|t| t == new_sort_type).unwrap_or(true) {
                           sort_type = Some(new_sort_type);
                       } else {
                           return Err(Error::Interp(String::from("array has elements which can't be compared")));
                       }
                    }
                    match sort_type {
                        Some(SortType::Incomparable) => return Err(Error::Interp(String::from("array has incomparable elements"))),
                        _ => (),
                    }
                    elems.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    Ok(Value::None)
                },
                _ => Err(Error::Interp(String::from("unsupported type for function sort"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function sort"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn reverse(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Ref(object)) => {
            let mut object_g = rw_lock_write(object)?;
            match &mut *object_g {
                MutObject::Array(elems) => {
                    elems.reverse();
                    Ok(Value::None)
                },
                _ => Err(Error::Interp(String::from("unsupported type for function reverse"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function reverse"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn any(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(a_value), Some(data_value), Some(fun_value)) => {
            match a_value.iter()? {
                Some(mut iter) => {
                    loop {
                        match iter.next() {
                            Some(Ok(elem)) => {
                                if fun_value.apply(interp, env, &[data_value.clone(), elem])?.to_bool() {
                                    return Ok(Value::Bool(true));
                                }
                            },
                            Some(Err(err)) => return Err(err),
                            None => break,
                        }
                    }
                    Ok(Value::Bool(false))
                },
                None => Err(Error::Interp(String::from("value isn't iterable"))),
            }
        },
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn all(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(a_value), Some(data_value), Some(fun_value)) => {
            match a_value.iter()? {
                Some(mut iter) => {
                    loop {
                        match iter.next() {
                            Some(Ok(elem)) => {
                                if !fun_value.apply(interp, env, &[data_value.clone(), elem])?.to_bool() {
                                    return Ok(Value::Bool(false));
                                }
                            },
                            Some(Err(err)) => return Err(err),
                            None => break,
                        }
                    }
                    Ok(Value::Bool(true))
                },
                None => Err(Error::Interp(String::from("value isn't iterable"))),
            }
        },
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn find(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(a_value), Some(data_value), Some(fun_value)) => {
            match a_value.iter()? {
                Some(mut iter) => {
                    let mut i = 1i64;
                    loop {
                        match iter.next() {
                            Some(Ok(elem)) => {
                                if fun_value.apply(interp, env, &[data_value.clone(), elem])?.to_bool() {
                                    return Ok(Value::Int(i));
                                }
                                match i.checked_add(1) {
                                    Some(j) => i = j,
                                    None => return Err(Error::Interp(String::from("too large index"))),
                                }
                            },
                            Some(Err(err)) => return Err(err),
                            None => break,
                        }
                    }
                    Ok(Value::None)
                },
                None => Err(Error::Interp(String::from("value isn't iterable"))),
            }
        },
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn filter(interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(a_value), Some(data_value), Some(fun_value)) => {
            match a_value.iter()? {
                Some(mut iter) => {
                    let mut i_values: Vec<Value> = Vec::new();
                    let mut i = 1i64;
                    loop {
                        match iter.next() {
                            Some(Ok(elem)) => {
                                if fun_value.apply(interp, env, &[data_value.clone(), elem])?.to_bool() {
                                    i_values.push(Value::Int(i));
                                }
                                match i.checked_add(1) {
                                    Some(j) => i = j,
                                    None => return Err(Error::Interp(String::from("too large index"))),
                                }
                            },
                            Some(Err(err)) => return Err(err),
                            None => break,
                        }
                    }
                    Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(i_values)))))
                },
                None => Err(Error::Interp(String::from("value isn't iterable"))),
            }
        },
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn push(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Ref(a_object)), Some(value)) => {
            let mut a_object_g = rw_lock_write(a_object)?;
            match &mut *a_object_g {
                MutObject::Array(elems) => {
                    elems.push(value.clone());
                    Ok(Value::None)
                },
                _ => Err(Error::Interp(String::from("unsupported type for function push"))),
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported type for function push"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn pop(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Ref(a_object)) => {
            let mut a_object_g = rw_lock_write(a_object)?;
            match &mut *a_object_g {
                MutObject::Array(elems) => {
                    match elems.pop() {
                        Some(value) => Ok(value),
                        None => Ok(Value::None),
                    }
                },
                _ => Err(Error::Interp(String::from("unsupported type for function pop"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function pop"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn append(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Ref(a_object)), Some(Value::Ref(b_object))) => {
            let mut a_object_g = rw_lock_write(a_object)?;
            let b_object_g = rw_lock_read(b_object)?;
            match (&mut *a_object_g, &*b_object_g) {
                (MutObject::Array(elems), MutObject::Array(elems2)) => {
                    elems.extend_from_slice(elems2.as_slice());
                    Ok(Value::None)
                },
                _ => Err(Error::Interp(String::from("unsupported type for function append"))),
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported type for function append"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn insert(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 3 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1), arg_values.get(2)) {
        (Some(Value::Ref(a_object)), Some(i_value @ (Value::Int(_) | Value::Float(_) | Value::Object(_))), Some(value)) => {
            let mut a_object_g = rw_lock_write(a_object)?;
            match &mut *a_object_g {
                MutObject::Array(elems) => {
                    match i_value {
                        Value::Int(_) | Value::Float(_) => {
                            let i = i_value.to_i64();
                            if i < 1 || i > (elems.len() as i64).saturating_add(1) {
                                return Err(Error::Interp(String::from("index out of bounds")));
                            }
                            elems.insert((i - 1) as usize, value.clone());
                            Ok(Value::None)
                        },
                        _ => Err(Error::Interp(String::from("unsupported type for function insert"))),
                    }
                },
                MutObject::Struct(fields) => {
                    match i_value {
                        Value::Object(i_object) => {
                            match &**i_object {
                                Object::String(ident) => Ok(fields.insert(ident.clone(), value.clone()).unwrap_or(Value::None)),
                                _ => Err(Error::Interp(String::from("unsupported type for function insert"))),
                            }
                        },
                        _ => Err(Error::Interp(String::from("unsupported type for function insert"))),
                    }
                },
            }
        },
        (Some(_), Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported type for function insert"))),
        (_, _, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn remove(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Ref(a_object)), Some(i_value @ (Value::Int(_) | Value::Float(_) | Value::Object(_)))) => {
            let mut a_object_g = rw_lock_write(a_object)?;
            match &mut *a_object_g {
                MutObject::Array(elems) => {
                    match i_value {
                        Value::Int(_) | Value::Float(_) => {
                            let i = i_value.to_i64();
                            if i < 1 || i > (elems.len() as i64) {
                                return Ok(Value::None);
                            }
                            Ok(elems.remove((i - 1) as usize))
                        },
                        _ => Err(Error::Interp(String::from("unsupported type for function remove"))),
                    }
                },
                MutObject::Struct(fields) => {
                    match i_value {
                        Value::Object(i_object) => {
                            match &**i_object {
                                Object::String(ident) => Ok(fields.remove(ident).unwrap_or(Value::None)),
                                _ => Err(Error::Interp(String::from("unsupported type for function remove"))),
                            }
                        },
                        _ => Err(Error::Interp(String::from("unsupported type for function remove"))),
                    }
                },
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported type for function insert"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn errorkind(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::Error(kind, _) => Ok(Value::Object(Arc::new(Object::String(kind.clone())))),
                _ => Err(Error::Interp(String::from("unsupported type for function errorkind"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function errorkind"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn errormsg(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::Error(_, msg) => Ok(Value::Object(Arc::new(Object::String(msg.clone())))),
                _ => Err(Error::Interp(String::from("unsupported type for function errormsg"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function errormsg"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn isequal(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(a.eq_without_types(b)?))) }

pub fn isnotequal(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(!a.eq_without_types(b)?))) }

pub fn isless(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(a < b))) }

pub fn isgreaterequal(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(a >= b))) }

pub fn isgreater(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(a > b))) }

pub fn islessequal(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun2(arg_values, |a, b| Ok(Value::Bool(a <= b))) }

pub fn sigmoid(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1_for_f32_and_matrix(arg_values, "unsupported type for function sigmoid", |a| 1.0 / (1.0 + (-a).exp()), matrix_sigmoid) }

pub fn tanh(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1_for_f32_and_matrix(arg_values, "unsupported type for function tanh", f32::tanh, matrix_tanh) }

pub fn swish(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1_for_f32_and_matrix(arg_values, "unsupported type for function swish", |a| a / (1.0 + (-a).exp()), matrix_swish) }

pub fn softmax(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value @ (Value::Int(_) | Value::Float(_))) => Ok(Value::Float(value.to_f32().exp() / value.to_f32().exp())),
        Some(Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_softmax(a)?)))),
                _ => Err(Error::Interp(String::from("unsupported type for function softmax"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function softmax"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn sqrt(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{ fun1_for_f32_and_matrix(arg_values, "unsupported type for function sqrt", f32::tanh, matrix_sqrt) }

pub fn reallytranspose(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 1 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match arg_values.get(0) {
        Some(value @ (Value::Int(_) | Value::Float(_))) => Ok(value.clone()),
        Some(Value::Object(object)) => {
            match &**object {
                Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_really_transpose(a)?)))),
                _ => Err(Error::Interp(String::from("unsupported type for function reallytranspose"))),
            }
        },
        Some(_) => Err(Error::Interp(String::from("unsupported type for function reallytranspose"))),
        None => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn repeat(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    match (arg_values.get(0), arg_values.get(1)) {
        (Some(Value::Object(object)), Some(n_value @ (Value::Int(_) | Value::Float(_)))) => {
            match &**object {
                Object::Matrix(a) => {
                    let n = n_value.to_i64();
                    let (m, l) = if a.col_count() == 1 {
                        (a.row_count() as i64, n) 
                    } else if a.row_count() == 1 {
                        (n, a.col_count() as i64)
                    } else {
                        return Err(Error::Interp(String::from("number of columns or rows isn't one")));
                    };
                    match checked_mul_row_count_and_col_count(m, l) {
                        Ok(_) => {
                            match matrix_repeat(a, n as usize)? {
                                Some(b) => Ok(Value::Object(Arc::new(Object::Matrix(b)))),
                                None => return Err(Error::Interp(String::from("number of columns or rows isn't one"))),
                            }
                        },
                        Err(err) => Err(err),
                    }
                },
                _ => Err(Error::Interp(String::from("unsupported types for function repeat"))),
            }
        },
        (Some(_), Some(_)) => Err(Error::Interp(String::from("unsupported types for function repeat"))),
        (_, _) => Err(Error::Interp(String::from("no argument"))),
    }
}

pub fn add_builtin_fun(root_mod: &mut ModNode<Value, ()>, ident: String, f: fn(&mut Interp, &mut Env, &[Value]) -> Result<Value>)
{ root_mod.add_var(ident.clone(), Value::Object(Arc::new(Object::BuiltinFun(ident, f)))) }

pub fn add_alias(root_mod: &mut ModNode<Value, ()>, new_ident: String, old_ident: &String)
{
    match root_mod.var(old_ident) {
        Some(value) => root_mod.add_var(new_ident, value.clone()),
        None => (),
    }
}

pub fn add_std_builtin_funs(root_mod: &mut ModNode<Value, ()>)
{
    add_builtin_fun(root_mod, String::from("type"), typ);
    add_builtin_fun(root_mod, String::from("bool"), boolean);
    add_builtin_fun(root_mod, String::from("int"), int);
    add_builtin_fun(root_mod, String::from("float"), float);
    add_builtin_fun(root_mod, String::from("string"), string);
    add_builtin_fun(root_mod, String::from("zeros"), zeros);
    add_builtin_fun(root_mod, String::from("ones"), ones);
    add_builtin_fun(root_mod, String::from("eye"), eye);
    add_builtin_fun(root_mod, String::from("init"), init);
    add_builtin_fun(root_mod, String::from("initdiag"), initdiag);
    add_builtin_fun(root_mod, String::from("matrix"), matrix);
    add_builtin_fun(root_mod, String::from("rowvector"), rowvector);
    add_builtin_fun(root_mod, String::from("colvector"), colvector);
    add_builtin_fun(root_mod, String::from("matrixarray"), matrixarray);
    add_builtin_fun(root_mod, String::from("error"), error);
    add_builtin_fun(root_mod, String::from("array"), array);
    add_builtin_fun(root_mod, String::from("strong"), strong);
    add_builtin_fun(root_mod, String::from("weak"), weak);
    add_builtin_fun(root_mod, String::from("length"), length);
    add_builtin_fun(root_mod, String::from("rows"), rows);
    add_builtin_fun(root_mod, String::from("columns"), columns);
    add_builtin_fun(root_mod, String::from("get"), get);
    add_builtin_fun(root_mod, String::from("getdiag"), getdiag);
    add_builtin_fun(root_mod, String::from("sort"), sort);
    add_builtin_fun(root_mod, String::from("reverse"), reverse);
    add_builtin_fun(root_mod, String::from("any"), any);
    add_builtin_fun(root_mod, String::from("all"), all);
    add_builtin_fun(root_mod, String::from("find"), find);
    add_builtin_fun(root_mod, String::from("filter"), filter);
    add_builtin_fun(root_mod, String::from("psuh"), push);
    add_builtin_fun(root_mod, String::from("pop"), pop);
    add_builtin_fun(root_mod, String::from("append"), append);
    add_builtin_fun(root_mod, String::from("insert"), insert);
    add_builtin_fun(root_mod, String::from("remove"), remove);
    add_builtin_fun(root_mod, String::from("errorkind"), errorkind);
    add_builtin_fun(root_mod, String::from("errormsg"), errormsg);
    add_builtin_fun(root_mod, String::from("isequal"), isequal);
    add_builtin_fun(root_mod, String::from("isnotequal"), isnotequal);
    add_builtin_fun(root_mod, String::from("isless"), isless);
    add_builtin_fun(root_mod, String::from("isgreaterequal"), isgreaterequal);
    add_builtin_fun(root_mod, String::from("isgreater"), isgreater);
    add_builtin_fun(root_mod, String::from("islessequal"), islessequal);
    add_builtin_fun(root_mod, String::from("sigmoid"), sigmoid);
    add_builtin_fun(root_mod, String::from("tanh"), tanh);
    add_builtin_fun(root_mod, String::from("swish"), swish);
    add_builtin_fun(root_mod, String::from("softmax"), softmax);
    add_builtin_fun(root_mod, String::from("sqrt"), sqrt);
    add_builtin_fun(root_mod, String::from("reallytranspose"), reallytranspose);
    add_alias(root_mod, String::from("rt"), &String::from("reallytranspose"));
    add_builtin_fun(root_mod, String::from("repeat"), repeat);
}
