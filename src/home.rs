//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A home module.
use std::env::JoinPathsError;
use std::env::join_paths;
use std::env::split_paths;
use std::env::var_os;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// A structure of home.
///
/// The home contains paths to the Unlab-gpu home directory, configuration files, history file. By
/// default, the Unlab-gpu home directory is located in the home directory. Also, the binary
/// paths, the library paths, and the documentation paths are in the home. The Unlab-gpu home
/// directory or the work directory of current package has the binary directory, the library
/// directory, and the documentation directory by default.
#[derive(Clone, Debug)]
pub struct Home
{
    home_dir: PathBuf,
    backend_config_file: PathBuf,
    history_file: PathBuf,
    pkg_config_file: PathBuf,
    bin_path: OsString,
    lib_path: OsString,
    doc_path: OsString,
}

impl Home
{
    fn path_from<K: AsRef<OsStr>, L: AsRef<OsStr>, D: AsRef<Path>>(path: &Option<String>, path_var_name: K, work_path_var_name: L, home_dir: &PathBuf, dir: D, is_work_dir: bool) -> OsString
    {
        match path {
            Some(path) => OsString::from(path.as_str()),
            None => {
                if !is_work_dir {
                    match var_os(path_var_name) {
                        Some(tmp_lib_path) => tmp_lib_path,
                        None => {
                            let mut tmp_lib_path = home_dir.clone();
                            tmp_lib_path.push(dir);
                            tmp_lib_path.into_os_string()
                        },
                    }
                } else {
                    match var_os(work_path_var_name) {
                        Some(tmp_lib_path) => tmp_lib_path,
                        None => {
                            let mut tmp_lib_path = PathBuf::from("work");
                            tmp_lib_path.push(dir);
                            tmp_lib_path.into_os_string()
                        },
                    }
                }
            },
        }
    }
    
    /// Creates a home.
    ///
    /// This method takes the Unlab-gpu home directory, the paths. The binary directory, the 
    /// library directory, and the documentation directory are located in the Unlab-gpu by default
    /// if the flag of work directory isn't set, otherwise these directories are located in the work directory of
    /// current package by default.
    pub fn new(home_dir: &Option<String>, bin_path: &Option<String>, lib_path: &Option<String>, doc_path: &Option<String>, is_work_dir: bool) -> Option<Self>
    {
        let home_dir = match home_dir {
            Some(home_dir) => PathBuf::from(home_dir.as_str()),
            None => {
                match home::home_dir() {
                    Some(user_home_dir) => {
                        let mut tmp_home_dir = user_home_dir.clone();
                        match var_os("UNLAB_GPU_HOME") {
                            Some(tmp_home_dir2) => tmp_home_dir.push(tmp_home_dir2.as_os_str()),
                            None => tmp_home_dir.push(".unlab-gpu"),
                        }
                        tmp_home_dir
                    },
                    None => {
                        match var_os("UNLAB_GPU_HOME") {
                            Some(tmp_home_dir) => PathBuf::from(tmp_home_dir.as_os_str()),
                            None => return None,
                        }
                    },
                }
            },
        };
        let mut backend_config_file = home_dir.clone();
        backend_config_file.push("backend.toml");
        let mut history_file = home_dir.clone();
        history_file.push("history.txt");
        let mut pkg_config_file = home_dir.clone();
        pkg_config_file.push("pkg.toml");
        let bin_path = Self::path_from(bin_path, "UNLAB_GPU_BIN_PATH", "UNLAB_GPU_WORK_BIN_PATH", &home_dir, "bin", is_work_dir);
        let lib_path = Self::path_from(lib_path, "UNLAB_GPU_LIB_PATH", "UNLAB_GPU_WORK_LIB_PATH", &home_dir, "lib", is_work_dir);
        let doc_path = Self::path_from(doc_path, "UNLAB_GPU_DOC_PATH", "UNLAB_GPU_WORK_DOC_PATH", &home_dir, "doc", is_work_dir);
        Some(Home {
                home_dir,
                backend_config_file,
                history_file,
                pkg_config_file,
                bin_path,
                lib_path,
                doc_path,
        })
    }
    
    /// Returns the path to the Unlab-gpu home directory.
    pub fn home_dir(&self) -> &Path
    { self.home_dir.as_path() }

    /// Returns th path to the file of backend configuration.
    pub fn backend_config_file(&self) -> &Path
    { self.backend_config_file.as_path() }

    /// Returns the path to the history file.
    pub fn history_file(&self) -> &Path
    { self.history_file.as_path() }

    /// Returns the path to the file of package configuration.
    pub fn pkg_config_file(&self) -> &Path
    { self.pkg_config_file.as_path() }

    /// Returns the binary paths.
    pub fn bin_path(&self) -> &OsStr
    { self.bin_path.as_os_str() }

    /// Returns the library paths.
    pub fn lib_path(&self) -> &OsStr
    { self.lib_path.as_os_str() }

    /// Returns the documentation paths.
    pub fn doc_path(&self) -> &OsStr
    { self.doc_path.as_os_str() }

    fn add_dirs_to_path(path: &mut OsString, dirs: &[String]) -> Result<(), JoinPathsError>
    {
        if !dirs.is_empty() {
            let mut tmp_dirs: Vec<OsString> = dirs.iter().map(|d| OsString::from(d)).collect();
            let mut tmp_dirs_from_path: Vec<OsString> = split_paths(path).map(|d| d.into_os_string()).collect();
            tmp_dirs.reverse();
            tmp_dirs.append(&mut tmp_dirs_from_path);
            *path = join_paths(tmp_dirs)?;
        }
        Ok(())
    }

    /// Adds the directory paths to the binary paths.
    ///
    /// Each directory path is pushed front to the binary paths. An addition order of directory
    /// paths is determined by the order of directory paths on the slice.
    pub fn add_dirs_to_bin_path(&mut self, dirs: &[String]) -> Result<(), JoinPathsError>
    { Self::add_dirs_to_path(&mut self.bin_path, dirs) }

    /// Adds the directory paths to the library paths.
    ///
    /// See [`add_dirs_to_bin_path`](Self::add_dirs_to_bin_path).
    pub fn add_dirs_to_lib_path(&mut self, dirs: &[String]) -> Result<(), JoinPathsError>
    { Self::add_dirs_to_path(&mut self.lib_path, dirs) }
    
    /// Adds the directory paths to the documentation paths.
    ///
    /// See [`add_dirs_to_bin_path`](Self::add_dirs_to_bin_path).
    pub fn add_dirs_to_doc_path(&mut self, dirs: &[String]) -> Result<(), JoinPathsError>
    { Self::add_dirs_to_path(&mut self.doc_path, dirs) }
}
