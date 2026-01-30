//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::process::exit;
use clap::Parser;
use clap::Subcommand;
use unlab_gpu::Home;
use unlab_gpu::pkg_cmds::list;
use unlab_gpu::pkg_cmds::list_deps;
use unlab_gpu::pkg_cmds::search;
use unlab_gpu::pkg_cmds::search_deps;
use unlab_gpu::pkg_cmds::show;
use unlab_gpu::pkg_cmds::show_dep;
use unlab_gpu::pkg_cmds::update;
use unlab_gpu::pkg_cmds::update_deps;
use unlab_gpu::pkg_cmds::install;
use unlab_gpu::pkg_cmds::install_all;
use unlab_gpu::pkg_cmds::install_deps;
use unlab_gpu::pkg_cmds::remove;
use unlab_gpu::pkg_cmds::cont;
use unlab_gpu::pkg_cmds::continue_deps;
use unlab_gpu::pkg_cmds::clean;
use unlab_gpu::pkg_cmds::clean_deps;
use unlab_gpu::pkg_cmds::lock;
use unlab_gpu::pkg_cmds::config;
use unlab_gpu::pkg_cmds::init;
use unlab_gpu::pkg_cmds::new;
use unlab_gpu::pkg_cmds::run;
use unlab_gpu::pkg_cmds::console;

#[derive(Parser, Debug)]
struct SearchArgs
{
    /// Patterns
    patterns: Vec<String>,
}

#[derive(Parser, Debug)]
struct ShowArgs
{
    /// Show package manifest
    #[arg(short, long)]
    manifest: bool,
    /// Show package dependents
    #[arg(short, long)]
    dependents: bool,
    /// Show package paths
    #[arg(short, long)]
    paths: bool,
    /// Package
    package: String,
}

#[derive(Parser, Debug)]
struct UpdateArgs
{
    /// Package
    packages: Vec<String>,
}

#[derive(Parser, Debug)]
struct InstallArgs
{
    /// Update version lists of packages
    #[arg(short, long)]
    update: bool,
    /// Force reinstall installed packages
    #[arg(short, long)]
    force: bool,
    /// Don't install documentations of packages
    #[arg(short, long)]
    no_doc: bool,
    /// Packages
    packages: Vec<String>,
}

#[derive(Parser, Debug)]
struct InstallAllArgs
{
    /// Update version lists of packages
    #[arg(short, long)]
    update: bool,
    /// Force reinstall installed packages
    #[arg(short, long)]
    force: bool,
    /// Don't install documentations of packages
    #[arg(short, long)]
    no_doc: bool,
}
                                       
#[derive(Parser, Debug)]
struct InstallDepsArgs
{
    /// Update version lists of packages
    #[arg(short, long)]
    update: bool,
    /// Force reinstall installed packages
    #[arg(short, long)]
    force: bool,
    /// Don't install documentations of packages
    #[arg(short, long)]
    no_doc: bool,
    /// Lock versions of packages
    #[arg(short, long)]
    lock: bool,
    /// Unlock versions of packages
    #[arg(short = 'm', long)]
    unlock: bool,
}

#[derive(Parser, Debug)]
struct RemoveArgs
{
    /// Packages
    packages: Vec<String>,
}

#[derive(Parser, Debug)]
struct ContinueArgs
{
    /// Don't install documentations of packages
    #[arg(short, long)]
    no_doc: bool,
}

#[derive(Parser, Debug)]
struct ConfigArgs
{
    /// Set account for package name
    #[arg(short, long)]
    account: Option<String>,
    /// Set domain for library name
    #[arg(short, long)]
    domain: Option<String>,
}

#[derive(Parser, Debug)]
struct InitArgs
{
    /// Package name
    #[arg(short, long)]
    name: Option<String>,
    /// Account for package name 
    #[arg(short, long)]
    account: Option<String>,
    /// Domain for library name 
    #[arg(short, long)]
    domain: Option<String>,
    /// Use template binery 
    #[arg(short, long)]
    bin: bool,
    /// Use template library 
    #[arg(short, long)]
    lib: bool,
    /// Directory
    dir: Option<String>,
}

#[derive(Parser, Debug)]
struct NewArgs
{
    /// Package name
    #[arg(short, long)]
    name: Option<String>,
    /// Account for package name 
    #[arg(short, long)]
    account: Option<String>,
    /// Domain for library name 
    #[arg(short, long)]
    domain: Option<String>,
    /// Use template binery 
    #[arg(short, long)]
    bin: bool,
    /// Use template library 
    #[arg(short, long)]
    lib: bool,
    /// Directory
    dir: String,
}

#[derive(Parser, Debug)]
struct RunArgs
{
    /// Don't handle CTRL-C
    #[arg(short, long)]
    no_ctrl_c: bool,
    /// Don't show plotter windows
    #[arg(short = 'p', long)]
    no_plotter_windows: bool,
    /// Binary name
    bin_name: String,
    /// Arguments
    args: Vec<String>,
}

#[derive(Parser, Debug)]
struct ConsoleArgs
{
    /// Don't handle CTRL-C
    #[arg(short, long)]
    no_ctrl_c: bool,
    /// Don't show plotter windows
    #[arg(short = 'p', long)]
    no_plotter_windows: bool,
}

#[derive(Subcommand, Debug)]
enum Subcmd
{
    /// List packages 
    List,
    /// List dependencies of current package
    ListDeps,
    /// Search packages
    Search(SearchArgs),
    /// Search dependencies of current package
    SearchDeps(SearchArgs),
    /// Show package details
    Show(ShowArgs),
    /// Show dependency details of current package
    ShowDep(ShowArgs),
    /// Update version lists of packages
    Update(UpdateArgs),
    /// Update version lists of dependencies of current package
    UpdateDeps(UpdateArgs),
    /// Install packages
    Install(InstallArgs),
    /// Reinstall all packages
    InstallAll(InstallAllArgs),
    /// Install depedencies of current package
    InstallDeps(InstallDepsArgs),
    /// Remove packages
    Remove(RemoveArgs),
    /// Continue interrupted last operation for packages
    Continue(ContinueArgs),
    /// Continue interrupted last operation for current package
    ContinueDeps(ContinueArgs),
    /// Clean after interrupted preparation to last operation for packages
    Clean,
    /// Clean after interrupted preparation to last operation for current package
    CleanDeps,
    /// Lock versions of dependencies of current package
    Lock,
    /// Show or set configuration
    Config(ConfigArgs),
    /// Create new package in existent directory
    Init(InitArgs),
    /// Create new package
    New(NewArgs),
    /// Run binary for current package
    Run(RunArgs),
    /// Run interpreter for current package
    Console(RunArgs),
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args
{
    /// Unlab-gpu home directory
    #[arg(short = 'H', long)]
    home_dir: Option<String>,
    /// Binary path
    #[arg(short = 'B', long)]
    bin_path: Option<String>,
    /// Library path
    #[arg(short = 'L', long)]
    lib_path: Option<String>,
    /// Documentation path
    #[arg(short = 'D', long)]
    doc_path: Option<String>,
    /// Add directory to binary path
    #[arg(short, long)]
    bin_dir: Vec<String>,
    /// Add directory to library path
    #[arg(short, long)]
    lib_dir: Vec<String>,
    /// Add directory to documentation path
    #[arg(short, long)]
    doc_dir: Vec<String>,
    #[clap(subcommand)]
    subcommand: Subcmd,
}

fn main()
{
    let args = Args::parse();
    let add_dirs = |home: &mut Home| {
        match home.add_dirs_to_bin_path(args.bin_dir.as_slice()) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                return false;
            },
        }
        match home.add_dirs_to_lib_path(args.lib_dir.as_slice()) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                return false;
            },
        }
        match home.add_dirs_to_doc_path(args.doc_dir.as_slice()) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                return false;
            },
        }
        true
    };
    let exit_code = match &args.subcommand {
        Subcmd::List => {
            list(&args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::ListDeps => {
            list_deps(&args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Search(args2) => {
            search(args2.patterns.as_slice(), &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::SearchDeps(args2) => {
            search_deps(args2.patterns.as_slice(), &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Show(args2) => {
            show(args2.package.as_str(), args2.manifest, args2.dependents, args2.paths, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::ShowDep(args2) => {
            show_dep(args2.package.as_str(), args2.manifest, args2.dependents, args2.paths, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Update(args2) => {
            update(args2.packages.as_slice(), &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::UpdateDeps(args2) => {
            update_deps(args2.packages.as_slice(), &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Install(args2) => {
            install(args2.packages.as_slice(), args2.update, args2.force, !args2.no_doc, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::InstallAll(args2) => {
            install_all(args2.update, args2.force, !args2.no_doc, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::InstallDeps(args2) => {
            install_deps(args2.update, args2.force, !args2.no_doc, args2.lock, args2.unlock, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Remove(args2) => {
            remove(args2.packages.as_slice(), &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Continue(args2) => {
            cont(!args2.no_doc, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::ContinueDeps(args2) => {
            continue_deps(!args2.no_doc, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Clean => {
            clean(&args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::CleanDeps => {
            clean_deps(&args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Lock => {
            lock(&args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Config(args2) => {
            config(&args2.account, &args2.domain, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Init(args2) => {
            init(&args2.dir, &args2.name, &args2.account, &args2.domain, args2.bin, args2.lib, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::New(args2) => {
            new(args2.dir.as_str(), &args2.name, &args2.account, &args2.domain, args2.bin, args2.lib, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Run(args2) => {
            run(args2.bin_name.clone(), args2.args.clone(), !args2.no_ctrl_c, !args2.no_plotter_windows, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
        Subcmd::Console(args2) => {
            console(!args2.no_ctrl_c, !args2.no_plotter_windows, &args.home_dir, &args.bin_path, &args.lib_path, &args.doc_path, add_dirs)
        },
    };
    match exit_code {
        Some(exit_code) => exit(exit_code),
        None => (),
    }
}
