//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::env::var_os;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Home
{
    home_dir: PathBuf,
    backend_config_file: PathBuf,
    history_file: PathBuf,
    lib_path: OsString,
}

impl Home
{
    pub fn new(home_dir: &Option<String>, lib_path: &Option<String>) -> Option<Self>
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
        let lib_path = match lib_path {
            Some(lib_path) => OsString::from(lib_path.as_str()),
            None => {
                match var_os("UNLAB_GPU_LIB_PATH") {
                    Some(tmp_lib_path) => tmp_lib_path,
                    None => {
                        let mut tmp_lib_path = home_dir.clone();
                        tmp_lib_path.push("lib");
                        tmp_lib_path.into_os_string()
                    },
                }
            },
        };
        Some(Home { home_dir, backend_config_file, history_file, lib_path, })
    }
    
    pub fn home_dir(&self) -> &Path
    { self.home_dir.as_path() }

    pub fn backend_config_file(&self) -> &Path
    { self.backend_config_file.as_path() }

    pub fn history_file(&self) -> &Path
    { self.history_file.as_path() }

    pub fn lib_path(&self) -> &OsStr
    { self.lib_path.as_os_str() }
}
