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
    used_mods: HashMap<String, Arc<RwLock<ModNode<T, U>>>>,
    mods: HashMap<String, Arc<RwLock<ModNode<T, U>>>>,
    vars: HashMap<String, T>,
    parent: Option<Weak<RwLock<ModNode<T, U>>>>,
    value: U,
}

impl<T, U> ModNode<T, U>
{
    pub fn new(value: U) -> Self
    { ModNode { used_mods: HashMap::new(), mods: HashMap::new(), vars: HashMap::new(), parent: None, value, } }
    
    pub fn used_mods(&self) -> &HashMap<String, Arc<RwLock<ModNode<T, U>>>>
    { &self.used_mods }

    pub fn has_used_mod(&self, ident: &String) -> bool
    { self.used_mods.contains_key(ident) }
    
    pub fn used_mod(&self, ident: &String) -> Option<&Arc<RwLock<ModNode<T, U>>>>
    { self.used_mods.get(ident) }

    pub fn add_used_mod(mod1: &Arc<RwLock<ModNode<T, U>>>, ident: String, used_mod: Arc<RwLock<ModNode<T, U>>>) -> Result<()>
    {
        let mut node = Some(mod1.clone());
        loop {
            match &node {
                Some(tmp_node) => { 
                    if Arc::ptr_eq(&tmp_node, &used_mod) {
                        return Err(Error::RecursivelyUsedModNode);
                    }
                    let parent = {
                        let tmp_node_g = rw_lock_read(&**tmp_node)?;
                        tmp_node_g.parent()
                    };
                    node = parent;
                },
                None => break,
            }
        }
        let mut mod1_g = rw_lock_write(&**mod1)?;
        mod1_g.used_mods.insert(ident, used_mod);
        Ok(())
    }

    pub fn remove_used_mod(&mut self, ident: &String)
    { self.used_mods.remove(ident); }
    
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
        parent_g.remove_mod(&ident)?;
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
    
    pub fn mod_from(root: &Arc<RwLock<ModNode<T, U>>>, idents: &[String], are_used_mods: bool) -> Result<Option<Arc<RwLock<ModNode<T, U>>>>>
    {
        let mut node = root.clone();
        let mut is_first = true;
        for ident in idents {
            let child: Arc<RwLock<ModNode<T, U>>>;
            {
                let node_g = rw_lock_read(&*node)?;
                child = match node_g.mods.get(ident) {
                    Some(tmp_child) => tmp_child.clone(),
                    None => {
                        if is_first && are_used_mods {
                            match node_g.used_mods.get(ident) {
                                Some(tmp_child) => tmp_child.clone(),
                                None => return Ok(None),
                            }
                        } else {
                            return Ok(None);
                        }
                    },
                };
            };
            node = child;
            is_first = false;
        }
        Ok(Some(node))
    }
}

#[cfg(test)]
mod tests;
