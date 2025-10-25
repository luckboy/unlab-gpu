//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Neg;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::str::Chars;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::matrix;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::env::*;
use crate::error::*;
use crate::tree::*;
use crate::utils::*;

fn matrix_res_add(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add(a, b, &c)?;
    Ok(c)
}

fn matrix_add(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_add(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sub(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub(a, b, &c)?;
    Ok(c)
}

fn matrix_sub(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_sub(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = if  frontend.backend().has_cublas() {
        frontend.create_matrix_and_set_zeros(a.row_count(), b.col_count())?
    } else {
        unsafe { frontend.create_matrix(a.row_count(), b.col_count())? }
    };
    frontend.mul(a, b, &c)?;
    Ok(c)
}

fn matrix_mul(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_mul(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_elems(a, b, &c)?;
    Ok(c)
}

fn matrix_mul_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_mul_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_div_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_elems(a, b, &c)?;
    Ok(c)
}

fn matrix_div_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_div_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_add_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_add_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_add_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_sub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_sub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_rsub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rsub_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_rsub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rsub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_mul_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_mul_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_div_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_div_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_div_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_rdiv_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rdiv_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_rdiv_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rdiv_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_to_matrix_array(a: &Matrix) -> matrix::Result<Object>
{
    let frontend = Frontend::new()?;
    let (xs, is_transposed) = frontend.elems_and_transpose_flag(a)?;
    let transpose_flag = if is_transposed {
        TransposeFlag::Transpose
    } else {
        TransposeFlag::NoTranspose
    };
    Ok(Object::MatrixArray(a.row_count(), a.col_count(), transpose_flag, xs))
}

fn matrix_to_matrix_array(a: &Matrix) -> Result<Object>
{
    match matrix_res_to_matrix_array(a) {
        Ok(object) => Ok(object),
        Err(err) => Err(Error::Matrix(err)),
    }
}

#[derive(Clone, Debug)]
pub enum Value
{
    None,
    Bool(bool),
    Int(i64),
    Float(f32),
    Object(Arc<Object>),
    Ref(Arc<RwLock<MutObject>>),
    Weak(Weak<RwLock<MutObject>>),
}

impl Value
{
    pub fn to_bool(&self) -> bool
    {
        match self {
            Value::None => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => false,
                    _ => true,
                }
            },
            _ => true,
        }
    }

    pub fn to_i64(&self) -> i64
    {
        match self {
            Value::None => 0,
            Value::Bool(b) => if *b { 1 } else { 0 },
            Value::Int(n) => *n,
            Value::Float(n) => *n as i64,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => 0,
                    _ => 1,
                }
            },
            _ => 1,
        }
    }

    pub fn to_f32(&self) -> f32
    {
        match self {
            Value::None => 0.0,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::Int(n) => *n as f32,
            Value::Float(n) => *n,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => 0.0,
                    _ => 1.0,
                }
            },
            _ => 1.0,
        }
    }

    pub fn to_opt_bool(&self) -> Option<bool>
    {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn to_opt_i64(&self) -> Option<i64>
    {
        match self {
            Value::Int(n) => Some(*n),
            Value::Float(n) => Some(*n as i64),
            _ => None,
        }
    }

    pub fn to_opt_f32(&self) -> Option<f32>
    {
        match self {
            Value::Int(n) => Some(*n as f32),
            Value::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn eq_with_types(&self, value: &Value) -> Result<bool>
    {
        match (&self, value) {
            (Value::None, Value::None) => Ok(true),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Float(a), Value::Float(b)) => Ok(a == b),
            (Value::Object(object), Value::Object(object2)) => object.priv_eq(&**object2),
            (Value::Ref(object), Value::Ref(object2)) => {
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_eq(&*object2_g, Self::eq_with_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Value::Ref(object).eq_with_types(&Value::Ref(object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => Ok(false),
        }
    }

    pub fn eq_without_types(&self, value: &Value) -> Result<bool>
    {
        match (&self, value) {
            (Value::None, Value::None) => Ok(true),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(self.to_f32() == value.to_f32()),
            (Value::Object(object), Value::Object(object2)) => object.priv_eq(&**object2),
            (Value::Ref(object), Value::Ref(object2)) => {
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_eq(&*object2_g, Self::eq_without_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Value::Ref(object).eq_with_types(&Value::Ref(object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => Ok(false),
        }
    }
    
    fn apply_dot_fun1_for_elem_with_fun_ref<F>(&self, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    {
        match self {
            Value::Float(_) => f(self),
            Value::Object(object) => {
                match &**object {
                    Object::Matrix(_) => f(self),
                    _ => Ok(self.clone()),
                }
            },
            Value::Ref(_) => self.apply_dot_fun1_with_fun_ref(err_msg, f),
            _ => Ok(self.clone()),
        }
    }
    
    fn apply_dot_fun1_with_fun_ref<F>(&self, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    {
        match self {
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(xs) => {
                        let mut ys: Vec<Value> = Vec::new();
                        for x in xs {
                            ys.push(x.apply_dot_fun1_for_elem_with_fun_ref(err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(ys)))))
                    },
                    MutObject::Struct(xs) => {
                        let mut ys: BTreeMap<String, Value> = BTreeMap::new();
                        for (ident, x) in xs {
                            ys.insert(ident.clone(), x.apply_dot_fun1_for_elem_with_fun_ref(err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(ys)))))
                    },
                }
            },
            _ => Err(Error::Interp(String::from(err_msg))),
        }
    }
    
    pub fn apply_dot_fun1<F>(&self, msg: &str, mut f: F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    { self.apply_dot_fun1_with_fun_ref(msg, &mut f) }

    fn apply_dot_fun2_for_elem_with_fun_ref<F>(&self, value: &Value, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    {
        match (self, value) {
            (Value::Float(_), Value::Float(_)) => f(self, value),
            (Value::Object(object), Value::Object(object2)) => {
                match (&**object, &**object2) {
                    (Object::Matrix(_), Object::Matrix(_)) => f(self, value),
                    (_, _) => {
                        if !self.eq_with_types(value)? {
                            return Err(Error::Interp(String::from("two values aren't equal")))
                        }
                        Ok(self.clone())
                    },
                }
            },
            (Value::Ref(_), Value::Ref(_)) => self.apply_dot_fun2_with_fun_ref(value, err_msg, f),
            (Value::Weak(_), _) | (_, Value::Weak(_)) => Err(Error::Interp(String::from("values are weak references"))),
            (_, _) => {
                if !self.eq_with_types(value)? {
                    return Err(Error::Interp(String::from("two values aren't equal")))
                }
                Ok(self.clone())
            },
        }
    }
    
    fn apply_dot_fun2_with_fun_ref<F>(&self, value: &Value, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    {
        match (self, value) {
            (Value::Ref(object), Value::Ref(object2)) => {
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                match (&*object_g, &*object2_g) {
                    (MutObject::Array(xs), MutObject::Array(ys)) => {
                        if xs.len() != ys.len() {
                            return Err(Error::Interp(String::from("lengths of two arrays aren't equal")));
                        }
                        let mut zs: Vec<Value> = Vec::new();
                        for (x, y) in xs.iter().zip(ys.iter()) {
                            zs.push(x.apply_dot_fun2_for_elem_with_fun_ref(y, err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(zs)))))
                    },
                    (MutObject::Struct(xs), MutObject::Struct(ys)) => {
                        let idents: BTreeSet<String> = xs.keys().map(|s| s.clone()).collect();
                        let idents2: BTreeSet<String> = ys.keys().map(|s| s.clone()).collect();
                        if idents != idents2 {
                            return Err(Error::Interp(String::from("field names of two structures aren't equal")));
                        }
                        let mut zs: BTreeMap<String, Value> = BTreeMap::new();
                        for ident in &idents {
                            match (xs.get(ident), ys.get(ident)) {
                                (Some(x), Some(y)) => {
                                    zs.insert(ident.clone(), x.apply_dot_fun2_for_elem_with_fun_ref(y, err_msg, f)?);
                                },
                                (_, _) => return Err(Error::Interp(String::from("no field value"))),
                            }
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(zs)))))
                    },
                    (_, _) => Err(Error::Interp(String::from("two object types aren't equal"))),
                }
            },
            (_, _) => Err(Error::Interp(String::from(err_msg))),
        }
    }

    pub fn apply_dot_fun2<F>(&self, value: &Value, msg: &str, mut f: F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    { self.apply_dot_fun2_with_fun_ref(value, msg, &mut f) }

    pub fn elem(&self, idx_value: &Value) -> Result<Value>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i >= 1 && i <= (s.chars().count() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match s.chars().nth((i - 1) as usize) {
                                    Some(c) => {
                                        let mut t = String::new();
                                        t.push(c);
                                        Ok(Value::Object(Arc::new(Object::String(t))))
                                    }
                                    None => Err(Error::Interp(String::from("index out of bounds"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for indexing"))),
                        }
                    },
                    Object::MatrixArray(row_count, _, _, _) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i >= 1 && i <= (*row_count as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                Ok(Value::Object(Arc::new(Object::MatrixRowSlice(object.clone(), (i - 1) as usize))))
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for indexing"))),
                        }
                    },
                    Object::MatrixRowSlice(matrix_array, i) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let j = idx_value.to_i64();
                                match &**matrix_array {
                                    Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                                        if j >= 1 && j <= (*col_count as i64) {
                                            return Err(Error::Interp(String::from("index out of bounds")));
                                        }
                                        let k = match transpose_flag {
                                            TransposeFlag::NoTranspose => i * (*col_count) + ((j - 1) as usize),
                                            TransposeFlag::Transpose => ((j - 1) as usize) * (*row_count) + i,
                                        };
                                        match xs.get(k) {
                                            Some(x) => Ok(Value::Float(*x)),
                                            None => Err(Error::Interp(String::from("index out of bounds"))),
                                        }
                                    },
                                    _ => Err(Error::Interp(String::from("unsupported object type"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for indexing"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported object type for indexing"))),
                }
            },
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(xs) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i >= 1 && i <= (xs.len() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match xs.get((i - 1) as usize) { 
                                    Some(x) => Ok(x.clone()),
                                    None => Err(Error::Interp(String::from("index out of bounds"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for indexing"))),
                        }
                    },
                    MutObject::Struct(xs) => {
                        match idx_value {
                            Value::Object(idx_object) => {
                                match &**idx_object {
                                    Object::String(ident) => {
                                        match xs.get(ident) {
                                            Some(x) => Ok(x.clone()),
                                            None => Err(Error::Interp(String::from("not found key")))
                                        }
                                    },
                                    _ => Err(Error::Interp(String::from("unsupported index object type for indexing"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for indexing"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type for indexing"))),
        }
    }

    pub fn set_elem(&self, idx_value: &Value, value: Value) -> Result<()>
    {
        match self {
            Value::Ref(object) => {
                let mut object_g = rw_lock_write(&**object)?;
                match &mut *object_g {
                    MutObject::Array(xs) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i >= 1 && i <= (xs.len() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match xs.get_mut((i - 1) as usize) {
                                    Some(x) => {
                                        *x = value;
                                        Ok(())
                                    }
                                    None => Err(Error::Interp(String::from("index out of bounds"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for mutable indexing"))),
                        }
                    },
                    MutObject::Struct(xs) => {
                        match idx_value {
                            Value::Object(idx_object) => {
                                match &**idx_object {
                                    Object::String(ident) => {
                                        xs.insert(ident.clone(), value);
                                        Ok(())
                                    },
                                    _ => Err(Error::Interp(String::from("unsupported index object type"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type for mutable indexing"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type for mutable indexing"))),
        }
    }

    pub fn field(&self, ident: &String) -> Result<Value>
    {
        match self {
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Struct(xs) => {
                        match xs.get(ident) {
                            Some(x) => Ok(x.clone()),
                            None => Err(Error::Interp(format!("structure hasn't field {}", ident))),
                        }
                    },
                    _ => Err(Error::Interp(format!("unsupported object type for field {}", ident))),
                }
            },
            _ => Err(Error::Interp(format!("unsupported value type for field {}", ident))),
        }
    }

    pub fn set_field(&self, ident: String, value: Value) -> Result<()>
    {
        match self {
            Value::Ref(object) => {
                let mut object_g = rw_lock_write(&**object)?;
                match &mut *object_g {
                    MutObject::Struct(xs) => {
                        xs.insert(ident.clone(), value);
                        Ok(())
                    },
                    _ => Err(Error::Interp(format!("unsupported object type for mutable field {}", ident))),
                }
            },
            _ => Err(Error::Interp(format!("unsupported value type for mutable field {}", ident))),
        }
    }

    pub fn unary_op(&self, op: UnaryOp) -> Result<Value>
    {
        match op {
            UnaryOp::Neg => {
                match self {
                    Value::Int(a) => {
                        match a.checked_neg() {
                            Some(b) => Ok(Value::Int(b)),
                            None => Err(Error::Interp(String::from("overflow"))),
                        }
                    },
                    Value::Float(a) => Ok(Value::Float(-a)),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(a, 0.0)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object type for negation"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported value type for negation"))),
                }
            },
            UnaryOp::DotNeg => {
                match self {
                    Value::Int(_) | Value::Float(_) => Ok(Value::Float(self.to_f32())),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(a, 0.0)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object type for dot negation"))),
                        }
                    },
                    _ => self.apply_dot_fun1("unsupported object type for dot negation", |v| v.unary_op(op)),
                }
            },
            UnaryOp::Not => Ok(Value::Bool(!self.to_bool())),
            UnaryOp::Transpose => {
                match self {
                    Value::Int(_) | Value::Float(_) => Ok(self.clone()),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(a.transpose())))),
                            _ => Err(Error::Interp(String::from("unsupported object type for transpose"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported value type for transpose"))),
                }
            },
        }
    }

    pub fn bin_op(&self, op: BinOp, value: &Value) -> Result<Value>
    {
        match op {
            BinOp::Index => self.elem(value),
            BinOp::Mul => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_mul(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() * value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for multiplication"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for multiplication"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for multiplication"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported value types for multiplication"))),
                }
            },
            BinOp::DotMul => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() * value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot multiplication"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot multiplication"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_elems(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot multiplication"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.apply_dot_fun1("unsupported value types for dot multiplication", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.apply_dot_fun1("unsupported value types for dot multiplication", |v| self.bin_op(op, v)),
                    _ => self.apply_dot_fun2(value, "unsupported value types for dot multiplication", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Div => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_div(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => {
                                if *b == 0 {
                                    Err(Error::Interp(String::from("division by zero")))
                                } else {
                                    Err(Error::Interp(String::from("overflow")))
                                }
                            },
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() / value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_div_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for division"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rdiv_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for division"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported value types for division"))),
                }
            },
            BinOp::DotDiv => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() * value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot division"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot division"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_div_elems(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot divistion"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.apply_dot_fun1("unsupported value types for dot division", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.apply_dot_fun1("unsupported value types for dot division", |v| self.bin_op(op, v)),
                    (_, _) => self.apply_dot_fun2(value, "unsupported value types for dot division", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Add => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_add(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() + value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for addition"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for addition"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::String(s), Object::String(t)) => Ok(Value::Object(Arc::new(Object::String(s.clone() + t.as_str())))),
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for addition"))),
                        }
                    },
                    (Value::Ref(object), Value::Ref(object2)) => {
                        let object_g = rw_lock_read(&**object)?;
                        let object2_g = rw_lock_read(&**object2)?;
                        match (&*object_g, &*object2_g) {
                            (MutObject::Array(xs), MutObject::Array(ys)) => {
                                let mut zs = xs.clone();
                                zs.extend_from_slice(ys.as_slice());
                                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(zs)))))
                            },
                            (MutObject::Struct(xs), MutObject::Struct(ys)) => {
                                let mut zs: BTreeMap<String, Value> = BTreeMap::new();
                                let idents: BTreeSet<String> = xs.keys().map(|s| s.clone()).collect();
                                let idents2: BTreeSet<String> = ys.keys().map(|s| s.clone()).collect();
                                let idents3: Vec<String> = idents.union(&idents2).map(|s| s.clone()).collect();
                                for ident in &idents3 {
                                    match xs.get(ident) {
                                        Some(x) => {
                                            zs.insert(ident.clone(), x.clone());
                                        },
                                        None => {
                                            match ys.get(ident) {
                                                Some(y) => {
                                                    zs.insert(ident.clone(), y.clone());
                                                },
                                                None => (),
                                            }
                                        },
                                    }
                                }
                                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(zs)))))
                            },
                            _ => Err(Error::Interp(String::from("unsupported object types for addition"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported value types for addition"))),
                }
            },
            BinOp::DotAdd => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_add(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() + value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot addition"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot addition"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot addition"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.apply_dot_fun1("unsupported value types for dot addition", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.apply_dot_fun1("unsupported value types for dot addition", |v| self.bin_op(op, v)),
                    (_, _) => self.apply_dot_fun2(value, "unsupported value types for dot addition", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Sub => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_sub(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() - value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for subtraction"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for subtraction"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for subtraction"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported value types for subtraction"))),
                }
            },
            BinOp::DotSub => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() - value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot subtraction"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(b, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot subtraction"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported object types for dot subtraction"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.apply_dot_fun1("unsupported value types for dot subtraction", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.apply_dot_fun1("unsupported value types for dot subtraction", |v| self.bin_op(op, v)),
                    (_, _) => self.apply_dot_fun2(value, "unsupported value types for dot subtraction", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Lt => {
                match (self, value) {
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a < b)),
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Bool(self.to_f32() < value.to_f32())),
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::String(s), Object::String(t)) => Ok(Value::Bool(s < t)),
                            (_, _) => Err(Error::Interp(String::from("unsupported object types for comparation"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported value types for subtraction"))),
                }
            },
            BinOp::Ge => Ok(Value::Bool(!self.bin_op(BinOp::Lt, value)?.to_bool())),
            BinOp::Gt => Ok(Value::Bool(value.bin_op(BinOp::Lt, self)?.to_bool())),
            BinOp::Le => Ok(Value::Bool(!value.bin_op(BinOp::Lt, self)?.to_bool())),
            BinOp::Eq => Ok(Value::Bool(self.eq_without_types(value)?)),
            BinOp::Ne => Ok(Value::Bool(!self.bin_op(BinOp::Eq, value)?.to_bool())),
        }
    }
    
    pub fn iter(&self) -> Result<Option<Iter<'_>>>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::IntRange(a, b, c) => Ok(Some(Iter::new(IterEnum::IntRange(*a, *b, *c, false)))),
                    Object::FloatRange(a, b, c) => Ok(Some(Iter::new(IterEnum::FloatRange(*a, *b, *c, false)))),
                    Object::String(s) => Ok(Some(Iter::new(IterEnum::String(s.chars())))),
                    Object::MatrixArray(_, _, _, _) => Ok(Some(Iter::new(IterEnum::MatrixArray(object.clone(), 0, false)))), 
                    Object::MatrixRowSlice(matrix_array, i) => Ok(Some(Iter::new(IterEnum::MatrixRowSlice(matrix_array.clone(), *i, 0, false)))),
                    _ => Ok(None),
                }
            },
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(_) => Ok(Some(Iter::new(IterEnum::Array(object.clone(), 0, false)))),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }
    
    pub fn to_matrix_array(&self) -> Result<Value>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::Matrix(a) => Ok(Value::Object(Arc::new(matrix_to_matrix_array(a)?))),
                    _ => Err(Error::Interp(String::from("unsupported object type for conversion to matrix array"))),
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type for conversion to matrix array"))),
        }
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize, is_width: bool) -> fmt::Result
    {
        let width = if is_width { 10 } else { 0 };
        match self {
            Value::None => write!(f, "{:>width$}", "none")?,
            Value::Bool(false) => write!(f, "{:>width$}", "false")?,
            Value::Bool(true) => write!(f, "{:>width$}", "true")?,
            Value::Int(n) => write!(f, "{:>width$}", n)?,
            Value::Float(n) => {
                if format!("{:.4}", n).len() > 10 {
                    write!(f, "{:>width$.4e}", n)?;
                } else {
                    write!(f, "{:>width$.4}", n)?;
                }
            },
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => write!(f, "{}", s)?,
                    Object::IntRange(a, b, c) => {
                        Value::Int(*a).fmt_with_indent(f, indent, is_width)?;
                        write!(f, " to ")?;
                        Value::Int(*b).fmt_with_indent(f, indent, is_width)?;
                        write!(f, " by ")?;
                        Value::Int(*c).fmt_with_indent(f, indent, is_width)?;
                    },
                    Object::FloatRange(a, b, c) => {
                        Value::Float(*a).fmt_with_indent(f, indent, is_width)?;
                        write!(f, " to ")?;
                        Value::Float(*b).fmt_with_indent(f, indent, is_width)?;
                        write!(f, " by ")?;
                        Value::Float(*c).fmt_with_indent(f, indent, is_width)?;
                    },
                    Object::Matrix(_) => self.to_matrix_array().unwrap().fmt_with_indent(f, indent, is_width)?,
                    Object::Fun(idents, ident, _) => {
                        for ident2 in idents {
                            write!(f, "{}::", ident2)?;
                        }
                        write!(f, "{}", ident)?;
                    },
                    Object::BuiltinFun(ident, _) => write!(f, "{}", ident)?,
                    Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                        if *row_count > 0 { 
                            let new_indent = indent + 4;
                            writeln!(f, "[")?;
                            for i in 0..*row_count {
                                write!(f, "{:new_indent$}", "")?;
                                for j in 0..*col_count {
                                    let k = match transpose_flag {
                                        TransposeFlag::NoTranspose => i * (*col_count) + j,
                                        TransposeFlag::Transpose => j * (*row_count) + i,
                                    };
                                    Value::Float(xs[k]).fmt_with_indent(f, new_indent, true)?;
                                    if j + 1 < *col_count {
                                        write!(f, " ")?;
                                    }
                                }
                                writeln!(f, "")?;          
                            }
                            write!(f, "{:indent$}]", "")?;
                        } else {
                            write!(f, "[]")?;
                        }
                    },
                    Object::MatrixRowSlice(matrix_array, i) => {
                        match &**matrix_array {
                            Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                                if *col_count > 0 {
                                    let new_indent = indent + 4;
                                    write!(f, "[")?;
                                    for j in 0..*col_count {
                                        let k = match transpose_flag {
                                            TransposeFlag::NoTranspose => (*i) * (*col_count) + j,
                                            TransposeFlag::Transpose => j * (*row_count) + (*i),
                                        };
                                        write!(f, " ")?;
                                        Value::Float(xs[k]).fmt_with_indent(f, new_indent, is_width)?;
                                    }
                                    write!(f, " ]")?;
                                } else {
                                    write!(f, "[]")?;
                                }
                            },
                            _ => panic!("invalid object type"),
                        }
                    },
                    Object::Error(_, msg) => write!(f, "{}", msg)?,
                }
            },
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object).unwrap();
                match &*object_g {
                    MutObject::Array(xs) => {
                        if !xs.is_empty() {
                            let new_indent = indent + 4;
                            write!(f, ".[")?;
                            for x in xs {
                                write!(f, " ")?;
                                x.fmt_with_indent(f, new_indent, is_width)?;
                            }
                            write!(f, " .]")?;
                        } else {
                            write!(f, ".[.]")?;
                        }
                    },
                    MutObject::Struct(xs) => {
                        if !xs.is_empty() {
                            let new_indent = indent + 4;
                            writeln!(f, "{{")?;
                            for (ident, x) in xs {
                                write!(f, "{}: ", ident)?;
                                x.fmt_with_indent(f, new_indent, is_width)?;
                                writeln!(f, "")?;
                            }
                            write!(f, "}}")?;
                        } else {
                            write!(f, "{{}}")?;
                        }
                    },
                }
            },
            Value::Weak(object) => {
                match object.upgrade() {
                    Some(object) => {
                        write!(f, "weak(")?;
                        Value::Ref(object).fmt_with_indent(f, indent, is_width)?;
                        write!(f, ")")?;
                    },
                    None => write!(f, "weak()")?,
                }
            },
        }
        Ok(())
    }
}

impl Neg for Value
{
    type Output = Self;
    
    fn neg(self) -> Self::Output
    { self.unary_op(UnaryOp::Neg).unwrap() }
}

impl Neg for &Value
{
    type Output = Value;
    
    fn neg(self) -> Self::Output
    { self.unary_op(UnaryOp::Neg).unwrap() }
}

impl Add for Value
{
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output
    { self.bin_op(BinOp::Add, &rhs).unwrap() }
}

impl Add<&Value> for Value
{
    type Output = Self;
    
    fn add(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Add, rhs).unwrap() }
}

impl Add<Value> for &Value
{
    type Output = Value;
    
    fn add(self, rhs: Value) -> Self::Output
    { self.bin_op(BinOp::Add, &rhs).unwrap() }
}

impl Add<&Value> for &Value
{
    type Output = Value;
    
    fn add(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Add, rhs).unwrap() }
}

impl AddAssign for Value
{
    fn add_assign(&mut self, rhs: Self)
    { *self = self.bin_op(BinOp::Add, &rhs).unwrap(); }
}

impl AddAssign<&Value> for Value
{
    fn add_assign(&mut self, rhs: &Self)
    { *self = self.bin_op(BinOp::Add, rhs).unwrap(); }
}

impl Sub for Value
{
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output
    { self.bin_op(BinOp::Sub, &rhs).unwrap() }
}

impl Sub<&Value> for Value
{
    type Output = Self;
    
    fn sub(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Sub, rhs).unwrap() }
}

impl Sub<Value> for &Value
{
    type Output = Value;
    
    fn sub(self, rhs: Value) -> Self::Output
    { self.bin_op(BinOp::Sub, &rhs).unwrap() }
}

impl Sub<&Value> for &Value
{
    type Output = Value;
    
    fn sub(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Sub, rhs).unwrap() }
}

impl SubAssign for Value
{
    fn sub_assign(&mut self, rhs: Self)
    { *self = self.bin_op(BinOp::Sub, &rhs).unwrap(); }
}

impl SubAssign<&Value> for Value
{
    fn sub_assign(&mut self, rhs: &Self)
    { *self = self.bin_op(BinOp::Sub, rhs).unwrap(); }
}

impl Mul for Value
{
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output
    { self.bin_op(BinOp::Mul, &rhs).unwrap() }
}

impl Mul<&Value> for Value
{
    type Output = Self;
    
    fn mul(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Mul, rhs).unwrap() }
}

impl Mul<Value> for &Value
{
    type Output = Value;
    
    fn mul(self, rhs: Value) -> Self::Output
    { self.bin_op(BinOp::Mul, &rhs).unwrap() }
}

impl Mul<&Value> for &Value
{
    type Output = Value;
    
    fn mul(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Mul, rhs).unwrap() }
}

impl MulAssign for Value
{
    fn mul_assign(&mut self, rhs: Self)
    { *self = self.bin_op(BinOp::Mul, &rhs).unwrap(); }
}

impl MulAssign<&Value> for Value
{
    fn mul_assign(&mut self, rhs: &Self)
    { *self = self.bin_op(BinOp::Mul, rhs).unwrap(); }
}

impl Div for Value
{
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output
    { self.bin_op(BinOp::Div, &rhs).unwrap() }
}

impl Div<&Value> for Value
{
    type Output = Self;
    
    fn div(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Div, rhs).unwrap() }
}

impl Div<Value> for &Value
{
    type Output = Value;
    
    fn div(self, rhs: Value) -> Self::Output
    { self.bin_op(BinOp::Div, &rhs).unwrap() }
}

impl Div<&Value> for &Value
{
    type Output = Value;
    
    fn div(self, rhs: &Value) -> Self::Output
    { self.bin_op(BinOp::Div, rhs).unwrap() }
}

impl DivAssign for Value
{
    fn div_assign(&mut self, rhs: Self)
    { *self = self.bin_op(BinOp::Div, &rhs).unwrap(); }
}

impl DivAssign<&Value> for Value
{
    fn div_assign(&mut self, rhs: &Self)
    { *self = self.bin_op(BinOp::Div, rhs).unwrap(); }
}

impl PartialEq for Value
{
    fn eq(&self, other: &Self) -> bool
    { self.bin_op(BinOp::Eq, other).unwrap().to_bool() }
}

impl PartialOrd for Value
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => self.to_f32().partial_cmp(&other.to_f32()),
            (Value::Object(object), Value::Object(object2)) => {
                match (&**object, &**object2) {
                    (Object::String(s), Object::String(t)) => s.partial_cmp(t),
                    (_, _) => None,
                }
            },
            (_, _) => None,
        }
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { self.fmt_with_indent(f, 0, false) }
}

#[derive(Clone, Debug)]
pub enum Object
{
    String(String),
    IntRange(i64, i64, i64),
    FloatRange(f32, f32, f32),
    Matrix(Matrix),
    Fun(Vec<String>, String, Arc<Fun>),
    BuiltinFun(String, fn(&mut Env, Vec<Value>) -> Result<Value>),
    MatrixArray(usize, usize, TransposeFlag, Vec<f32>),
    MatrixRowSlice(Arc<Object>, usize),
    Error(String, String),
}

impl Object
{
    fn priv_eq(&self, object: &Object) -> Result<bool>
    {
        match (self, object) {
            (Object::String(s), Object::String(t)) => Ok(s == t),
            (Object::IntRange(a, b, c), Object::IntRange(d, e, f)) => Ok(a == d && b == e && c == f),
            (Object::FloatRange(a, b, c), Object::FloatRange(d, e, f)) => Ok(a == d && b == e && c == f),
            (Object::Matrix(_), Object::Matrix(_)) => Ok(false),
            (Object::Fun(idents, ident, fun), Object::Fun(idents2, ident2, fun2)) => Ok(idents == idents2 && ident == ident2 && Arc::ptr_eq(fun, fun2)),
            (Object::BuiltinFun(ident, _), Object::BuiltinFun(ident2, _)) => Ok(ident == ident2),
            (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                if a_row_count != b_row_count || a_col_count != b_col_count {
                    return Ok(false);
                }
                for i in 0..(*a_row_count) {
                    for j in 0..(*a_col_count) {
                        let ak = match a_transpose_flag {
                            TransposeFlag::NoTranspose => i * (*a_col_count) + j,
                            TransposeFlag::Transpose => j * (*a_row_count) + i,
                        };
                        let bk = match b_transpose_flag {
                            TransposeFlag::NoTranspose => i * (*b_col_count) + j,
                            TransposeFlag::Transpose => j * (*b_row_count) + i,
                        };
                        match (xs.get(ak), ys.get(bk)) {
                            (Some(x), Some(y)) => {
                                if x != y {
                                    return Ok(false);
                                }
                            },
                            (_, _) => return Err(Error::Interp(String::from("no element"))),
                        }
                    }
                }
                Ok(true)
            },
            (Object::MatrixRowSlice(a, ai), Object::MatrixRowSlice(b, bi)) => {
                match (&**a, &**b) {
                    (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                        if a_col_count != b_col_count {
                            return Ok(false);
                        }
                        for j in 0..(*a_col_count) {
                            let ak = match a_transpose_flag {
                                TransposeFlag::NoTranspose => ai * (*a_col_count) + j,
                                TransposeFlag::Transpose => j * (*a_row_count) + ai,
                            };
                            let bk = match b_transpose_flag {
                                TransposeFlag::NoTranspose => bi * (*b_col_count) + j,
                                TransposeFlag::Transpose => j * (*b_row_count) + bi,
                            };
                            match (xs.get(ak), ys.get(bk)) {
                                (Some(x), Some(y)) => {
                                    if x != y {
                                        return Ok(false);
                                    }
                                },
                                (_, _) => return Err(Error::Interp(String::from("no element"))),
                            }
                        }
                        Ok(true)
                    },
                    (_, _) => return Err(Error::Interp(String::from("invalid object type")))
                }
            },
            (Object::Error(kind, msg), Object::Error(kind2, msg2)) => Ok(kind == kind2 && msg == msg2),
            (_, _) => Ok(false),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TransposeFlag
{
    NoTranspose,
    Transpose,
}

#[derive(Clone, Debug)]
pub enum MutObject
{
    Array(Vec<Value>),
    Struct(BTreeMap<String, Value>),
}

impl MutObject
{
    fn priv_eq<F>(&self, object: &MutObject, mut f: F) -> Result<bool>
        where F: FnMut(&Value, &Value) -> Result<bool>
    {
        match (self, object) {
            (MutObject::Array(xs), MutObject::Array(ys)) => {
                for (x, y) in xs.iter().zip(ys.iter()) {
                    if !f(x, y)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            (MutObject::Struct(xs), MutObject::Struct(ys)) => {
                let idents: BTreeSet<String> = xs.keys().map(|s| s.clone()).collect();
                let idents2: BTreeSet<String> = ys.keys().map(|s| s.clone()).collect();
                if idents != idents2 {
                    return Ok(false);
                }
                for ident in &idents {
                    match (xs.get(ident), ys.get(ident)) {
                        (Some(x), Some(y)) => {
                            if !f(x, y)? {
                                return Ok(false);
                            }
                        },
                        (_, _) => return Err(Error::Interp(String::from("no field value"))),
                    }
                }
                Ok(true)
            },
            (_, _) => Ok(false),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a>
{
    iter_enum: IterEnum<'a>,
}

impl<'a> Iter<'a>
{
    fn new(iter_enum: IterEnum<'a>) -> Self
    { Iter { iter_enum, } }
}

impl<'a> Iterator for Iter<'a>
{
    type Item = Result<Value>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match &mut self.iter_enum {
            IterEnum::IntRange(from, to, step, is_stopped) => {
                if !*is_stopped {
                    let current = if *from <= *to {
                        Some(*from)
                    } else {
                        None
                    };
                    if *from < *to {
                        *from += *step;
                    } else {
                        *is_stopped = true;
                    }
                    match current {
                        Some(current) => Some(Ok(Value::Int(current))),
                        None => None,
                    }
                } else {
                    None
                }
            },
            IterEnum::FloatRange(from, to, step, is_stopped) => {
                if !*is_stopped {
                    let current = if *from <= *to {
                        Some(*from)
                    } else {
                        None
                    };
                    if *from < *to {
                        *from += *step;
                    } else {
                        *is_stopped = true;
                    }
                    match current {
                        Some(current) => Some(Ok(Value::Float(current))),
                        None => None,
                    }
                } else {
                    None
                }
            },
            IterEnum::String(cs) => {
                match cs.next() {
                    Some(c) => {
                        let mut s = String::new();
                        s.push(c);
                        Some(Ok(Value::Object(Arc::new(Object::String(s)))))
                    },
                    None => None,
                }
            },
            IterEnum::MatrixArray(matrix_array, i, is_stopped) => {
                if !*is_stopped {
                    match &**matrix_array {
                        Object::MatrixArray(row_count, _, _, _) => {
                            if *i < *row_count {
                                *i += 1;
                                Some(Ok(Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array.clone(), *i)))))
                            } else {
                                None
                            }
                        },
                        _ => {
                            *is_stopped = true;
                            Some(Err(Error::Interp(String::from("invalid object type"))))
                        },
                    }
                } else {
                    None
                }
            },
            IterEnum::MatrixRowSlice(matrix_array, i, j, is_stopped) => {
                if !*is_stopped {
                    match &**matrix_array {
                        Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                            if *j < *col_count {
                                let k = match transpose_flag {
                                    TransposeFlag::NoTranspose => (*i) * (*col_count) + (*j),
                                    TransposeFlag::Transpose => (*j) * (*row_count) + (*i),
                                };
                                *j += 1;
                                match xs.get(k) {
                                    Some(x) => Some(Ok(Value::Float(*x))),
                                    None => {
                                        *is_stopped = true;
                                        Some(Err(Error::Interp(String::from("invalid index"))))
                                    },
                                }
                            } else {
                                None
                            }
                        },
                        _ => {
                            *is_stopped = true;
                            Some(Err(Error::Interp(String::from("invalid object type"))))
                        },
                    }
                } else {
                    None
                }
            },
            IterEnum::Array(array, i, is_stopped) => {
                if !*is_stopped {
                    match rw_lock_read(&**array) {
                        Ok(array_g) => {
                            match &*array_g {
                                MutObject::Array(xs) => {
                                    if *i < xs.len() {
                                        let j = *i;
                                        *i += 1;
                                        match xs.get(j) {
                                            Some(x) => Some(Ok(x.clone())),
                                            None => {
                                                *is_stopped = true;
                                                Some(Err(Error::Interp(String::from("invalid index"))))
                                            },
                                        }
                                    } else {
                                        None
                                    }
                                },
                                _ => {
                                    *is_stopped = true;
                                    Some(Err(Error::Interp(String::from("invalid object type"))))
                                },
                            }
                        },
                        Err(err) => {
                            *is_stopped = true;
                            Some(Err(err))
                        },
                    }
                } else {
                    None
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
enum IterEnum<'a>
{
    IntRange(i64, i64, i64, bool),
    FloatRange(f32, f32, f32, bool),
    String(Chars<'a>),
    MatrixArray(Arc<Object>, usize, bool),
    MatrixRowSlice(Arc<Object>, usize, usize, bool),
    Array(Arc<RwLock<MutObject>>, usize, bool),
}
