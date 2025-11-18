//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub use ctrlc;
pub use ini;
pub use unmtx_gpu as matrix;

pub mod backend;
pub mod builtins;
pub mod doc;
pub mod env;
pub mod error;
pub mod home;
pub mod interp;
pub mod intr;
pub mod io;
pub mod lexer;
pub mod main_loop;
pub mod mod_node;
pub mod parser;
pub mod tree;
pub mod utils;
pub mod value;

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
pub use tree::Tree;
pub use value::Value;

#[cfg(test)]
pub(crate) mod test_helpers;
