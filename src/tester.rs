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
use std::io::Write;
use std::io::stdout;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
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
    
    pub fn has_stdout_data(&self) -> Result<bool>
    {
        match &self.stdout {
            Some(stdout) => {
                let stdout_g = rw_lock_read(stdout)?;
                Ok(!stdout_g.get_ref().is_empty())
            },
            None => Ok(false),
        }
    }

    pub fn has_stderr_data(&self) -> Result<bool>
    {
        match &self.stderr {
            Some(stderr) => {
                let stderr_g = rw_lock_read(stderr)?;
                Ok(!stderr_g.get_ref().is_empty())
            },
            None => Ok(false),
        }
    }
}

pub trait Print
{
    fn print_loading(&self, is_done: bool);

    fn print_running_test(&self, idents: &Vec<String>, ident: &String, is_done: bool, is_ok: bool);

    fn print_empty_line(&self);
    
    fn print_successes(&self);

    fn print_failures(&self);
    
    fn print_test_result(&self, idents: &Vec<String>, ident: &String, test_result: &TestResult) -> Result<()>;
    
    fn print_test_counts(&self, passed_test_count: usize, failed_test_count: usize);
    
    fn print_nl_for_error(&self);
}

#[derive(Copy, Clone, Debug)]
pub struct EmptyPrinter;

impl EmptyPrinter
{
    pub fn new() -> Self
    { EmptyPrinter }
}

impl Print for EmptyPrinter
{
    fn print_loading(&self, _is_done: bool)
    {}

    fn print_running_test(&self, _idents: &Vec<String>, _ident: &String, _is_done: bool, _is_ok: bool)
    {}

    fn print_empty_line(&self)
    {}
    
    fn print_successes(&self)
    {}

    fn print_failures(&self)
    {}
    
    fn print_test_result(&self, _idents: &Vec<String>, _ident: &String, _test_result: &TestResult) -> Result<()>
    { Ok(()) }
    
    fn print_test_counts(&self, _passed_test_count: usize, _failed_test_count: usize)
    {}
    
    fn print_nl_for_error(&self)
    {}
}

fn idents_and_ident_to_string(idents: &[String], ident: &String) -> String
{
    let mut s = String::new();
    let mut is_first = true;
    for ident2 in idents {
        if !is_first {
            s.push_str("::");
        }
        s.push_str(ident2.as_str());
        is_first = false;
    }
    s.push_str("::");
    s.push_str(ident.as_str());
    s
}

#[derive(Debug)]
pub struct StdPrinter
{
    has_nl_for_error: AtomicBool,
}

impl StdPrinter
{
    pub fn new() -> Self
    { StdPrinter { has_nl_for_error: AtomicBool::new(false), } }
}

impl Print for StdPrinter
{
    fn print_loading(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Loading tests ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_running_test(&self, idents: &Vec<String>, ident: &String, is_done: bool, is_ok: bool)
    {
        if is_done {
            if is_ok {
                println!(" ok");
            } else {
                println!(" FAILED");
            }
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Test {} ...", idents_and_ident_to_string(idents, ident));
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_empty_line(&self)
    { println!(""); }
    
    fn print_successes(&self)
    {
        println!("Successes:");
        println!("");
    }

    fn print_failures(&self)
    {
        println!("Failures:");
        println!("");
    }
    
    fn print_test_result(&self, idents: &Vec<String>, ident: &String, test_result: &TestResult) -> Result<()>
    {
        if test_result.has_stdout_data()? {
            match &test_result.stdout {
                Some(stdout) => {
                    println!("---- {} stdout ----", idents_and_ident_to_string(idents, ident));
                    let stdout_g = rw_lock_read(stdout)?;
                    let _res = io::stdout().write_all(stdout_g.get_ref());
                },
                None => (),
            }
        }
        if test_result.has_stderr_data()? {
            match &test_result.stderr {
                Some(stderr) => {
                    println!("---- {} stderr ----", idents_and_ident_to_string(idents, ident));
                    let stderr_g = rw_lock_read(stderr)?;
                    let _res = io::stdout().write_all(stderr_g.get_ref());
                },
                None => (),
            }
        }
        match &test_result.error_pair {
            Some((err, stack_trace)) => {
                println!("Test {} failed", idents_and_ident_to_string(idents, ident));
                println!("{}", err);
                for (fun_value, pos) in stack_trace {
                    match fun_value {
                        Some(fun_value) => println!("    at {} ({}: {}.{})", fun_value, pos.path, pos.line, pos.column),
                        None => println!("    at {}: {}.{}", pos.path, pos.line, pos.column),
                    }
                }
            },
            None => (),
        }
        println!("");
        Ok(())
    }
    
    fn print_test_counts(&self, passed_test_count: usize, failed_test_count: usize)
    {
        if failed_test_count == 0 {
            println!("Test result: ok. {} passed; {} failed", passed_test_count, failed_test_count);
        } else {
            println!("Test result: FAILED. {} passed; {} failed", passed_test_count, failed_test_count);
        }
    }
    
    fn print_nl_for_error(&self)
    {
        if self.has_nl_for_error.swap(false, Ordering::SeqCst) {
            println!("");
        }
    }
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
    printer: Arc<dyn Print + Send + Sync>,
    has_stdout_cursors: bool,
    has_stderr_cursors: bool,
}

impl Tester
{
    pub fn new(root_mod: Arc<RwLock<ModNode<Value, ()>>>, lib_path: OsString, doc_path: OsString, printer: Arc<dyn Print + Send + Sync>, are_stdout_cursors: bool, are_stderr_cursors: bool) -> Self
    {
        Tester {
            root_mod,
            shared_env: Arc::new(RwLock::new(SharedEnv::new(lib_path, doc_path, Vec::new()))),
            stack_trace: Vec::new(),
            test_results: Vec::new(),
            printer,
            has_stdout_cursors: are_stdout_cursors,
            has_stderr_cursors: are_stderr_cursors,
        }
    }

    pub fn root_mod(&self) -> &Arc<RwLock<ModNode<Value, ()>>>
    { &self.root_mod }
    
    pub fn shared_env(&self) -> &Arc<RwLock<SharedEnv>>
    { &self.shared_env }
    
    pub fn stack_trace(&self) -> &[(Option<Value>, Pos)]
    { self.stack_trace.as_slice() }

    pub fn test_results(&self) -> &[((Vec<String>, String), TestResult)]
    { self.test_results.as_slice() }
    
    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }
    
    pub fn has_stdout_cursors(&self) -> bool
    { self.has_stdout_cursors }

    pub fn has_stderr_cursors(&self) -> bool
    { self.has_stderr_cursors }
    
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
                    env.set_stdin(Input::Null);
                    if self.has_stdout_cursors {
                        env.set_stdout(Output::Cursor(Arc::new(RwLock::new(Cursor::new(Vec::new())))));
                    }
                    if self.has_stderr_cursors {
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
    
    pub fn print_empty_line(&self)
    { self.printer.print_empty_line() }

    pub fn print_successes(&self) -> Result<()>
    {
        let mut count = 0usize;
        for (_, test_result) in &self.test_results {
            if test_result.is_ok() && (test_result.has_stdout_data()? || test_result.has_stderr_data()?) {
                count += 1;
            }
        }
        if count > 0 {
            self.printer.print_successes();
            for ((idents, ident), test_result) in &self.test_results {
                if test_result.is_ok() && (test_result.has_stdout_data()? || test_result.has_stderr_data()?) {
                    self.printer.print_test_result(idents, ident, test_result)?;
                }
            }
        }
        Ok(())
    }

    pub fn print_failures(&self) -> Result<()>
    {
        let mut count = 0usize;
        for (_, test_result) in &self.test_results {
            if !test_result.is_ok() {
                count += 1;
            }
        }
        if count > 0 {
            self.printer.print_failures();
            for ((idents, ident), test_result) in &self.test_results {
                if !test_result.is_ok() {
                    self.printer.print_test_result(idents, ident, test_result)?;
                }
            }
        }
        Ok(())
    }
    
    pub fn print_test_counts(&self)
    {
        let mut passed_test_count = 0usize;
        let mut failed_test_count = 0usize;
        for (_, test_result) in &self.test_results {
            if test_result.is_ok() {
                passed_test_count += 1;
            } else {
                failed_test_count += 1;
            }
        }
        self.printer.print_test_counts(passed_test_count, failed_test_count);
    }
}
