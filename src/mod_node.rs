//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::error::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct ModNode<T, U>
{
    mods: HashMap<String, Arc<RwLock<ModNode<T, U>>>>,
    vars: HashMap<String, T>,
    parent: Option<Weak<RwLock<ModNode<T, U>>>>,
    value: U,
}

impl<T, U> ModNode<T, U>
{
    pub fn new(value: U) -> Self
    { ModNode { mods: HashMap::new(), vars: HashMap::new(), parent: None, value, } }
    
    pub fn mods(&self) -> &HashMap<String, Arc<RwLock<ModNode<T, U>>>>
    { &self.mods }

    pub fn has_mod(&self, ident: &String) -> bool
    { self.mods.contains_key(ident) }
    
    pub fn mod1(&self, ident: &String) -> Option<&Arc<RwLock<ModNode<T, U>>>>
    { self.mods.get(ident) }

    pub fn add_mod(parent: &Arc<RwLock<ModNode<T, U>>>, ident: String, child: Arc<RwLock<ModNode<T, U>>>) -> Result<()>
    {
        {
            let mut child_g = rw_lock_write(&*child)?;
            if child_g.parent.is_some() {
                return Err(Error::AlreadyAddedModNode);
            }
            child_g.parent = Some(Arc::downgrade(&parent));
        }
        let mut parent_g = rw_lock_write(&**parent)?;
        parent_g.mods.insert(ident, child);
        Ok(())
    }

    pub fn remove_mod(&mut self, ident: &String) -> Result<()>
    {
        match self.mods.get(ident) {
            Some(child) => {
                {
                    let mut child_g = rw_lock_write(&*child)?;
                    child_g.parent = None;
                }
                self.mods.remove(ident);
                Ok(())
            },
            None => Ok(()),
        }
    }

    pub fn vars(&self) -> &HashMap<String, T>
    { &self.vars }

    pub fn has_var(&self, ident: &String) -> bool
    { self.vars.contains_key(ident) }

    pub fn var(&self, ident: &String) -> Option<&T>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, value: T)
    { self.vars.insert(ident, value); }

    pub fn remove_var(&mut self, ident: &String)
    { self.vars.remove(ident); }
    
    pub fn parent(&self) -> Option<Arc<RwLock<ModNode<T, U>>>>
    {
        match &self.parent{
            Some(parent) => parent.upgrade(),
            None => None,
        }
    }

    pub fn value(&self) -> &U
    { &self.value }
    
    pub fn mod_from(root: &Arc<RwLock<ModNode<T, U>>>, idents: &[String]) -> Result<Option<Arc<RwLock<ModNode<T, U>>>>>
    {
        let mut node = root.clone();
        for ident in idents {
            let child: Arc<RwLock<ModNode<T, U>>>;
            {
                let node_g = rw_lock_read(&*node)?;
                child = match node_g.mods.get(ident) {
                    Some(tmp_child) => tmp_child.clone(),
                    None => return Ok(None),
                };
            };
            node = child;
        }
        Ok(Some(node))
    }
}
