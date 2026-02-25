//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::env::current_dir;
use std::env::set_current_dir;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::io;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use crate::env::*;
use crate::error::*;
use crate::fs::*;
use crate::interp::*;
use crate::mod_node::*;
use crate::parser::*;
use crate::utils::*;
use crate::value::*;

pub struct TestResult
{
    error_pair: Option<(Error, Vec<(Option<Value>, Pos)>)>,
    stdout: Option<Arc<RwLock<Cursor<Vec<u8>>>>>,
    stderr: Option<Arc<RwLock<Cursor<Vec<u8>>>>>,
}

impl TestResult
{
    pub fn new(error_pair: Option<(Error, Vec<(Option<Value>, Pos)>)>, stdout: Option<Arc<RwLock<Cursor<Vec<u8>>>>>, stderr: Option<Arc<RwLock<Cursor<Vec<u8>>>>>) -> TestResult
    { TestResult { error_pair, stdout, stderr } }
    
    pub fn is_ok(&self) -> bool
    { self.error_pair.is_none() }
    
    pub fn error_pair(&self) -> Option<&(Error, Vec<(Option<Value>, Pos)>)>
    {
        match &self.error_pair {
            Some(error_pair) => Some(error_pair),
            None => None,
        }
    }
    
    pub fn stdout(&self) -> Option<&Arc<RwLock<Cursor<Vec<u8>>>>>
    {
        match &self.stdout {
            Some(stdout) => Some(stdout),
            None => None,
        }
    }

    pub fn stderr(&self) -> Option<&Arc<RwLock<Cursor<Vec<u8>>>>>
    {
        match &self.stderr {
            Some(stderr) => Some(stderr),
            None => None,
        }
    }
}

pub trait Print
{
    fn print_loading(&self, is_done: bool);

    fn print_running_test(&self, idents: &Vec<String>, ident: &String, is_done: bool, is_ok: bool);
    
    fn print_test_result(&self, idents: &Vec<String>, ident: &String, test_result: &TestResult);
    
    fn print_nl_for_error(&self);
}

fn create_and_change_dir<P: AsRef<Path>>(path: P) -> io::Result<PathBuf>
{
    let saved_current_dir = current_dir()?;
    create_dir_all(path.as_ref())?;
    set_current_dir(path.as_ref())?;
    Ok(saved_current_dir)
}

fn change_and_recusively_remove_dir<P: AsRef<Path>, Q: AsRef<Path>>(path: P, saved_current_dir: Q) -> io::Result<()>
{
    set_current_dir(saved_current_dir)?;
    recursively_remove(path, true)?;
    Ok(())
}

pub struct Tester
{
    root_mod: Arc<RwLock<ModNode<Value, ()>>>,
    shared_env: Arc<RwLock<SharedEnv>>,
    stack_trace: Vec<(Option<Value>, Pos)>,
    test_results: Vec<((Vec<String>, String), TestResult)>,
    has_stdout_cursor: bool,
    has_stderr_cursor: bool,
    printer: Arc<dyn Print + Send + Sync>,
}

impl Tester
{
    pub fn new(root_mod: Arc<RwLock<ModNode<Value, ()>>>, lib_path: OsString, doc_path: OsString, printer: Arc<dyn Print + Send + Sync>, is_stdout_cursor: bool, is_stderr_cursor: bool) -> Self
    {
        Tester {
            root_mod,
            shared_env: Arc::new(RwLock::new(SharedEnv::new(lib_path, doc_path, Vec::new()))),
            stack_trace: Vec::new(),
            test_results: Vec::new(),
            has_stdout_cursor: is_stdout_cursor,
            has_stderr_cursor: is_stderr_cursor,
            printer,
        }
    }
    
    pub fn load(&mut self) -> Result<()>
    {
        self.printer.print_loading(false);
        let test_paths = match paths_in_dir("tests", Some(2)) {
            Ok(tmp_paths) => tmp_paths,
            Err(err) => return Err(Error::Io(err)),
        };
        for test_path in test_paths {
            let mut script_dir = PathBuf::from("tests");
            script_dir.push(test_path.as_path());
            let mut path = script_dir.clone();
            path.push("tests.un");
            let mut domain_path_buf = test_path.clone();
            let domain = if domain_path_buf.components().count() >= 2 {
                domain_path_buf.pop();
                match domain_path_buf.to_str() {
                    Some(tmp_domain) => Some(String::from(tmp_domain)),
                    None => return Err(Error::Tester(String::from("test path component contains invalid UTF-8 character"))),
                }
            } else {
                None
            };
            let tree = parse(path)?;
            let mut env = Env::new_with_script_dir_and_domain_and_shared_env(self.root_mod.clone(), script_dir.clone(), domain, self.shared_env.clone());
            let mut interp = Interp::new();
            env.set_stdin(Input::Null);
            env.set_stdout(Output::Null);
            env.set_stderr(Output::Null);
            match interp.interpret(&mut env, &tree) {
                Ok(()) => (),
                Err(err) => {
                    self.stack_trace = interp.stack_trace().to_vec();
                    return Err(err);
                },
            }
        }
        self.printer.print_loading(true);
        Ok(())
    }
    
    pub fn run_test(&mut self, idents: &Vec<String>, ident: &String) -> Result<()>
    {
        self.printer.print_running_test(idents, ident, false, false);
        let is_test_suite = {
            let shared_env_g = rw_lock_read(&self.shared_env)?;
            shared_env_g.has_test_suite(idents)
        };
        let mut is_ok = false;
        if is_test_suite {
            match ModNode::mod_from(&self.root_mod, idents.as_slice(), false)? {
                Some(mod1) => {
                    let fun_value = {
                        let mod_g = rw_lock_read(&mod1)?;
                        match mod_g.var(ident) {
                            Some(fun_value) => fun_value.clone(),
                            None => return Err(Error::Tester(String::from("undefined test function"))),
                        }
                    };
                    let mut work_test_dir = PathBuf::from("work");
                    work_test_dir.push("test");
                    let saved_current_dir = match create_and_change_dir(work_test_dir.as_path()) {
                        Ok(tmp_saved_current_dir) => tmp_saved_current_dir,
                        Err(err) => return Err(Error::Io(err)),
                    };
                    let mut env = Env::new_with_script_dir_and_domain_and_shared_env(self.root_mod.clone(), PathBuf::from("."), None, self.shared_env.clone());
                    if self.has_stdout_cursor {
                        env.set_stdout(Output::Cursor(Arc::new(RwLock::new(Cursor::new(Vec::new())))));
                    }
                    if self.has_stderr_cursor {
                        env.set_stderr(Output::Cursor(Arc::new(RwLock::new(Cursor::new(Vec::new())))));
                    }
                    let mut interp = Interp::new();
                    let error_pair = match fun_value.apply(&mut interp, &mut env, &[]) {
                        Ok(_) => {
                            is_ok = true;
                            None
                        },
                        Err(err) => Some((err, interp.stack_trace().to_vec())),
                    };
                    let stdout = match env.stdout() {
                        Output::Cursor(cursor) => Some(cursor.clone()),
                        _ => None,
                    };
                    let stderr = match env.stderr() {
                        Output::Cursor(cursor) => Some(cursor.clone()),
                        _ => None,
                    };
                    self.test_results.push(((idents.clone(), ident.clone()), TestResult::new(error_pair, stdout, stderr)));
                    match change_and_recusively_remove_dir(saved_current_dir, work_test_dir) {
                        Ok(tmp_saved_current_dir) => tmp_saved_current_dir,
                        Err(err) => return Err(Error::Io(err)),
                    }
                },
                None => return Err(Error::Tester(String::from("undefined test module"))),
            }
        } else {
            return Err(Error::Tester(String::from("module isn't test suite")));
        }
        self.printer.print_running_test(idents, ident, true, is_ok);
        Ok(())
    }

    pub fn run_tests_in_test_suite(&mut self, idents: &Vec<String>) -> Result<()>
    {
        let is_test_suite = {
            let shared_env_g = rw_lock_read(&self.shared_env)?;
            shared_env_g.has_test_suite(idents)
        };
        if is_test_suite {
            match ModNode::mod_from(&self.root_mod, idents.as_slice(), false)? {
                Some(mod1) => {
                    let mut fun_idents: Vec<String> = {
                        let mod_g = rw_lock_read(&mod1)?;
                        mod_g.vars().keys().map(|id| id.clone()).collect()
                    };
                    fun_idents.sort();
                    for fun_ident in &fun_idents {
                        self.run_test(idents, fun_ident)?;
                    }
                },
                None => return Err(Error::Tester(String::from("undefined test module"))),
            }
        } else {
            return Err(Error::Tester(String::from("module isn't test suite")));
        }
        Ok(())
    }

    pub fn run_all_tests(&mut self) -> Result<()>
    {
        let mut test_suites: Vec<Vec<String>> = {
            let shared_env_g = rw_lock_read(&self.shared_env)?;
            shared_env_g.test_suites().iter().map(|ids| ids.clone()).collect()
        };
        test_suites.sort();
        for test_suite in &test_suites {
            self.run_tests_in_test_suite(test_suite)?;
        }
        Ok(())
    }
}
