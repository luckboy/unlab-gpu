//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;
use getopts::HasArg;
use getopts::Occur;
use getopts::Options;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::utils::*;
use crate::value::*;

fn create_options<F>(value: &Value, err_msg: &str, mut f: F) -> Result<Options>
    where F: FnMut(String)
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(object)?;
            match &*object_g {
                MutObject::Array(opt_values) => {
                    let mut opts = Options::new();
                    for opt_value in opt_values {
                        match opt_value {
                            Value::Ref(opt_object) => {
                                let opt_object_g = rw_lock_read(opt_object)?;
                                match &*opt_object_g {
                                    MutObject::Array(opt_arg_values) => {
                                        let short_name = match opt_arg_values.get(0) {
                                            Some(opt_arg_value) => {
                                                let mut s = String::new();
                                                match format!("{}", opt_arg_value).chars().next() {
                                                    Some(c) => s.push(c),
                                                    None => (),                                                    
                                                }
                                                s
                                            },
                                            None => return Err(Error::Interp(String::from("no short name for option"))),
                                        };
                                        let long_name = match opt_arg_values.get(1) {
                                            Some(opt_arg_value) => format!("{}", opt_arg_value),
                                            None => return Err(Error::Interp(String::from("no long name for option"))),
                                        };
                                        let desc = match opt_arg_values.get(2) {
                                            Some(opt_arg_value) => format!("{}", opt_arg_value),
                                            None => return Err(Error::Interp(String::from("no description for option"))),
                                        };
                                        let hint = match opt_arg_values.get(3) {
                                            Some(opt_arg_value) => format!("{}", opt_arg_value),
                                            None => String::new(),
                                        };
                                        let hasarg = match opt_arg_values.get(4) {
                                            Some(opt_arg_value) => {
                                                let s = format!("{}", opt_arg_value);
                                                if s == String::from("yes") {
                                                    HasArg::Yes
                                                } else if s == String::from("no") {
                                                    HasArg::No
                                                } else if s == String::from("maybe") {
                                                    HasArg::Maybe
                                                } else {
                                                    return Err(Error::Interp(String::from("invalid has argument for option")));
                                                }
                                            },
                                            None => {
                                                if !hint.is_empty() {
                                                    HasArg::Yes
                                                } else {
                                                    HasArg::No
                                                }
                                            },
                                        };
                                        let occur = match opt_arg_values.get(5) {
                                            Some(opt_arg_value) => {
                                                let s = format!("{}", opt_arg_value);
                                                if s == String::from("req") {
                                                    Occur::Req
                                                } else if s == String::from("optional") {
                                                    Occur::Optional
                                                } else if s == String::from("multi") {
                                                    Occur::Multi
                                                } else {
                                                    return Err(Error::Interp(String::from("invalid has argument for option")));
                                                }
                                            },
                                            None => Occur::Optional,
                                        };
                                        let name = if !long_name.is_empty() {
                                            long_name.clone()
                                        } else {
                                            short_name.clone()
                                        };
                                        if name.is_empty() {
                                            return Err(Error::Interp(String::from("no name for option")))
                                        }
                                        f(name);
                                        opts.opt(short_name.as_str(), long_name.as_str(), desc.as_str(), hint.as_str(), hasarg, occur);
                                    },
                                    _ => return Err(Error::Interp(String::from("invalid option"))),
                                }
                            },
                            _ => return Err(Error::Interp(String::from("invalid option"))),
                        }
                    }
                    Ok(opts)
                },
                _ => Err(Error::Interp(String::from(err_msg))),
            }
        },
        _ => Err(Error::Interp(String::from(err_msg))),
    }
}

pub fn create_args(value: &Value, err_msg: &str) -> Result<Vec<String>>
{
    match value {
        Value::Ref(object) => {
            let object_g = rw_lock_read(object)?;
            match &*object_g {
                MutObject::Array(arg_values) => Ok(arg_values.iter().map(|v| format!("{}", v)).collect()),
                _ => Err(Error::Interp(String::from(err_msg))),
            }
        },
        _ => Err(Error::Interp(String::from(err_msg))),
    }
}

pub fn getopts(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let mut names: Vec<String> = Vec::new();
    let (opts, args) = match (arg_values.get(0), arg_values.get(1)) {
        (Some(opt_value), Some(arg_value)) => {
            let tmp_opts = create_options(&opt_value, "unsupported types for getopts", |s| names.push(s))?;
            let tmp_args = create_args(&arg_value, "unsupported types for getopts")?;
            (tmp_opts, tmp_args)
        },
        (_, _) => return Err(Error::Interp(String::from("no argument"))),
    };
    let matches = match opts.parse(args.as_slice()) {
        Ok(tmp_matches) => tmp_matches,
        Err(err) => return Ok(Value::Object(Arc::new(Object::Error(String::from("getopts"), format!("{}", err))))),
    };
    let mut opt_fields: BTreeMap<String, Value> = BTreeMap::new();
    for name in &names {
        if matches.opt_present(name.as_str()) {
            opt_fields.insert(name.clone(), Value::Ref(Arc::new(RwLock::new(MutObject::Array(matches.opt_strs(name.as_str()).iter().map(|s| Value::Object(Arc::new(Object::String(s.clone())))).collect())))));
        } else {
            opt_fields.insert(name.clone(), Value::None);
        }
    }
    let mut fields: BTreeMap<String, Value> = BTreeMap::new();
    fields.insert(String::from("opts"), Value::Ref(Arc::new(RwLock::new(MutObject::Struct(opt_fields)))));
    fields.insert(String::from("free"), Value::Ref(Arc::new(RwLock::new(MutObject::Array(matches.free.iter().map(|s| Value::Object(Arc::new(Object::String(s.clone())))).collect())))));
    Ok(Value::Ref(Arc::new(RwLock::new(MutObject::Struct(fields)))))
}

pub fn getoptsusage(_interp: &mut Interp, _env: &mut Env, arg_values: &[Value]) -> Result<Value>
{
    if arg_values.len() != 2 {
        return Err(Error::Interp(String::from("invalid number of arguments")));
    }
    let (opts, brief) = match (arg_values.get(0), arg_values.get(1)) {
        (Some(opt_value), Some(brief_value)) => {
            let tmp_opts = create_options(&opt_value, "unsupported type for getoptsusage", |_| ())?;
            (tmp_opts, format!("{}", brief_value))
        },
        (_, _) => return Err(Error::Interp(String::from("no argument"))),
    };
    Ok(Value::Object(Arc::new(Object::String(opts.usage(brief.as_str())))))
}
