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
use crate::error::*;
use crate::mod_node::*;
use crate::tree::*;
use crate::utils::*;
use crate::value::*;

#[derive(Clone, Debug)]
pub struct Env
{
    root_mod: Arc<RwLock<ModNode<Value, ()>>>,
    current_mod: Arc<RwLock<ModNode<Value, ()>>>,
    mod_idents: Vec<String>,
    stack: Vec<(Arc<RwLock<ModNode<Value, ()>>>, BTreeMap<String, Value>)>,
}

impl Env
{
    pub fn new(root_mod: Arc<RwLock<ModNode<Value, ()>>>) -> Self
    {
        Env {
            root_mod: root_mod.clone(),
            current_mod: root_mod,
            mod_idents: Vec::new(),
            stack: Vec::new(),
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
        let fun_mod = match ModNode::mod_from(&self.current_mod, fun_mod_idents)? {
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
    
    fn mod_pair_for_name<'a>(&self, name: &'a Name, is_var: &mut bool) -> Result<(Option<Arc<RwLock<ModNode<Value, ()>>>>, &'a String)>
    {
        *is_var = false;
        match name {
            Name::Abs(idents, ident) => {
                match ModNode::mod_from(&self.root_mod, idents.as_slice())? {
                    Some(tmp_mod) => Ok((Some(tmp_mod), ident)),
                    None => Ok((None, ident)),
                }
            },
            Name::Rel(idents, ident) => {
                let mod1 = match self.stack.last() {
                    Some((fun_mod, _)) => fun_mod.clone(),
                    None => self.current_mod.clone(),
                };
                match ModNode::mod_from(&mod1, idents.as_slice())? {
                    Some(tmp_mod) => Ok((Some(tmp_mod), ident)),
                    None => {
                        match ModNode::mod_from(&self.root_mod, idents.as_slice())? {
                            Some(tmp_mod) => Ok((Some(tmp_mod), ident)),
                            None => Ok((None, ident)),
                        }
                    }
                }
            },
            Name::Var(ident) => {
                *is_var = true;
                Ok((Some(self.current_mod.clone()), ident))
            },
        }
    }
    
    pub fn var(&self, name: &Name) -> Result<Option<Value>>
    {
        let mut is_var = false;
        let (mod1, ident) = self.mod_pair_for_name(name, &mut is_var)?;
        if is_var {
            match self.stack.last() {
                Some((_, local_vars)) => {
                    match local_vars.get(ident) {
                        Some(value) => return Ok(Some(value.clone())),
                        None => (),
                    }
                },
                None => (),
            }
        }
        match mod1 {
            Some(mod1) => {
                let mod1_g = rw_lock_read(&mod1)?;
                match mod1_g.var(ident) {
                    Some(value) => Ok(Some(value.clone())),
                    None => Ok(None),
                }
            },
            None => Ok(None),
        }
    }

    pub fn set_var(&mut self, name: &Name, value: Value) -> Result<bool>
    {
        let mut is_var = false;
        let (mod1, ident) = self.mod_pair_for_name(name, &mut is_var)?;
        if is_var {
            match self.stack.last_mut() {
                Some((_, local_vars)) => {
                    local_vars.insert(ident.clone(), value);
                    return Ok(true)
                },
                None => (),
            }
        }
        match mod1 {
            Some(mod1) => {
                let mut mod1_g = rw_lock_write(&mod1)?;
                mod1_g.add_var(ident.clone(), value);
                Ok(true)
            },
            None => Ok(false),
        }
    }
}
