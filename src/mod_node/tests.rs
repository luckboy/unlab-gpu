//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_mod_node_add_used_mod_adds_used_module_nodes_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod1, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod1_g = mod1.read().unwrap();
    match mod1_g.used_mod(&String::from("a")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod2, &used_mod)),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.used_mod(&String::from("b")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod3, &used_mod)),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.used_mod(&String::from("c")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod4, &used_mod)),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_add_used_mod_recursively_adds_used_module_node_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod2, String::from("b"), mod1.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod2_g = mod2.read().unwrap();
    match mod2_g.used_mod(&String::from("b")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Weak(used_mod) => {
                    match used_mod.upgrade() {
                        Some(used_mod) => assert!(Arc::ptr_eq(&mod1, &used_mod)),
                        None => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_remove_used_mod_removes_used_module_node_from_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod1, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut mod1_g = mod1.write().unwrap();
    mod1_g.remove_used_mod(&String::from("b"));
    match mod1_g.used_mod(&String::from("a")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod2, &used_mod)),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
    match mod1_g.used_mod(&String::from("b")) {
        None => assert!(true),
        Some(_) => assert!(false),
    }
    match mod1_g.used_mod(&String::from("c")) {
        Some(used_mod_ref) => {
            match used_mod_ref {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod4, &used_mod)),
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_add_used_var_adds_used_variables_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_used_var(&mod1, String::from("a"), mod2.clone(), String::from("d")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_var(&mod1, String::from("b"), mod3.clone(), String::from("e")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_var(&mod1, String::from("c"), mod4.clone(), String::from("f")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod1_g = mod1.read().unwrap();
    match mod1_g.used_var(&String::from("a")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod2, &used_mod)),
                _ => assert!(false),
            }
            assert_eq!(String::from("d"), *used_var.ident())
        },
        None => assert!(false),
    }
    match mod1_g.used_var(&String::from("b")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod3, &used_mod)),
                _ => assert!(false),
            }
            assert_eq!(String::from("e"), *used_var.ident())
        },
        None => assert!(false),
    }
    match mod1_g.used_var(&String::from("c")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod4, &used_mod)),
                _ => assert!(false),
            }
            assert_eq!(String::from("f"), *used_var.ident())
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_add_used_var_recursively_adds_used_variable_to_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    match ModNode::add_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_var(&mod2, String::from("b"), mod1.clone(), String::from("c")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mod2_g = mod2.read().unwrap();
    match mod2_g.used_var(&String::from("b")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Weak(used_mod) => {
                    match used_mod.upgrade() {
                        Some(used_mod) => assert!(Arc::ptr_eq(&mod1, &used_mod)),
                        None => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("c"), *used_var.ident())
        },
        None => assert!(false),
    }
}

#[test]
fn test_mod_node_remove_used_var_removes_used_variable_from_module_node()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    match ModNode::add_used_var(&mod1, String::from("a"), mod2.clone(), String::from("d")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_var(&mod1, String::from("b"), mod3.clone(), String::from("e")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_var(&mod1, String::from("c"), mod4.clone(), String::from("f")) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut mod1_g = mod1.write().unwrap();
    mod1_g.remove_used_var(&String::from("b"));
    match mod1_g.used_var(&String::from("a")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod2, &used_mod)),
                _ => assert!(false),
            }
            assert_eq!(String::from("d"), *used_var.ident())
        },
        None => assert!(false),
    }
    match mod1_g.used_var(&String::from("b")) {
        None => assert!(true),
        Some(_) => assert!(false),
    }
    match mod1_g.used_var(&String::from("c")) {
        Some(used_var) => {
            match used_var.mod1() {
                ModNodeRef::Arc(used_mod) => assert!(Arc::ptr_eq(&mod4, &used_mod)),
                _ => assert!(false),
            }
            assert_eq!(String::from("f"), *used_var.ident())
        },
        None => assert!(false),
    }
}

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
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("c")], true) {
        Ok(Some(nested_mod)) => assert!(Arc::ptr_eq(&mod4, &nested_mod)),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_returns_same_module_for_empty_identifiers()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    match ModNode::mod_from(&mod1, &[], true) {
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
    match ModNode::mod_from(&mod1, &[String::from("b"), String::from("c")], true) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_returns_module_in_used_module_for_identifiers_and_set_flag_of_used_modules()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
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
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("c")], true) {
        Ok(Some(nested_mod)) => assert!(Arc::ptr_eq(&mod4, &nested_mod)),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_returns_module_in_module_for_identifiers_and_set_flag_of_used_modules()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("a"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("b"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod3, String::from("b"), mod5.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("b")], true) {
        Ok(Some(nested_mod)) => assert!(Arc::ptr_eq(&mod5, &nested_mod)),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_does_not_return_nested_used_module_for_identifiers_and_set_flag_of_used_modules()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod1, String::from("b"), mod3.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_used_mod(&mod2, String::from("c"), mod4.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::add_mod(&mod2, String::from("d"), mod5.clone()) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("c")], true) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_mod_node_mod_from_does_not_return_module_in_used_module_for_identifiers_and_unset_flag_of_used_modules()
{
    let mod1: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(1)));
    let mod2: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(2)));
    let mod3: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(3)));
    let mod4: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(4)));
    let mod5: Arc<RwLock<ModNode<i32, i32>>> = Arc::new(RwLock::new(ModNode::new(5)));
    match ModNode::add_used_mod(&mod1, String::from("a"), mod2.clone()) {
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
    match ModNode::mod_from(&mod1, &[String::from("a"), String::from("c")], false) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}
