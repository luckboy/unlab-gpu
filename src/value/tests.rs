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
    match Value::Float(1234.0).eq_without_types(&Value::Int(1234)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Float(1234.0)) {
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
    match Value::Float(1234.0).eq_without_types(&Value::Int(4567)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).eq_without_types(&Value::Float(4567.0)) {
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
fn test_value_bin_op_compares_values_for_eq_operator()
{
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Int(1234)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Eq, &Value::Int(1234)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Eq, &Value::Float(1234.0)) {
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
fn test_value_bin_op_compares_values_for_ne_operator()
{
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Int(1234)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1234.0).bin_op(BinOp::Ne, &Value::Int(1234)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1234).bin_op(BinOp::Ne, &Value::Float(1234.0)) {
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
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2, Value::Float(1.0), Value::Int(1)]))));
    match value.dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Ok(value2) => {
            match &value2 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(4, elems.len());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 2, 2, f32::neg)));
                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 2, f32::neg)));
                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                            assert_eq!(Value::Float(-1.0), elems[2]);
                            assert_eq!(Value::Int(1), elems[3]);
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
    fields.insert(String::from("c"), Value::Int(1));
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
                            match fields.get(&String::from("c")) {
                                Some(Value::Int(1)) => assert!(true),
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
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value, Value::Float(1.0), Value::Int(1)]))));
    match value.dot1("some message", |v| v.unary_op(UnaryOp::DotNeg)) {
        Ok(value2) => {
            match &value2 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(3, elems.len());
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
                            assert_eq!(Value::Int(1), elems[2]);
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
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value, matrix_value2, Value::Float(1.0), Value::Int(1)]))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value3, matrix_value4, Value::Float(2.0), Value::Int(1)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Ok(value3) => {
            match &value3 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(4, elems.len());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), c.as_slice(), 2, 2, f32::add)));
                            assert_eq!(Value::Object(matrix_array), elems[0].to_matrix_array().unwrap());
                            let matrix_array = Arc::new(Object::MatrixArray(2, 2, TransposeFlag::NoTranspose, expected_bin_op(b.as_slice(), d.as_slice(), 2, 2, f32::add)));
                            assert_eq!(Value::Object(matrix_array), elems[1].to_matrix_array().unwrap());
                            assert_eq!(Value::Float(3.0), elems[2]);
                            assert_eq!(Value::Int(1), elems[3]);
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
    fields.insert(String::from("c"), Value::Int(1));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let array_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![matrix_value3, matrix_value4]))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), array_value2);
    fields2.insert(String::from("b"), Value::Float(2.0));
    fields2.insert(String::from("c"), Value::Int(1));
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
                            match fields.get(&String::from("c")) {
                                Some(Value::Int(1)) => assert!(true),
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
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value, Value::Float(1.0), Value::Int(1)]))));
    let matrix_value3 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, c.as_slice()))));
    let matrix_value4 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 2, d.as_slice()))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), matrix_value3);
    fields2.insert(String::from("b"), matrix_value4);
    let struct_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![struct_value2, Value::Float(2.0), Value::Int(1)]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Ok(value3) => {
            match &value3 {
                Value::Ref(object) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(3, elems.len());
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
                            assert_eq!(Value::Int(1), elems[2]);
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
fn test_value_dot2_complains_on_two_values_are_weak_references()
{
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Weak(Arc::downgrade(&object))]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Weak(Arc::downgrade(&object))]))));
    match value.dot2(&value2, "some message", |v, w| v.bin_op(BinOp::DotAdd, w)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("two values are weak references"), *msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_elem_returns_elements()
{
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.elem(&Value::Int(1)) {
        Ok(elem) => {
            let expected_elem = Value::Object(Arc::new(Object::String(String::from("a"))));
            assert_eq!(expected_elem, elem);
        },
        Err(_) => assert!(false),
    }
    match value.elem(&Value::Int(3)) {
        Ok(elem) => {
            let expected_elem = Value::Object(Arc::new(Object::String(String::from("c"))));
            assert_eq!(expected_elem, elem);
        },
        Err(_) => assert!(false),
    }
    match value.elem(&Value::Float(2.5)) {
        Ok(elem) => {
            let expected_elem = Value::Object(Arc::new(Object::String(String::from("b"))));
            assert_eq!(expected_elem, elem);
        },
        Err(_) => assert!(false),
    }
    let a = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
        10.0, 11.0, 12.0
    ];
    let b = vec![
        2.0, 5.0, 8.0, 11.0,
        3.0, 6.0, 9.0, 12.0,
        4.0, 7.0, 10.0, 13.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(matrix_array.clone());
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::Transpose, b.clone()));
    let value2 = Value::Object(matrix_array2.clone());
    match value.elem(&Value::Int(1)) {
        Ok(elem) => {
            let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 0));
            assert_eq!(Value::Object(matrix_row_slice), elem);
        },
        _ => assert!(false),
    }
    match value.elem(&Value::Int(4)) {
        Ok(elem) => {
            let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 3));
            assert_eq!(Value::Object(matrix_row_slice), elem);
        },
        _ => assert!(false),
    }
    match value.elem(&Value::Float(2.5)) {
        Ok(elem) => {
            let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 1));
            assert_eq!(Value::Object(matrix_row_slice), elem);
        },
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(2)) {
        Ok(elem) => {
            let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array2.clone(), 1));
            assert_eq!(Value::Object(matrix_row_slice), elem);
        },
        _ => assert!(false),
    }
    let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 1));
    let value = Value::Object(matrix_row_slice);
    let matrix_row_slice2 = Arc::new(Object::MatrixRowSlice(matrix_array2.clone(), 1));
    let value2 = Value::Object(matrix_row_slice2);
    match value.elem(&Value::Int(1)) {
        Ok(Value::Float(n)) => assert_eq!(4.0, n),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(6.0, n),
        _ => assert!(false),
    }
    match value.elem(&Value::Float(2.5)) {
        Ok(Value::Float(n)) => assert_eq!(5.0, n),
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(2)) {
        Ok(Value::Float(n)) => assert_eq!(6.0, n),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.elem(&Value::Int(1)) {
        Ok(Value::Int(1)) => assert!(true),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(3)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match value.elem(&Value::Float(2.5)) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let idx_value = Value::Object(Arc::new(Object::String(String::from("b"))));
    match value.elem(&idx_value) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
}

#[test]
fn test_value_elem_complains_on_unsupported_type_for_indexing()
{
    match Value::Int(1).elem(&Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for indexing"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.elem(&Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for indexing"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_elem_complains_on_unsupported_index_type_for_indexing()
{
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.elem(&Value::Bool(true)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
        10.0, 11.0, 12.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(matrix_array.clone());
    match value.elem(&Value::Bool(true)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 1));
    let value = Value::Object(matrix_row_slice);
    match value.elem(&Value::Bool(true)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.elem(&Value::Bool(true)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.elem(&Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let idx_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.elem(&idx_value) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_elem_complains_on_index_out_of_bounds()
{
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(4)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
        10.0, 11.0, 12.0
    ];
    let b = vec![
        2.0, 5.0, 8.0, 11.0,
        3.0, 6.0, 9.0, 12.0,
        4.0, 7.0, 10.0, 13.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(matrix_array.clone());
    let matrix_array2 = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::Transpose, b.clone()));
    let value2 = Value::Object(matrix_array2.clone());
    match value.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(5)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(5)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    let matrix_row_slice = Arc::new(Object::MatrixRowSlice(matrix_array.clone(), 1));
    let value = Value::Object(matrix_row_slice);
    let matrix_row_slice2 = Arc::new(Object::MatrixRowSlice(matrix_array2.clone(), 1));
    let value2 = Value::Object(matrix_row_slice2);
    match value.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(4)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value2.elem(&Value::Int(4)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.elem(&Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    match value.elem(&Value::Int(4)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_elem_complains_on_not_found_key()
{
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let idx_value = Value::Object(Arc::new(Object::String(String::from("d"))));
    match value.elem(&idx_value) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("not found key"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_set_elem_sets_elements()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Int(1), Value::Float(2.5)) {
        Ok(()) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Float(2.5), Value::Float(2.0), Value::Bool(false)])))), value),
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Int(3), Value::Float(2.5)) {
        Ok(()) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Float(2.5)])))), value),
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Float(2.5), Value::Int(3)) {
        Ok(()) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Bool(false)])))), value),
        Err(_) => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let idx_value = Value::Object(Arc::new(Object::String(String::from("b"))));
    match value.set_elem(&idx_value, Value::Int(3)) {
        Ok(()) => {
            let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
            expected_fields.insert(String::from("a"), Value::Int(1));
            expected_fields.insert(String::from("b"), Value::Int(3));
            expected_fields.insert(String::from("c"), Value::Bool(false));
            assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields)))), value);
        },
        Err(_) => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let idx_value = Value::Object(Arc::new(Object::String(String::from("d"))));
    match value.set_elem(&idx_value, Value::Int(3)) {
        Ok(()) => {
            let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
            expected_fields.insert(String::from("a"), Value::Int(1));
            expected_fields.insert(String::from("b"), Value::Float(2.0));
            expected_fields.insert(String::from("c"), Value::Bool(false));
            expected_fields.insert(String::from("d"), Value::Int(3));
            assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields)))), value);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_set_elem_complains_on_unsupported_type_for_indexing()
{
    match Value::Int(1).set_elem(&Value::Int(1), Value::Int(2)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for indexing"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_set_elem_complains_on_unsupported_index_type_for_indexing()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Bool(true), Value::Float(2.5)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.set_elem(&Value::Int(1), Value::Int(3)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
    let idx_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.set_elem(&idx_value, Value::Int(3)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported index type for indexing"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_set_elem_complains_on_index_out_of_bounds()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Int(0), Value::Float(2.5)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_elem(&Value::Int(4), Value::Float(2.5)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("index out of bounds"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_field_returns_field()
{
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.field(&String::from("b")) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
}

#[test]
fn test_value_field_complains_on_unsupported_type_for_field()
{
    match Value::Int(1).field(&String::from("a")) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for field a"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.field(&String::from("b")) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for field b"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_field_complains_on_structure_has_not_field()
{
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.field(&String::from("d")) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("structure hasn't field d"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_set_field_sets_fields()
{
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.set_field(String::from("b"), Value::Int(3)) {
        Ok(()) => {
            let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
            expected_fields.insert(String::from("a"), Value::Int(1));
            expected_fields.insert(String::from("b"), Value::Int(3));
            expected_fields.insert(String::from("c"), Value::Bool(false));
            assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields)))), value);
        },
        Err(_) => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    match value.set_field(String::from("d"), Value::Int(3)) {
        Ok(()) => {
            let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
            expected_fields.insert(String::from("a"), Value::Int(1));
            expected_fields.insert(String::from("b"), Value::Float(2.0));
            expected_fields.insert(String::from("c"), Value::Bool(false));
            expected_fields.insert(String::from("d"), Value::Int(3));
            assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields)))), value);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_set_field_complains_on_unsupported_type_for_field()
{
    match Value::Int(1).set_field(String::from("a"), Value::Int(2)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for field a"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.set_field(String::from("b"), Value::Int(2)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for field b"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_negates_values_for_neg_operator()
{
    match Value::Int(2).unary_op(UnaryOp::Neg) {
        Ok(Value::Int(-2)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).unary_op(UnaryOp::Neg) {
        Ok(Value::Float(n)) => assert_eq!(-2.5, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.unary_op(UnaryOp::Neg) {
        Ok(value2) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, f32::neg)));
            assert_eq!(Value::Object(matrix_array), value2.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_unary_op_complains_on_unsupported_type_for_negation_for_neg_operator()
{
    match Value::Bool(true).unary_op(UnaryOp::Neg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for negation"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.unary_op(UnaryOp::Neg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for negation"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.unary_op(UnaryOp::Neg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for negation"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.unary_op(UnaryOp::Neg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for negation"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_complains_on_overflow_in_negation_for_neg_operator()
{
    match Value::Int(i64::MIN).unary_op(UnaryOp::Neg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("overflow in negation"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_negates_values_for_dot_neg_operator()
{
    match Value::Int(2).unary_op(UnaryOp::DotNeg) {
        Ok(Value::Float(n)) => assert_eq!(-2.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).unary_op(UnaryOp::DotNeg) {
        Ok(Value::Float(n)) => assert_eq!(-2.5, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.unary_op(UnaryOp::DotNeg) {
        Ok(value2) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, f32::neg)));
            assert_eq!(Value::Object(matrix_array), value2.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    match value.unary_op(UnaryOp::DotNeg) {
        Ok(value2) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-2.0)])))), value2),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_unary_op_complains_on_unsupported_type_for_dot_negation_for_dot_neg_operator()
{
    match Value::Bool(true).unary_op(UnaryOp::DotNeg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for dot negation"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.unary_op(UnaryOp::DotNeg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for dot negation"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.unary_op(UnaryOp::DotNeg) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for dot negation"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_negates_values_for_not_operator()
{
    match Value::None.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Bool(true).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Bool(false).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(1).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(0).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1.0).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(0.0).unary_op(UnaryOp::Not) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let a = matrix![[1.0, 2.0], [3.0, 4.0]];
    let value = Value::Object(Arc::new(Object::Matrix(a.clone())));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone())));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 1.0,
        2.0, 3.0,
        1.0, 1.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
    let value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields.clone()))));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let value = Value::Weak(Arc::downgrade(&object));
    match value.unary_op(UnaryOp::Not) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_transposes_values_for_transpose_operator()
{
    match Value::Int(1).unary_op(UnaryOp::Transpose) {
        Ok(Value::Int(1)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(1.0).unary_op(UnaryOp::Transpose) {
        Ok(Value::Float(n)) => assert_eq!(1.0, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.unary_op(UnaryOp::Transpose) {
        Ok(Value::Object(object)) => {
            match &*object {
                Object::Matrix(b) => {
                    assert_eq!(2, b.row_count());
                    assert_eq!(3, b.col_count());
                    assert_eq!(true, b.is_transposed());
                    assert_eq!(a, b.elems());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_value_unary_op_complains_on_unsupported_type_for_transpose_for_transpose_operator()
{
    match Value::Bool(true).unary_op(UnaryOp::Transpose) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for transpose"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.unary_op(UnaryOp::Transpose) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for transpose"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    match value.unary_op(UnaryOp::Transpose) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for transpose"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.unary_op(UnaryOp::Transpose) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported type for transpose"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_return_element_for_index_operator()
{
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    match value.bin_op(BinOp::Index, &Value::Int(2)) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_multiplies_values_for_mul_operator()
{
    match Value::Int(2).bin_op(BinOp::Mul, &Value::Int(3)) {
        Ok(Value::Int(6)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Mul, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(7.5, n),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Mul, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(7.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Mul, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(8.75, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 5.0, 7.0,
        4.0, 6.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::Mul, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x * 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Mul, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x * 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(2, 3, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::Mul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(2, 3, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 * y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Mul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(2, 3, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 * y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Mul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, expected_mul(a.as_slice(), b.as_slice(), 3, 3, 2)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_multiplication_for_mul_operator()
{
    match Value::Bool(true).bin_op(BinOp::Mul, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for multiplication"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::Mul, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for multiplication"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::Mul, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for multiplication"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Float(1.0), Value::Bool(true)]))));
    match value.bin_op(BinOp::Mul, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for multiplication"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_overflow_in_multiplication_for_mul_operator()
{
    match Value::Int((u32::MAX as i64) + 1).bin_op(BinOp::Mul, &Value::Int((u32::MAX as i64) + 1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("overflow in multiplication"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_multiplies_values_for_dot_mul_operator()
{
    match Value::Int(2).bin_op(BinOp::DotMul, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(6.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotMul, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(8.75, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::DotMul, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x * 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotMul, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x * 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 * y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 * y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::mul)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0)]))));
    match value.bin_op(BinOp::DotMul, &Value::Int(3)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(6.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotMul, &Value::Float(3.0)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(6.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(6.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Float(2.0).bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(6.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotMul, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(6.0)])))), value3),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_dot_multiplication_for_dot_mul_operator()
{
    match Value::Bool(true).bin_op(BinOp::DotMul, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot multiplication"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::DotMul, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot multiplication"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::DotMul, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot multiplication"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_divides_values_for_div_operator()
{
    match Value::Int(2 * 3).bin_op(BinOp::Div, &Value::Int(3)) {
        Ok(Value::Int(2)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5 * 3.0).bin_op(BinOp::Div, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    match Value::Int(2 * 3).bin_op(BinOp::Div, &Value::Float(3.0)) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5 * 3.5).bin_op(BinOp::Div, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0 * 3.0, 2.0 * 3.0,
        3.0 * 3.0, 4.0 * 3.0,
        5.0 * 3.0, 6.0 * 3.0
    ];
    let b = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::Div, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x / 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Div, &Value::Float(3.0)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x / 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(1 * 2 * 3 * 4 * 5 * 6).bin_op(BinOp::Div, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| (1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float((1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / 32.0).bin_op(BinOp::Div, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| ((1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / 32.0) / y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_division_for_div_operator()
{
    match Value::Bool(true).bin_op(BinOp::Div, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for division"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::Div, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for division"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::Div, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for division"), msg),
        _ => assert!(false),
    }
    let a = matrix![[1.0, 2.0], [3.0, 4.0]];
    let value = Value::Object(Arc::new(Object::Matrix(a.clone())));
    let a = matrix![[3.0, 4.0], [5.0, 6.0]];
    let value2 = Value::Object(Arc::new(Object::Matrix(a)));
    match value.bin_op(BinOp::Div, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for division"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Float(1.0), Value::Bool(true)]))));
    match value.bin_op(BinOp::Div, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for division"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_division_by_zero_for_div_operator()
{
    match Value::Int(2).bin_op(BinOp::Div, &Value::Int(0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("division by zero"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_overflow_in_division_for_div_operator()
{
    match Value::Int(i64::MIN).bin_op(BinOp::Div, &Value::Int(-1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("overflow in division"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_divides_values_for_dot_div_operator()
{
    match Value::Int(2 * 3).bin_op(BinOp::DotDiv, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(2.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5 * 3.5).bin_op(BinOp::DotDiv, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0 * 3.0, 2.0 * 4.0,
        3.0 * 5.0, 4.0 * 6.0,
        5.0 * 7.0, 6.0 * 8.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let c = vec![
        1.0 * 3.0, 2.0 * 3.0,
        3.0 * 3.0, 4.0 * 3.0,
        5.0 * 3.0, 6.0 * 3.0
    ];
    let d = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, c.as_slice()))));
    match value.bin_op(BinOp::Div, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(c.as_slice(), 3, 2, |x| x / 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Div, &Value::Float(3.0)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(c.as_slice(), 3, 2, |x| x / 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, d.as_slice()))));
    match Value::Int(1 * 2 * 3 * 4 * 5 * 6).bin_op(BinOp::Div, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(d.as_slice(), 2, 3, |y| (1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float((1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / 32.0).bin_op(BinOp::Div, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(d.as_slice(), 2, 3, |y| ((1.0 * 2.0 * 3.0 * 4.0 * 5.0 * 6.0) / 32.0) / y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match value.bin_op(BinOp::DotDiv, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::div)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0 * 3.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0)]))));
    match value.bin_op(BinOp::DotDiv, &Value::Int(3)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotDiv, &Value::Float(3.0)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Int(2 * 3).bin_op(BinOp::DotDiv, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Float(2.0 * 3.0).bin_op(BinOp::DotDiv, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotDiv, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)])))), value3),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_dot_division_for_dot_div_operator()
{
    match Value::Bool(true).bin_op(BinOp::DotDiv, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot division"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::DotDiv, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot division"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::DotDiv, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot division"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_adds_values_for_add_operator()
{
    match Value::Int(2).bin_op(BinOp::Add, &Value::Int(3)) {
        Ok(Value::Int(5)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Add, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(5.5, n),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Add, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(5.5, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Add, &Value::Float(3.25)) {
        Ok(Value::Float(n)) => assert_eq!(5.75, n),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::Add, &value2) {
        Ok(value3) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("abcdef")))), value3),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::Add, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x + 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Add, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x + 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::Add, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 + y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Add, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 + y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Add, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::add)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(3), Value::Float(4.0)]))));
    match value.bin_op(BinOp::Add, &value2) {
        Ok(value3) => {
            let array = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Int(3), Value::Float(4.0)])));
            assert_eq!(Value::Ref(array), value3);
        },
        _ => assert!(false),
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
    fields2.insert(String::from("a"), Value::Int(3));
    fields2.insert(String::from("d"), Value::Float(4.0));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
    match value.bin_op(BinOp::Add, &value2) {
        Ok(value3) => {
            let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
            expected_fields.insert(String::from("a"), Value::Int(1));
            expected_fields.insert(String::from("b"), Value::Float(2.0));
            expected_fields.insert(String::from("c"), Value::Bool(false));
            expected_fields.insert(String::from("d"), Value::Float(4.0));
            assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields)))), value3);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_addition_for_add_operator()
{
    match Value::Bool(true).bin_op(BinOp::Add, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for addition"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::Add, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for addition"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_overflow_in_addition_for_add_operator()
{
    match Value::Int(i64::MAX).bin_op(BinOp::Add, &Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("overflow in addition"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_adds_values_for_dot_add_operator()
{
    match Value::Int(2).bin_op(BinOp::DotAdd, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(5.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotAdd, &Value::Float(3.25)) {
        Ok(Value::Float(n)) => assert_eq!(5.75, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::DotAdd, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x + 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotAdd, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x + 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 + y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 + y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::add)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0)]))));
    match value.bin_op(BinOp::DotAdd, &Value::Int(3)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(5.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotAdd, &Value::Float(3.0)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(5.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(5.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Float(2.0).bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(5.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotAdd, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(5.0)])))), value3),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_dot_addition_for_dot_add_operator()
{
    match Value::Bool(true).bin_op(BinOp::DotAdd, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot addition"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::DotAdd, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot addition"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::DotAdd, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot addition"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_subtracts_values_for_sub_operator()
{
    match Value::Int(2).bin_op(BinOp::Sub, &Value::Int(3)) {
        Ok(Value::Int(-1)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Sub, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(-0.5, n),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Sub, &Value::Float(3.5)) {
        Ok(Value::Float(n)) => assert_eq!(-1.5, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Sub, &Value::Float(3.25)) {
        Ok(Value::Float(n)) => assert_eq!(-0.75, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::Sub, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x - 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Sub, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x - 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::Sub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 - y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Sub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 - y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::Sub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::sub)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_subtraction_for_sub_operator()
{
    match Value::Bool(true).bin_op(BinOp::Sub, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for subtraction"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::Sub, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for subtraction"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::Sub, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for subtraction"), msg),
        _ => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Float(1.0), Value::Bool(true)]))));
    match value.bin_op(BinOp::Sub, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for subtraction"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_overflow_in_subtraction_for_sub_operator()
{
    match Value::Int(i64::MIN).bin_op(BinOp::Sub, &Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("overflow in subtraction"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_subtracts_values_for_dot_sub_operator()
{
    match Value::Int(2).bin_op(BinOp::DotSub, &Value::Int(3)) {
        Ok(Value::Float(n)) => assert_eq!(-1.0, n),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotSub, &Value::Float(3.25)) {
        Ok(Value::Float(n)) => assert_eq!(-0.75, n),
        _ => assert!(false),
    }
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let b = vec![
        3.0, 4.0,
        5.0, 6.0,
        7.0, 8.0
    ];
    let value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
    match value.bin_op(BinOp::DotSub, &Value::Int(3)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x - 3.0)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotSub, &Value::Float(3.5)) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(a.as_slice(), 3, 2, |x| x - 3.5)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value2 = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, b.as_slice()))));
    match Value::Int(2).bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.0 - y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_unary_op(b.as_slice(), 2, 3, |y| 2.5 - y)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => {
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, expected_bin_op(a.as_slice(), b.as_slice(), 3, 2, f32::sub)));
            assert_eq!(Value::Object(matrix_array), value3.to_matrix_array().unwrap());
        },
        Err(_) => assert!(false),
    }
    let value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0)]))));
    match value.bin_op(BinOp::DotSub, &Value::Int(3)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-1.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotSub, &Value::Float(3.0)) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-1.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-1.0)])))), value3),
        Err(_) => assert!(false),
    }
    match Value::Float(2.0).bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-1.0)])))), value3),
        Err(_) => assert!(false),
    }
    match value.bin_op(BinOp::DotSub, &value2) {
        Ok(value3) => assert_eq!(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(-1.0)])))), value3),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_dot_subtraction_for_dot_sub_operator()
{
    match Value::Bool(true).bin_op(BinOp::DotSub, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot subtraction"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::DotSub, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot subtraction"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::DotSub, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for dot subtraction"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_compares_values_for_lt_operator()
{
    match Value::Bool(false).bin_op(BinOp::Lt, &Value::Bool(true)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Bool(false).bin_op(BinOp::Lt, &Value::Bool(false)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Lt, &Value::Int(3)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Lt, &Value::Int(2)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.0).bin_op(BinOp::Lt, &Value::Int(3)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.0).bin_op(BinOp::Lt, &Value::Int(2)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Lt, &Value::Float(2.5)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Lt, &Value::Float(2.0)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Lt, &Value::Float(3.5)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Float(2.5).bin_op(BinOp::Lt, &Value::Float(2.5)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
    match value.bin_op(BinOp::Lt, &value2) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::String(String::from("abc"))));
    let value2 = Value::Object(Arc::new(Object::String(String::from("abc"))));
    match value.bin_op(BinOp::Lt, &value2) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_complains_on_unsupported_types_for_comparison_for_lt_operator()
{
    match Value::Bool(true).bin_op(BinOp::Lt, &Value::Int(1)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for comparison"), msg),
        _ => assert!(false),
    }
    match Value::Bool(true).bin_op(BinOp::Lt, &Value::Float(1.0)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for comparison"), msg),
        _ => assert!(false),
    }
    match Value::Int(1).bin_op(BinOp::Lt, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for comparison"), msg),
        _ => assert!(false),
    }
    match Value::Float(1.0).bin_op(BinOp::Lt, &Value::Bool(false)) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for comparison"), msg),
        _ => assert!(false),
    }
    let value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
    let value2 = Value::Object(Arc::new(Object::Error(String::from("def"), String::from("abc"))));
    match value.bin_op(BinOp::Lt, &value2) {
        Err(Error::Interp(msg)) => assert_eq!(String::from("unsupported types for comparison"), msg),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_compares_values_for_ge_operator()
{
    match Value::Int(2).bin_op(BinOp::Ge, &Value::Int(3)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Ge, &Value::Int(2)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_compares_values_for_gt_operator()
{
    match Value::Int(3).bin_op(BinOp::Gt, &Value::Int(2)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Gt, &Value::Int(2)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_bin_op_compares_values_for_le_operator()
{
    match Value::Int(3).bin_op(BinOp::Le, &Value::Int(2)) {
        Ok(Value::Bool(false)) => assert!(true),
        _ => assert!(false),
    }
    match Value::Int(2).bin_op(BinOp::Le, &Value::Int(2)) {
        Ok(Value::Bool(true)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_value_neg_negates_values()
{
    assert!(Value::Int(-2).eq_with_types(&(-Value::Int(2))).unwrap());
    assert!(Value::Int(-2).eq_with_types(&(-&Value::Int(2))).unwrap());
}

#[test]
fn test_value_not_negates_values()
{
    assert_eq!(Value::Bool(false), !Value::Bool(true));
    assert_eq!(Value::Bool(true), !Value::Bool(false));
}

#[test]
fn test_value_add_adds_values()
{
    assert!(Value::Int(5).eq_with_types(&(Value::Int(2) + Value::Int(3))).unwrap());
    assert!(Value::Int(5).eq_with_types(&(Value::Int(2) + &Value::Int(3))).unwrap());
    assert!(Value::Int(5).eq_with_types(&(&Value::Int(2) + Value::Int(3))).unwrap());
    assert!(Value::Int(5).eq_with_types(&(&Value::Int(2) + &Value::Int(3))).unwrap());
}

#[test]
fn test_value_add_assign_adds_values()
{
    let mut value = Value::Int(2);
    value += Value::Int(3);
    assert!(Value::Int(5).eq_with_types(&value).unwrap());
    let mut value = Value::Int(2);
    value += &Value::Int(3);
    assert!(Value::Int(5).eq_with_types(&value).unwrap());
}

#[test]
fn test_value_sub_subtracts_values()
{
    assert!(Value::Int(-1).eq_with_types(&(Value::Int(2) - Value::Int(3))).unwrap());
    assert!(Value::Int(-1).eq_with_types(&(Value::Int(2) - &Value::Int(3))).unwrap());
    assert!(Value::Int(-1).eq_with_types(&(&Value::Int(2) - Value::Int(3))).unwrap());
    assert!(Value::Int(-1).eq_with_types(&(&Value::Int(2) - &Value::Int(3))).unwrap());
}

#[test]
fn test_value_sub_assign_subtracts_values()
{
    let mut value = Value::Int(2);
    value -= Value::Int(3);
    assert!(Value::Int(-1).eq_with_types(&value).unwrap());
    let mut value = Value::Int(2);
    value -= &Value::Int(3);
    assert!(Value::Int(-1).eq_with_types(&value).unwrap());
}

#[test]
fn test_value_mul_multiplies_values()
{
    assert!(Value::Int(6).eq_with_types(&(Value::Int(2) * Value::Int(3))).unwrap());
    assert!(Value::Int(6).eq_with_types(&(Value::Int(2) * &Value::Int(3))).unwrap());
    assert!(Value::Int(6).eq_with_types(&(&Value::Int(2) * Value::Int(3))).unwrap());
    assert!(Value::Int(6).eq_with_types(&(&Value::Int(2) * &Value::Int(3))).unwrap());
}

#[test]
fn test_value_mul_assign_mutliplies_values()
{
    let mut value = Value::Int(2);
    value *= Value::Int(3);
    assert!(Value::Int(6).eq_with_types(&value).unwrap());
    let mut value = Value::Int(2);
    value *= &Value::Int(3);
    assert!(Value::Int(6).eq_with_types(&value).unwrap());
}

#[test]
fn test_value_div_divides_values()
{
    assert!(Value::Int(2).eq_with_types(&(Value::Int(2 * 3) / Value::Int(3))).unwrap());
    assert!(Value::Int(2).eq_with_types(&(Value::Int(2 * 3) / &Value::Int(3))).unwrap());
    assert!(Value::Int(2).eq_with_types(&(&Value::Int(2 * 3) / Value::Int(3))).unwrap());
    assert!(Value::Int(2).eq_with_types(&(&Value::Int(2 * 3) / &Value::Int(3))).unwrap());
}

#[test]
fn test_value_div_assign_divides_values()
{
    let mut value = Value::Int(2 * 3);
    value /= Value::Int(3);
    assert!(Value::Int(2).eq_with_types(&value).unwrap());
    let mut value = Value::Int(2 * 3);
    value /= &Value::Int(3);
    assert!(Value::Int(2).eq_with_types(&value).unwrap());
}
