//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::ffi::OsString;
use std::path::PathBuf;
use crate::builtins::add_std_builtin_funs;
use crate::mod_node::*;
use super::*;

fn str_array(ss: &[&str]) -> Value
{
    let mut elems: Vec<Value> = Vec::new();
    for s in ss {
        elems.push(Value::Object(Arc::new(Object::String(String::from(*s)))));
    }
    Value::Ref(Arc::new(RwLock::new(MutObject::Array(elems))))
}

fn str_vec(ss: &[&str]) -> Vec<String>
{
    let mut ts: Vec<String> = Vec::new();
    for s in ss {
        ts.push(String::from(*s));
    }
    ts
}

#[test]
fn test_getopts_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let shared_env = SharedEnv::new(OsString::from("."), OsString::from("."), str_vec(&["aaa", "bbb", "-a", "xxx", "-b", "-c", "yyy"]));
    let mut env = Env::new_with_script_dir_and_domain_and_shared_env(Arc::new(RwLock::new(root_mod)), PathBuf::from("."), None, Arc::new(RwLock::new(shared_env)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("getopts")) {
        Some(fun_value) => {
            let mut arg_elems: Vec<Value> = Vec::new();
            arg_elems.push(str_array(&["a", "a-a", "a desc", "AAA", "yes", "req"]));
            arg_elems.push(str_array(&["b", "b-b", "b desc", "BBB", "no", "req"]));
            arg_elems.push(str_array(&["c", "c-c", "c desc", "CCC", "maybe", "req"]));
            arg_elems.push(str_array(&["d", "d-d", "d desc", "DDD", "yes", "optional"]));
            arg_elems.push(str_array(&["e", "e-e", "e desc", "EEE", "no", "optional"]));
            arg_elems.push(str_array(&["f", "f-f", "f desc", "FFF", "maybe", "optional"]));
            arg_elems.push(str_array(&["g", "g-g", "g desc", "GGG", "yes", "multi"]));
            arg_elems.push(str_array(&["h", "h-h", "h desc", "HHH", "no", "multi"]));
            arg_elems.push(str_array(&["i", "i-i", "i desc", "III", "maybe", "multi"]));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(arg_elems))));
            let arg_value2 = str_array(&["aaa", "bbb", "-a", "xxx", "-b", "-c"]);
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    let mut opt_fields: BTreeMap<String, Value> = BTreeMap::new();
                    opt_fields.insert(String::from("a_a"), str_array(&["xxx"]));
                    opt_fields.insert(String::from("b_b"), str_array(&[]));
                    opt_fields.insert(String::from("c_c"), str_array(&[]));
                    opt_fields.insert(String::from("d_d"), Value::None);
                    opt_fields.insert(String::from("e_e"), Value::None);
                    opt_fields.insert(String::from("f_f"), Value::None);
                    opt_fields.insert(String::from("g_g"), Value::None);
                    opt_fields.insert(String::from("h_h"), Value::None);
                    opt_fields.insert(String::from("i_i"), Value::None);
                    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
                    fields2.insert(String::from("opts"), Value::Ref(Arc::new(RwLock::new(MutObject::Struct(opt_fields)))));
                    fields2.insert(String::from("free"), str_array(&["aaa", "bbb"]));
                    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
                    assert_eq!(value2, value);
                },
                Err(_) => assert!(false),
            }
            let arg_value2 = str_array(&[
                "-a", "aaa",
                "-b",
                "-c", "bbb",
                "-d", "ccc",
                "-e",
                "-f", "ddd",
                "-g", "eee",
                "-g", "fff",
                "-h",
                "-h",
                "-i", "ggg",
                "-i",
                "-i", "hhh",
                "xxx",
                "yyy"
            ]);
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(value) => {
                    let mut opt_fields: BTreeMap<String, Value> = BTreeMap::new();
                    opt_fields.insert(String::from("a_a"), str_array(&["aaa"]));
                    opt_fields.insert(String::from("b_b"), str_array(&[]));
                    opt_fields.insert(String::from("c_c"), str_array(&["bbb"]));
                    opt_fields.insert(String::from("d_d"), str_array(&["ccc"]));
                    opt_fields.insert(String::from("e_e"), str_array(&[]));
                    opt_fields.insert(String::from("f_f"), str_array(&["ddd"]));
                    opt_fields.insert(String::from("g_g"), str_array(&["eee", "fff"]));
                    opt_fields.insert(String::from("h_h"), str_array(&[]));
                    opt_fields.insert(String::from("i_i"), str_array(&["ggg", "hhh"]));
                    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
                    fields2.insert(String::from("opts"), Value::Ref(Arc::new(RwLock::new(MutObject::Struct(opt_fields)))));
                    fields2.insert(String::from("free"), str_array(&["xxx", "yyy"]));
                    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
                    assert_eq!(value2, value);
                },
                Err(_) => assert!(false),
            }
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone()]) {
                Ok(value) => {
                    let mut opt_fields: BTreeMap<String, Value> = BTreeMap::new();
                    opt_fields.insert(String::from("a_a"), str_array(&["xxx"]));
                    opt_fields.insert(String::from("b_b"), str_array(&[]));
                    opt_fields.insert(String::from("c_c"), str_array(&["yyy"]));
                    opt_fields.insert(String::from("d_d"), Value::None);
                    opt_fields.insert(String::from("e_e"), Value::None);
                    opt_fields.insert(String::from("f_f"), Value::None);
                    opt_fields.insert(String::from("g_g"), Value::None);
                    opt_fields.insert(String::from("h_h"), Value::None);
                    opt_fields.insert(String::from("i_i"), Value::None);
                    let mut fields2: BTreeMap<String, Value> = BTreeMap::new();
                    fields2.insert(String::from("opts"), Value::Ref(Arc::new(RwLock::new(MutObject::Struct(opt_fields)))));
                    fields2.insert(String::from("free"), str_array(&["aaa", "bbb"]));
                    let value2 = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields2))));
                    assert_eq!(value2, value);
                },
                Err(_) => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_getoptsusage_is_applied_with_success()
{
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
    let mut interp = Interp::new();
    let root_mod = env.root_mod().clone();
    let root_mod_g = root_mod.read().unwrap();
    match root_mod_g.var(&String::from("getoptsusage")) {
        Some(fun_value) => {
            let mut arg_elems: Vec<Value> = Vec::new();
            arg_elems.push(str_array(&["a", "a-a", "a desc", "AAA", "yes", "req"]));
            arg_elems.push(str_array(&["b", "b-b", "b desc", "BBB", "no", "req"]));
            arg_elems.push(str_array(&["c", "c-c", "c desc", "CCC", "maybe", "req"]));
            arg_elems.push(str_array(&["d", "d-d", "d desc", "DDD", "yes", "optional"]));
            arg_elems.push(str_array(&["e", "e-e", "e desc", "EEE", "no", "optional"]));
            arg_elems.push(str_array(&["f", "f-f", "f desc", "FFF", "maybe", "optional"]));
            arg_elems.push(str_array(&["g", "g-g", "g desc", "GGG", "yes", "multi"]));
            arg_elems.push(str_array(&["h", "h-h", "h desc", "HHH", "no", "multi"]));
            arg_elems.push(str_array(&["i", "i-i", "i desc", "III", "maybe", "multi"]));
            let arg_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(arg_elems))));
            let arg_value2 = Value::Object(Arc::new(Object::String(String::from("Usage: script [OPTIONS]"))));
            match fun_value.apply(&mut interp, &mut env, &[arg_value.clone(), arg_value2]) {
                Ok(Value::Object(object)) => {
                    match &*object {
                        Object::String(s) => assert!(s.starts_with("Usage: script [OPTIONS]")),
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        None => assert!(false),
    }
}
