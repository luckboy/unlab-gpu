//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
#[cfg(feature = "plot")]
use std::thread;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
#[cfg(feature = "plot")]
use crate::winit::event_loop::ControlFlow;
#[cfg(feature = "plot")]
use crate::winit::event_loop::EventLoop;
use crate::doc::*;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::intr::*;
use crate::lexer::*;
use crate::mod_node::*;
use crate::parser::*;
#[cfg(feature = "plot")]
use crate::plot::*;
#[cfg(feature = "plot")]
use crate::utils::*;
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

#[cfg(feature = "plot")]
fn run_plotter_app<F>(are_plotter_windows: bool, f: F) -> Option<i32>
    where F: FnOnce(Option<EventLoopProxy>) -> Option<i32> + Send + 'static
{
    if are_plotter_windows {
        let event_loop = match EventLoop::<PlotterAppEvent>::with_user_event().build() {
            Ok(tmp_event_loop) => tmp_event_loop,
            Err(err) => {
                eprintln!("{}", err);
                return Some(1);
            },
        };
        let event_loop_proxy = event_loop.create_proxy();
        let thr = thread::spawn(move || f(Some(event_loop_proxy)));
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut plotter_app = PlotterApp::new(&event_loop);
        match event_loop.run_app(&mut plotter_app) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                return Some(1);
            },
        }
        match thr.join() {
            Ok(res) => res,
            Err(_) => {
                eprintln!("can't join thread");
                Some(1)
            },
        }
    } else {
        f(None)
    }
}

#[cfg(feature = "plot")]
fn quit_from_plotter_app(env: &Env) -> bool
{
    match rw_lock_read(env.shared_env()) {
        Ok(shared_env_g) => {
            match shared_env_g.event_loop_proxy() {
                Some(event_loop_proxy) => {
                    match event_loop_proxy.send_event(PlotterAppEvent::Quit) {
                        Ok(()) => true,
                        Err(err) => {
                            eprintln!("{}", err);
                            false
                        },
                    }
                },
                None => true,
            }
        },
        Err(err) => {
            eprint_error(&err);
            false
        },
    }
}

#[cfg(not(feature = "plot"))]
fn run_plotter_app<F>(_are_plotter_windows: bool, f: F) -> Option<i32>
    where F: FnOnce(Option<EventLoopProxy>) -> Option<i32> + Send + Sync + 'static
{ f(None) }

#[cfg(not(feature = "plot"))]
fn quit_from_plotter_app(_env: &Env) -> bool
{ true }

fn non_interactive_main_loop(path: String, args: Vec<String>, root_mod: Arc<RwLock<ModNode<Value, ()>>>, lib_path: OsString, is_ctrl_c_intr_checker: bool, are_plotter_windows: bool) -> Option<i32>
{
    run_plotter_app(are_plotter_windows, move |event_loop_proxy| {
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
            let shared_env = SharedEnv::new_with_intr_checker_and_event_loop_proxy(lib_path, args, intr_checker, event_loop_proxy);
            let mut env = Env::new_with_script_dir_and_shared_env(root_mod, PathBuf::from("."), Arc::new(RwLock::new(shared_env)));
            let mut interp = Interp::new();
            let res = match parse(path) {
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
            };
            if !quit_from_plotter_app(&env) {
                return Some(1);
            }
            res
    })
}

fn interactive_main_loop(args: Vec<String>, history_file: PathBuf, root_mod: Arc<RwLock<ModNode<Value, ()>>>, lib_path: OsString, is_ctrl_c_intr_checker: bool, are_plotter_windows: bool) -> Option<i32>
{
    run_plotter_app(are_plotter_windows, move |event_loop_proxy| {
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
            let shared_env = SharedEnv::new_with_intr_checker_and_event_loop_proxy(lib_path, args, intr_checker, event_loop_proxy);
            let mut env = Env::new_with_script_dir_and_shared_env(root_mod, PathBuf::from("."), Arc::new(RwLock::new(shared_env)));
            let mut interp = Interp::new();
            let mut editor = match DefaultEditor::new() {
                Ok(tmp_editor) => tmp_editor,
                Err(err) => {
                    eprintln!("{}", err);
                    return Some(1);
                },
            };
            let _res = editor.load_history(history_file.as_path());
            let mut line_num = 1u64;
            let mut res: Option<i32> = None;
            loop {
                match editor.readline(format!("unlab-gpu:{}> ", line_num).as_str()) {
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
            let mut history_dir = history_file.clone();
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
            match editor.save_history(history_file.as_path()) {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("{}", err);
                    return Some(1);
                },
            }
            if !quit_from_plotter_app(&env) {
                return Some(1);
            }
            res
    })
}

pub fn main_loop(path: Option<String>, args: Vec<String>, history_file: PathBuf, root_mod: Arc<RwLock<ModNode<Value, ()>>>, lib_path: OsString, is_ctrl_c_intr_checker: bool, are_plotter_windows: bool) -> Option<i32>
{
    match path {
        Some(path) => non_interactive_main_loop(path, args, root_mod, lib_path, is_ctrl_c_intr_checker, are_plotter_windows),
        None => interactive_main_loop(args, history_file, root_mod, lib_path, is_ctrl_c_intr_checker, are_plotter_windows),
    }
}
