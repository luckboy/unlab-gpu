//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A value module.
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::result;
use std::str::Chars;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::serde::de;
use crate::serde::de::MapAccess;
use crate::serde::de::SeqAccess;
use crate::serde::de::Visitor;
use crate::serde::ser;
use crate::serde::ser::SerializeSeq;
use crate::serde::ser::SerializeMap;
use crate::serde::Deserialize;
use crate::serde::Deserializer;
use crate::serde::Serialize;
use crate::serde::Serializer;
use crate::matrix::Matrix;
#[cfg(feature = "plot")]
use crate::winit;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::tree::*;
use crate::utils::*;

/// A type of window identifier.
#[cfg(feature = "plot")]
pub type WindowId = winit::window::WindowId;

/// A type of window identifier.
#[cfg(not(feature = "plot"))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WindowId(());

fn nearly_eq(a: f32, b: f32, eps: f32) -> bool
{
    if a == b {
        true
    } else {
        (a - b).abs() <= eps
    }
}

/// A value enumeration.
///
/// The value enumeration represents a value of the Unlab scripting language.
#[derive(Clone, Debug)]
pub enum Value
{
    /// A none value.
    None,
    /// A boolean value.
    Bool(bool),
    /// An integer number.
    Int(i64),
    /// Floating-point number.
    Float(f32),
    /// An immutablke object.
    Object(Arc<Object>),
    /// A strong reference to a mutable object.
    Ref(Arc<RwLock<MutObject>>),
    /// A weak reference to a mutable object.
    Weak(Weak<RwLock<MutObject>>),
}

impl Value
{
    /// Converts any value to a boolean value.
    pub fn to_bool(&self) -> bool
    {
        match self {
            Value::None => false,
            Value::Bool(a) => *a,
            Value::Int(a) => *a != 0,
            Value::Float(a) => *a != 0.0,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => false,
                    _ => true,
                }
            },
            _ => true,
        }
    }

    /// Converts any value to an integer number.
    pub fn to_i64(&self) -> i64
    {
        match self {
            Value::None => 0,
            Value::Bool(a) => if *a { 1 } else { 0 },
            Value::Int(a) => *a,
            Value::Float(a) => *a as i64,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => 0,
                    _ => 1,
                }
            },
            _ => 1,
        }
    }

    /// Converts any value to a floating-point number.
    pub fn to_f32(&self) -> f32
    {
        match self {
            Value::None => 0.0,
            Value::Bool(a) => if *a { 1.0 } else { 0.0 },
            Value::Int(a) => *a as f32,
            Value::Float(a) => *a,
            Value::Object(object) => {
                match &**object {
                    Object::Error(_, _) => 0.0,
                    _ => 1.0,
                }
            },
            _ => 1.0,
        }
    }

    /// Converts the value to a boolean value if the value is a boolean type, otherwise this
    /// method returns `None`.
    pub fn to_opt_bool(&self) -> Option<bool>
    {
        match self {
            Value::Bool(a) => Some(*a),
            _ => None,
        }
    }

    /// Converts the value to an integer number if the value is an integer type or floating-point
    /// type, otherwise this method returns `None`.
    pub fn to_opt_i64(&self) -> Option<i64>
    {
        match self {
            Value::Int(a) => Some(*a),
            Value::Float(a) => Some(*a as i64),
            _ => None,
        }
    }

    /// Converts the value to a floating-point number if the value is an integer type or
    /// floating-point type, otherwise this method returns `None`.
    pub fn to_opt_f32(&self) -> Option<f32>
    {
        match self {
            Value::Int(a) => Some(*a as f32),
            Value::Float(a) => Some(*a),
            _ => None,
        }
    }

    /// Converts the value to a string if the value is a string type, otherwise this method
    /// returns `None`.
    pub fn to_opt_string(&self) -> Option<String>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => Some(s.clone()),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    /// Returns `true` if the value is a function or a built-in function, otherwise `false`.
    pub fn is_fun(&self) -> bool
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::Fun(_, _, _) | Object::BuiltinFun(_, _) => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
    
    /// Returns `true` if two values are equal with types, otherwise `false`.
    ///
    /// This method also compares types of two values for integer numbers and floating-point
    /// numbers. If two values are weak references, this method compares their pointers instead
    /// values. If two values are matrices, this method doesn't compare two values and returns
    /// `false`. 
    pub fn eq_with_types(&self, value: &Value) -> Result<bool>
    {
        match (self, value) {
            (Value::None, Value::None) => Ok(true),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Float(a), Value::Float(b)) => Ok(a == b),
            (Value::Object(object), Value::Object(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                object.priv_eq(&**object2)
            },
            (Value::Ref(object), Value::Ref(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_eq(&*object2_g, Self::eq_with_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Ok(Arc::ptr_eq(&object, &object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => Ok(false),
        }
    }

    /// Returns `true` if two values are equal without types, otherwise `false`.
    ///
    /// This method doesn't compare types of two values for integer numbers and floating-point
    /// numbers. If two values are weak references, this method compares their pointers instead
    /// values. If two values are matrices, this method doesn't compare two values and returns
    /// `false`.
    pub fn eq_without_types(&self, value: &Value) -> Result<bool>
    {
        match (self, value) {
            (Value::None, Value::None) => Ok(true),
            (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
            (Value::Int(a), Value::Int(b)) => Ok(a == b),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(self.to_f32() == value.to_f32()),
            (Value::Object(object), Value::Object(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                object.priv_eq(&**object2)
            },
            (Value::Ref(object), Value::Ref(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_eq(&*object2_g, Self::eq_without_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Ok(Arc::ptr_eq(&object, &object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => Ok(false),
        }
    }
    
    /// Returns `true` if two values are nearly equal with types, otherwise `false`.
    ///
    /// If absolute difference between two numbers is less than or equal to the epsilon, two
    /// numbers are nearly equal for floating-point numbers, matrix arrays, and matrix row slices.
    /// Other values are compered as for [`eq_with_types`](Self::eq_with_types). 
    pub fn nearly_eq_with_types(&self, value: &Value, eps: f32) -> Result<bool>
    {
        match (self, value) {
            (Value::Float(a), Value::Float(b)) => Ok(nearly_eq(*a, *b, eps)),
            (Value::Object(object), Value::Object(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                object.priv_nearly_eq(&**object2, eps)
            },
            (Value::Ref(object), Value::Ref(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_nearly_eq(&*object2_g, eps, Self::nearly_eq_with_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Ok(Arc::ptr_eq(&object, &object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => self.eq_with_types(value),
        }
    }

    /// Returns `true` if two values are nearly equal without types, otherwise `false`.
    ///
    /// If absolute difference between two numbers is less than or equal to the epsilon, two
    /// numbers are nearly equal for numbers, matrix arrays, and matrix row slices. Other values
    /// are compered as for [`eq_without_types`](Self::eq_without_types). 
    pub fn nearly_eq_without_types(&self, value: &Value, eps: f32) -> Result<bool>
    {
        match (self, value) {
            (Value::Int(a), Value::Int(b)) => Ok(nearly_eq(*a as f32, *b as f32, eps)),
            (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(nearly_eq(self.to_f32(), value.to_f32(), eps)),
            (Value::Object(object), Value::Object(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                object.priv_nearly_eq(&**object2, eps)
            },
            (Value::Ref(object), Value::Ref(object2)) => {
                if Arc::ptr_eq(object, object2) {
                    return Ok(true);
                }
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                object_g.priv_nearly_eq(&*object2_g, eps, Self::nearly_eq_without_types)
            },
            (Value::Weak(object), Value::Weak(object2)) => {
                match (object.upgrade(), object2.upgrade()) {
                    (Some(object), Some(object2)) => Ok(Arc::ptr_eq(&object, &object2)),
                    (None, None) => Ok(true),
                    (_, _) => Ok(false),
                }
            },
            (_, _) => self.eq_without_types(value),
        }
    }    
    
    fn dot1_for_elem_with_fun_ref<F>(&self, err_msg: &str, f: &mut F) -> Result<Value>
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
            Value::Ref(_) => self.dot1_with_fun_ref(err_msg, f),
            _ => Ok(self.clone()),
        }
    }
    
    fn dot1_with_fun_ref<F>(&self, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    {
        match self {
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(elems) => {
                        let mut new_elems: Vec<Value> = Vec::new();
                        for elem in elems {
                            new_elems.push(elem.dot1_for_elem_with_fun_ref(err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(new_elems)))))
                    },
                    MutObject::Struct(fields) => {
                        let mut new_fields: BTreeMap<String, Value> = BTreeMap::new();
                        for (ident, field) in fields {
                            new_fields.insert(ident.clone(), field.dot1_for_elem_with_fun_ref(err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(new_fields)))))
                    },
                }
            },
            _ => Err(Error::Interp(String::from(err_msg))),
        }
    }
    
    /// Applies the function with one argument for a dot operation.
    ///
    /// If one element of one value or one field of one value is a floating-point number or a
    /// matrix, this method applies the function to one argument for this element or this field.
    /// If this element or this field is an array or a structure, this method recursively invokes
    /// itself for this element or this field. This method ignores this element or this field
    /// otherwise. This method returns an error with the error message if one value isn't an
    /// array or an structure.
    pub fn dot1<F>(&self, err_msg: &str, mut f: F) -> Result<Value>
        where F: FnMut(&Value) -> Result<Value>
    { self.dot1_with_fun_ref(err_msg, &mut f) }

    fn dot2_for_elem_with_fun_ref<F>(&self, value: &Value, err_msg: &str, f: &mut F) -> Result<Value>
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
            (Value::Ref(_), Value::Ref(_)) => self.dot2_with_fun_ref(value, err_msg, f),
            (Value::Weak(_), Value::Weak(_)) => Err(Error::Interp(String::from("two values are weak references"))),
            (_, _) => {
                if !self.eq_with_types(value)? {
                    return Err(Error::Interp(String::from("two values aren't equal")))
                }
                Ok(self.clone())
            },
        }
    }
    
    fn dot2_with_fun_ref<F>(&self, value: &Value, err_msg: &str, f: &mut F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    {
        match (self, value) {
            (Value::Ref(object), Value::Ref(object2)) => {
                let object_g = rw_lock_read(&**object)?;
                let object2_g = rw_lock_read(&**object2)?;
                match (&*object_g, &*object2_g) {
                    (MutObject::Array(elems), MutObject::Array(elems2)) => {
                        if elems.len() != elems2.len() {
                            return Err(Error::Interp(String::from("lengths of two arrays aren't equal")));
                        }
                        let mut new_elems: Vec<Value> = Vec::new();
                        for (elem, elem2) in elems.iter().zip(elems2.iter()) {
                            new_elems.push(elem.dot2_for_elem_with_fun_ref(elem2, err_msg, f)?);
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(new_elems)))))
                    },
                    (MutObject::Struct(fields), MutObject::Struct(fields2)) => {
                        let idents: BTreeSet<&String> = fields.keys().collect();
                        let idents2: BTreeSet<&String> = fields2.keys().collect();
                        if idents != idents2 {
                            return Err(Error::Interp(String::from("field names of two structures aren't equal")));
                        }
                        let mut new_fields: BTreeMap<String, Value> = BTreeMap::new();
                        for ident in &idents {
                            match (fields.get(*ident), fields2.get(*ident)) {
                                (Some(field), Some(field2)) => {
                                    new_fields.insert((*ident).clone(), field.dot2_for_elem_with_fun_ref(field2, err_msg, f)?);
                                },
                                (_, _) => return Err(Error::Interp(String::from("no field value"))),
                            }
                        }
                        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(new_fields)))))
                    },
                    (_, _) => Err(Error::Interp(String::from("two types aren't equal"))),
                }
            },
            (_, _) => Err(Error::Interp(String::from(err_msg))),
        }
    }

    /// Applies the function with two arguments for a dot operation.
    ///
    /// If two elements of two values or two fields of two values are floating-point numbers or
    /// matrices, this method applies the function to two arguments for these elements or these
    /// fields. If these elements or these fields are arrays or structures, this method
    /// recursively invokes itself for these elements or these fields. This method compares these
    /// elements or these fields otherwise. If these elements or these fields aren't equal, this
    /// method returns an error. This method returns an error with the error message if two
    /// values aren't arrays or structures.
    pub fn dot2<F>(&self, value: &Value, err_msg: &str, mut f: F) -> Result<Value>
        where F: FnMut(&Value, &Value) -> Result<Value>
    { self.dot2_with_fun_ref(value, err_msg, &mut f) }

    /// Applies the function to the arguments.
    ///
    /// See [`Interp::apply_fun`].
    pub fn apply(&self, interp: &mut Interp, env: &mut Env, arg_values: &[Value]) -> Result<Value>
    { interp.apply_fun(env, self, arg_values) }
    
    /// Returns the element or the field if the value has the element or the field, otherwise
    /// `None` or an error.
    ///
    /// If the value isn't a string, a matrix array, a matrix row slice, or a mutable object,
    /// this method returns an error.
    pub fn elem(&self, idx_value: &Value) -> Result<Value>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i < 1 || i > (s.chars().count() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match s.chars().nth((i - 1) as usize) {
                                    Some(c) => {
                                        let mut t = String::new();
                                        t.push(c);
                                        Ok(Value::Object(Arc::new(Object::String(t))))
                                    }
                                    None => Err(Error::Interp(String::from("no character"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                    Object::MatrixArray(row_count, _, _, _) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i < 1 || i > (*row_count as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                Ok(Value::Object(Arc::new(Object::MatrixRowSlice(object.clone(), (i - 1) as usize))))
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                    Object::MatrixRowSlice(matrix_array, i) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let j = idx_value.to_i64();
                                match &**matrix_array {
                                    Object::MatrixArray(row_count, col_count, transpose_flag, xs) => {
                                        if j < 1 || j > (*col_count as i64) {
                                            return Err(Error::Interp(String::from("index out of bounds")));
                                        }
                                        let k = match transpose_flag {
                                            TransposeFlag::NoTranspose => i * (*col_count) + ((j - 1) as usize),
                                            TransposeFlag::Transpose => ((j - 1) as usize) * (*row_count) + i,
                                        };
                                        match xs.get(k) {
                                            Some(x) => Ok(Value::Float(*x)),
                                            None => Err(Error::Interp(String::from("no element"))),
                                        }
                                    },
                                    _ => Err(Error::Interp(String::from("invalid matrix array type"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported type for indexing"))),
                }
            },
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Array(elems) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i < 1 || i > (elems.len() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match elems.get((i - 1) as usize) { 
                                    Some(elem) => Ok(elem.clone()),
                                    None => Err(Error::Interp(String::from("no element"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                    MutObject::Struct(fields) => {
                        match idx_value {
                            Value::Object(idx_object) => {
                                match &**idx_object {
                                    Object::String(ident) => {
                                        match fields.get(ident) {
                                            Some(field) => Ok(field.clone()),
                                            None => Err(Error::Interp(String::from("not found key")))
                                        }
                                    },
                                    _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported type for indexing"))),
        }
    }

    /// Sets the element or the field for the value.
    ///
    /// If the value isn't a mutable object, this method returns an error.
    pub fn set_elem(&self, idx_value: &Value, value: Value) -> Result<()>
    {
        match self {
            Value::Ref(object) => {
                let mut object_g = rw_lock_write(&**object)?;
                match &mut *object_g {
                    MutObject::Array(elems) => {
                        match idx_value {
                            Value::Int(_) | Value::Float(_) => {
                                let i = idx_value.to_i64();
                                if i < 1 || i > (elems.len() as i64) {
                                    return Err(Error::Interp(String::from("index out of bounds")));
                                }
                                match elems.get_mut((i - 1) as usize) {
                                    Some(elem) => {
                                        *elem = value;
                                        Ok(())
                                    }
                                    None => Err(Error::Interp(String::from("no element"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                    MutObject::Struct(fields) => {
                        match idx_value {
                            Value::Object(idx_object) => {
                                match &**idx_object {
                                    Object::String(ident) => {
                                        fields.insert(ident.clone(), value);
                                        Ok(())
                                    },
                                    _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                                }
                            },
                            _ => Err(Error::Interp(String::from("unsupported index type for indexing"))),
                        }
                    },
                }
            },
            _ => Err(Error::Interp(String::from("unsupported type for indexing"))),
        }
    }

    /// Returns the field if the value has the field, otherwise `None` or an error.
    ///
    /// If the value isn't a structure, this method returns an error.
    pub fn field(&self, ident: &String) -> Result<Value>
    {
        match self {
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object)?;
                match &*object_g {
                    MutObject::Struct(fields) => {
                        match fields.get(ident) {
                            Some(field) => Ok(field.clone()),
                            None => Err(Error::Interp(format!("structure hasn't field {}", ident))),
                        }
                    },
                    _ => Err(Error::Interp(format!("unsupported type for field {}", ident))),
                }
            },
            _ => Err(Error::Interp(format!("unsupported type for field {}", ident))),
        }
    }

    /// Sets the field for the value.
    ///
    /// If the value isn't a structure, this method returns an error.
    pub fn set_field(&self, ident: String, value: Value) -> Result<()>
    {
        match self {
            Value::Ref(object) => {
                let mut object_g = rw_lock_write(&**object)?;
                match &mut *object_g {
                    MutObject::Struct(fields) => {
                        fields.insert(ident.clone(), value);
                        Ok(())
                    },
                    _ => Err(Error::Interp(format!("unsupported type for field {}", ident))),
                }
            },
            _ => Err(Error::Interp(format!("unsupported type for field {}", ident))),
        }
    }

    /// Performs an operation on one value for the unary operator.
    pub fn unary_op(&self, op: UnaryOp) -> Result<Value>
    {
        match op {
            UnaryOp::Neg => {
                match self {
                    Value::Int(a) => {
                        match a.checked_neg() {
                            Some(b) => Ok(Value::Int(b)),
                            None => Err(Error::Interp(String::from("overflow in negation"))),
                        }
                    },
                    Value::Float(a) => Ok(Value::Float(-a)),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(a, 0.0)?)))),
                            _ => Err(Error::Interp(String::from("unsupported type for negation"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported type for negation"))),
                }
            },
            UnaryOp::DotNeg => {
                match self {
                    Value::Int(_) | Value::Float(_) => Ok(Value::Float(-self.to_f32())),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(a, 0.0)?)))),
                            _ => Err(Error::Interp(String::from("unsupported type for dot negation"))),
                        }
                    },
                    _ => self.dot1("unsupported type for dot negation", |v| v.unary_op(op)),
                }
            },
            UnaryOp::Not => Ok(Value::Bool(!self.to_bool())),
            UnaryOp::Transpose => {
                match self {
                    Value::Int(_) | Value::Float(_) => Ok(self.clone()),
                    Value::Object(object) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(a.transpose())))),
                            _ => Err(Error::Interp(String::from("unsupported type for transpose"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported type for transpose"))),
                }
            },
        }
    }

    /// Performs an operation on two values for the binary operator.
    pub fn bin_op(&self, op: BinOp, value: &Value) -> Result<Value>
    {
        match op {
            BinOp::Index => self.elem(value),
            BinOp::Mul => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_mul(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow in multiplication"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() * value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for multiplication"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for multiplication"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for multiplication"))),
                        }
                    },
                    _ => Err(Error::Interp(String::from("unsupported types for multiplication"))),
                }
            },
            BinOp::DotMul => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() * value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot multiplication"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot multiplication"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_elems(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot multiplication"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.dot1("unsupported types for dot multiplication", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.dot1("unsupported types for dot multiplication", |v| self.bin_op(op, v)),
                    _ => self.dot2(value, "unsupported types for dot multiplication", |v, w| v.bin_op(op, w)),
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
                                    Err(Error::Interp(String::from("overflow in division")))
                                }
                            },
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() / value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_div_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for division"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rdiv_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for division"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported types for division"))),
                }
            },
            BinOp::DotDiv => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() / value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot division"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_mul_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot division"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_div_elems(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot division"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.dot1("unsupported types for dot division", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.dot1("unsupported types for dot division", |v| self.bin_op(op, v)),
                    (_, _) => self.dot2(value, "unsupported types for dot division", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Add => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_add(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow in addition"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() + value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for addition"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for addition"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::String(s), Object::String(t)) => Ok(Value::Object(Arc::new(Object::String(s.clone() + t.as_str())))),
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for addition"))),
                        }
                    },
                    (Value::Ref(object), Value::Ref(object2)) => {
                        let object_g = rw_lock_read(&**object)?;
                        let object2_g = rw_lock_read(&**object2)?;
                        match (&*object_g, &*object2_g) {
                            (MutObject::Array(elems), MutObject::Array(elems2)) => {
                                let mut new_elems = elems.clone();
                                new_elems.extend_from_slice(elems2.as_slice());
                                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(new_elems)))))
                            },
                            (MutObject::Struct(fields), MutObject::Struct(fields2)) => {
                                let mut new_fields: BTreeMap<String, Value> = BTreeMap::new();
                                let idents: BTreeSet<&String> = fields.keys().collect();
                                let idents2: BTreeSet<&String> = fields2.keys().collect();
                                let idents3: Vec<&String> = idents.union(&idents2).map(|s| *s).collect();
                                for ident in &idents3 {
                                    match fields.get(*ident) {
                                        Some(field) => {
                                            new_fields.insert((*ident).clone(), field.clone());
                                        },
                                        None => {
                                            match fields2.get(*ident) {
                                                Some(field2) => {
                                                    new_fields.insert((*ident).clone(), field2.clone());
                                                },
                                                None => return Err(Error::Interp(String::from("no field"))),
                                            }
                                        },
                                    }
                                }
                                Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(new_fields)))))
                            },
                            _ => Err(Error::Interp(String::from("unsupported types for addition"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported types for addition"))),
                }
            },
            BinOp::DotAdd => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() + value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot addition"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot addition"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_add(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot addition"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.dot1("unsupported types for dot addition", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.dot1("unsupported types for dot addition", |v| self.bin_op(op, v)),
                    (_, _) => self.dot2(value, "unsupported types for dot addition", |v, w| v.bin_op(op, w)),
                }
            },
            BinOp::Sub => {
                match (self, value) {
                    (Value::Int(a), Value::Int(b)) => {
                        match a.checked_sub(*b) {
                            Some(c) => Ok(Value::Int(c)),
                            None => Err(Error::Interp(String::from("overflow in subtraction"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() - value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for subtraction"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for subtraction"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for subtraction"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported types for subtraction"))),
                }
            },
            BinOp::DotSub => {
                match (self, value) {
                    (Value::Int(_) | Value::Float(_), Value::Int(_) | Value::Float(_)) => Ok(Value::Float(self.to_f32() - value.to_f32())),
                    (Value::Object(object), Value::Int(_) | Value::Float(_)) => {
                        match &**object {
                            Object::Matrix(a) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub_for_scalar(a, value.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot subtraction"))),
                        }
                    },
                    (Value::Int(_) | Value::Float(_), Value::Object(object2)) => {
                        match &**object2 {
                            Object::Matrix(b) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_rsub_for_scalar(b, self.to_f32())?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot subtraction"))),
                        }
                    },
                    (Value::Object(object), Value::Object(object2)) => {
                        match (&**object, &**object2) {
                            (Object::Matrix(a), Object::Matrix(b)) => Ok(Value::Object(Arc::new(Object::Matrix(matrix_sub(a, b)?)))),
                            _ => Err(Error::Interp(String::from("unsupported types for dot subtraction"))),
                        }
                    },
                    (Value::Ref(_), Value::Int(_) | Value::Float(_)) => self.dot1("unsupported types for dot subtraction", |v| v.bin_op(op, value)),
                    (Value::Int(_) | Value::Float(_), Value::Ref(_)) => value.dot1("unsupported types for dot subtraction", |v| self.bin_op(op, v)),
                    (_, _) => self.dot2(value, "unsupported types for dot subtraction", |v, w| v.bin_op(op, w)),
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
                            (_, _) => Err(Error::Interp(String::from("unsupported types for comparison"))),
                        }
                    },
                    (_, _) => Err(Error::Interp(String::from("unsupported types for comparison"))),
                }
            },
            BinOp::Ge => Ok(Value::Bool(!self.bin_op(BinOp::Lt, value)?.to_bool())),
            BinOp::Gt => Ok(Value::Bool(value.bin_op(BinOp::Lt, self)?.to_bool())),
            BinOp::Le => Ok(Value::Bool(!value.bin_op(BinOp::Lt, self)?.to_bool())),
            BinOp::Eq => Ok(Value::Bool(self.eq_without_types(value)?)),
            BinOp::Ne => Ok(Value::Bool(!self.bin_op(BinOp::Eq, value)?.to_bool())),
        }
    }
    
    /// Returns an interator if the value is iterable, otherwise `None`.
    pub fn iter(&self) -> Result<Option<Iter<'_>>>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => Ok(Some(Iter::new(IterEnum::String(s.chars())))),
                    Object::IntRange(a, b, c) => Ok(Some(Iter::new(IterEnum::IntRange(*a, *b, *c, false)))),
                    Object::FloatRange(a, b, c) => Ok(Some(Iter::new(IterEnum::FloatRange(*a, *b, *c, false)))),
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

    /// Converts the value to a matrix array.
    ///
    /// If the value isn't a matrix or a matrix array, this method returns an error.
    pub fn to_matrix_array(&self) -> Result<Value>
    {
        match self {
            Value::Object(object) => {
                match &**object {
                    Object::Matrix(a) => Ok(Value::Object(Arc::new(matrix_to_matrix_array(a)?))),
                    Object::MatrixArray(_, _, _, _) => Ok(self.clone()),
                    _ => Err(Error::Interp(String::from("unsupported type for conversion to matrix array"))),
                }
            },
            _ => Err(Error::Interp(String::from("unsupported type for conversion to matrix array"))),
        }
    }

    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize, is_width: bool) -> fmt::Result
    {
        let width = if is_width { 11 } else { 0 };
        match self {
            Value::None => write!(f, "{:>width$}", "none")?,
            Value::Bool(false) => write!(f, "{:>width$}", "false")?,
            Value::Bool(true) => write!(f, "{:>width$}", "true")?,
            Value::Int(a) => write!(f, "{:>width$}", a)?,
            Value::Float(a) => {
                if a.floor() == *a {
                    if format!("{}", a).len() > 11 {
                        write!(f, "{:>width$.4e}", a)?;
                    } else {
                        write!(f, "{:>width$}", a)?;
                    }
                } else {
                    if format!("{:.4}", a).len() > 11 || (a.abs() < 0.0001 && *a != 0.0) {
                        write!(f, "{:>width$.4e}", a)?;
                    } else {
                        write!(f, "{:>width$.4}", a)?;
                    }
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
                        if *row_count > 0 && *col_count > 0 { 
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
                            _ => write!(f, "[]")?,
                        }
                    },
                    Object::Error(_, msg) => write!(f, "{}", msg)?,
                    Object::WindowId(_) => write!(f, "windowid(...)")?,
                }
            },
            Value::Ref(object) => {
                let object_g = rw_lock_read(&**object).unwrap();
                match &*object_g {
                    MutObject::Array(elems) => {
                        if !elems.is_empty() {
                            let new_indent = indent + 4;
                            write!(f, ".[")?;
                            for elem in elems {
                                write!(f, " ")?;
                                elem.fmt_with_indent(f, new_indent, is_width)?;
                            }
                            write!(f, " .]")?;
                        } else {
                            write!(f, ".[.]")?;
                        }
                    },
                    MutObject::Struct(fields) => {
                        if !fields.is_empty() {
                            let new_indent = indent + 4;
                            writeln!(f, "{{")?;
                            for (ident, field) in fields {
                                write!(f, "{:new_indent$}{}: ", "", ident)?;
                                field.fmt_with_indent(f, new_indent, is_width)?;
                                writeln!(f, "")?;
                            }
                            write!(f, "{:indent$}}}", "")?;
                        } else {
                            write!(f, "{{}}")?;
                        }
                    },
                }
            },
            Value::Weak(object) => {
                match object.upgrade() {
                    Some(_) => write!(f, "weak(...)")?,
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

impl Not for Value
{
    type Output = Self;
    
    fn not(self) -> Self::Output
    { self.unary_op(UnaryOp::Not).unwrap() }
}

impl Not for &Value
{
    type Output = Value;
    
    fn not(self) -> Self::Output
    { self.unary_op(UnaryOp::Not).unwrap() }
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

impl Serialize for Value
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            Value::None => serializer.serialize_unit(),
            Value::Bool(a) => serializer.serialize_bool(*a),
            Value::Int(a) => serializer.serialize_i64(*a),
            Value::Float(a) => serializer.serialize_f32(*a),
            Value::Object(object) => {
                match &**object {
                    Object::String(s) => serializer.serialize_str(s.as_str()),
                    _ => Err(ser::Error::custom("unsupported type for serialization")),
                }
            },
            Value::Ref(object) => {
                let object_g = match rw_lock_read(&**object) {
                    Ok(tmp_object_g) => tmp_object_g,
                    Err(err) => return Err(ser::Error::custom(format!("{}", err))),
                };
                match &*object_g {
                    MutObject::Array(elems) => {
                        let mut seq = serializer.serialize_seq(Some(elems.len()))?;
                        for elem in elems {
                            seq.serialize_element(elem)?;
                        }
                        seq.end()
                    },
                    MutObject::Struct(fields) => {
                        let mut map = serializer.serialize_map(Some(fields.len()))?;
                        for (ident, field) in fields {
                            map.serialize_entry(ident, field)?;
                        }
                        map.end()
                    },
                }
            },
            _ => Err(ser::Error::custom("unsupported type for serialization")),
        }
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor
{
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    { write!(formatter, "a value") }

    fn visit_unit<E>(self) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::None) }

    fn visit_bool<E>(self, v: bool) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::Bool(v)) }

    fn visit_i64<E>(self, v: i64) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::Int(v)) }

    fn visit_u64<E>(self, v: u64) -> result::Result<Self::Value, E>
        where E: de::Error
    { 
        if v <= (i64::MAX as u64) {
            Ok(Value::Int(v as i64))
        } else {
            Err(E::custom(String::from("too large integer number")))
        }
    }
    
    fn visit_f32<E>(self, v: f32) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::Float(v)) }

    fn visit_f64<E>(self, v: f64) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::Float(v as f32)) }
    
    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
        where E: de::Error
    { Ok(Value::Object(Arc::new(Object::String(String::from(v))))) }
    
    fn visit_seq<A>(self, mut seq: A) -> result::Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
    {
        let mut elems: Vec<Value> = Vec::new();
        loop {
            match seq.next_element()? {
                Some(elem) => elems.push(elem),
                None => break,
            }
        }
        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems)))))
    }
    
    fn visit_map<A>(self, mut map: A) -> result::Result<Self::Value, A::Error>
        where A: MapAccess<'de>
    {
        let mut fields: BTreeMap<String, Value> = BTreeMap::new();
        loop {
            match map.next_key()? {
                Some(ident) => {
                    fields.insert(ident, map.next_value()?);
                },
                None => break,
            }
        }
        Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields)))))
    }
}

impl<'de> Deserialize<'de> for Value
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    { deserializer.deserialize_any(ValueVisitor) }
}

/// An enumeration of immutable object.
#[derive(Clone, Debug)]
pub enum Object
{
    /// A string.
    String(String),
    /// An integer number range.
    IntRange(i64, i64, i64),
    /// A floating-point number range.
    FloatRange(f32, f32, f32),
    /// A matrix.
    Matrix(Matrix),
    /// A function.
    Fun(Vec<String>, String, Arc<Fun>),
    /// A built-in function.
    BuiltinFun(String, fn(&mut Interp, &mut Env, &[Value]) -> Result<Value>),
    /// A matrix array.
    MatrixArray(usize, usize, TransposeFlag, Vec<f32>),
    /// A matrix row slice.
    MatrixRowSlice(Arc<Object>, usize),
    /// An error.
    Error(String, String),
    /// A window identifier.
    WindowId(WindowId),
}

impl Object
{
    fn priv_eq(&self, object: &Object) -> Result<bool>
    {
        match (self, object) {
            (Object::String(s), Object::String(t)) => Ok(s == t),
            (Object::IntRange(a, b, c), Object::IntRange(d, e, f)) => Ok(a == d && b == e && c == f),
            (Object::FloatRange(a, b, c), Object::FloatRange(d, e, f)) => Ok(a == d && b == e && c == f),
            (Object::Fun(idents, ident, fun), Object::Fun(idents2, ident2, fun2)) => Ok(idents == idents2 && ident == ident2 && Arc::ptr_eq(fun, fun2)),
            (Object::BuiltinFun(ident, f), Object::BuiltinFun(ident2, g)) => Ok(ident == ident2 && f == g),
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
            (Object::MatrixRowSlice(matrix_array, ai), Object::MatrixRowSlice(matrix_array2, bi)) => {
                match (&**matrix_array, &**matrix_array2) {
                    (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                        if a_col_count != b_col_count {
                            return Ok(false);
                        }
                        for j in 0..(*a_col_count) {
                            let ak = match a_transpose_flag {
                                TransposeFlag::NoTranspose => (*ai) * (*a_col_count) + j,
                                TransposeFlag::Transpose => j * (*a_row_count) + (*ai),
                            };
                            let bk = match b_transpose_flag {
                                TransposeFlag::NoTranspose => (*bi) * (*b_col_count) + j,
                                TransposeFlag::Transpose => j * (*b_row_count) + (*bi),
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
                    (_, _) => return Err(Error::Interp(String::from("invalid matrix array type")))
                }
            },
            (Object::Error(kind, msg), Object::Error(kind2, msg2)) => Ok(kind == kind2 && msg == msg2),
            (Object::WindowId(window_id), Object::WindowId(window_id2)) => Ok(window_id == window_id2),
            (_, _) => Ok(false),
        }
    }

    fn priv_nearly_eq(&self, object: &Object, eps: f32) -> Result<bool>
    {
        match (self, object) {
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
                                if !nearly_eq(*x, *y, eps) {
                                    return Ok(false);
                                }
                            },
                            (_, _) => return Err(Error::Interp(String::from("no element"))),
                        }
                    }
                }
                Ok(true)
            },
            (Object::MatrixRowSlice(matrix_array, ai), Object::MatrixRowSlice(matrix_array2, bi)) => {
                match (&**matrix_array, &**matrix_array2) {
                    (Object::MatrixArray(a_row_count, a_col_count, a_transpose_flag, xs), Object::MatrixArray(b_row_count, b_col_count, b_transpose_flag, ys)) => {
                        if a_col_count != b_col_count {
                            return Ok(false);
                        }
                        for j in 0..(*a_col_count) {
                            let ak = match a_transpose_flag {
                                TransposeFlag::NoTranspose => (*ai) * (*a_col_count) + j,
                                TransposeFlag::Transpose => j * (*a_row_count) + (*ai),
                            };
                            let bk = match b_transpose_flag {
                                TransposeFlag::NoTranspose => (*bi) * (*b_col_count) + j,
                                TransposeFlag::Transpose => j * (*b_row_count) + (*bi),
                            };
                            match (xs.get(ak), ys.get(bk)) {
                                (Some(x), Some(y)) => {
                                    if !nearly_eq(*x, *y, eps) {
                                        return Ok(false);
                                    }
                                },
                                (_, _) => return Err(Error::Interp(String::from("no element"))),
                            }
                        }
                        Ok(true)
                    },
                    (_, _) => return Err(Error::Interp(String::from("invalid matrix array type")))
                }
            },
            (_, _) => self.priv_eq(object),
        }
    }
}

/// An enumeration of transpose flag.
///
/// The transpose flag determines whether a matrix array is transposed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TransposeFlag
{
    /// No transpose, e.i. a matrix array isn't transposed.
    NoTranspose,
    /// Transpose, e.i. a matrix array is transposed.
    Transpose,
}

/// An enumeration of mutable object.
#[derive(Clone, Debug)]
pub enum MutObject
{
    /// An array.
    Array(Vec<Value>),
    /// A structure.
    Struct(BTreeMap<String, Value>),
}

impl MutObject
{
    fn priv_eq<F>(&self, object: &MutObject, mut f: F) -> Result<bool>
        where F: FnMut(&Value, &Value) -> Result<bool>
    {
        match (self, object) {
            (MutObject::Array(elems), MutObject::Array(elems2)) => {
                if elems.len() != elems2.len() {
                    return Ok(false);
                }
                for (elem, elem2) in elems.iter().zip(elems2.iter()) {
                    if !f(elem, elem2)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            (MutObject::Struct(fields), MutObject::Struct(fields2)) => {
                let idents: BTreeSet<&String> = fields.keys().collect();
                let idents2: BTreeSet<&String> = fields2.keys().collect();
                if idents != idents2 {
                    return Ok(false);
                }
                for ident in &idents {
                    match (fields.get(*ident), fields2.get(*ident)) {
                        (Some(field), Some(field2)) => {
                            if !f(field, field2)? {
                                return Ok(false);
                            }
                        },
                        (_, _) => return Err(Error::Interp(String::from("no field"))),
                    }
                }
                Ok(true)
            },
            (_, _) => Ok(false),
        }
    }

    fn priv_nearly_eq<F>(&self, object: &MutObject, eps: f32, mut f: F) -> Result<bool>
        where F: FnMut(&Value, &Value, f32) -> Result<bool>
    {
        match (self, object) {
            (MutObject::Array(elems), MutObject::Array(elems2)) => {
                if elems.len() != elems2.len() {
                    return Ok(false);
                }
                for (elem, elem2) in elems.iter().zip(elems2.iter()) {
                    if !f(elem, elem2, eps)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            (MutObject::Struct(fields), MutObject::Struct(fields2)) => {
                let idents: BTreeSet<&String> = fields.keys().collect();
                let idents2: BTreeSet<&String> = fields2.keys().collect();
                if idents != idents2 {
                    return Ok(false);
                }
                for ident in &idents {
                    match (fields.get(*ident), fields2.get(*ident)) {
                        (Some(field), Some(field2)) => {
                            if !f(field, field2, eps)? {
                                return Ok(false);
                            }
                        },
                        (_, _) => return Err(Error::Interp(String::from("no field"))),
                    }
                }
                Ok(true)
            },
            (_, _) => Ok(false),
        }
    }
}

/// A structure of iterator of values.
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
            IterEnum::IntRange(from, to, step, is_stopped) => {
                if !*is_stopped {
                    let current = if *step > 0 {
                        let tmp_current = if *from <= *to {
                            Some(*from)
                        } else {
                            None
                        };
                        if *from < *to {
                            match from.checked_add(*step) {
                                Some(tmp_from) => *from = tmp_from,
                                None => {
                                    *is_stopped = true;
                                    return Some(Err(Error::Interp(String::from("overflow in iteration"))));
                                },
                            }
                        } else {
                            *is_stopped = true;
                        }
                        tmp_current
                    } else if *step < 0 {
                        let tmp_current = if *from >= *to {
                            Some(*from)
                        } else {
                            None
                        };
                        if *from > *to {
                            match from.checked_add(*step) {
                                Some(tmp_from) => *from = tmp_from,
                                None => {
                                    *is_stopped = true;
                                    return Some(Err(Error::Interp(String::from("overflow in iteration"))));
                                },
                            }
                        } else {
                            *is_stopped = true;
                        }
                        tmp_current
                    } else {
                        *is_stopped = true;
                        return Some(Err(Error::Interp(String::from("range step is zero"))));
                    };
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
                    let current = if *step > 0.0 {
                        let tmp_current = if *from <= *to {
                            Some(*from)
                        } else {
                            None
                        };
                        if *from < *to {
                            *from += *step;
                        } else {
                            *is_stopped = true;
                        }
                        tmp_current
                    } else if *step < 0.0 {
                        let tmp_current = if *from >= *to {
                            Some(*from)
                        } else {
                            None
                        };
                        if *from > *to {
                            *from += *step;
                        } else {
                            *is_stopped = true;
                        }
                        tmp_current
                    } else {
                        *is_stopped = true;
                        return Some(Err(Error::Interp(String::from("range step is zero"))));
                    };
                    match current {
                        Some(current) => Some(Ok(Value::Float(current))),
                        None => None,
                    }
                } else {
                    None
                }
            },
            IterEnum::MatrixArray(matrix_array, i, is_stopped) => {
                if !*is_stopped {
                    match &**matrix_array {
                        Object::MatrixArray(row_count, _, _, _) => {
                            if *i < *row_count {
                                let j = *i;
                                *i += 1;
                                Some(Ok(Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array.clone(), j)))))
                            } else {
                                None
                            }
                        },
                        _ => {
                            *is_stopped = true;
                            Some(Err(Error::Interp(String::from("invalid matrix array type"))))
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
                            Some(Err(Error::Interp(String::from("invalid matrix array type"))))
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
                                MutObject::Array(elems) => {
                                    if *i < elems.len() {
                                        let j = *i;
                                        *i += 1;
                                        match elems.get(j) {
                                            Some(elem) => Some(Ok(elem.clone())),
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
                                    Some(Err(Error::Interp(String::from("invalid array type"))))
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
    String(Chars<'a>),
    IntRange(i64, i64, i64, bool),
    FloatRange(f32, f32, f32, bool),
    MatrixArray(Arc<Object>, usize, bool),
    MatrixRowSlice(Arc<Object>, usize, usize, bool),
    Array(Arc<RwLock<MutObject>>, usize, bool),
}

#[cfg(test)]
mod tests;
