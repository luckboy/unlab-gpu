//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_env_add_and_push_mod_adds_and_pushes_modules()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a")], env.mod_idents());
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a"), String::from("b")], env.mod_idents());
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a"), String::from("b"), String::from("c")], env.mod_idents());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a"), String::from("b")], env.mod_idents());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a")], env.mod_idents());
    match env.add_and_push_mod(String::from("d")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a"), String::from("d")], env.mod_idents());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("a")], env.mod_idents());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(true, env.mod_idents().is_empty());
    match env.add_and_push_mod(String::from("e")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(!Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(&[String::from("e")], env.mod_idents());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert!(Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(true, env.mod_idents().is_empty());
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.mod1(&String::from("c")) {
                        Some(_) => assert!(true),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.mod1(&String::from("d")) {
                Some(_) => assert!(true),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("e")) {
        Some(_) => assert!(true),
        None => assert!(false),
    }
}

#[test]
fn test_env_add_and_push_mod_does_not_add_and_push_module()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_add_fun_adds_function()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun2 = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun3 = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun4 = Arc::new(Fun(Vec::new(), Vec::new()));
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("f"), fun.clone()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("g"), fun2.clone()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("h"), fun3.clone()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("i"), fun4.clone()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.var(&String::from("f")) {
                Some(Value::Object(object)) => {
                    match &**object {
                        Object::Fun(idents, ident, f_fun) => {
                            assert_eq!(vec![String::from("a")], *idents);
                            assert_eq!(String::from("f"), *ident);
                            assert!(Arc::ptr_eq(&fun, f_fun));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("g")) {
                        Some(Value::Object(object)) => {
                            match &**object {
                                Object::Fun(idents, ident, g_fun) => {
                                    assert_eq!(vec![String::from("a"), String::from("b")], *idents);
                                    assert_eq!(String::from("g"), *ident);
                                    assert!(Arc::ptr_eq(&fun2, g_fun));
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("h")) {
                Some(Value::Object(object)) => {
                    match &**object {
                        Object::Fun(idents, ident, h_fun) => {
                            assert_eq!(vec![String::from("a")], *idents);
                            assert_eq!(String::from("h"), *ident);
                            assert!(Arc::ptr_eq(&fun3, h_fun));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.var(&String::from("i")) {
        Some(Value::Object(object)) => {
            match &**object {
                Object::Fun(idents, ident, i_fun) => {
                    assert_eq!(true, idents.is_empty());
                    assert_eq!(String::from("i"), *ident);
                    assert!(Arc::ptr_eq(&fun4, i_fun));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
