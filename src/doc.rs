//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use crate::error::*;
use crate::mod_node::*;
use crate::parser::*;
use crate::tree::*;
use crate::utils::*;

pub trait DocIterator: Iterator
{
    fn take_doc(&mut self) -> Option<String>;
}

pub struct DocIter<T: Iterator>
{
    iter: T,
}

impl<T: Iterator> DocIter<T>
{
    pub fn new(iter: T) -> Self
    { DocIter { iter, } }

    pub fn iter(&self) -> &T
    { &self.iter }
    
    pub fn iter_mut(&mut self) -> &mut T
    { &mut self.iter }
}

impl<T: Iterator> Iterator for DocIter<T>
{
    type Item = T::Item;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next() }
}

impl<T: Iterator> DocIterator for DocIter<T>
{
    fn take_doc(&mut self) -> Option<String>
    { None }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BuiltinFunArg
{
    Arg(String),
    OptArg(String),
    DotDotDot,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Sig
{
    Var,
    Fun(Vec<String>),
    BuiltinFun(Vec<BuiltinFunArg>),
}

#[derive(Clone, Debug)]
pub struct DocTree
{
    sig_root_mod: Arc<RwLock<ModNode<Sig, ()>>>,
    doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>,
}

impl DocTree
{
    pub fn new(sig_root_mod: Arc<RwLock<ModNode<Sig, ()>>>, doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>) -> Self
    { DocTree { sig_root_mod, doc_root_mod, } }
    
    pub fn sig_root_mod(&self) -> &Arc<RwLock<ModNode<Sig, ()>>>
    { &self.sig_root_mod }

    pub fn doc_root_mod(&self) -> &Arc<RwLock<ModNode<String, Option<String>>>>
    { &self.doc_root_mod }
    
    pub fn read(&self) -> Result<DocTreeReadGuard<'_>>
    {
        let sig_root_mod_g = rw_lock_read(&*self.sig_root_mod)?;
        let doc_root_mod_g = rw_lock_read(&*self.doc_root_mod)?;
        Ok(DocTreeReadGuard { sig_root_mod_g, doc_root_mod_g, })
    }
}

#[derive(Debug)]
pub struct DocTreeReadGuard<'a>
{
    sig_root_mod_g: RwLockReadGuard<'a, ModNode<Sig, ()>>,
    doc_root_mod_g: RwLockReadGuard<'a, ModNode<String, Option<String>>>,
}

impl<'a> DocTreeReadGuard<'a>
{
    pub fn desc(&self) -> Option<&String>
    { 
        match self.doc_root_mod_g.value() {
            Some(desc) => Some(desc),
            None => None,
        }
    }
    
    pub fn subtrees(&self) -> Vec<(&String, DocTree)>
    { self.sig_root_mod_g.mods().iter().map(|(id, sm)| self.doc_root_mod_g.mod1(id).map(|dm| (id, DocTree::new(sm.clone(), dm.clone())))).flatten().collect() }
    
    pub fn var_desc_pairs(&self) -> Vec<(&String, (&Sig, Option<&String>))>
    { self.sig_root_mod_g.vars().iter().map(|(id, s)| (id, (s, self.doc_root_mod_g.var(id)))).collect() }
}

#[derive(Clone, Debug)]
struct DocTreeEnv
{
    sig_root_mod: Arc<RwLock<ModNode<Sig, ()>>>,
    sig_current_mod: Arc<RwLock<ModNode<Sig, ()>>>,
    doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>,
    doc_current_mod: Arc<RwLock<ModNode<String, Option<String>>>>,
}

impl DocTreeEnv
{
    fn new(doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>) -> Self
    {
        let sig_root_mod: Arc<RwLock<ModNode<Sig, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
        DocTreeEnv {
            sig_root_mod: sig_root_mod.clone(),
            sig_current_mod: sig_root_mod,
            doc_root_mod: doc_root_mod.clone(),
            doc_current_mod: doc_root_mod,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocTreeGen
{
    env: DocTreeEnv,
    script_dir: PathBuf,
    run_with_doc_ident: String,
}

impl DocTreeGen
{
    pub fn new_with_script_dir_and_run_with_doc_ident(doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>, script_dir: PathBuf, run_with_doc_ident: String) -> Self
    { DocTreeGen { env: DocTreeEnv::new(doc_root_mod), script_dir, run_with_doc_ident, } }

    pub fn new_with_script_dir(doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>, script_dir: PathBuf) -> Self
    { Self::new_with_script_dir_and_run_with_doc_ident(doc_root_mod, script_dir, String::from("runwithdoc")) }

    pub fn new(doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>>) -> Self
    { Self::new_with_script_dir(doc_root_mod, PathBuf::from(".")) }
    
    pub fn script_dir(&self) -> &Path
    { self.script_dir.as_path() }
    
    pub fn run_with_doc_ident(&self) -> &str
    { self.run_with_doc_ident.as_str() }
    
    pub fn generate_doc_tree(&mut self, tree: &Tree) -> Result<DocTree>
    {
        self.generate_doc_tree_for_tree(tree)?;
        Ok(DocTree::new(self.env.sig_root_mod.clone(), self.env.doc_root_mod.clone()))
    }

    fn generate_doc_tree_for_tree(&mut self, tree: &Tree) -> Result<()>
    {
        match tree {
            Tree(nodes) => self.generate_doc_tree_for_nodes(nodes.as_slice()),
        }
    }
    
    fn generate_doc_tree_for_node(&mut self, node: &Node, script_names: &mut Vec<String>) -> Result<()>
    {
        match node {
            Node::Def(def) => self.generate_doc_tree_for_def(&**def),
            Node::Stat(stat) => self.generate_doc_tree_for_stat(&**stat, script_names),
        }
    }

    fn generate_doc_tree_for_nodes(&mut self, nodes: &[Node]) -> Result<()>
    {
        let mut script_names: Vec<String> = Vec::new();
        for node in nodes {
            self.generate_doc_tree_for_node(node, &mut script_names)?;
        }
        for script_name in &script_names {
            let mut path_buf = PathBuf::from(self.script_dir.clone());
            path_buf.push(script_name.replace('/', path::MAIN_SEPARATOR_STR).as_str());
            let tree = parse_with_doc_root_mod_and_doc_current_mod(path_buf, Some(self.env.doc_root_mod.clone()), Some(self.env.doc_current_mod.clone()))?;
            self.generate_doc_tree_for_tree(&tree)?;
        }
        Ok(())
    }

    fn generate_doc_tree_for_def(&mut self, def: &Def) -> Result<()>
    {
        match def {
            Def::Mod(ident, mod1, _) => {
                match &**mod1 {
                    Mod(nodes) => {
                        let new_sig_mod: Arc<RwLock<ModNode<Sig, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
                        ModNode::add_mod(&self.env.sig_current_mod, ident.clone(), new_sig_mod.clone())?;
                        self.env.sig_current_mod = new_sig_mod;
                        self.env.doc_current_mod = {
                            let sig_current_mod_g = rw_lock_read(&*self.env.doc_current_mod)?;
                            match sig_current_mod_g.mod1(ident) {
                                Some(doc_mod1) => doc_mod1.clone(),
                                None => return Err(Error::NoDocMod),
                            }
                        };
                        self.generate_doc_tree_for_nodes(nodes.as_slice())?;
                        let doc_parent = {
                            let doc_current_mod_g = rw_lock_read(&*self.env.doc_current_mod)?;
                            doc_current_mod_g.parent()
                        };
                        match doc_parent {
                            Some(doc_parent) => self.env.doc_current_mod = doc_parent,
                            None => (),
                        }
                        let sig_parent = {
                            let sig_current_mod_g = rw_lock_read(&*self.env.sig_current_mod)?;
                            sig_current_mod_g.parent()
                        };
                        match sig_parent {
                            Some(sig_parent) => self.env.sig_current_mod = sig_parent,
                            None => (),
                        }
                        
                    },
                }
            },
            Def::Fun(ident, fun, _) => {
                match &**fun {
                    Fun(args, _) => {
                        let mut sig_current_mod_g = rw_lock_write(&*self.env.sig_current_mod)?;
                        sig_current_mod_g.add_var(ident.clone(), Sig::Fun(args.iter().map(|a| a.0.clone()).collect()));
                    },
                }
            },
        }
        Ok(())
    }

    fn generate_doc_tree_for_stat(&mut self, stat: &Stat, script_names: &mut Vec<String>) -> Result<()>
    {
        match stat {
            Stat::Expr(expr, _) => {
                match &**expr {
                    Expr::App(expr2, exprs, _) => {
                        let is_run_with_doc = match &**expr2 {
                            Expr::Var(Name::Abs(idents, ident), _) => idents.is_empty() && ident == &self.run_with_doc_ident,
                            Expr::Var(Name::Rel(_, _), _) => false,
                            Expr::Var(Name::Var(ident), _) => {
                                if ident == &self.run_with_doc_ident {
                                    let sig_current_mod_g = rw_lock_read(&self.env.sig_current_mod)?;
                                    !sig_current_mod_g.has_var(&self.run_with_doc_ident)
                                } else {
                                    false
                                }
                            },
                            _ => false,
                        };
                        if is_run_with_doc {
                            match exprs.first() {
                                Some(expr3) => {
                                    match &**expr3 {
                                        Expr::Lit(Lit::String(s), _) => script_names.push(s.clone()),
                                        _ => (),
                                    }
                                },
                                None => (),
                            }
                        }
                    },
                    _ => (),
                }
            },
            Stat::Assign(expr, _, _) => {
                let pair = match &**expr {
                    Expr::Var(Name::Abs(idents, ident), _) => {
                        match ModNode::mod_from(&self.env.sig_root_mod, idents.as_slice(), false)? {
                            Some(tmp_sig_mod) => Some((tmp_sig_mod, ident.clone())),
                            None => None,
                        }
                    },
                    Expr::Var(Name::Rel(idents, ident), _) => {
                        match ModNode::mod_from(&self.env.sig_current_mod, idents.as_slice(), true)? {
                            Some(tmp_sig_mod) => Some((tmp_sig_mod, ident.clone())),
                            None => {
                                match ModNode::mod_from(&self.env.sig_root_mod, idents.as_slice(), false)? {
                                    Some(tmp_sig_mod) => Some((tmp_sig_mod, ident.clone())),
                                    None => None,
                                }
                            },
                        }
                    },
                    Expr::Var(Name::Var(ident), _) => Some((self.env.sig_current_mod.clone(), ident.clone())),
                    _ => None,
                };
                match pair {
                    Some((sig_mod, ident)) => {
                        let mut sig_mod_g = rw_lock_write(&*sig_mod)?;
                        sig_mod_g.add_var(ident, Sig::Var);
                    },
                    None => (),
                }
            },
            _ => (),
        }
        Ok(())
    }
}
