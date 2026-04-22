//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! Micro neural scripting language for GPU is simple scripting langauge that operates on matrices.
//!
//! This scripting language is implemented in the Unlab-gpu library that uses the Unmtx-gpu library
//! to operate on matrices. Also, the Unlab-gpu library implements a package manager that can be
//! used to install and remove packages.
//!
//! The Unlab-gpu library is a very modular library that contains the following components for this
//! scripting language:
//!
//! - lexer
//! - parser
//! - interpreter
//! - standard built-in functions
//! - getopts
//! - plotter
//! - documentation generator
//! - package manager
//! - tester
//!
//! This scripting language is extensible by the Unlab-gpu library. The Unlab-gpu library allows to
//! extend this scripting language with own built-in functions. The standard built-in functions of
//! this scripting language also can be replaced by own built-in functions.
pub use ctrlc;
pub use curl;
pub use serde;
pub use serde_json;
pub use toml;
pub use unmtx_gpu as matrix;
#[cfg(feature = "plot")]
pub use winit;

pub mod backend;
pub mod builtin_doc;
pub mod builtins;
pub mod dfs;
pub mod doc;
pub mod env;
pub mod error;
pub mod fs;
pub mod getopts;
pub mod getopts_doc;
pub mod home;
pub mod interp;
pub mod intr;
pub mod io;
pub mod lexer;
pub mod main_loop;
pub mod mod_node;
pub mod parser;
pub mod pkg;
pub mod pkg_cmds;
#[cfg(feature = "plot")]
pub mod plot;
#[cfg(feature = "plot")]
pub mod plot_doc;
pub mod tester;
pub mod tree;
pub mod utils;
pub mod value;
pub mod version;

pub use backend::initialize_backend;
pub use backend::finalize_backend;
pub use builtins::add_std_builtin_funs;
pub use env::Env;
pub use error::Error;
pub use error::Result;
pub use home::Home;
pub use interp::Interp;
pub use main_loop::main_loop;
pub use mod_node::ModNode;
pub use parser::parse;
pub use parser::parse_with_doc_root_mod;
pub use parser::parse_with_doc_root_mod_and_doc_current_mod;
pub use tree::Tree;
pub use value::Value;

#[cfg(test)]
pub(crate) mod test_helpers;
