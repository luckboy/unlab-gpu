//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of module node.
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::error::*;
use crate::utils::*;

/// An enumeration of reference to module node.
///
/// The reference of module node can be a strong reference to module or a weak reference to
/// module. If the reference of module node refers to ascestor, the reference of module node
/// should be the weak reference because it is reference cycle.
#[derive(Clone, Debug)]
pub enum ModNodeRef<T, U>
{
    /// A strong reference to the module.
    Arc(Arc<RwLock<ModNode<T, U>>>),
    /// A weak refenece to the module.
    Weak(Weak<RwLock<ModNode<T, U>>>),
}

impl<T, U> ModNodeRef<T, U>
{
    pub fn to_arc(&self) -> Option<Arc<RwLock<ModNode<T, U>>>>
    {
        match self {
            ModNodeRef::Arc(arc) => Some(arc.clone()),
            ModNodeRef::Weak(weak) => weak.upgrade(),
        }
    }
}

/// A structure of object of used variable.
///
/// The used variables available in a module, but are defined in other module. This object refers
/// the used variable by the referecne to module node and the variable identifier.
#[derive(Clone, Debug)]
pub struct UsedVar<T, U>
{
    mod1: ModNodeRef<T, U>,
    ident: String,
}

impl<T, U> UsedVar<T, U>
{
    /// Creates an object of used variable.
    pub fn new(mod1: ModNodeRef<T, U>, ident: String) -> Self
    { UsedVar { mod1, ident, } }

    /// Returns the module.
    pub fn mod1(&self) -> &ModNodeRef<T, U>
    { &self.mod1 }

    /// Returns the identifier.
    pub fn ident(&self) -> &String
    { &self.ident }
}

/// A structure of module node.
///
/// The module nodes creates a tree of modules that has modules and variables. The module node
/// contains the modules which are the module nodes, the variables, and module value. Also, the
/// module node has used modules and used variables. The used modules and the used variables are
/// from other module, but also are available in the module node by identifiers.
#[derive(Clone, Debug)]
pub struct ModNode<T, U>
{
    used_mods: HashMap<String, ModNodeRef<T, U>>,
    used_vars: HashMap<String, UsedVar<T, U>>,
    mods: HashMap<String, Arc<RwLock<ModNode<T, U>>>>,
    vars: HashMap<String, T>,
    parent: Option<Weak<RwLock<ModNode<T, U>>>>,
    value: U,
}

impl<T, U> ModNode<T, U>
{
    /// Creates a module node with the module value.
    pub fn new(value: U) -> Self
    {
        ModNode {
            used_mods: HashMap::new(),
            used_vars: HashMap::new(),
            mods: HashMap::new(),
            vars: HashMap::new(),
            parent: None,
            value,
        }
    }
    
    /// Returns the used modules.
    pub fn used_mods(&self) -> &HashMap<String, ModNodeRef<T, U>>
    { &self.used_mods }

    /// Returns `true` if the module node has the used module, otherwise `false`.
    pub fn has_used_mod(&self, ident: &String) -> bool
    { self.used_mods.contains_key(ident) }
    
    /// Returns the reference to the used module if the module node has the used module,
    /// otherwise `None`.
    pub fn used_mod(&self, ident: &String) -> Option<&ModNodeRef<T, U>>
    { self.used_mods.get(ident) }

    fn mod_to_mod_ref(mod1: &Arc<RwLock<ModNode<T, U>>>, mod2: Arc<RwLock<ModNode<T, U>>>) -> Result<ModNodeRef<T, U>>
    {
        let mut node = Some(mod1.clone());
        loop {
            match &node {
                Some(tmp_node) => { 
                    if Arc::ptr_eq(&tmp_node, &mod2) {
                        return Ok(ModNodeRef::Weak(Arc::downgrade(&mod2)));
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
        Ok(ModNodeRef::Arc(mod2.clone()))
    }
    
    /// Adds the used module to the module node.
    pub fn add_used_mod(mod1: &Arc<RwLock<ModNode<T, U>>>, ident: String, used_mod: Arc<RwLock<ModNode<T, U>>>) -> Result<()>
    {
        let used_mod_ref = Self::mod_to_mod_ref(mod1, used_mod)?;
        let mut mod_g = rw_lock_write(&**mod1)?;
        mod_g.used_mods.insert(ident, used_mod_ref);
        Ok(())
    }

    /// Removes the used module from the module node.
    pub fn remove_used_mod(&mut self, ident: &String)
    { self.used_mods.remove(ident); }

    /// Returns the used variable from the module node.
    pub fn used_vars(&self) -> &HashMap<String, UsedVar<T, U>>
    { &self.used_vars }

    /// Returns `true` if the module node has the used variable, otherwise `false`.
    pub fn has_used_var(&self, ident: &String) -> bool
    { self.used_vars.contains_key(ident) }
    
    /// Returns the object of used variable if the module node has the used variable, otherwise
    /// `None`.
    pub fn used_var(&self, ident: &String) -> Option<&UsedVar<T, U>>
    { self.used_vars.get(ident) }

    /// Adds the used variable to the module node.
    pub fn add_used_var(mod1: &Arc<RwLock<ModNode<T, U>>>, ident: String, used_var_mod: Arc<RwLock<ModNode<T, U>>>, used_var_ident: String) -> Result<()>
    {
        let used_var_mod_ref = Self::mod_to_mod_ref(mod1, used_var_mod)?;
        let mut mod_g = rw_lock_write(&**mod1)?;
        mod_g.used_vars.insert(ident, UsedVar::new(used_var_mod_ref, used_var_ident));
        Ok(())
    }
    
    /// Removes the used variable from the module node.
    pub fn remove_used_var(&mut self, ident: &String)
    { self.used_vars.remove(ident); }

    /// Returns the modules.
    pub fn mods(&self) -> &HashMap<String, Arc<RwLock<ModNode<T, U>>>>
    { &self.mods }

    /// Returns `true` if the module node has the module, otherwise `false`.
    pub fn has_mod(&self, ident: &String) -> bool
    { self.mods.contains_key(ident) }
    
    /// Returns the module if the module node has the module, otherwise `None`.
    pub fn mod1(&self, ident: &String) -> Option<&Arc<RwLock<ModNode<T, U>>>>
    { self.mods.get(ident) }

    /// Adds the module to the module node.
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

    /// Removes the module from the module node.
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

    /// Returns the variables.
    pub fn vars(&self) -> &HashMap<String, T>
    { &self.vars }

    /// Returns `true` if the module node has the variable, otherwise `false`.
    pub fn has_var(&self, ident: &String) -> bool
    { self.vars.contains_key(ident) }

    /// Returns the variable if the module node has the variable, otherwise `None`.
    pub fn var(&self, ident: &String) -> Option<&T>
    { self.vars.get(ident) }

    /// Adds the variable to module node.
    pub fn add_var(&mut self, ident: String, value: T)
    { self.vars.insert(ident, value); }

    /// Removes the variable from module node.
    pub fn remove_var(&mut self, ident: &String)
    { self.vars.remove(ident); }
    
    /// Returns the parent if the module node if the module node has the parent, otherwise `None`.
    pub fn parent(&self) -> Option<Arc<RwLock<ModNode<T, U>>>>
    {
        match &self.parent{
            Some(parent) => parent.upgrade(),
            None => None,
        }
    }

    /// Returns the module value.
    pub fn value(&self) -> &U
    { &self.value }
    
    /// Returns the module for the identifiers of modules if the module exists, otherwise `None`.
    ///
    /// If flag of used modules is set, the first identifier of module can refers to the used
    /// module.
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
                                Some(tmp_child) => {
                                    match tmp_child.to_arc() {
                                        Some(child_arc) => child_arc,
                                        None => return Ok(None),
                                    }
                                },
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
