//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::process::exit;
use std::sync::Arc;
use std::sync::RwLock;
use clap::Parser;
use unlab_gpu::Home;
use unlab_gpu::ModNode;
use unlab_gpu::Value;
use unlab_gpu::add_std_builtin_funs;
use unlab_gpu::finalize_backend;
use unlab_gpu::initialize_backend;
use unlab_gpu::main_loop;

#[derive(Parser, Debug)]
#[command(version)]
struct Args
{
    /// Unlab-gpu home directory
    #[arg(short = 'H', long)]
    home_dir: Option<String>,
    /// Library path
    #[arg(short = 'L', long)]
    lib_path: Option<String>,
    /// Script file
    script_file: Option<String>,
    /// Arguments
    args: Vec<String>,
}

fn main()
{
    let args = Args::parse();
    let home = match Home::new(&args.home_dir, &args.lib_path) {
        Some(tmp_home) => tmp_home,
        None => {
            eprintln!("no unlab-gpu home directory");
            exit(1);
        },
    };
    match initialize_backend(home.backend_config_file()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
    let exit_code = {
        let mut root_mod: ModNode<Value, ()> = ModNode::new(());
        add_std_builtin_funs(&mut root_mod);
        let root_mod_arc = Arc::new(RwLock::new(root_mod));
        main_loop(&args.script_file, args.args.as_slice(), home.history_file(), &root_mod_arc, home.lib_path())
    };
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
    match exit_code {
        Some(exit_code) => exit(exit_code),
        None => (),
    }
}
