//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::env::current_dir;
use std::env::set_current_dir;
use std::env::split_paths;
use std::ffi::OsString;
use std::fs::File;
use std::fs::create_dir_all;
use std::fs::remove_dir;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use crate::toml;
use crate::backend::*;
use crate::builtins::add_std_builtin_funs;
use crate::error::*;
use crate::home::*;
use crate::main_loop::*;
use crate::mod_node::*;
use crate::pkg::*;
use crate::utils::*;
use crate::value::*;

fn create_home<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, is_work_dir: bool, f: F) -> Option<Home>
    where F: FnOnce(&mut Home) -> bool
{
    match Home::new(home_dir, bin_path, lib_path, doc_path, is_work_dir) {
        Some(mut home) => {
            if !f(&mut home) {
                return None;
            }
            Some(home)
        },
        None => {
            eprintln!("no unlab-gpu home directory");
            None
        },
    }
}

fn create_pkg_manager<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, is_work_dir: bool, f: F) -> Option<PkgManager>
    where F: FnOnce(&mut Home) -> bool
{
    let home = create_home(home_dir, bin_path, lib_path, doc_path, is_work_dir, f)?;
    let home_dir = PathBuf::from(home.home_dir());
    let work_dir = if !is_work_dir {
        PathBuf::from(home.home_dir())
    } else {
        PathBuf::from("work")
    };
    let bin_dir = match split_paths(home.bin_path()).next() {
        Some(tmp_bin_dir) => PathBuf::from(tmp_bin_dir),
        None => {
            eprintln!("no binary directory");
            return None;
        },
    };
    let lib_dir = match split_paths(home.bin_path()).next() {
        Some(tmp_lib_dir) => PathBuf::from(tmp_lib_dir),
        None => {
            eprintln!("no library directory");
            return None;
        },
    };
    let doc_dir = match split_paths(home.doc_path()).next() {
        Some(tmp_doc_dir) => PathBuf::from(tmp_doc_dir),
        None => {
            eprintln!("no documentation directory");
            return None;
        },
    };
    match PkgManager::new(home_dir, work_dir, bin_dir, lib_dir, doc_dir, default_src_factories(), Arc::new(StdPrinter::new())) {
        Ok(pkg_manager) => Some(pkg_manager),
        Err(err) => {
            eprint_error(&err);
            None
        },
    }
}

fn parse_pkg_name(s: &str) -> Option<PkgName>
{
    match PkgName::parse(s) {
        Ok(pkg_name) => Some(pkg_name),
        Err(err) => {
            eprint_error(&err);
            None
        },
    }
}

fn parse_pkg_names(ss: &[String]) -> Option<Vec<PkgName>>
{
    let mut pkg_names: Vec<PkgName> = Vec::new();
    for s in ss {
        pkg_names.push(parse_pkg_name(s.as_str())?);
    }
    Some(pkg_names)
}

fn res_list(pkg_manager: &PkgManager, are_deps: bool) -> Result<()>
{
    pkg_manager.check_last_op(are_deps)?;
    pkg_manager.pkg_versions_in(|name, version| {
            println!("{} v{}", name, version);
            Ok(())
    })?;
    Ok(())
}

fn list_with_dep_flag<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, are_deps: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, are_deps, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_list(&pkg_manager, are_deps) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn list<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ list_with_dep_flag(home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn list_deps<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ list_with_dep_flag(home_dir, bin_path, lib_path, doc_path, true, f) }

fn res_search(pkg_manager: &PkgManager, patterns :&[String], are_deps: bool) -> Result<()>
{
    pkg_manager.check_last_op(are_deps)?;
    pkg_manager.pkg_versions_in(|name, version| {
            match pkg_manager.pkg_manifest(name)? {
                Some(manifest) => {
                    if patterns.iter().all(|p| name.name().contains(p.as_str()) || manifest.package.description.as_ref().map(|d| d.contains(p.as_str())).unwrap_or(false)) {
                        println!("{} v{}", name, version);
                    }
                },
                None => (),
            }
            Ok(())
    })?;
    Ok(())
}

fn search_with_dep_flag<F>(patterns :&[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, are_deps: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, are_deps, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_search(&pkg_manager, patterns, are_deps) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn search<F>(patterns :&[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ search_with_dep_flag(patterns, home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn search_deps<F>(patterns :&[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ search_with_dep_flag(patterns, home_dir, bin_path, lib_path, doc_path, true, f) }

fn res_show(pkg_manager: &PkgManager, pkg_name: &PkgName, is_manifest: bool, are_dependents: bool, are_paths: bool, is_dep: bool) -> Result<()>
{
    pkg_manager.check_last_op(is_dep)?;
    let version = match pkg_manager.pkg_version(pkg_name)? {
        Some(tmp_version) => tmp_version,
        None => return Err(Error::PkgName(pkg_name.clone(), String::from("not found package"))),
    };
    let manifest = match pkg_manager.pkg_manifest(pkg_name)? {
        Some(tmp_manifest) => tmp_manifest,
        None => return Err(Error::PkgName(pkg_name.clone(), String::from("no package manifest"))),
    };
    let dependents = match pkg_manager.pkg_dependents(pkg_name)? {
        Some(tmp_dependents) => tmp_dependents,
        None => return Err(Error::PkgName(pkg_name.clone(), String::from("no package dependents"))),
    };
    let paths = match pkg_manager.pkg_paths(pkg_name)? {
        Some(tmp_paths) => tmp_paths,
        None => return Err(Error::PkgName(pkg_name.clone(), String::from("no package paths"))),
    };
    println!("{} v{}", pkg_name, version);
    if is_manifest || (!is_manifest && !are_dependents && !are_paths) {
        println!("Manifest:");
        match toml::to_string_pretty(&manifest) {
            Ok(s) => println!("{}", s),
            Err(err) => return Err(Error::TomlSer(err)),
        }
    }
    if are_dependents {
        println!("Dependents:");
        match toml::to_string_pretty(&dependents) {
            Ok(s) => println!("{}", s),
            Err(err) => return Err(Error::TomlSer(err)),
        }
    }
    if are_paths {
        println!("Paths:");
        match toml::to_string_pretty(&paths) {
            Ok(s) => println!("{}", s),
            Err(err) => return Err(Error::TomlSer(err)),
        }
    }
    Ok(())
}

fn show_with_dep_flag<F>(name :&str, is_manifest: bool, are_dependents: bool, are_paths: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, is_dep: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_name = match parse_pkg_name(name) {
        Some(tmp_pkg_name) => tmp_pkg_name,
        None => return Some(1),
    };
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, is_dep, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_show(&pkg_manager, &pkg_name, is_manifest, are_dependents, are_paths, is_dep) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn show<F>(name :&str, is_manifest: bool, are_dependents: bool, are_paths: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ show_with_dep_flag(name, is_manifest, are_dependents, are_paths, home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn show_dep<F>(name :&str, is_manifest: bool, are_dependents: bool, are_paths: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ show_with_dep_flag(name, is_manifest, are_dependents, are_paths, home_dir, bin_path, lib_path, doc_path, true, f) }

fn res_update(pkg_manager: &PkgManager, pkg_names: &[PkgName], are_deps: bool) -> Result<()>
{
    pkg_manager.check_last_op(are_deps)?;
    if pkg_names.is_empty() {
        pkg_manager.update_all()?;
    } else {
        pkg_manager.update(pkg_names)?;
    }
    Ok(())
}

fn update_with_dep_flag<F>(names: &[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, are_deps: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_names = match parse_pkg_names(names) {
        Some(tmp_pkg_names) => tmp_pkg_names,
        None => return Some(1),
    };
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, are_deps, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_update(&pkg_manager, pkg_names.as_slice(), are_deps) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn update<F>(names: &[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ update_with_dep_flag(names, home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn update_deps<F>(names: &[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ update_with_dep_flag(names, home_dir, bin_path, lib_path, doc_path, true, f) }

fn res_install(pkg_manager: &mut PkgManager, pkg_names: &[PkgName], is_update: bool, is_force: bool, is_doc: bool) -> Result<()>
{
    pkg_manager.check_last_op(false)?;
    pkg_manager.load_constraints()?;
    pkg_manager.load_sources()?;
    pkg_manager.install(pkg_names, is_update, is_force, is_doc)?;
    Ok(())
}

pub fn install<F>(names: &[String], is_update: bool, is_force: bool, is_doc: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_names = match parse_pkg_names(names) {
        Some(tmp_pkg_names) => tmp_pkg_names,
        None => return Some(1),
    };
    let mut pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_install(&mut pkg_manager, pkg_names.as_slice(), is_update, is_force, is_doc) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn res_install_all(pkg_manager: &mut PkgManager, is_update: bool, is_force: bool, is_doc: bool) -> Result<()>
{
    pkg_manager.check_last_op(false)?;
    pkg_manager.load_constraints()?;
    pkg_manager.load_sources()?;
    pkg_manager.install_all(is_update, is_force, is_doc)?;
    Ok(())
}

pub fn install_all<F>(is_update: bool, is_force: bool, is_doc: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let mut pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_install_all(&mut pkg_manager, is_update, is_force, is_doc) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn res_install_deps(pkg_manager: &mut PkgManager, is_update: bool, is_force: bool, is_doc: bool, is_locked: bool, is_unlocked: bool) -> Result<()>
{
    pkg_manager.check_last_op(true)?;
    if (!is_update || is_locked) && (!is_unlocked || is_locked) {
        pkg_manager.load_locks()?;
    }
    pkg_manager.install_deps(is_update, is_force, is_doc)?;
    pkg_manager.save_locks_from_pkg_versions()?;
    Ok(())
}

pub fn install_deps<F>(is_update: bool, is_force: bool, is_doc: bool, is_locked: bool, is_unlocked: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let mut pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_install_deps(&mut pkg_manager, is_update, is_force, is_doc, is_locked, is_unlocked) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn res_remove(pkg_manager: &mut PkgManager, pkg_names: &[PkgName]) -> Result<()>
{
    pkg_manager.check_last_op(false)?;
    pkg_manager.remove(pkg_names)?;
    Ok(())
}

pub fn remove<F>(names: &[String], home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_names = match parse_pkg_names(names) {
        Some(tmp_pkg_names) => tmp_pkg_names,
        None => return Some(1),
    };
    let mut pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_remove(&mut pkg_manager, pkg_names.as_slice()) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn res_continue(pkg_manager: &PkgManager, is_doc: bool, are_deps: bool) -> Result<()>
{
    pkg_manager.cont(is_doc, are_deps)?;
    if are_deps {
        pkg_manager.save_locks_from_pkg_versions()?;
    }
    Ok(())
}

fn continue_with_dep_flag<F>(is_doc: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, are_deps: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, are_deps, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_continue(&pkg_manager, is_doc, are_deps) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn cont<F>(is_doc: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ continue_with_dep_flag(is_doc, home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn continue_deps<F>(is_doc: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ continue_with_dep_flag(is_doc, home_dir, bin_path, lib_path, doc_path, true, f) }

fn clean_with_dep_flag<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, are_deps: bool, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, are_deps, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match pkg_manager.clean() {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

pub fn clean<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ clean_with_dep_flag(home_dir, bin_path, lib_path, doc_path, false, f) }

pub fn clean_deps<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ clean_with_dep_flag(home_dir, bin_path, lib_path, doc_path, true, f) }

fn res_lock(pkg_manager: &PkgManager) -> Result<()>
{
    pkg_manager.check_last_op(true)?;
    pkg_manager.save_locks_from_pkg_versions()?;
    Ok(())
}

pub fn lock<F>(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let pkg_manager = match create_pkg_manager(home_dir, bin_path, lib_path, doc_path, true, f) {
        Some(tmp_pkg_manager) => tmp_pkg_manager,
        None => return Some(1),
    };
    match res_lock(&pkg_manager) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn res_config(home: &Home, account: &Option<String>, domain: &Option<String>) -> Result<()>
{
    let config = PkgConfig::load_opt(home.pkg_config_file())?;
    if account.is_none() && domain.is_none() {
        println!("Configuration:");
        match &config {
            Some(config) => {
                match toml::to_string_pretty(&config) {
                    Ok(s) => println!("{}", s),
                    Err(err) => return Err(Error::TomlSer(err)),
                }
            },
            None => println!(""),
        }
    } else {
        let mut old_account: Option<String> = None;
        let mut old_domain: Option<String> = None;
        match config {
            Some(config) => {
                old_account = config.account.clone();
                old_domain = config.domain.clone();
            },
            None => (),
        }
        let new_config = PkgConfig::new(account.clone().or(old_account), domain.clone().or(old_domain));
        new_config.save(home.pkg_config_file())?;
    }
    Ok(())
}

pub fn config<F>(account: &Option<String>, domain: &Option<String>, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let home = match create_home(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_home) => tmp_home,
        None => return Some(1),
    };
    match res_config(&home, account, domain) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn io_res_init(bin_name: &str, lib_name: &str, is_bin: bool, is_lib: bool) -> io::Result<()>
{
    {
        let file = File::create(".gitignore")?;
        let mut w = BufWriter::new(file);
        write!(&mut w, "/work")?;
    }
    if is_bin {
        let mut path_buf = PathBuf::from("bin");
        create_dir_all(path_buf.as_path())?;
        path_buf.push(bin_name);
        let file = File::create(path_buf)?;
        let mut w = BufWriter::new(file);
        writeln!(&mut w, "#!/usr/bin/env unlab-gpu --")?;
        writeln!(&mut w, "")?;
        writeln!(&mut w, "println(\"Hello world!!!\")")?;
    }
    if is_lib || (!is_bin && !is_lib) {
        let mut path_buf = PathBuf::from("lib");
        path_buf.push(lib_name.replace('/', path::MAIN_SEPARATOR_STR));
        create_dir_all(path_buf.as_path())?;
        path_buf.push("lib.un");
        let file = File::create(path_buf)?;
        let mut w = BufWriter::new(file);
        writeln!(&mut w, "module {}", str_to_ident(lib_name))?;
        writeln!(&mut w, "    function add(x, y)")?;
        writeln!(&mut w, "        x + y")?;
        writeln!(&mut w, "    end")?;
        writeln!(&mut w, "end")?;
    }
    Ok(())
}

fn res_init(pkg_name: &PkgName, bin_name: &str, lib_name: &str, is_bin: bool, is_lib: bool) -> Result<()>
{
    let manifest = Manifest::new(pkg_name.clone());
    PkgManager::save_manifest(&manifest)?;
    match io_res_init(bin_name, lib_name, is_bin, is_lib) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn init<F>(path: &Option<String>, name: &Option<String>, account: &Option<String>, domain: &Option<String>, is_bin: bool, is_lib: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    match path {
        Some(path) => {
            match set_current_dir(path) {
                Ok(()) => (),
                Err(err) => {
                    eprint_error(&Error::Io(err));
                    return Some(1);
                },
            }
        },
        None => (),
    }
    let home = match create_home(home_dir, bin_path, lib_path, doc_path, false, f) {
        Some(tmp_home) => tmp_home,
        None => return Some(1),
    };
    let config = match PkgConfig::load_opt(home.pkg_config_file()) {
        Ok(tmp_config) => tmp_config,
        Err(err) => {
            eprint_error(&err);
            return Some(1);
        },
    };
    let mut config_account: Option<String> = None;
    let mut config_domain: Option<String> = None;
    match config {
        Some(config) => {
            config_account = config.account.clone();
            config_domain = config.domain.clone();
        },
        None => (),
    }
    let new_account = account.clone().or(config_account);
    let new_domain = domain.clone().or(config_domain);
    let new_name = match name {
        Some(tmp_new_name) => tmp_new_name.clone(),
        None => {
            let dir = match current_dir() {
                Ok(tmp_dir) => tmp_dir,
                Err(err) => {
                    eprint_error(&Error::Io(err));
                    return Some(1);
                },
            };
            let dir_name = match dir.to_str() {
                Some(tmp_dir_name) => tmp_dir_name,
                None => {
                    eprintln!("directory name contains invalid UTF-8 character");
                    return Some(1);
                },
            };
            let mut tmp_new_name = match &new_account {
                Some(new_account) => new_account.clone(),
                None => {
                    eprintln!("no account");
                    return Some(1);
                }
            };
            tmp_new_name.push('/');
            tmp_new_name.push_str(dir_name);
            tmp_new_name
        },
    };
    let pkg_name = match parse_pkg_name(new_name.as_str()) {
        Some(tmp_pkg_name) => tmp_pkg_name,
        None => return Some(1),
    };
    let ss: Vec<&str> = pkg_name.name().split('/').collect();
    let last_pkg_name_comp = match ss.last() {
        Some(tmp_last_pkg_name_comp) => String::from(*tmp_last_pkg_name_comp),
        None => {
            eprintln!("no last package name component");
            return Some(1);
        },
    };
    let bin_name = last_pkg_name_comp.clone();
    let lib_name = match &new_domain {
        Some(new_domain) => {
            let mut tmp_lib_name = new_domain.clone();
            tmp_lib_name.push('/');
            tmp_lib_name.push_str(last_pkg_name_comp.as_str());
            tmp_lib_name
        },
        None => {
            eprintln!("no domain");
            return Some(1);
        },
    };
    match res_init(&pkg_name, bin_name.as_str(), lib_name.as_str(), is_bin, is_lib) {
        Ok(()) => None,
        Err(err) => {
            eprint_error(&err);
            Some(1)
        },
    }
}

fn create_and_change_dir<P: AsRef<Path>>(path: P) -> io::Result<PathBuf>
{
    let saved_current_dir = current_dir()?;
    create_dir_all(path.as_ref())?;
    match set_current_dir(path.as_ref()) {
        Ok(()) => Ok(saved_current_dir),
        Err(err) => {
            remove_dir(path.as_ref())?;
            return Err(err);
        },
    }
}

fn change_and_remove_dir<P: AsRef<Path>, Q: AsRef<Path>>(path: P, saved_current_dir: Q) -> io::Result<()>
{
    set_current_dir(saved_current_dir)?;
    remove_dir(path)?;
    Ok(())
}

pub fn new<F>(path: &str, name: &Option<String>, account: &Option<String>, domain: &Option<String>, is_bin: bool, is_lib: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let saved_current_dir = match create_and_change_dir(path) {
        Ok(tmp_saved_current_dir) => tmp_saved_current_dir,
        Err(err) => {
            eprint_error(&Error::Io(err));
            return Some(1);
        },
    };
    match init(&None, name, account, domain, is_bin, is_lib, home_dir, bin_path, lib_path, doc_path, f) {
        None => None,
        Some(exit_code) => {
            match change_and_remove_dir(path, saved_current_dir) {
                Ok(()) => (),
                Err(err) => {
                    eprint_error(&Error::Io(err));
                    return Some(1);
                },
            }
            Some(exit_code)
        },
    }
}

fn run_with_opt_name<F>(name: Option<String>, args: Vec<String>, is_ctrl_c_intr_checker: bool, are_plotter_windows: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{
    let mut home = match create_home(home_dir, bin_path, lib_path, doc_path, true, f) {
        Some(tmp_home) => tmp_home,
        None => return Some(1),
    };
    match home.add_dirs_to_bin_path(&[String::from("bin")]) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return Some(1);
        },
    }
    match home.add_dirs_to_lib_path(&[String::from("lib")]) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            return Some(1);
        },
    }
    match initialize_backend(home.backend_config_file()) {
        Ok(()) => (),
        Err(err) => {
            eprint_error(&err);
            return Some(1);
        },
    }
    let script_file = match &name {
        Some(name) => {
            let mut tmp_script_file = String::from("bin");
            tmp_script_file.push(path::MAIN_SEPARATOR);
            tmp_script_file.push_str(name.as_str());
            Some(tmp_script_file)
        },
        None => None,
    };
    let exit_code = {
        let mut root_mod: ModNode<Value, ()> = ModNode::new(());
        add_std_builtin_funs(&mut root_mod);
        let root_mod_arc = Arc::new(RwLock::new(root_mod));
        main_loop(script_file, args, PathBuf::from(home.history_file()), root_mod_arc, OsString::from(home.lib_path()), OsString::from(home.doc_path()), is_ctrl_c_intr_checker, are_plotter_windows)
    };
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprint_error(&err);
            return Some(1);
        },
    }
    exit_code
}

pub fn run<F>(name: String, args: Vec<String>, is_ctrl_c_intr_checker: bool, are_plotter_windows: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ run_with_opt_name(Some(name), args, is_ctrl_c_intr_checker, are_plotter_windows, home_dir, bin_path, lib_path, doc_path, f) }

pub fn console<F>(is_ctrl_c_intr_checker: bool, are_plotter_windows: bool, home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, f: F) -> Option<i32>
    where F: FnOnce(&mut Home) -> bool
{ run_with_opt_name(None, Vec::new(), is_ctrl_c_intr_checker, are_plotter_windows, home_dir, bin_path, lib_path, doc_path, f) }
