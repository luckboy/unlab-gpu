//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::fs::create_dir;
use std::fs::write;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use markdown::CompileOptions;
use markdown::Options;
use markdown::ParseOptions;
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
        Ok(DocTreeReadGuard::new(sig_root_mod_g, doc_root_mod_g))
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
    fn new(sig_root_mod_g: RwLockReadGuard<'a, ModNode<Sig, ()>>, doc_root_mod_g: RwLockReadGuard<'a, ModNode<String, Option<String>>>) -> Self
    { DocTreeReadGuard { sig_root_mod_g, doc_root_mod_g, } }

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
    
    pub fn generate(&mut self, tree: &Tree) -> Result<DocTree>
    {
        self.generate_for_tree(tree)?;
        Ok(DocTree::new(self.env.sig_root_mod.clone(), self.env.doc_root_mod.clone()))
    }

    fn generate_for_tree(&mut self, tree: &Tree) -> Result<()>
    {
        match tree {
            Tree(nodes) => self.generate_for_nodes(nodes.as_slice()),
        }
    }
    
    fn generate_for_node(&mut self, node: &Node, script_names: &mut Vec<String>) -> Result<()>
    {
        match node {
            Node::Def(def) => self.generate_for_def(&**def),
            Node::Stat(stat) => self.generate_for_stat(&**stat, script_names),
        }
    }

    fn generate_for_nodes(&mut self, nodes: &[Node]) -> Result<()>
    {
        let mut script_names: Vec<String> = Vec::new();
        for node in nodes {
            self.generate_for_node(node, &mut script_names)?;
        }
        for script_name in &script_names {
            let mut path_buf = PathBuf::from(self.script_dir.clone());
            path_buf.push(script_name.replace('/', path::MAIN_SEPARATOR_STR).as_str());
            let tree = parse_with_doc_root_mod_and_doc_current_mod(path_buf, Some(self.env.doc_root_mod.clone()), Some(self.env.doc_current_mod.clone()))?;
            self.generate_for_tree(&tree)?;
        }
        Ok(())
    }

    fn generate_for_def(&mut self, def: &Def) -> Result<()>
    {
        match def {
            Def::Mod(ident, mod1, _) => {
                match &**mod1 {
                    Mod(nodes) => {
                        let new_sig_mod: Arc<RwLock<ModNode<Sig, ()>>> = Arc::new(RwLock::new(ModNode::new(())));
                        ModNode::add_mod(&self.env.sig_current_mod, ident.clone(), new_sig_mod.clone())?;
                        self.env.sig_current_mod = new_sig_mod;
                        self.env.doc_current_mod = {
                            let doc_current_mod_g = rw_lock_read(&*self.env.doc_current_mod)?;
                            match doc_current_mod_g.mod1(ident) {
                                Some(doc_mod) => doc_mod.clone(),
                                None => return Err(Error::NoDocMod),
                            }
                        };
                        self.generate_for_nodes(nodes.as_slice())?;
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

    fn generate_for_stat(&mut self, stat: &Stat, script_names: &mut Vec<String>) -> Result<()>
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

pub fn generate_doc_tree<P: AsRef<Path>>(script_dir: P) -> Result<DocTree>
{
    let mut lib_file = PathBuf::from(script_dir.as_ref());
    lib_file.push("lib.un");
    let doc_root_mod: Arc<RwLock<ModNode<String, Option<String>>>> = Arc::new(RwLock::new(ModNode::new(None)));
    let tree = parse_with_doc_root_mod(lib_file, Some(doc_root_mod.clone()))?;
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(doc_root_mod, PathBuf::from(script_dir.as_ref()));
    doc_tree_gen.generate(&tree)
}

fn str_to_hmtl(s: &str) -> String
{ s.replace('&', "&amp").replace('<', "&lt").replace(">", "&gt") }

fn idents_to_string(idents: &[String]) -> String
{
    let mut s = String::new();
    let mut is_first = true;
    for ident in idents {
        if !is_first {
            s.push_str("::");
        }
        s.push_str(ident);
        is_first = false;
    }
    s
}

#[derive(Clone, Debug)]
pub struct DocGen
{
    lib_name: String,
    lib_doc_dir: PathBuf,
}

impl DocGen
{
    pub fn new<P: AsRef<Path>>(doc_dir: PathBuf, lib_path: P) -> Self
    {
        let lib_name = lib_path.as_ref().to_string_lossy().into_owned().replace(path::MAIN_SEPARATOR, "/");
        let mut lib_doc_dir = doc_dir;
        lib_doc_dir.push(lib_path);
        DocGen { lib_name, lib_doc_dir, }
    }
    
    pub fn lib_name(&self) -> &str
    { self.lib_name.as_str() }

    pub fn lib_doc_dir(&self) -> &Path
    { self.lib_doc_dir.as_path() }
    
    fn str_to_href(s: &str, depth: usize) -> String
    {
        let mut url = String::new();
        for _ in 0..depth {
            url.push_str("../");
        }
        url.push_str(s);
        url
    }

    fn ident_and_sig_to_html(ident: &str, sig: &Sig) -> String
    {
        let mut html = String::new();
        html.push_str("<h3>");
        html.push_str(format!("<a href=\"#var.{}\" class=\"var\">{}</a>", ident, ident).as_str());
        match sig {
            Sig::Var => (),
            Sig::Fun(args) => {
                html.push('(');
                let mut is_first = true;
                for arg in args {
                    if !is_first {
                        html.push_str(", ");
                    }
                    html.push_str(format!("<span class=\"arg\">{}</span>", arg).as_str());
                    is_first = false;
                }
                html.push(')');
            },
            Sig::BuiltinFun(args) => {
                html.push('(');
                let mut is_first = true;
                for arg in args {
                    if !is_first {
                        html.push_str(", ");
                    }
                    match arg {
                        BuiltinFunArg::Arg(ident) => html.push_str(format!("<span class=\"arg\">{}</span>", ident).as_str()),
                        BuiltinFunArg::OptArg(ident) => html.push_str(format!("<span class=\"arg\">{}</span>?", ident).as_str()),
                        BuiltinFunArg::DotDotDot => html.push_str("..."),
                    }
                    is_first = false;
                }
                html.push(')');
            },
        }
        html.push_str("</h3>");
        html
    }
    
    fn markdown_to_html(s: &str) -> Result<String>
    {
        match latex2mathml::replace(s) {
            Ok(t) => {
                let options = Options {
                    parse: ParseOptions::gfm(),
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        gfm_tagfilter: true,
                        ..CompileOptions::default()
                    },
                    ..Options::default()
                };
                match markdown::to_html_with_options(t.as_str(), &options) {
                    Ok(u) => Ok(u),
                    Err(msg) => Err(Error::Markdown(format!("{}", msg))),
                }
            },
            Err(err) => Err(Error::Latex2mathml(Box::new(err))),
        }
    }
    
    fn io_res_generate_html_file<P: AsRef<Path>>(&self, path: P, idents: &[String], content: &str, depth: usize) -> io::Result<()>
    {
        let file = File::create(path)?;
        let mut w = BufWriter::new(file);
        writeln!(&mut w, "<!DOCTYPE html>")?;
        writeln!(&mut w, "<html>")?;
        writeln!(&mut w, "<head>")?;
        writeln!(&mut w, "<meta charset=\"utf-8\" />")?;
        writeln!(&mut w, "<link rel=\"stylesheet\" href=\"{}\" />", Self::str_to_href("styles.css", depth))?;
        write!(&mut w, "<title>")?;
        if !idents.is_empty() {
            write!(&mut w, "{} in ", idents_to_string(idents))?;
        }
        write!(&mut w, "{} - Unlab", str_to_hmtl(self.lib_name.as_str()))?;
        writeln!(&mut w, "</title>")?;
        writeln!(&mut w, "</head>")?;
        writeln!(&mut w, "<body>")?;
        writeln!(&mut w, "<header>")?;
        if !idents.is_empty() {
            writeln!(&mut w, "<h1><a href=\"{}\">{}</a></h1>", Self::str_to_href("index.html", depth), str_to_hmtl(self.lib_name.as_str()))?;
        } else {
            writeln!(&mut w, "<h1>{}</h1>", str_to_hmtl(self.lib_name.as_str()))?;
        }
        writeln!(&mut w, "</header>")?;
        match idents.first() {
            Some(first_ident) => {
                writeln!(&mut w, "<nav>")?;
                let mut mod_path = String::from(first_ident);
                write!(&mut w, "<h2><a href=\"{}\">{}</a>", Self::str_to_href(format!("{}.html", mod_path).as_str(), depth), first_ident)?;
                for ident in &idents[1..] {
                    mod_path.push('/');
                    mod_path.push_str(ident.as_str());
                    write!(&mut w, "::")?;
                    write!(&mut w, "<a href=\"{}\">{}</a>", Self::str_to_href(format!("{}.html", mod_path).as_str(), depth), ident)?;
                }
                writeln!(&mut w, "</h2>")?;
                writeln!(&mut w, "</nav>")?;
            },
            None => (),
        }
        writeln!(&mut w, "<main>")?;
        write!(&mut w, "{}", content)?;
        writeln!(&mut w, "</main>")?;
        writeln!(&mut w, "</body>")?;
        writeln!(&mut w, "</html>")?;
        Ok(())
    }

    fn generate_mod_doc(&self, doc_tree: &DocTree, idents: &[String], path: &Path, mod_path: &str, index_content: &mut String, depth: usize) -> Result<()>
    {
        index_content.push_str(format!("<li><a href=\"{}\">{}</a></li>\n", Self::str_to_href(format!("{}.html", mod_path).as_str(), depth), idents_to_string(idents)).as_str());
        let doc_tree_g = doc_tree.read()?;
        let mut subtrees = doc_tree_g.subtrees();
        if !subtrees.is_empty() {
            match create_dir(path) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            subtrees.sort_by(|p1, p2| p1.0.cmp(p2.0));
            for (ident, subtree) in subtrees {
                let mut new_idents = idents.to_vec();
                new_idents.push(ident.clone());
                let mut new_path = PathBuf::from(path);
                new_path.push(ident);
                let mut new_mod_path = String::from(mod_path);
                new_mod_path.push('/');
                new_mod_path.push_str(ident);
                self.generate_mod_doc(&subtree, new_idents.as_slice(), new_path.as_path(), new_mod_path.as_str(), index_content, depth + 1)?;
            }
        }
        let mut mod_content = String::new();
        mod_content.push_str("<section>\n");
        match doc_tree_g.desc() {
            Some(desc) => mod_content.push_str(format!("{}\n", Self::markdown_to_html(desc.as_str())?).as_str()),
            None => (),
        }
        mod_content.push_str("</section>\n");
        let mut var_desc_pairs = doc_tree_g.var_desc_pairs();
        var_desc_pairs.sort_by(|p1, p2| p1.0.cmp(p2.0));
        for (ident, (sig, desc))  in var_desc_pairs {
            mod_content.push_str(format!("<section id=\"var.{}\">\n", ident).as_str());
            mod_content.push_str(Self::ident_and_sig_to_html(ident, sig).as_str());
            match desc {
                Some(desc) => mod_content.push_str(format!("{}\n", Self::markdown_to_html(desc.as_str())?).as_str()),
                None => (),
            }
            mod_content.push_str("</section>\n");
        }
        match self.io_res_generate_html_file(path.with_extension(".html"), idents, mod_content.as_str(), depth) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        Ok(())
    }
    
    pub fn generate(&self, doc_tree: &DocTree) -> Result<()>
    {
        let mut styles_path_buf = self.lib_doc_dir.clone();
        styles_path_buf.push("styles.css");
        match write(styles_path_buf, "") {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        let mut path_buf = self.lib_doc_dir.clone();
        path_buf.push("root");
        let mut index_content = String::new();
        index_content.push_str("<ul>\n");
        self.generate_mod_doc(doc_tree, &[String::from("root")], path_buf.as_path(), "root", &mut index_content, 0)?;
        index_content.push_str("</ul>\n");
        let mut index_path_buf = self.lib_doc_dir.clone();
        index_path_buf.push("index.html");
        match self.io_res_generate_html_file(index_path_buf, &[], index_content.as_str(), 0) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        Ok(())
    }
}

pub fn generate_doc<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(lib_dir: P, doc_dir: Q, lib_path: R) -> Result<()>
{
    let mut script_dir = PathBuf::from(lib_dir.as_ref());
    script_dir.push(lib_path.as_ref());
    let doc_tree = generate_doc_tree(script_dir)?;
    let doc_gen = DocGen::new(PathBuf::from(doc_dir.as_ref()), lib_path.as_ref());
    doc_gen.generate(&doc_tree)
}

#[cfg(test)]
mod tests;
