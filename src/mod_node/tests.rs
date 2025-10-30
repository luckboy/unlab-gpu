//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_mod_node_add_mod_adds_module_nodes_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod1_g = mod1.read().unwrap();
    match mod1_g.mod1(&String::from("a")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod2, &child));
            let mod2_g = mod2.read().unwrap();
            match mod2_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.mod1(&String::from("b")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod3, &child));
            let mod3_g = mod3.read().unwrap();
            match mod3_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.mod1(&String::from("c")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod4, &child));
            let mod4_g = mod4.read().unwrap();
            match mod4_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_add_mod_replaces_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("a"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod1_g = mod1.read().unwrap();
    match mod1_g.mod1(&String::from("a")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod3, &child));
            let mod3_g = mod3.read().unwrap();
            match mod3_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    let mod2_g = mod2.read().unwrap();
    match mod2_g.parent() {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[test]
fn test_mod_node_add_mod_complains_on_already_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod2.clone()) {
        Err(Error::AlreadyAddedModNode) => assert!(true),
        _ => assert!(false),
    }
    let mod1_g = mod1.read().unwrap();
    match mod1_g.mod1(&String::from("a")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod2, &child));
            let mod2_g = mod2.read().unwrap();
            match mod2_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_remove_mod_removes_module_node_from_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut mod1_g = mod1.write().unwrap();
    match mod1_g.remove_mod(&String::from("b")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match mod1_g.mod1(&String::from("a")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod2, &child));
            let mod2_g = mod2.read().unwrap();
            match mod2_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.mod1(&String::from("b")) {
        None => {
            let mod3_g = mod3.read().unwrap();
            match mod3_g.parent() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
        },
        Some(_) => assert!(false),
    }
    match mod1_g.mod1(&String::from("c")) {
        Some(child) => {
            assert!(Arc::ptr_eq(&mod4, &child));
            let mod4_g = mod4.read().unwrap();
            match mod4_g.parent() {
                Some(parent) => assert!(Arc::ptr_eq(&mod1, &parent)),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_add_mod_adds_variables_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mut mod1_g = mod1.write().unwrap();
    mod1_g.add_var(String::from("a"), 2);
    mod1_g.add_var(String::from("b"), 3);
    mod1_g.add_var(String::from("c"), 4);
    assert_eq!(Some(&2), mod1_g.var(&String::from("a")));
    assert_eq!(Some(&3), mod1_g.var(&String::from("b")));
    assert_eq!(Some(&4), mod1_g.var(&String::from("c")));
}

#[test]
fn test_mod_node_add_mod_removes_variable_from_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mut mod1_g = mod1.write().unwrap();
    mod1_g.add_var(String::from("a"), 2);
    mod1_g.add_var(String::from("b"), 3);
    mod1_g.add_var(String::from("c"), 4);
    mod1_g.remove_var(&String::from("b"));
    assert_eq!(Some(&2), mod1_g.var(&String::from("a")));
    assert_eq!(None, mod1_g.var(&String::from("b")));
    assert_eq!(Some(&4), mod1_g.var(&String::from("c")));
}

#[test]
fn test_mod_node_mod_from_returns_nested_module_for_identifiers()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("d"), mod5.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("c")]) {
        Ok(Some(nested_mod)) => assert!(Arc::ptr_eq(&mod4, &nested_mod)),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_returns_same_module_for_empty_identifiers()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    match ModNode::mod_from(&mod1, &[]) {
        Ok(Some(mod2)) => assert!(Arc::ptr_eq(&mod1, &mod2)),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_does_not_returns_nested_module_for_identifiers()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("d"), mod5.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::mod_from(&mod1, &[String::from("b"), String::from("c")]) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}
