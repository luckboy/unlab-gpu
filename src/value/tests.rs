//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::matrix;
use crate::tree::Fun;
use crate::test_helpers::*;
use super::*;

fn f(_interp: &mut Interp, _env: &mut Env, _arg_values: &[Value]) -> Result<Value>
{ Ok(Value::None) }

#[test]
fn test_value_eq_with_types_returns_true()
{
    match Value::None.eq_with_types(&Value::None) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Bool(true).eq_with_types(&Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_with_types(&Value::Int(1234)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).eq_with_types(&Value::Float(12.34)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    let value2 = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let at = vec![
        1.0, 3.0, 5.0,
        2.0, 4.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 1.0,
        2.0, 3.0,
        1.0, 1.0
    ];
    let at = vec![
        1.0, 2.0, 1.0,
        1.0, 3.0, 1.0
    ];
    let b = vec![
        2.0, 3.0,
        4.0, 4.0,
        4.0, 4.0,
        4.0, 4.0
    ];
    let bt = vec![
        2.0, 4.0, 4.0, 4.0,
        3.0, 4.0, 4.0, 4.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::Transpose, bt.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields.clone()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Weak(Arc::downgrade(&object));
    let value2 = Value::Weak(Arc::downgrade(&object));
    match value.eq_with_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_with_types_returns_false()
{
    match Value::Bool(true).eq_with_types(&Value::Bool(false)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_with_types(&Value::Int(4567)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).eq_with_types(&Value::Float(45.67)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(3, 4, 1)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(3, 4, 1)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 5, 1)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 4, 2)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(3.0, 4.5, 1.5)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 5.5, 1.5)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 2.5)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = matrix![[1.0, 2.0], [3.0, 4.0]];
    let value = Value::Object(Arc::new(Object::Matrix(a.clone())));
    let value2 = Value::Object(Arc::new(Object::Matrix(a)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun2 = Arc::new(Fun(Vec::new(), Vec::new()));
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("c")], String::from("f"), fun.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("g"), fun.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun2)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    let value2 = Value::Object(Arc::new(Object::BuiltinFun(String::from("g"), f)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let at = vec![
        1.0, 3.0, 5.0,
        2.0, 4.0, 6.0
    ];
    let b = vec![
        1.0, 2.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let bt = vec![
        1.0, 5.0, 7.0,
        2.0, 6.0, 8.0
    ];
    let c = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    ];
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(2, 3, TransposeFlag::NoTranspose, at.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, c.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, bt.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b.clone())));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 1.0,
        2.0, 3.0,
        1.0, 1.0
    ];
    let at = vec![
        1.0, 2.0, 1.0,
        1.0, 3.0, 1.0
    ];
    let b = vec![
        3.0, 4.0,
        1.0, 1.0,
        1.0, 1.0,
        1.0, 1.0
    ];
    let bt = vec![
        3.0, 1.0, 1.0, 1.0,
        4.0, 1.0, 1.0, 1.0
    ];
    let c = vec![
        1.0, 1.0, 1.0,
        2.0, 3.0, 4.0,
        1.0, 1.0, 1.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, c.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 1)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::Transpose, bt.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false), Value::Int(3)]))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0), Value::Bool(false)]))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(1));
    fields2.insert(String::from("b"), Value::Float(2.0));
    fields2.insert(String::from("c"), Value::Bool(false));
    fields2.insert(String::from("d"), Value::Int(2));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(1));
    fields2.insert(String::from("b"), Value::Float(3.0));
    fields2.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Weak(Arc::downgrade(&object));
    let object2 = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value2 = Value::Weak(Arc::downgrade(&object2));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_with_types_returns_false_for_different_types()
{
    match Value::Int(1234).eq_with_types(&Value::Float(12.34)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_without_types_returns_true()
{
    match Value::None.eq_without_types(&Value::None) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Bool(true).eq_without_types(&Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Int(1234)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Float(1234.0)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).eq_without_types(&Value::Int(1234)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).eq_without_types(&Value::Float(12.34)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    let value2 = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let at = vec![
        1.0, 3.0, 5.0,
        2.0, 4.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 1.0,
        2.0, 3.0,
        1.0, 1.0
    ];
    let at = vec![
        1.0, 2.0, 1.0,
        1.0, 3.0, 1.0
    ];
    let b = vec![
        2.0, 3.0,
        4.0, 4.0,
        4.0, 4.0,
        4.0, 4.0
    ];
    let bt = vec![
        2.0, 4.0, 4.0, 4.0,
        3.0, 4.0, 4.0, 4.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::Transpose, bt.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields.clone()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Weak(Arc::downgrade(&object));
    let value2 = Value::Weak(Arc::downgrade(&object));
    match value.eq_without_types(&value2) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_without_types_returns_false()
{
    match Value::Bool(true).eq_without_types(&Value::Bool(false)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Int(4567)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Float(4567.0)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).eq_without_types(&Value::Int(4567)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).eq_without_types(&Value::Float(45.67)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(3, 4, 1)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(3, 4, 1)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 5, 1)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::IntRange(2, 4, 2)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(3.0, 4.5, 1.5)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 5.5, 1.5)));
    match value.eq_with_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 2.5)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = matrix![[1.0, 2.0], [3.0, 4.0]];
    let value = Value::Object(Arc::new(Object::Matrix(a.clone())));
    let value2 = Value::Object(Arc::new(Object::Matrix(a)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun2 = Arc::new(Fun(Vec::new(), Vec::new()));
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("c")], String::from("f"), fun.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("g"), fun.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    let value2 = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun2)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    let value2 = Value::Object(Arc::new(Object::BuiltinFun(String::from("g"), f)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let at = vec![
        1.0, 3.0, 5.0,
        2.0, 4.0, 6.0
    ];
    let b = vec![
        1.0, 2.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let bt = vec![
        1.0, 5.0, 7.0,
        2.0, 6.0, 8.0
    ];
    let c = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    ];
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(2, 3, TransposeFlag::NoTranspose, at.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, c.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, bt.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone())));
    let value2 = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b.clone())));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 1.0,
        2.0, 3.0,
        1.0, 1.0
    ];
    let at = vec![
        1.0, 2.0, 1.0,
        1.0, 3.0, 1.0
    ];
    let b = vec![
        3.0, 4.0,
        1.0, 1.0,
        1.0, 1.0,
        1.0, 1.0
    ];
    let bt = vec![
        3.0, 1.0, 1.0, 1.0,
        4.0, 1.0, 1.0, 1.0
    ];
    let c = vec![
        1.0, 1.0, 1.0,
        2.0, 3.0, 4.0,
        1.0, 1.0, 1.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, c.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 1)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::Transpose, bt.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::Transpose, at.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 2, TransposeFlag::NoTranspose, b.clone()));
    let value2 = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array2, 0)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false), Value::Int(3)]))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0), Value::Bool(false)]))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(1));
    fields2.insert(String::from("b"), Value::Float(2.0));
    fields2.insert(String::from("c"), Value::Bool(false));
    fields2.insert(String::from("d"), Value::Int(2));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(1));
    fields2.insert(String::from("b"), Value::Float(3.0));
    fields2.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Weak(Arc::downgrade(&object));
    let object2 = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value2 = Value::Weak(Arc::downgrade(&object2));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_without_types_returns_false_for_different_types()
{
    match Value::Int(1234).eq_with_types(&Value::Bool(false)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    let value2 = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.eq_without_types(&value2) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_calculates_result_for_equal_operator()
{
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Int(1234)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Float(1234.0)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Eq, &Value::Int(1234)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).bin_op(BinOp::Eq, &Value::Float(12.34)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Int(4567)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Float(4567.0)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Eq, &Value::Int(4567)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).bin_op(BinOp::Eq, &Value::Float(45.67)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_calculates_result_for_not_equal_operator()
{
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Int(1234)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Float(1234.0)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Ne, &Value::Int(1234)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).bin_op(BinOp::Ne, &Value::Float(12.34)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Int(4567)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Float(4567.0)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Ne, &Value::Int(4567)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(12.34).bin_op(BinOp::Ne, &Value::Float(45.67)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_eq_returns_true()
{
    assert_eq!(true, Value::Int(1234) == Value::Int(1234));
    assert_eq!(true, Value::Int(1234) == Value::Float(1234.0));
    assert_eq!(true, Value::Float(1234.0) == Value::Int(1234));
    assert_eq!(true, Value::Float(12.34) == Value::Float(12.34));
}

#[test]
fn test_value_eq_returns_false()
{
    assert_eq!(false, Value::Int(1234) == Value::Int(4567));
    assert_eq!(false, Value::Int(1234) == Value::Float(4567.0));
    assert_eq!(false, Value::Float(1234.0) == Value::Int(4567));
    assert_eq!(false, Value::Float(12.34) == Value::Float(45.67));
}

#[test]
fn test_value_dot1_calculates_result_for_values()
{
    let a = vec![
        1.0, 2.0,
        3.0, 4.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0
    ];
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2, Value::Float(1.0)]))));
    match value.dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Ok(value2) => {
            match &value2 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(3, elems.len());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 2, 2, f32::neg)));
                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 2, f32::neg)));
                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                            assert_eq!(Value::Float(-1.0), elems[2]);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let array_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2]))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), array_value);
    fields.insert(String::from("b"), Value::Float(1.0));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Ok(value2) => {
            match &value2 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Struct(fields) => {
                            match fields.get(&String::from("a")) {
                                Some(Value::Ref(object2)) => {
                                    let object2_g = object2.read().unwrap();
                                    match &*object2_g {
                                        MutObject::Array(elems) => {
                                            assert_eq!(2, elems.len());
                                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 2, 2, f32::neg)));
                                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 2, f32::neg)));
                                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match fields.get(&String::from("b")) {
                                Some(Value::Float(n)) => assert_eq!(-1.0, *n),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), matrix_value);
    fields.insert(String::from("b"), matrix_value2);
    let struct_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value, Value::Float(1.0)]))));
    match value.dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Ok(value2) => {
            match &value2 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(2, elems.len());
                            match &elems[0] {
                                Value::Ref(object2) => {
                                    let object2_g = object2.read().unwrap();
                                    match &*object2_g {
                                        MutObject::Struct(fields) => {
                                            match fields.get(&String::from("a")) {
                                                Some(field) => {
                                                    let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 2, 2, f32::neg)));
                                                    assert_eq!(Value::Object(matrix_array), field.to_matrix_array().unwrap());
                                                },
                                                _ => assert!(false),
                                            }
                                            match fields.get(&String::from("b")) {
                                                Some(field) => {
                                                    let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 2, f32::neg)));
                                                    assert_eq!(Value::Object(matrix_array), field.to_matrix_array().unwrap());
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(Value::Float(-1.0), elems[1]);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_dot1_complains_on_unsupported_type()
{
    match Value::Int(1).dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("some message"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_dot2_calculates_result_for_values()
{
    let a = vec![
        1.0, 2.0,
        3.0, 4.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0
    ];
    let c = vec![
        3.0, 6.0,
        7.0, 8.0
    ];
    let d = vec![
        7.0, 8.0,
        9.0, 10.0,
    ];
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2, Value::Float(1.0)]))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value3, matrix_value4, Value::Float(2.0)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Ok(value3) => {
            match &value3 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(3, elems.len());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), c.as_slice(), 2, 2, f32::add)));
                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(b.as_slice(), d.as_slice(), 2, 2, f32::add)));
                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                            assert_eq!(Value::Float(3.0), elems[2]);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let array_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2]))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), array_value);
    fields.insert(String::from("b"), Value::Float(1.0));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let array_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value3, matrix_value4]))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), array_value2);
    fields2.insert(String::from("b"), Value::Float(2.0));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Ok(value3) => {
            match &value3 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Struct(fields) => {
                            match fields.get(&String::from("a")) {
                                Some(Value::Ref(object2)) => {
                                    let object2_g = object2.read().unwrap();
                                    match &*object2_g {
                                        MutObject::Array(elems) => {
                                            assert_eq!(2, elems.len());
                                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), c.as_slice(), 2, 2, f32::add)));
                                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(b.as_slice(), d.as_slice(), 2, 2, f32::add)));
                                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match fields.get(&String::from("b")) {
                                Some(Value::Float(n)) => assert_eq!(3.0, *n),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let matrix_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, a.as_slice()))));
    let matrix_value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, b.as_slice()))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), matrix_value);
    fields.insert(String::from("b"), matrix_value2);
    let struct_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value, Value::Float(1.0)]))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), matrix_value3);
    fields2.insert(String::from("b"), matrix_value4);
    let struct_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value2, Value::Float(2.0)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Ok(value3) => {
            match &value3 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(2, elems.len());
                            match &elems[0] {
                                Value::Ref(object2) => {
                                    let object2_g = object2.read().unwrap();
                                    match &*object2_g {
                                        MutObject::Struct(fields) => {
                                            match fields.get(&String::from("a")) {
                                                Some(field) => {
                                                    let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), c.as_slice(), 2, 2, f32::add)));
                                                    assert_eq!(Value::Object(matrix_array), field.to_matrix_array().unwrap());
                                                },
                                                _ => assert!(false),
                                            }
                                            match fields.get(&String::from("b")) {
                                                Some(field) => {
                                                    let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(b.as_slice(), d.as_slice(), 2, 2, f32::add)));
                                                    assert_eq!(Value::Object(matrix_array), field.to_matrix_array().unwrap());
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(Value::Float(3.0), elems[1]);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_dot2_complains_on_unsupported_types()
{
    match Value::Int(1).dot2(&Value::Int(2), "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("some message"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_dot2_complains_on_two_types_are_not_equal()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(BTreeMap::new()))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("two types aren't equal"), *msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(BTreeMap::new()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("two types aren't equal"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_dot2_complains_on_lengths_of_two_arrays_are_not_equal()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("lengths of two arrays aren't equal"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_dot2_complains_on_field_names_of_two_structures_are_not_equal()
{
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(1));
    fields2.insert(String::from("c"), Value::Float(2.0));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("field names of two structures aren't equal"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_dot2_complains_on_value_is_weak_reference()
{
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Weak(Arc::downgrade(&object))]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("value is weak reference"), *msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Weak(Arc::downgrade(&object))]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("value is weak reference"), *msg),
        _ => assert!(false),
    }
}
