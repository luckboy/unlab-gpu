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
use std::sync::Weak;
use crate::matrix;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::env::*;
use crate::error::*;
use crate::tree::*;
use crate::utils::*;

fn matrix_result_add(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add(a, b, &c)?;
    Ok(c)
}

fn matrix_add(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_result_add(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_sub(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub(a, b, &c)?;
    Ok(c)
}

fn matrix_sub(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_result_sub(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_mul(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
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
    match matrix_result_mul(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_mul_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_elems(a, b, &c)?;
    Ok(c)
}

fn matrix_mul_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_result_mul_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_div_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_elems(a, b, &c)?;
    Ok(c)
}

fn matrix_div_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_result_div_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_add_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_add_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_add_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_sub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_sub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_sub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_rsub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rsub_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_rsub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_rsub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_mul_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_mul_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_mul_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_div_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_div_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_div_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_result_rdiv_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rdiv_for_scalar(a, b, &c)?;
    Ok(c)
}

fn matrix_rdiv_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_result_rdiv_for_scalar(a, b) {
        Ok(c) => Ok(c),
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

    pub fn eq_with_types(&self, value: &Value) -> Result<bool>
    {
        match (&self, value) {
            (Value::None, Value::None) => Ok(true),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Float(a), Value::Float(b)) => Ok(a == b),
            (Value::Object(object), Value::Object(object2)) => {
                match (&**object, &**object2) {
                    (Object::String(s), Object::String(t)) => Ok(s == t),
                    (Object::IntRange(a, b, c), Object::IntRange(d, e, f)) => Ok(a == d && b == e && c == f),
                    (Object::FloatRange(a, b, c), Object::FloatRange(d, e, f)) => Ok(a == d && b == e && c == f),
                    (Object::Matrix(_), Object::Matrix(_)) => Ok(false),
                    (Object::Fun(idents, ident, fun), Object::Fun(idents2, ident2, fun2)) => {
                        Ok(idents == idents2 && ident == ident2 && Arc::ptr_eq(fun, fun2))
                    },
                    (Object::BuiltinFun(ident, _), Object::BuiltinFun(ident2, _)) => {
                        Ok(ident == ident2)
                    },
                    (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                        Ok(a_row_count == b_row_count && a_col_count == b_col_count && a_transpose_flag == b_transpose_flag && xs == ys)
                    },
                    (Object::MatrixRowSlice(a, ai), Object::MatrixRowSlice(b, bi)) => {
                        match (&**a, &**b) {
                            (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                                if a_col_count != b_col_count {
                                    return Ok(false);
                                }
                                for j in 0..*a_col_count {
                                    let ak = match a_transpose_flag {
                                        TransposeFlag::NoTranspose => ai * (*a_col_count) + j,
                                        TransposeFlag::Transpose => j * (*a_row_count) + ai,
                                    };
                                    let bk = match b_transpose_flag {
                                        TransposeFlag::NoTranspose => bi * *b_col_count + j,
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
                            (_, _) => return Err(Error::Interp(String::from("unsupported object type")))
                        }
                    },
                    (Object::Error(kind, msg), Object::Error(kind2, msg2)) => {
                        Ok(kind == kind2 && msg == msg2)
                    },
                    (_, _) => Ok(false),
                }
            },
            (Value::Ref(object), Value::Ref(object2)) => {
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                match (&*object_g, &*object2_g) {
                    (MutObject::Array(xs), MutObject::Array(ys)) => {
                        for (x, y) in xs.iter().zip(ys.iter()) {
                            if !x.eq_with_types(y)? {
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
                                    if !x.eq_with_types(y)? {
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
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Value::Ref(object).eq_with_types(&Value::Ref(object2)),
                    _ => Ok(false),
                }
            },
            (_, _) => Ok(false),
        }
    }
    
    fn apply_dot_fun1_for_elem_with_fun_ref<F>(&self, f: &mut F) -> Result<Value>
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
            Value::Ref(_) => self.apply_dot_fun1_with_fun_ref(f),
            _ => Ok(self.clone()),
        }
    }
    
    fn apply_dot_fun1_with_fun_ref<F>(&self, f: &mut F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    {
        match self {
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(xs) => {
                        let mut ys: Vec<Value> = Vec::new();
                        for x in xs {
                            ys.push(x.apply_dot_fun1_for_elem_with_fun_ref(f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(ys)))))
                    },
                    MutObject::Struct(xs) => {
                        let mut ys: BTreeMap<String, Value> = BTreeMap::new();
                        for (ident, x) in xs {
                            ys.insert(ident.clone(), x.apply_dot_fun1_for_elem_with_fun_ref(f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(ys)))))
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type"))),
        }
    }
    
    pub fn apply_dot_fun1<F>(&self, mut f: F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    { self.apply_dot_fun1_with_fun_ref(&mut f) }

    fn apply_dot_fun2_for_elem_with_fun_ref<F>(&self, value: &Value, f: &mut F) -> Result<Value>
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
            (_, _) => {
                if !self.eq_with_types(value)? {
                    return Err(Error::Interp(String::from("two values aren't equal")))
                }
                Ok(self.clone())
            },
        }
    }
    
    fn apply_dot_fun2_with_fun_ref<F>(&self, value: &Value, f: &mut F) -> Result<Value>
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
                            zs.push(x.apply_dot_fun2_for_elem_with_fun_ref(y, f)?);
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
                                    zs.insert(ident.clone(), x.apply_dot_fun2_for_elem_with_fun_ref(y, f)?);
                                },
                                (_, _) => return Err(Error::Interp(String::from("no field value"))),
                            }
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(zs)))))
                    },
                    (_, _) => Err(Error::Interp(String::from("two value types aren't equal"))),
                }
            },
            (_, _) => Err(Error::Interp(String::from("unsupported value type"))),
        }
    }

    pub fn apply_dot_fun2<F>(&self, value2: &Value, f: &mut F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    { self.apply_dot_fun2_with_fun_ref(value2, f) }

    pub fn elem(&self, idx_value: &Value) -> Result<Value>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::MatrixArray(row_count, _, _, _) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i >= 1 && i <= (*row_count as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                Ok(Value::Object(Arc::new(Object::MatrixRowSlice(object.clone(), (i - 1) as usize))))
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
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
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported object type"))),
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
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
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
                                    _ => Err(Error::Interp(String::from("unsupported index object type"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type"))),
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
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
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
                            _ => Err(Error::Interp(String::from("unsupported index value type"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type"))),
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
                            None => Err(Error::Interp(String::from("no field")))
                        }
                    },
                    _ => Err(Error::Interp(String::from("object isn't structure"))),
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type"))),
        }
    }

    pub fn set_field(&self, ident: &String, value: Value) -> Result<()>
    {
        match self {
            Value::Ref(object) => {
                let mut object_g = rw_lock_write(&**object)?;
                match &mut *object_g {
                    MutObject::Struct(xs) => {
                        xs.insert(ident.clone(), value);
                        Ok(())
                    },
                    _ => Err(Error::Interp(String::from("object isn't structure"))),
                }
            },
            _ => Err(Error::Interp(String::from("unsupported value type"))),
        }
    }
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
