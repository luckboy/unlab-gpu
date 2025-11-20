//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::doc::*;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::intr::*;
use crate::lexer::*;
use crate::mod_node::*;
use crate::parser::*;
use crate::value::*;

pub fn eprint_error(err: &Error)
{ eprintln!("{}", err); }

pub fn eprint_error_with_stack_trace(err: &Error, stack_trace: &[(Option<Value>, Pos)])
{
    eprintln!("{}", err);
    for (fun_value, pos) in stack_trace {
        match fun_value {
            Some(fun_value) => eprintln!("    at {} ({}: {}.{})", fun_value, pos.path, pos.line, pos.column),
            None => eprintln!("    at {}: {}.{}", pos.path, pos.line, pos.column),
        }
    }
}

fn non_interactive_main_loop(path: &str, args: &[String], root_mod: &Arc<RwLock<ModNode<Value, ()>>>, lib_path: &OsStr, is_ctrl_c_intr_checker: bool) -> Option<i32>
{
    let intr_checker: Arc<dyn IntrCheck + Send + Sync> = if is_ctrl_c_intr_checker {
        match CtrlCIntrChecker::initialize() {
            Ok(()) => (),
            Err(err) => {
                eprint_error(&err);
                return Some(1);
            },
        }
        Arc::new(CtrlCIntrChecker::new())
    } else {
        Arc::new(EmptyIntrChecker::new())
    };
    let shared_env = SharedEnv::new_with_intr_checker(OsString::from(lib_path), args.to_vec(), intr_checker);
    let mut env = Env::new_with_script_dir_and_shared_env(root_mod.clone(), PathBuf::from("."), Arc::new(RwLock::new(shared_env)));
    let mut interp = Interp::new();
    match parse(path) {
        Ok(tree) => {
            match interp.interpret(&mut env, &tree) {
                Ok(()) => None,
                Err(Error::Stop(Stop::ErrorPropagation)) => {
                    eprintln!("{}", interp.ret_value());
                    Some(1)
                },
                Err(Error::Stop(Stop::Quit)) => None,
                Err(Error::Stop(Stop::Exit(code))) => Some(code),
                Err(err @ Error::Intr) => {
                    eprint_error(&err);
                    Some(1)
                },
                Err(err) => {
                    eprint_error_with_stack_trace(&err, interp.stack_trace());
                    Some(1)
                },
            }
        },
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn interactive_main_loop(args: &[String], history_file: &Path, root_mod: &Arc<RwLock<ModNode<Value, ()>>>, lib_path: &OsStr, is_ctrl_c_intr_checker: bool) -> Option<i32>
{
    let intr_checker: Arc<dyn IntrCheck + Send + Sync> = if is_ctrl_c_intr_checker {
        match CtrlCIntrChecker::initialize() {
            Ok(()) => (),
            Err(err) => {
                eprint_error(&err);
                return Some(1);
            },
        }
        Arc::new(CtrlCIntrChecker::new())
    } else {
        Arc::new(EmptyIntrChecker::new())
    };
    let shared_env = SharedEnv::new_with_intr_checker(OsString::from(lib_path), args.to_vec(), intr_checker);
    let mut env = Env::new_with_script_dir_and_shared_env(root_mod.clone(), PathBuf::from("."), Arc::new(RwLock::new(shared_env)));
    let mut interp = Interp::new();
    let mut editor = match DefaultEditor::new() {
        Ok(tmp_editor) => tmp_editor,
        Err(err) => {
            eprintln!("{}", err);
            return Some(1);
        },
    };
    let _res = editor.load_history(history_file);
    let mut line_num = 1u64;
    let mut res: Option<i32> = None;
    loop {
        match editor.readline("unlab-gpu> ") {
            Ok(line) => {
                match editor.add_history_entry(line.as_str()) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("{}", err);
                        return Some(1);
                    },
                }
                let mut new_line_num = line_num;
                let mut lines = line.clone();
                lines.push('\n');
                new_line_num += 1;
                let tree = loop {
                    let mut cursor = Cursor::new(lines.as_str());
                    let mut lexer = Lexer::new_with_line(Arc::new(String::from("(stdin)")), &mut cursor, line_num);
                    let parser_path = lexer.path().clone();
                    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
                    let mut parser = Parser::new(parser_path, tokens);
                    match parser.parse() {
                        Ok(tree) => break Some(tree),
                        Err(err @ Error::ParserEof(_, ParserEofFlag::Repetition)) => {
                            match editor.readline("> ") {
                                Ok(next_line) => {
                                    match editor.add_history_entry(next_line.as_str()) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            eprintln!("{}", err);
                                            return Some(1);
                                        },
                                    }
                                    lines.push_str(next_line.as_str());
                                    lines.push('\n');
                                    new_line_num += 1;
                                },
                                Err(ReadlineError::Interrupted) => (),
                                Err(ReadlineError::Eof) => {
                                    eprint_error(&err);
                                    break None;
                                },
                                Err(err) => {
                                    eprintln!("{}", err);
                                    return Some(1);
                                },
                            }
                        },
                        Err(err) => {
                            eprint_error(&err);
                            break None;
                        },
                    }
                };
                line_num = new_line_num;
                if is_ctrl_c_intr_checker {
                    CtrlCIntrChecker::reset();
                }
                match tree {
                    Some(tree) => {
                        match interp.interpret(&mut env, &tree) {
                            Ok(()) => (),
                            Err(Error::Stop(Stop::ErrorPropagation)) => eprintln!("{}", interp.ret_value()),
                            Err(Error::Stop(Stop::Quit)) => break,
                            Err(Error::Stop(Stop::Exit(code))) => {
                                res = Some(code);
                                break;
                            },
                            Err(err @ Error::Intr) => eprint_error(&err),
                            Err(err) => eprint_error_with_stack_trace(&err, interp.stack_trace()),
                        }
                        interp.clear_stack_trace();
                    },
                    None => (),
                }
            },
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("{}", err);
                return Some(1);
            },
        }
    }
    interp.clear_stack_trace();
    let mut history_dir = PathBuf::from(history_file);
    history_dir.pop();
    if history_dir != PathBuf::from("") {
        match create_dir_all(history_dir.as_path()) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                return Some(1);
            },
        }
    }
    match editor.save_history(history_file) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return Some(1);
        },
    }
    res
}

pub fn main_loop(path: &Option<String>, args: &[String], history_file: &Path, root_mod: &Arc<RwLock<ModNode<Value, ()>>>, lib_path: &OsStr, is_ctrl_c_intr_checker: bool) -> Option<i32>
{
    match path {
        Some(path) => non_interactive_main_loop(path.as_str(), args, root_mod, lib_path, is_ctrl_c_intr_checker),
        None => interactive_main_loop(args, history_file, root_mod, lib_path, is_ctrl_c_intr_checker),
    }
}
