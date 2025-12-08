//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;
#[cfg(feature = "plot")]
use crate::winit;
use crate::error::*;
use crate::intr::*;
use crate::mod_node::*;
#[cfg(feature = "plot")]
use crate::plot::*;
use crate::tree::*;
use crate::utils::*;
use crate::value::*;

#[cfg(feature = "plot")]
pub type EventLoopProxy = winit::event_loop::EventLoopProxy<PlotterAppEvent>;

#[cfg(not(feature = "plot"))]
#[derive(Clone, Debug)]
pub struct EventLoopProxy(());

#[derive(Clone)]
pub struct SharedEnv
{
    lib_path: OsString,
    doc_path: OsString,
    args: Vec<String>,
    used_libs: HashSet<String>,
    intr_checker: Arc<dyn IntrCheck + Send + Sync>,
    event_loop_proxy: Option<EventLoopProxy>,
    instant: Instant,
}

impl SharedEnv
{
    pub fn new_with_intr_checker_and_event_loop_proxy(lib_path: OsString, doc_path: OsString, args: Vec<String>, intr_checker: Arc<dyn IntrCheck + Send + Sync>, event_loop_proxy: Option<EventLoopProxy>) -> Self
    {
        SharedEnv {
            lib_path,
            doc_path,
            args,
            used_libs: HashSet::new(),
            intr_checker,
            event_loop_proxy,
            instant: Instant::now(),
        }
    }

    pub fn new_with_intr_checker(lib_path: OsString, doc_path: OsString, args: Vec<String>, intr_checker: Arc<dyn IntrCheck + Send + Sync>) -> Self
    { Self::new_with_intr_checker_and_event_loop_proxy(lib_path, doc_path, args, intr_checker, None) }

    pub fn new(lib_path: OsString, doc_path: OsString, args: Vec<String>) -> Self
    { Self::new_with_intr_checker(lib_path, doc_path, args, Arc::new(EmptyIntrChecker::new())) }
    
    pub fn lib_path(&self) -> &OsStr
    { self.lib_path.as_os_str() }

    pub fn doc_path(&self) -> &OsStr
    { self.doc_path.as_os_str() }
    
    pub fn args(&self) -> &[String]
    { self.args.as_slice() }

    pub fn used_libs(&self) -> &HashSet<String>
    { &self.used_libs }

    pub fn has_used_lib(&self, lib: &String) -> bool
    { self.used_libs.contains(lib) }

    pub fn add_used_lib(&mut self, lib: String)
    { self.used_libs.insert(lib); }

    pub fn remove_used_lib(&mut self, lib: &String)
    { self.used_libs.remove(lib); }
    
    pub fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>
    { &self.intr_checker }
    
    pub fn event_loop_proxy(&self) -> Option<&EventLoopProxy>
    {
        match &self.event_loop_proxy {
            Some(event_loop_proxy) => Some(event_loop_proxy),
            None => None,
        }
    }
    
    pub fn instant(&self) -> &Instant
    { &self.instant }
}

#[derive(Clone)]
pub struct Env
{
    root_mod: Arc<RwLock<ModNode<Value, ()>>>,
    current_mod: Arc<RwLock<ModNode<Value, ()>>>,
    mod_idents: Vec<String>,
    stack: Vec<(Arc<RwLock<ModNode<Value, ()>>>, BTreeMap<String, Value>)>,
    script_dir: PathBuf,
    shared_env: Arc<RwLock<SharedEnv>>,
}

impl Env
{
    pub fn new_with_script_dir_and_shared_env(root_mod: Arc<RwLock<ModNode<Value, ()>>>, script_dir: PathBuf, shared_env: Arc<RwLock<SharedEnv>>) -> Self
    {
        Env {
            root_mod: root_mod.clone(),
            current_mod: root_mod,
            mod_idents: Vec::new(),
            stack: Vec::new(),
            script_dir,
            shared_env,
        }
    }

    pub fn new(root_mod: Arc<RwLock<ModNode<Value, ()>>>) -> Self
    { Self::new_with_script_dir_and_shared_env(root_mod, PathBuf::from("."), Arc::new(RwLock::new(SharedEnv::new(OsString::from("."), OsString::from("."), Vec::new())))) }

    pub fn clone_without_stack(&self) -> Self
    {
        Env {
            root_mod: self.root_mod.clone(),
            current_mod: self.current_mod.clone(),
            mod_idents: self.mod_idents.clone(),
            stack: Vec::new(),
            script_dir: self.script_dir.clone(),
            shared_env: self.shared_env.clone(),
        }
    }
    
    pub fn root_mod(&self) -> &Arc<RwLock<ModNode<Value, ()>>>
    { &self.root_mod }

    pub fn current_mod(&self) -> &Arc<RwLock<ModNode<Value, ()>>>
    { &self.current_mod }

    pub fn mod_idents(&self) -> &[String]
    { self.mod_idents.as_slice() }
    
    pub fn stack(&self) -> &[(Arc<RwLock<ModNode<Value, ()>>>, BTreeMap<String, Value>)]
    { self.stack.as_slice() }

    pub fn script_dir(&self) -> &Path
    { self.script_dir.as_path() }

    pub fn shared_env(&self) -> &Arc<RwLock<SharedEnv>>
    { &self.shared_env }
    
    pub fn add_and_push_mod(&mut self, ident: String) -> Result<bool>
    {
        {
            let current_mod_g = rw_lock_read(&self.current_mod)?;
            if current_mod_g.has_mod(&ident) {
                return Ok(false);
            }
        }
        let new_mod: Arc<RwLock<ModNode<Value, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
        ModNode::add_mod(&self.current_mod, ident.clone(), new_mod.clone())?;
        self.current_mod = new_mod;
        self.mod_idents.push(ident);
        Ok(true)
    }
    
    pub fn pop_mod(&mut self) -> Result<bool>
    {
        let parent = {
            let current_mod_g = rw_lock_read(&self.current_mod)?;
            current_mod_g.parent()
        };
        match parent {
            Some(parent) => {
                self.current_mod = parent;
                self.mod_idents.pop();
                Ok(true)
            },
            None => Ok(false),
        }
    }
    
    pub fn add_fun(&self, ident: String, fun: Arc<Fun>) -> Result<bool>
    {
        let mut current_mod_g = rw_lock_write(&self.current_mod)?;
        if current_mod_g.has_var(&ident) {
            return Ok(false);
        }
        current_mod_g.add_var(ident.clone(), Value::Object(Arc::new(Object::Fun(self.mod_idents.clone(), ident, fun))));
        Ok(true)
    }
    
    pub fn push_fun_mod_and_local_vars(&mut self, fun_mod_idents: &[String], args: &[Arg], arg_values: &[Value]) -> Result<bool>
    {
        let fun_mod = match ModNode::mod_from(&self.root_mod, fun_mod_idents, false)? {
            Some(tmp_fun_mod) => tmp_fun_mod,
            None => return Err(Error::NoFunMod),
        };
        if args.len() != arg_values.len() {
            return Ok(false);
        }
        let mut local_vars: BTreeMap<String, Value> = BTreeMap::new();
        for (arg, value) in args.iter().zip(arg_values.iter()) {
            match arg {
                Arg(ident, _) => {
                    local_vars.insert(ident.clone(), value.clone());
                },
            }
        }
        self.stack.push((fun_mod, local_vars));
        Ok(true)
    }
    
    pub fn pop_fun_mod_and_local_vars(&mut self)
    { self.stack.pop(); }
    
    pub fn reset(&mut self) -> Result<()>
    {
        match self.mod_idents.first() { 
            Some(first_ident) => {
                let mut root_mod_g = rw_lock_write(&self.root_mod)?;
                root_mod_g.remove_mod(first_ident)?;
            },
            None => (),
        }
        self.current_mod = self.root_mod.clone();
        self.mod_idents.clear();
        self.stack.clear();
        Ok(())
    }
    
    fn mod_tuple_for_name<'a>(&self, name: &'a Name, is_var: &mut bool, is_set: bool) -> Result<(Option<Arc<RwLock<ModNode<Value, ()>>>>, Cow<'a, String>, Option<Value>)>
    {
        *is_var = false;
        match name {
            Name::Abs(idents, ident) => {
                match ModNode::mod_from(&self.root_mod, idents.as_slice(), false)? {
                    Some(tmp_mod) => Ok((Some(tmp_mod), Cow::Borrowed(ident), None)),
                    None => Ok((None, Cow::Borrowed(ident), None)),
                }
            },
            Name::Rel(idents, ident) => {
                let mod1 = match self.stack.last() {
                    Some((fun_mod, _)) => fun_mod.clone(),
                    None => self.current_mod.clone(),
                };
                if !idents.is_empty() {
                    match ModNode::mod_from(&mod1, idents.as_slice(), true)? {
                        Some(tmp_mod) => Ok((Some(tmp_mod), Cow::Borrowed(ident), None)),
                        None => {
                            match ModNode::mod_from(&self.root_mod, idents.as_slice(), false)? {
                                Some(tmp_mod) => Ok((Some(tmp_mod), Cow::Borrowed(ident), None)),
                                None => Ok((None, Cow::Borrowed(ident), None)),
                            }
                        }
                    }
                } else {
                    let is_defined_var = {
                        let mod_g = rw_lock_read(&mod1)?;
                        mod_g.has_var(ident)
                    };
                    if is_defined_var {
                        Ok((Some(mod1), Cow::Borrowed(ident), None))
                    } else {
                        let mod_g = rw_lock_read(&mod1)?;
                        match mod_g.used_var(ident) {
                            Some(used_var) => Ok((used_var.mod1().to_arc(), Cow::Owned(used_var.ident().clone()), None)),
                            None => Ok((Some(mod1.clone()), Cow::Borrowed(ident), None)),
                        }
                    }
                }
            },
            Name::Var(ident) => {
                *is_var = true;
                let mod1 = match self.stack.last() {
                    Some((fun_mod, _)) => fun_mod.clone(),
                    None => self.current_mod.clone(),
                };
                let local_var_value = if !is_set {
                    match self.stack.last() {
                        Some((_, local_vars)) => local_vars.get(ident).map(|v| v.clone()),
                        None => None, 
                    }
                } else {
                    None
                };
                if local_var_value.is_some() || (is_set && !self.stack.is_empty()) {
                    Ok((Some(mod1), Cow::Borrowed(ident), local_var_value))
                } else {
                    let is_defined_var = {
                        let mod_g = rw_lock_read(&mod1)?;
                        mod_g.has_var(ident)
                    };
                    if is_defined_var {
                        Ok((Some(mod1), Cow::Borrowed(ident), None))
                    } else {
                        let mod_g = rw_lock_read(&mod1)?;
                        match mod_g.used_var(ident) {
                            Some(used_var) => Ok((used_var.mod1().to_arc(), Cow::Owned(used_var.ident().clone()), None)),
                            None => Ok((Some(mod1.clone()), Cow::Borrowed(ident), None)),
                        }
                    }
                }
            },
        }
    }
    
    pub fn var(&self, name: &Name) -> Result<Option<Value>>
    {
        let mut is_var = false;
        let (mod1, ident, value) = self.mod_tuple_for_name(name, &mut is_var, false)?;
        match value {
            Some(value) => return Ok(Some(value)),
            None => (),
        }
        match mod1 {
            Some(mod1) => {
                let mut value: Option<Value>;
                {
                    let mod1_g = rw_lock_read(&mod1)?;
                    value = mod1_g.var(&*ident).map(|v| v.clone());
                }
                if is_var && value.is_none() {
                    let root_mod_g = rw_lock_read(&self.root_mod)?;
                    value = root_mod_g.var(&*ident).map(|v| v.clone());
                }
                Ok(value)
            },
            None => Ok(None),
        }
    }

    pub fn set_var(&mut self, name: &Name, value: Value) -> Result<bool>
    {
        let mut is_var = false;
        let (mod1, ident, _) = self.mod_tuple_for_name(name, &mut is_var, true)?;
        if is_var {
            match self.stack.last_mut() {
                Some((_, local_vars)) => {
                    local_vars.insert(ident.into_owned(), value);
                    return Ok(true)
                },
                None => (),
            }
        }
        match mod1 {
            Some(mod1) => {
                let mut mod1_g = rw_lock_write(&mod1)?;
                mod1_g.add_var(ident.into_owned(), value);
                Ok(true)
            },
            None => Ok(false),
        }
    }
    
    pub fn remove_local_var(&mut self, ident: &String) -> bool
    {
        match self.stack.last_mut() {
            Some((_, local_vars)) => {
                local_vars.remove(ident);
                true
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests;
