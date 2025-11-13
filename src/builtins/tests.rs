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
fn test_pi_is_pi_constant()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match root_mod.var(&String::from("pi")) {
        Some(value) => assert_eq!(Value::Float(f32::consts::PI), *value),
        None => assert!(false),
    }
}

#[test]
fn test_e_is_e_constant()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match root_mod.var(&String::from("e")) {
        Some(value) => assert_eq!(Value::Float(f32::consts::E), *value),
        None => assert!(false),
    }
}

#[test]
fn test_pathsep_is_path_separator()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match root_mod.var(&String::from("pathsep")) {
        Some(value) => assert_eq!(Value::Object(Arc::new(Object::String(format!("{}", path::MAIN_SEPARATOR)))), *value),
        None => assert!(false),
    }
}

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

#[test]
fn test_sort_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("sort")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Bool(false), Value::Bool(true), Value::Bool(true)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(3), Value::Int(1), Value::Float(2.5)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.5), Value::Int(3)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let elems = vec![
                Value::Object(Arc::new(Object::String(String::from("ghi")))),
                Value::Object(Arc::new(Object::String(String::from("abc")))),
                Value::Object(Arc::new(Object::String(String::from("def"))))
            ];
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_elems = vec![
                        Value::Object(Arc::new(Object::String(String::from("abc")))),
                        Value::Object(Arc::new(Object::String(String::from("def")))),
                        Value::Object(Arc::new(Object::String(String::from("ghi"))))
                    ];
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(expected_elems))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_reverse_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("reverse")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Bool(false), Value::Float(2.0), Value::Int(1)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_any_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("any")), root_mod_g.var(&String::from("islessequal"))) {
        (Some(fun_value), Some(islessequal_value)) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2), Value::Int(4)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(2), Value::Int(1)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}

#[test]
fn test_all_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("all")), root_mod_g.var(&String::from("islessequal"))) {
        (Some(fun_value), Some(islessequal_value)) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(3), Value::Int(3), Value::Int(4), Value::Int(4)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2), Value::Int(4)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}

#[test]
fn test_find_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("find")), root_mod_g.var(&String::from("islessequal"))) {
        (Some(fun_value), Some(islessequal_value)) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2), Value::Int(4)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(2), Value::Int(1)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}

#[test]
fn test_filter_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match (root_mod_g.var(&String::from("filter")), root_mod_g.var(&String::from("islessequal"))) {
        (Some(fun_value), Some(islessequal_value)) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2), Value::Int(4)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(2), Value::Int(4)]))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(2), Value::Int(1)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3), islessequal_value.clone()]) {
                Ok(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
                    assert_eq!(expected_value, value);
                },
                Err(_) => assert!(false),
            }
        },
        (_, _) => assert!(false),
    }
}

#[test]
fn test_max_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("max")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(2), Value::Int(1)]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_min_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("min")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(3), Value::Int(1), Value::Int(2)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::Int(1), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Int(1), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(2), Value::Int(1)]) {
                Ok(value) => assert_eq!(Value::Int(1), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_imax_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("imax")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(3), Value::Int(2)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_imin_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("imin")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(3), Value::Int(1), Value::Int(2)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => assert_eq!(Value::None, value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_push_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("push")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Float(3.0)]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false), Value::Float(3.0)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_pop_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("pop")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::Bool(false), value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(Vec::new()))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_append_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("append")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
            let arg_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Bool(false), Value::Float(3.0)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false), Value::Float(3.0)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Int(1), Value::Float(2.0)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
            fields2.insert(String::from("b"), Value::Float(3.0));
            fields2.insert(String::from("c"), Value::Bool(false));
            let arg_value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(3.0));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value.clone()]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(2.0));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_insert_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("insert")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2), Value::Float(3.0)]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(3.0), Value::Float(2.0), Value::Bool(false)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(4), Value::Float(3.0)]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false), Value::Float(3.0)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("b"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2, Value::Float(3.0)]) {
                Ok(value) => {
                    assert_eq!(Value::Float(2.0), value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(3.0));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("d"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2, Value::Float(3.0)]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(2.0));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    expected_fields.insert(String::from("d"), Value::Float(3.0));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_remove_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("remove")) {
        Some(fun_value) => {
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(2)]) {
                Ok(value) => {
                    assert_eq!(Value::Float(2.0), value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Bool(false)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), Value::Int(4)]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.0), Value::Bool(false)]))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("b"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    assert_eq!(Value::Float(2.0), value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
            let mut fields: BTreeMap<String, Value> = BTreeMap::new();
            fields.insert(String::from("a"), Value::Int(1));
            fields.insert(String::from("b"), Value::Float(2.0));
            fields.insert(String::from("c"), Value::Bool(false));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("d"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    assert_eq!(Value::None, value);
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(2.0));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_arg_value, arg_value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_errorkind_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("errorkind")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("abc")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_errormsg_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("errormsg")) {
        Some(fun_value) => {
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
fn test_isequal_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isequal")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234), Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234), Value::Int(4567)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_isnotequal_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isnotequal")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234), Value::Int(1234)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234), Value::Int(4567)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_isless_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isless")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(2), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_isgreatereqaul_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isgreaterequal")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(2), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_isgreater_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("isgreater")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_islesseqaul_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("islessequal")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(2)]) {
                Ok(value) => assert_eq!(Value::Bool(false), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Bool(true), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

fn shared_test_fun1_is_applied_with_success_for_f32_and_matrix<F>(fun_name: &str, a: f32, row_count: usize, col_count: usize, xs: &[f32], mut f: F)
    where F: FnMut(f32) -> f32
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from(fun_name)) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(a)]) {
                Ok(Value::Float(b)) => assert!((f(a) - b).abs() < 0.001),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(row_count, col_count, xs))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(Value::Object(object)) => {
                    match &*object {
                        Object::Matrix(matrix) => {
                            assert_eq!(row_count, matrix.row_count());
                            assert_eq!(col_count, matrix.col_count());
                            assert_eq!(false, matrix.is_transposed());
                            let ys = matrix.elems();
                            for i in 0..(row_count * col_count) {
                                assert!((f(xs[i]) - ys[i]).abs() < 0.001);
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Float(a), Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(row_count, col_count, xs))))]))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(Value::Ref(object)) => {
                    let object_g = object.read().unwrap();
                    match &*object_g {
                        MutObject::Array(elems) => {
                            assert_eq!(2, elems.len());
                            match &elems[0] {
                                Value::Float(b) => assert!((f(a) - b).abs() < 0.001),
                                _ => assert!(false),
                            }
                            match &elems[1] {
                                Value::Object(object2) => {
                                    match &**object2 {
                                        Object::Matrix(matrix) => {
                                            assert_eq!(row_count, matrix.row_count());
                                            assert_eq!(col_count, matrix.col_count());
                                            assert_eq!(false, matrix.is_transposed());
                                            let ys = matrix.elems();
                                            for i in 0..(row_count * col_count) {
                                                assert!((f(xs[i]) - ys[i]).abs() < 0.001);
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
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_sigmoid_is_applied_with_success()
{
    let xs = vec![
        -0.5, -0.25,
        0.0, 0.25,
        0.5, 0.75
    ];
    shared_test_fun1_is_applied_with_success_for_f32_and_matrix("sigmoid", 0.5, 3, 2, xs.as_slice(), |a| 1.0 / (1.0 + (-a).exp()));
}

#[test]
fn test_tanh_is_applied_with_success()
{
    let xs = vec![
        -0.5, -0.25,
        0.0, 0.25,
        0.5, 0.75
    ];
    shared_test_fun1_is_applied_with_success_for_f32_and_matrix("tanh", 0.5, 3, 2, xs.as_slice(), f32::tanh);
}

#[test]
fn test_swish_is_applied_with_success()
{
    let xs = vec![
        -0.5, -0.25,
        0.0, 0.25,
        0.5, 0.75
    ];
    shared_test_fun1_is_applied_with_success_for_f32_and_matrix("swish", 0.5, 3, 2, xs.as_slice(), |a| a / (1.0 + (-a).exp()));
}

#[test]
fn test_softmax_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("softmax")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.5)]) {
                Ok(Value::Float(b)) => assert!((((0.5f32).exp() / (0.5f32).exp()) - b).abs() < 0.001),
                _ => assert!(false),
            }
            let xs: Vec<f32> = vec![
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(Matrix::new_with_elems(3, 2, xs.as_slice()))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(Value::Object(object)) => {
                    match &*object {
                        Object::Matrix(matrix) => {
                            assert_eq!(3, matrix.row_count());
                            assert_eq!(2, matrix.col_count());
                            assert_eq!(false, matrix.is_transposed());
                            let ys = matrix.elems();
                            for i in 0..3 {
                                for j in 0..2 {
                                    let mut sum = 0.0f32;
                                    for k in 0..3 {
                                        sum += xs[k * 2 + j].exp();
                                    }
                                    assert!(((xs[i * 2 + j].exp() / sum) - ys[i * 2 + j]).abs() < 0.001);
                                }
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_sqrt_is_applied_with_success()
{
    let xs = vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ];
    shared_test_fun1_is_applied_with_success_for_f32_and_matrix("sqrt", 2.0, 3, 2, xs.as_slice(), f32::sqrt);
}

#[test]
fn test_reallytranspose_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("reallytranspose")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.0)]) {
                Ok(value) => assert_eq!(Value::Float(2.0), value),
                Err(_) => assert!(false),
            }
            let a = matrix![
                [1.0, 2.0],
                [3.0, 4.0],
                [5.0, 6.0]
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let at = vec![
                        1.0, 3.0, 5.0,
                        2.0, 4.0, 6.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(2, 3, TransposeFlag::NoTranspose, at));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_rt_is_alias_to_reallytranspose()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match (root_mod.var(&String::from("rt")), root_mod.var(&String::from("reallytranspose"))) {
        (Some(alias_value), Some(fun_value)) => assert_eq!(fun_value, alias_value),
        (_, _) => assert!(false),
    }
}

#[test]
fn test_repeat_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("repeat")) {
        Some(fun_value) => {
            let a = matrix![
                [1.0],
                [2.0],
                [3.0]
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(2)]) {
                Ok(value) => {
                    let b = vec![
                        1.0, 1.0,
                        2.0, 2.0,
                        3.0, 3.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
            let a = matrix![
                [1.0, 2.0]
            ];
            let arg_value = Value::Object(Arc::new(Object::Matrix(a)));
            match fun_value.apply(&mut interp, &mut env, &[arg_value, Value::Int(3)]) {
                Ok(value) => {
                    let b = vec![
                        1.0, 2.0,
                        1.0, 2.0,
                        1.0, 2.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 2, TransposeFlag::NoTranspose, b));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("mod")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(10), Value::Int(4)]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(10.5), Value::Float(3.5)]) {
                Ok(value) => assert_eq!(Value::Float(10.5 % 3.5), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_abs_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("abs")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3)]) {
                Ok(value) => assert_eq!(Value::Int(3), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(0)]) {
                Ok(value) => assert_eq!(Value::Int(0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(-2)]) {
                Ok(value) => assert_eq!(Value::Int(2), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(3.5)]) {
                Ok(value) => assert_eq!(Value::Float(3.5), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-2.5)]) {
                Ok(value) => assert_eq!(Value::Float(2.5), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

fn shared_test_fun1_is_applied_with_success_for_f32<F>(fun_name: &str, a: f32, mut f: F)
    where F: FnMut(f32) -> f32
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from(fun_name)) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(a)]) {
                Ok(Value::Float(b)) => assert!((f(a) - b).abs() < 0.001),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

fn shared_test_fun2_is_applied_with_success_for_f32<F>(fun_name: &str, a: f32, b: f32, mut f: F)
    where F: FnMut(f32, f32) -> f32
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from(fun_name)) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(a), Value::Float(b)]) {
                Ok(Value::Float(c)) => assert!((f(a, b) - c).abs() < 0.001),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_pow_is_applied_with_success()
{ shared_test_fun2_is_applied_with_success_for_f32("pow", 3.5, 2.5, f32::powf); }

#[test]
fn test_exp_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("exp", 2.5, f32::exp); }

#[test]
fn test_log_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("log", 10.5, f32::ln); }

#[test]
fn test_log2_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("log2", 10.5, f32::log2); }

#[test]
fn test_log10_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("log10", 10.5, f32::log10); }

#[test]
fn test_sin_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("sin", 0.5, f32::sin); }

#[test]
fn test_cos_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("cos", 0.5, f32::cos); }

#[test]
fn test_tan_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("tan", 0.5, f32::tan); }

#[test]
fn test_asin_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("asin", 0.5, f32::asin); }

#[test]
fn test_acos_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("acos", 0.5, f32::acos); }

#[test]
fn test_atan_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("atan", 0.5, f32::atan); }

#[test]
fn test_atan2_is_applied_with_success()
{ shared_test_fun2_is_applied_with_success_for_f32("atan2", 0.25, 0.5, f32::atan2); }

#[test]
fn test_sinh_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("sinh", 0.5, f32::sinh); }

#[test]
fn test_cosh_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("cosh", 0.5, f32::cosh); }

#[test]
fn test_asinh_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("asinh", 0.5, f32::asinh); }

#[test]
fn test_acosh_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("acosh", 1.5, f32::acosh); }

#[test]
fn test_atanh_is_applied_with_success()
{ shared_test_fun1_is_applied_with_success_for_f32("atanh", 0.5, f32::atanh); }

#[test]
fn test_sign_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("sign")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.5)]) {
                Ok(value) => assert_eq!(Value::Float(1.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(1.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-3.5)]) {
                Ok(value) => assert_eq!(Value::Float(-1.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_floor_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("floor")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.5)]) {
                Ok(value) => assert_eq!(Value::Float(2.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-2.5)]) {
                Ok(value) => assert_eq!(Value::Float(-3.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_ceil_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("ceil")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.5)]) {
                Ok(value) => assert_eq!(Value::Float(3.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-2.5)]) {
                Ok(value) => assert_eq!(Value::Float(-2.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_round_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("round")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.5)]) {
                Ok(value) => assert_eq!(Value::Float(3.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-2.5)]) {
                Ok(value) => assert_eq!(Value::Float(-3.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_trunc_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("trunc")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(2.5)]) {
                Ok(value) => assert_eq!(Value::Float(2.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(0.0)]) {
                Ok(value) => assert_eq!(Value::Float(0.0), value),
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Float(-2.5)]) {
                Ok(value) => assert_eq!(Value::Float(-2.0), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_rand_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("rand")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[]) {
                Ok(Value::Float(n)) => assert!(n >= 0.0 && n < 1.0),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_randi_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("randi")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(10)]) {
                Ok(Value::Int(n)) => assert!(n >= 1 && n <= 10),
                _ => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(3), Value::Int(12)]) {
                Ok(Value::Int(n)) => assert!(n >= 3 && n <= 12),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_str2int_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("str2int")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("1234"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(1234), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("1234xxx"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let expected_value = Value::Object(Arc::new(Object::Error(String::from("parseint"), String::from("invalid digit found in string"))));
                    assert_eq!(expected_value, value);
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_str2float_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("str2float")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("12.34"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Float(12.34), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("12.34xxx"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let expected_value = Value::Object(Arc::new(Object::Error(String::from("parsefloat"), String::from("invalid float literal"))));
                    assert_eq!(expected_value, value);
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_hex2dec_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("hex2dec")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("123abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(0x123abc), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("0x123abc"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(0x123abc), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("0X123ABC"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(0x123abc), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("123abcxxx"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => {
                    let expected_value = Value::Object(Arc::new(Object::Error(String::from("parseint"), String::from("invalid digit found in string"))));
                    assert_eq!(expected_value, value);
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_char2code_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("char2code")) {
        Some(fun_value) => {
            let arg_value = Value::Object(Arc::new(Object::String(String::from("a"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(97), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("ą"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(261), value),
                _ => assert!(false),
            }
            let arg_value = Value::Object(Arc::new(Object::String(String::from("1234"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
                Ok(value) => assert_eq!(Value::Int(49), value),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_code2char_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("code2char")) {
        Some(fun_value) => {
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(97)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("a")))), value),
                _ => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(261)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("ą")))), value),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

fn shared_test_fun_is_existent(fun_name: &str, f: fn(&mut Interp, &mut Env, &[Value]) -> Result<Value>)
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match root_mod.var(&String::from(fun_name)) {
        Some(fun_value) => {
            let expected_fun_value = Value::Object(Arc::new(Object::BuiltinFun(String::from(fun_name), f)));
            assert_eq!(expected_fun_value, *fun_value);
        },
        None => assert!(false),
    }
}

#[test]
fn test_readline_is_existent()
{ shared_test_fun_is_existent("readline", readline); }

#[test]
fn test_format_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("format")) {
        Some(fun_value) => {
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from(" abc "))));
            match fun_value.apply(&mut interp, &mut env, &[Value::Int(1234), arg_value2, Value::Float(12.34)]) {
                Ok(value) => assert_eq!(Value::Object(Arc::new(Object::String(String::from("1234 abc 12.3400")))), value),
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_print_is_existent()
{ shared_test_fun_is_existent("print", print); }

#[test]
fn test_println_is_existent()
{ shared_test_fun_is_existent("println", println); }

#[test]
fn test_eprint_is_existent()
{ shared_test_fun_is_existent("eprint", eprint); }

#[test]
fn test_eprintln_is_existent()
{ shared_test_fun_is_existent("eprintln", eprintln); }

#[test]
fn test_flush_is_existent()
{ shared_test_fun_is_existent("flush", flush); }

#[test]
fn test_eflush_is_existent()
{ shared_test_fun_is_existent("eflush", eflush); }

#[test]
fn test_cd_is_existent()
{ shared_test_fun_is_existent("cd", cd); }

#[test]
fn test_pwd_is_existent()
{ shared_test_fun_is_existent("pwd", pwd); }

#[test]
fn test_exist_is_existent()
{ shared_test_fun_is_existent("exist", exist); }

#[test]
fn test_filetype_is_existent()
{ shared_test_fun_is_existent("filetype", filetype); }

#[test]
fn test_dir_is_existent()
{ shared_test_fun_is_existent("dir", dir); }

#[test]
fn test_ls_is_alias_to_dir()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    match (root_mod.var(&String::from("ls")), root_mod.var(&String::from("dir"))) {
        (Some(alias_value), Some(fun_value)) => assert_eq!(fun_value, alias_value),
        (_, _) => assert!(false),
    }
}

#[test]
fn test_mkdir_is_existent()
{ shared_test_fun_is_existent("mkdir", mkdir); }

#[test]
fn test_rmdir_is_existent()
{ shared_test_fun_is_existent("rmdir", rmdir); }

#[test]
fn test_rmfile_is_existent()
{ shared_test_fun_is_existent("rmfile", rmfile); }

#[test]
fn test_copy_is_existent()
{ shared_test_fun_is_existent("copy", copy); }

#[test]
fn test_spawn_is_existent()
{ shared_test_fun_is_existent("spawn", spawn); }

#[test]
fn test_load_is_existent()
{ shared_test_fun_is_existent("load", load); }

#[test]
fn test_save_is_existent()
{ shared_test_fun_is_existent("save", save); }

#[test]
fn test_loadstr_is_existent()
{ shared_test_fun_is_existent("loadstr", loadstr); }

#[test]
fn test_savestr_is_existent()
{ shared_test_fun_is_existent("savestr", savestr); }

#[test]
fn test_args_is_existent()
{ shared_test_fun_is_existent("args", args); }

#[test]
fn test_env_is_existent()
{ shared_test_fun_is_existent("env", env); }

#[test]
fn test_uselib_is_existent()
{ shared_test_fun_is_existent("uselib", uselib); }

#[test]
fn test_reuselib_is_existent()
{ shared_test_fun_is_existent("reuselib", reuselib); }

#[test]
fn test_run_is_existent()
{ shared_test_fun_is_existent("run", run); }

#[test]
fn test_removemod_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let fun_value = {
        let root_mod_g = root_mod.read().unwrap();
        match root_mod_g.var(&String::from("removemod")) {
            Some(fun_value) => fun_value.clone(),
            None => {
                assert!(false);
                return;
            },
        }
    };
    env.add_and_push_mod(String::from("a")).unwrap();
    env.pop_mod().unwrap();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("a"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => {
            assert_eq!(Value::None, value);
            let root_mod_g = root_mod.read().unwrap();
            assert_eq!(false, root_mod_g.has_mod(&String::from("a")));
        },
        Err(_) => assert!(false),
    }
    env.add_and_push_mod(String::from("b")).unwrap();
    env.pop_mod().unwrap();
    env.add_and_push_mod(String::from("a")).unwrap();
    env.add_and_push_mod(String::from("b")).unwrap();
    env.pop_mod().unwrap();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("b"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => {
            assert_eq!(Value::None, value);
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    assert_eq!(false, a_mod_g.has_mod(&String::from("b")));
                },
                None => assert!(false),
            }
            assert_eq!(true, root_mod_g.has_mod(&String::from("b")));
        },
        Err(_) => assert!(false),
    }
    env.pop_mod().unwrap();
}

#[test]
fn test_removevar_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let fun_value = {
        let root_mod_g = root_mod.read().unwrap();
        match root_mod_g.var(&String::from("removevar")) {
            Some(fun_value) => fun_value.clone(),
            None => {
                assert!(false);
                return;
            },
        }
    };
    env.set_var(&Name::Var(String::from("X")), Value::Int(1234)).unwrap();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("X"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => {
            assert_eq!(Value::None, value);
            let root_mod_g = root_mod.read().unwrap();
            assert_eq!(false, root_mod_g.has_var(&String::from("X")));
        },
        Err(_) => assert!(false),
    }
    env.set_var(&Name::Var(String::from("Y")), Value::Int(1234)).unwrap();
    env.add_and_push_mod(String::from("a")).unwrap();
    env.set_var(&Name::Var(String::from("Y")), Value::Int(1234)).unwrap();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("Y"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => {
            assert_eq!(Value::None, value);
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    assert_eq!(false, a_mod_g.has_var(&String::from("Y")));
                },
                None => assert!(false),
            }
            assert_eq!(true, root_mod_g.has_var(&String::from("Y")));
        },
        Err(_) => assert!(false),
    }
    env.pop_mod().unwrap();
}

#[test]
fn test_removelocalvar_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let fun_value = {
        let root_mod_g = root_mod.read().unwrap();
        match root_mod_g.var(&String::from("removelocalvar")) {
            Some(fun_value) => fun_value.clone(),
            None => {
                assert!(false);
                return;
            },
        }
    };
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(1), Value::Float(2.5)];
    env.push_fun_mod_and_local_vars(&[], args.as_slice(), arg_values.as_slice()).unwrap();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("Y"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => {
            assert_eq!(Value::Bool(true), value);
            match env.stack().last() {
                Some((_, local_vars)) => {
                    assert_eq!(true, local_vars.contains_key(&String::from("X")));
                    assert_eq!(false, local_vars.contains_key(&String::from("Y")));
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    let arg_value = Value::Object(Arc::new(Object::String(String::from("X"))));
    match fun_value.apply(&mut interp, &mut env, &[arg_value]) {
        Ok(value) => assert_eq!(Value::Bool(false), value),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_checkintr_is_existent()
{ shared_test_fun_is_existent("checkintr", checkintr); }
