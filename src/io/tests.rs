//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::matrix::matrix;
use crate::interp::*;
use crate::mod_node::*;
use super::*;

fn f(_interp: &mut Interp, _env: &mut Env, _arg_values: &[Value]) -> Result<Value>
{ Ok(Value::None) }

#[test]
fn test_write_values_and_read_values_writes_values_and_reads_values()
{
    let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
    env.add_and_push_mod(String::from("a")).unwrap();
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    env.add_fun(String::from("f"), fun.clone()).unwrap();
    env.pop_mod().unwrap();
    env.set_var(&Name::Var(String::from("f")), Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)))).unwrap();
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut values: Vec<Value> = Vec::new();
    values.push(Value::None);
    values.push(Value::Bool(true));
    values.push(Value::Bool(false));
    values.push(Value::Int(1234));
    values.push(Value::Float(12.34));
    values.push(Value::Object(Arc::new(Object::String(String::from("abc")))));
    values.push(Value::Object(Arc::new(Object::IntRange(2, 4, 1))));
    values.push(Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5))));
    let a = matrix![
        [1.0, 2.0],
        [3.0, 4.0],
        [5.0, 6.0]
    ];
    values.push(Value::Object(Arc::new(Object::Matrix(a))));
    let a = matrix![
        [1.0, 3.0, 5.0],
        [2.0, 4.0, 6.0]
    ];
    values.push(Value::Object(Arc::new(Object::Matrix(a.transpose()))));
    values.push(Value::Object(Arc::new(Object::Fun(vec![String::from("a")], String::from("f"), fun))));
    values.push(Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f))));
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    values.push(Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a))));
    let at = vec![
        1.0, 3.0, 5.0,
        2.0, 4.0, 6.0
    ];
    values.push(Value::Object(Arc::new(Object::MatrixArray(2, 3, TransposeFlag::Transpose, at))));
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
    values.push(Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1))));
    values.push(Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def")))));
    values.push(Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])))));
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("a"), Value::Int(1));
    fields.insert(String::from("b"), Value::Float(2.0));
    fields.insert(String::from("c"), Value::Bool(false));
    values.push(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields)))));
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    values.push(Value::Weak(Arc::downgrade(&object)));
    values.push(Value::Ref(object.clone()));
    values.push(Value::Weak(Weak::new()));
    match write_values(&mut cursor, values.as_slice()) {
        Ok(()) => {
            cursor.set_position(0);
            match read_values(&mut cursor, &mut env) {
                Ok(values2) => {
                    assert_eq!(values.len(), values2.len());
                    for (value, value2) in values.iter().zip(values2.iter()) {
                        match (value, value2) {
                            (Value::Object(object), Value::Object(object2)) => {
                                match (&**object, &**object2) {
                                    (Object::Matrix(_), Object::Matrix(_)) => assert!(value.to_matrix_array().unwrap().eq_with_types(&value2.to_matrix_array().unwrap()).unwrap()),
                                    (_, _) => assert!(value.eq_with_types(&value2).unwrap()), 
                                }
                            },
                            (Value::Weak(object), Value::Weak(object2)) => {
                                match (object.upgrade(), object2.upgrade()) {
                                    (Some(object), Some(object2)) => assert!(Value::Ref(object).eq_with_types(&Value::Ref(object2)).unwrap()),
                                    (None, None) => assert!(true),
                                    (_, _) => assert!(false),
                                }
                            },
                            (_, _) => assert!(value.eq_with_types(&value2).unwrap()), 
                        }
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_values_and_read_values_writes_values_and_reads_values_for_object_indices()
{
    let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut values: Vec<Value> = Vec::new();
    let object = Arc::new(Object::String(String::from("abc")));
    values.push(Value::Object(object.clone()));
    let object2 = Arc::new(Object::String(String::from("def")));
    values.push(Value::Object(object2.clone()));
    values.push(Value::Object(object.clone()));
    values.push(Value::Object(object));
    values.push(Value::Object(object2));
    match write_values(&mut cursor, values.as_slice()) {
        Ok(()) => {
            cursor.set_position(0);
            match read_values(&mut cursor, &mut env) {
                Ok(values2) => {
                    assert_eq!(values.len(), values2.len());
                    assert!(values[0].eq_with_types(&values2[0]).unwrap());
                    assert!(values[1].eq_with_types(&values2[1]).unwrap());
                    match (&values2[0], &values2[2]) {
                        (Value::Object(object), Value::Object(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                    match (&values2[0], &values2[3]) {
                        (Value::Object(object), Value::Object(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                    match (&values2[1], &values2[4]) {
                        (Value::Object(object), Value::Object(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_values_and_read_values_writes_values_and_reads_values_for_object_index_and_matrix_row_slice()
{
    let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut values: Vec<Value> = Vec::new();
    let a = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
    values.push(Value::Object(matrix_array.clone()));
    values.push(Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1))));
    match write_values(&mut cursor, values.as_slice()) {
        Ok(()) => {
            cursor.set_position(0);
            match read_values(&mut cursor, &mut env) {
                Ok(values2) => {
                    assert_eq!(values.len(), values2.len());
                    assert!(values[0].eq_with_types(&values2[0]).unwrap());
                    match (&values2[0], &values2[1]) {
                        (Value::Object(object), Value::Object(object2)) => {
                            match &**object2 {
                                Object::MatrixRowSlice(matrix_array2, 1) => assert!(Arc::ptr_eq(object, matrix_array2)),
                                _ => assert!(false),
                            }
                        },
                        (_, _) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_values_and_read_values_writes_values_and_reads_values_for_mutable_object_indices()
{
    let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut values: Vec<Value> = Vec::new();
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    values.push(Value::Ref(object.clone()));
    let object2 = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Float(3.0), Value::Bool(true)])));
    values.push(Value::Ref(object2.clone()));
    values.push(Value::Ref(object.clone()));
    values.push(Value::Ref(object));
    values.push(Value::Ref(object2));
    match write_values(&mut cursor, values.as_slice()) {
        Ok(()) => {
            cursor.set_position(0);
            match read_values(&mut cursor, &mut env) {
                Ok(values2) => {
                    assert_eq!(values.len(), values2.len());
                    assert!(values[0].eq_with_types(&values2[0]).unwrap());
                    assert!(values[1].eq_with_types(&values2[1]).unwrap());
                    match (&values2[0], &values2[2]) {
                        (Value::Ref(object), Value::Ref(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                    match (&values2[0], &values2[3]) {
                        (Value::Ref(object), Value::Ref(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                    match (&values2[1], &values2[4]) {
                        (Value::Ref(object), Value::Ref(object2)) => assert!(Arc::ptr_eq(object, object2)),
                        (_, _) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_values_and_read_values_writes_values_and_reads_values_for_reference_cycle()
{
    let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    let mut values: Vec<Value> = Vec::new();
    let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
    let object2 = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Ref(object.clone())])));
    {
        let mut object_g = object.write().unwrap();
        match &mut *object_g {
            MutObject::Array(elems) => elems.push(Value::Weak(Arc::downgrade(&object2))),
            _ => assert!(false),
        }
    }
    values.push(Value::Ref(object2));
    match write_values(&mut cursor, values.as_slice()) {
        Ok(()) => {
            cursor.set_position(0);
            match read_values(&mut cursor, &mut env) {
                Ok(values2) => {
                    assert_eq!(values.len(), values2.len());
                    match &values2[0] {
                        Value::Ref(object2) => {
                            let expected_object2 = object2.clone();
                            let object2_g = object2.read().unwrap();
                            match &*object2_g {
                                MutObject::Array(elems2) => {
                                    assert_eq!(2, elems2.len());
                                    assert!(Value::Int(2).eq_with_types(&elems2[0]).unwrap());
                                    match &elems2[1] {
                                        Value::Ref(object3) => {
                                            let object3_g = object3.read().unwrap();
                                            match &*object3_g {
                                                MutObject::Array(elems3) => {
                                                    assert_eq!(4, elems3.len());
                                                    assert!(Value::Int(1).eq_with_types(&elems3[0]).unwrap());
                                                    assert!(Value::Float(2.0).eq_with_types(&elems3[1]).unwrap());
                                                    assert!(Value::Bool(false).eq_with_types(&elems3[2]).unwrap());
                                                    assert!(Value::Weak(Arc::downgrade(&expected_object2)).eq_with_types(&elems3[3]).unwrap());
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
        },
        Err(_) => assert!(false),
    }
}
