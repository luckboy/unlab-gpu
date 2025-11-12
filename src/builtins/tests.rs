//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use crate::matrix::matrix;
use crate::tree::*;
use super::*;

fn f(_interp: &mut Interp, _env: &mut Env, _arg_values: &[Value]) -> Result<Value>
{ Ok(Value::None) }

#[test]
fn test_type_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("type")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::None]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("none")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Bool(true)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("bool")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("int")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(12.34)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("float")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("string")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::IntRange(2, 4, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("intrange")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::FloatRange(2.0, 4.5, 1.5)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("floatrange")))), value),
                Err(_) => assert!(false),
            }
            let a = matrix![[1.0, 2.0], [3.0, 4.0]];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("matrix")))), value),
                Err(_) => assert!(false),
            }
            let fun = Arc::new(Fun(Vec::new(), Vec::new()));
            let arg_value = Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("f"), fun.clone())));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("function")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::BuiltinFun(String::from("f"), f)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("function")))), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("matrixarray")))), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("matrixrowslice")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("error")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("array")))), value),
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("struct")))), value),
                Err(_) => assert!(false),
            }
            let object = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
            let arg_value = Value::Weak(Arc::downgrade(&object));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("weak")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_bool_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("bool")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::None]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Bool(true)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_int_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("int")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::None]) {
                Ok(value) => assert_eq!(Value::Int(0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Bool(true)]) {
                Ok(value) => assert_eq!(Value::Int(1), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Int(1234), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(12.34)]) {
                Ok(value) => assert_eq!(Value::Int(12), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(1), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_float_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("float")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::None]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Bool(true)]) {
                Ok(value) => assert_eq!(Value::Float(1.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Float(1234.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(12.34)]) {
                Ok(value) => assert_eq!(Value::Float(12.34), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Float(1.0), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_string_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("string")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::None]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("none")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Bool(true)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("true")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("1234")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(12.34)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("12.3400")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("abc")))), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("def")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_zeros_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("zeros")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(2)]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, vec![0.0; 3 * 2]));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_ones_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("ones")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(2)]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, vec![1.0; 3 * 2]));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_eye_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("eye")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3)]) {
                Ok(value) => {
                    let a = vec![
                        1.0, 0.0, 0.0,
                        0.0, 1.0, 0.0,
                        0.0, 0.0, 1.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_init_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("init")), root_mod_g.var(&String::from("get"))) {
        (Some(fun_value), Some(get_value)) => {
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(2), Value::Object(matrix_array.clone()), get_value.clone()]) {
                Ok(value) => assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap()),
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}           

#[test]
fn test_initdiag_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("initdiag")), root_mod_g.var(&String::from("getdiag"))) {
        (Some(fun_value), Some(getdiag_value)) => {
            let a = vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, a));
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Object(matrix_array.clone()), getdiag_value.clone()]) {
                Ok(value) => {
                    let b = vec![
                        1.0, 0.0, 0.0,
                        0.0, 5.0, 0.0,
                        0.0, 0.0, 9.0
                    ];
                    let matrix_array2 = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, b));
                    assert_eq!(Value::Object(matrix_array2), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}

#[test]
fn test_matrix_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("matrix")) {
        Some(fun_value) => {
            let elems = vec![
                Value::Object(Arc::new(Object::IntRange(1, 3, 1))),
                Value::Object(Arc::new(Object::IntRange(4, 6, 1))),
                Value::Object(Arc::new(Object::IntRange(7, 9, 1))),
                Value::Object(Arc::new(Object::IntRange(10, 12, 1)))
            ];
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let a = vec![
                        1.0, 2.0, 3.0,
                        4.0, 5.0, 6.0,
                        7.0, 8.0, 9.0,
                        10.0, 11.0, 12.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_rowvector_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("rowvector")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::IntRange(1, 3, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let a = vec![1.0, 2.0, 3.0];
                    let matrix_array = Arc::new(Object::MatrixArray(1, 3, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
            let a = vec![1.0, 2.0, 3.0];
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(1, 3, a.as_slice()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(1, 3, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_colvector_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("colvector")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::IntRange(1, 3, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let a = vec![
                        1.0,
                        2.0,
                        3.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 1, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0,
                2.0,
                3.0
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 1, a.as_slice()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(3, 1, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_matrixarray_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("matrixarray")) {
        Some(fun_value) => {
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, a.as_slice()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
                    assert_eq!(Value::Object(matrix_array), value);
                },
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a.clone()));
            let arg_value = Value::Object(matrix_array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(matrix_array), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_error_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("error")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => {
                    let expected_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_array_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("array")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::IntRange(1, 3, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_strong_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("strong")) {
        Some(fun_value) => {
            let array = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])));
            let arg_value = Value::Weak(Arc::downgrade(&array));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Ref(array), value),
                Err(_) => assert!(false),
            }
            let array = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
            let arg_value = Value::Ref(array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Ref(array), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_weak_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("weak")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[]) {
                Ok(value) => assert_eq!(Value::Weak(Weak::new()), value),
                Err(_) => assert!(false),
            }
            let array = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
            let arg_value = Value::Ref(array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Weak(Arc::downgrade(&array)), value),
                Err(_) => assert!(false),
            }
            let array = Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)])));
            let arg_value = Value::Weak(Arc::downgrade(&array));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Weak(Arc::downgrade(&array)), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_isempty_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isempty")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::new())));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(0, 2, TransposeFlag::NoTranspose, Vec::new())));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            let matrix_array = Arc::new(Object::MatrixArray(3, 0, TransposeFlag::NoTranspose, Vec::new()));
            let arg_value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_length_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("length")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_rows_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("rows")) {
        Some(fun_value) => {
            let a = matrix![
                [1.0, 2.0],
                [3.0, 4.0],
                [5.0, 6.0]
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_columns_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("columns")) {
        Some(fun_value) => {
            let a = matrix![
                [1.0, 2.0],
                [3.0, 4.0],
                [5.0, 6.0]
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_get_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("get")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("b")))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(4)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(matrix_array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1))), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(4)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(Arc::new(Object::MatrixRowSlice(matrix_array, 1)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Float(4.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Float(2.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(4)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("b"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => assert_eq!(Value::Float(2.0), value),
                Err(_) => assert!(false),
            }
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("d"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            let a = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(matrix_array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2), Value::Int(1)]) {
                Ok(value) => assert_eq!(Value::Float(3.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(4), Value::Int(1)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(2), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_getdiag_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("getdiag")) {
        Some(fun_value) => {
            let a = vec![
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0
            ];
            let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, a));
            let arg_value = Value::Object(matrix_array.clone());
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Float(5.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(4)]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_split_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("split")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abc def   ghi"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let elems = vec![
                        Value::Object(Arc::new(Object::String(String::from("abc")))),
                        Value::Object(Arc::new(Object::String(String::from("def")))),
                        Value::Object(Arc::new(Object::String(String::from("ghi"))))
                    ];
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abcxxdefxxghi"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("xx"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => {
                    let elems = vec![
                        Value::Object(Arc::new(Object::String(String::from("abc")))),
                        Value::Object(Arc::new(Object::String(String::from("def")))),
                        Value::Object(Arc::new(Object::String(String::from("ghi"))))
                    ];
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_trim_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("trim")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("  abc  "))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("abc")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_contains_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("contains")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from(" abc  def ghi "))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from(" abc  def "))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("ghi"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_startswith_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("startswith")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abcdefghi"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abcdefghi"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_endswith_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("endswith")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abcdefghi"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("ghi"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("abcdefghi"))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_replace_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("replace")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from(" abc def abc "))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("abc"))));
            let arg_value3 = Value::Object(Arc::new(Object::String(String::from("ghi"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, arg_value2, arg_value3]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from(" ghi def ghi ")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_upper_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("upper")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from(" ABC def GHI "))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from(" ABC DEF GHI ")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_lower_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("lower")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from(" ABC def GHI "))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from(" abc def ghi ")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}
