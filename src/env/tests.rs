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
fn test_env_add_fun_adds_functions()
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
                Some(value) => assert_eq!(Value::Object(Arc::new(Object::Fun(vec![String::from("a")], String::from("f"), fun.clone()))), *value),
                None => assert!(false),
            }
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("g")) {
                        Some(value) => assert_eq!(Value::Object(Arc::new(Object::Fun(vec![String::from("a"), String::from("b")], String::from("g"), fun2.clone()))), *value),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("h")) {
                Some(value) => assert_eq!(Value::Object(Arc::new(Object::Fun(vec![String::from("a")], String::from("h"), fun3.clone()))), *value),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.var(&String::from("i")) {
        Some(value) => assert_eq!(Value::Object(Arc::new(Object::Fun(Vec::new(), String::from("i"), fun4.clone()))), *value),
        None => assert!(false),
    }
}

#[test]
fn test_env_add_fun_does_not_add_function()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    let fun = Arc::new(Fun(Vec::new(), Vec::new()));
    let fun2 = Arc::new(Fun(Vec::new(), Vec::new()));
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("f"), fun.clone()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_fun(String::from("f"), fun2.clone()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_push_fun_mod_and_local_vars_pushes_function_module_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(1), Value::Float(2.5)];
    match env.push_fun_mod_and_local_vars(&[String::from("a"), String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(1, env.stack().len());
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    match a_mod_g.mod1(&String::from("b")) {
                        Some(a_b_mod) => assert!(Arc::ptr_eq(a_b_mod, fun_mod)),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Z"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Bool(true), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(2, env.stack().len());
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => assert!(Arc::ptr_eq(a_mod, fun_mod)),
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Z")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    assert_eq!(1, env.stack().len());
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    match a_mod_g.mod1(&String::from("b")) {
                        Some(a_b_mod) => assert!(Arc::ptr_eq(a_b_mod, fun_mod)),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    assert_eq!(true, env.stack().is_empty());
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.push_fun_mod_and_local_vars(&[String::from("c")], &[], &[]) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(1, env.stack().len());
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("c")) {
                Some(c_mod) => assert!(Arc::ptr_eq(c_mod, fun_mod)),
                None => assert!(false),
            }
            assert_eq!(true, local_vars.is_empty());
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    assert_eq!(true, env.stack().is_empty());
}

#[test]
fn test_env_push_fun_mod_and_local_vars_does_not_push_function_module_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(1)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(true, env.stack().is_empty());
}

#[test]
fn test_env_push_fun_mod_and_local_vars_complains_on_no_function_module()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(1), Value::Float(2.5)];
    match env.push_fun_mod_and_local_vars(&[String::from("c")], args.as_slice(), arg_values.as_slice()) {
        Err(Error::NoFunMod) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(true, env.stack().is_empty());
}

#[test]
fn test_env_reset_resets_environment()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(1), Value::Float(2.5)];
    match env.push_fun_mod_and_local_vars(&[String::from("a"), String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Z"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Bool(true), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.reset() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();    
            assert_eq!(true, a_mod_g.has_mod(&String::from("b")));
        },
        None => assert!(false),
    }
    assert_eq!(false, root_mod_g.has_mod(&String::from("c")));
    assert!(Arc::ptr_eq(env.root_mod(), env.current_mod()));
    assert_eq!(true, env.mod_idents().is_empty());
    assert_eq!(true, env.stack().is_empty());
}

#[test]
fn test_env_var_returns_values_for_variable_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(true));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.var(&Name::Var(String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Z"))) {
        Ok(Some(Value::Bool(true))) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.var(&Name::Var(String::from("X"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y"))) {
        Ok(Some(Value::Int(2))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Z"))) {
        Ok(Some(Value::Bool(true))) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Z"))) {
        Ok(Some(Value::Bool(true))) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_does_not_return_value_for_variable_name()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.var(&Name::Var(String::from("Z"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_relative_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("X"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("Y"))) {
        Ok(Some(Value::Int(2))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("a")], String::from("Z"))) {
        Ok(Some(Value::Bool(false))) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("b")], String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("b")], String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_does_not_return_values_for_relative_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("b")], String::from("Z"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("X"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("a")], String::from("Z"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("X"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_absolute_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_does_not_returns_values_for_absolute_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("Z"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("Z"))) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_variable_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("W"), Value::Bool(true));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a"), String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("X"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y"))) {
        Ok(Some(Value::Int(2))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Z"))) {
        Ok(Some(Value::Bool(false))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("W"))) {
        Ok(Some(Value::Bool(true))) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(3), Value::Float(1.5)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("X"))) {
        Ok(Some(Value::Int(3))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("W"))) {
        Ok(Some(Value::Bool(true))) => assert!(true),
        _ => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_relative_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("d")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("b"), String::from("c")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(Vec::new(), String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(Vec::new(), String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("c")], String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(vec![String::from("a")], String::from("Z"))) {
        Ok(Some(Value::Bool(false))) => assert!(true),
        _ => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
}

#[test]
fn test_env_var_returns_values_for_absolute_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Bool(true));
        current_mod_g.add_var(String::from("Y"), Value::Int(3));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("X"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Abs(vec![String::from("a")], String::from("Y"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(2.5, n),
        _ => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_variable_names_and_used_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let b_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let c_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    ModNode::add_used_var(env.current_mod(), String::from("X2"), a_mod, String::from("X")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Y2"), b_mod, String::from("Y")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Z2"), c_mod, String::from("Z")).unwrap();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y2"), Value::Float(1.5));
    }
    match env.var(&Name::Var(String::from("X2"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Y2"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Var(String::from("Z2"))) {
        Ok(Some(Value::Bool(false))) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_var_returns_values_for_relative_names_and_used_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let b_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let c_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    ModNode::add_used_var(env.current_mod(), String::from("X2"), a_mod, String::from("X")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Y2"), b_mod, String::from("Y")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Z2"), c_mod, String::from("Z")).unwrap();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y2"), Value::Float(1.5));
    }
    match env.var(&Name::Rel(Vec::new(), String::from("X2"))) {
        Ok(Some(Value::Int(1))) => assert!(true),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(Vec::new(), String::from("Y2"))) {
        Ok(Some(Value::Float(n))) => assert_eq!(1.5, n),
        _ => assert!(false),
    }
    match env.var(&Name::Rel(Vec::new(), String::from("Z2"))) {
        Ok(Some(Value::Bool(false))) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_variable_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.set_var(&Name::Var(String::from("X")), Value::Int(3)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Z")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("X")), Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Z")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Int(3)) => assert!(true),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(2)) => assert!(true),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Z")) {
                        Some(Value::Bool(false)) => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_relative_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("X")), Value::Int(3)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("Z")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("a")], String::from("Z")), Value::Int(4)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("b")], String::from("X")), Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("b")], String::from("Z")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(c_mod) => {
            let c_mod_g = c_mod.read().unwrap();
            match c_mod_g.var(&String::from("Z")) {
                Some(Value::Int(4)) => assert!(true),
                _ => assert!(false),
            }
        }
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("b")) {
        Some(b_mod) => {
            let b_mod_g = b_mod.read().unwrap();
            match b_mod_g.mod1(&String::from("c")) {
                Some(b_c_mod) => {
                    let b_c_mod_g = b_c_mod.read().unwrap();
                    match b_c_mod_g.var(&String::from("X")) {
                        Some(Value::Int(3)) => assert!(true),
                        _ => assert!(false),
                    }
                    match b_c_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(2)) => assert!(true),
                        _ => assert!(false),
                    }
                    match b_c_mod_g.var(&String::from("Z")) {
                        Some(Value::Bool(false)) => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match b_mod_g.var(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match b_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
            match b_mod_g.var(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_does_not_set_values_for_relative_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("X")), Value::Bool(true)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("X")), Value::Int(1)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Float(n)) => assert_eq!(1.5, *n),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(2)) => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_absolute_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("X")), Value::Int(3)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("Z")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("Y")), Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("W")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("a")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Float(n)) => assert_eq!(1.5, *n),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(2)) => assert!(true),
                        _ => assert!(false),
                    }
                    assert_eq!(false, a_b_mod_g.has_var(&String::from("Z")));
                    assert_eq!(false, a_b_mod_g.has_var(&String::from("W")));
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Y")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Z")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("W")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_does_not_returns_values_for_absolute_names()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Float(1.5));
        current_mod_g.add_var(String::from("Y"), Value::Int(2));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("c")], String::from("X")), Value::Bool(true)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("c")], String::from("X")), Value::Int(1)) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("a")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Float(n)) => assert_eq!(1.5, *n),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(2)) => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_variable_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a"), String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("X")), Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Z")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    match a_mod_g.mod1(&String::from("b")) {
                        Some(a_b_mod) => assert!(Arc::ptr_eq(a_b_mod, fun_mod)),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Int(3), Value::Float(1.5)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("X")), Value::Int(3)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Z")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => assert!(Arc::ptr_eq(a_mod, fun_mod)),
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Z")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(a_mod) => {
                    let a_mod_g = a_mod.read().unwrap();
                    match a_mod_g.mod1(&String::from("b")) {
                        Some(a_b_mod) => assert!(Arc::ptr_eq(a_b_mod, fun_mod)),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("b")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Int(1)) => assert!(true),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Float(n)) => assert_eq!(2.5, *n),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Z")) {
                        Some(Value::Bool(false)) => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_relative_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("d")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("b"), String::from("c")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(Vec::new(), String::from("X")), Value::Int(3)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(Vec::new(), String::from("Z")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("b")) {
                Some(b_mod) => {
                    let b_mod_g = b_mod.read().unwrap();
                    match b_mod_g.mod1(&String::from("c")) {
                        Some(b_c_mod) => assert!(Arc::ptr_eq(b_c_mod, fun_mod)),
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("b")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("X")), Value::Bool(true)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("c")], String::from("Z")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(vec![String::from("a")], String::from("Z")), Value::Int(4)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("b")) {
                Some(b_mod) => assert!(Arc::ptr_eq(b_mod, fun_mod)),
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(c_mod) => {
            let c_mod_g = c_mod.read().unwrap();
            match c_mod_g.var(&String::from("Z")) {
                Some(Value::Int(4)) => assert!(true),
                _ => assert!(false),
            }
        }
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("b")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("c")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Bool(true)) => assert!(true),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Float(n)) => assert_eq!(2.5, *n),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Z")) {
                        Some(Value::Float(n)) => assert_eq!(3.5, *n),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    assert_eq!(true, root_mod_g.has_mod(&String::from("d")));
}

#[test]
fn test_env_set_var_sets_values_for_absolute_names_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Bool(true));
        current_mod_g.add_var(String::from("Y"), Value::Int(3));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[String::from("a")], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("X")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Abs(vec![String::from("a")], String::from("Z")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            let root_mod_g = root_mod.read().unwrap();
            match root_mod_g.mod1(&String::from("a")) {
                Some(b_mod) => assert!(Arc::ptr_eq(b_mod, fun_mod)),
                None => assert!(false),
            }
            match local_vars.get(&String::from("X")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.mod1(&String::from("a")) {
                Some(a_b_mod) => {
                    let a_b_mod_g = a_b_mod.read().unwrap();
                    match a_b_mod_g.var(&String::from("X")) {
                        Some(Value::Bool(true)) => assert!(true),
                        _ => assert!(false),
                    }
                    match a_b_mod_g.var(&String::from("Y")) {
                        Some(Value::Int(3)) => assert!(true),
                        _ => assert!(false),
                    }
                    assert_eq!(false, a_b_mod_g.has_var(&String::from("Z")));
                },
                None => assert!(false),
            }
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
            match a_mod_g.var(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_variable_names_and_used_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let b_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let c_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    ModNode::add_used_var(env.current_mod(), String::from("X2"), a_mod, String::from("X")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Y2"), b_mod, String::from("Y")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Z2"), c_mod, String::from("Z")).unwrap();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y2"), Value::Float(1.5));
    }
    match env.set_var(&Name::Var(String::from("X2")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Y2")), Value::Int(2)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Z2")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("b")) {
        Some(b_mod) => {
            let b_mod_g = b_mod.read().unwrap();
            match b_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("c")) {
        Some(c_mod) => {
            let c_mod_g = c_mod.read().unwrap();
            match c_mod_g.var(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.var(&String::from("Y2")) {
        Some(Value::Int(2)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_variable_names_and_used_variables_and_local_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let b_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    ModNode::add_used_var(env.current_mod(), String::from("X2"), a_mod, String::from("X")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Y2"), b_mod, String::from("Y")).unwrap();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y2"), Value::Float(1.5));
    }
    match env.push_fun_mod_and_local_vars(&[], &[], &[]) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("X2")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Var(String::from("Y2")), Value::Int(2)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            assert!(Arc::ptr_eq(&root_mod, fun_mod));
            match local_vars.get(&String::from("X2")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y2")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("b")) {
        Some(b_mod) => {
            let b_mod_g = b_mod.read().unwrap();
            match b_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.var(&String::from("Y2")) {
        Some(Value::Float(n)) => assert_eq!(1.5, *n),
        _ => assert!(false),
    }
}

#[test]
fn test_env_set_var_sets_values_for_relative_names_and_used_variables()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    match env.add_and_push_mod(String::from("a")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let a_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("X"), Value::Int(1));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("b")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let b_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y"), Value::Float(2.5));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.add_and_push_mod(String::from("c")) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let c_mod = env.current_mod().clone();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Z"), Value::Bool(false));
    }
    match env.pop_mod() {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    ModNode::add_used_var(env.current_mod(), String::from("X2"), a_mod, String::from("X")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Y2"), b_mod, String::from("Y")).unwrap();
    ModNode::add_used_var(env.current_mod(), String::from("Z2"), c_mod, String::from("Z")).unwrap();
    {
        let mut current_mod_g = env.current_mod().write().unwrap();
        current_mod_g.add_var(String::from("Y2"), Value::Float(1.5));
    }
    match env.set_var(&Name::Rel(Vec::new(), String::from("X2")), Value::Bool(false)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(Vec::new(), String::from("Y2")), Value::Int(2)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.set_var(&Name::Rel(Vec::new(), String::from("Z2")), Value::Float(3.5)) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.mod1(&String::from("a")) {
        Some(a_mod) => {
            let a_mod_g = a_mod.read().unwrap();
            match a_mod_g.var(&String::from("X")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("b")) {
        Some(b_mod) => {
            let b_mod_g = b_mod.read().unwrap();
            match b_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.mod1(&String::from("c")) {
        Some(c_mod) => {
            let c_mod_g = c_mod.read().unwrap();
            match c_mod_g.var(&String::from("Z")) {
                Some(Value::Float(n)) => assert_eq!(3.5, *n),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match root_mod_g.var(&String::from("Y2")) {
        Some(Value::Int(2)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_env_remove_local_var_removes_local_variable()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            assert!(Arc::ptr_eq(&root_mod, fun_mod));
            match local_vars.get(&String::from("X")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    let args = vec![
        Arg(String::from("X"), Pos::new(Arc::new(String::from("test.unl")), 1, 1)),
        Arg(String::from("Y"), Pos::new(Arc::new(String::from("test.unl")), 1, 2)),
    ];
    let arg_values = vec![Value::Float(1.5), Value::Int(2)];
    match env.push_fun_mod_and_local_vars(&[], args.as_slice(), arg_values.as_slice()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(true, env.remove_local_var(&String::from("X")));
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            assert!(Arc::ptr_eq(&root_mod, fun_mod));
            assert_eq!(false, local_vars.contains_key(&String::from("X")));
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
    match env.stack().last() {
        Some((fun_mod, local_vars)) => {
            assert!(Arc::ptr_eq(&root_mod, fun_mod));
            match local_vars.get(&String::from("X")) {
                Some(Value::Float(n)) => assert_eq!(1.5, *n),
                _ => assert!(false),
            }
            match local_vars.get(&String::from("Y")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    env.pop_fun_mod_and_local_vars();
}

#[test]
fn test_env_remove_local_var_does_not_remove_local_variable()
{
    let root_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
    let mut env = Env::new(root_mod.clone());
    assert_eq!(false, env.remove_local_var(&String::from("X")));
}
