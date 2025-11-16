//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::env::var_os;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Home
{
    home_dir: String,
    backend_config_file: String,
    history_file: String,
    lib_path: String,
}

impl Home
{
    pub fn new(lib_path: Option<String>) -> Option<Self>
    {
        let home_dir = match home::home_dir() {
            Some(os_user_home_dir) => {
                let mut os_home_dir = os_user_home_dir.clone();
                match var_os("UNLAB_GPU_HOME") {
                    Some(tmp_os_home_dir) => os_home_dir.push(tmp_os_home_dir.as_os_str()),
                    None => os_home_dir.push(".unlab-gpu"),
                }
                os_home_dir.to_string_lossy().into_owned()
            },
            None => {
                match var_os("UNLAB_GPU_HOME") {
                    Some(os_home_dir) => os_home_dir.to_string_lossy().into_owned(),
                    None => return None,
                }
            },
        };
        let mut os_backend_config_file = PathBuf::from(home_dir.as_str());
        os_backend_config_file.push("backend.toml");
        let backend_config_file = os_backend_config_file.to_string_lossy().into_owned();
        let mut os_history_file = PathBuf::from(home_dir.as_str());
        os_history_file.push("history.txt");
        let history_file = os_history_file.to_string_lossy().into_owned();
        let new_lib_path = match lib_path {
            Some(lib_path) => lib_path,
            None => {
                match var_os("UNLAB_GPU_LIB_PATH") {
                    Some(os_lib_path) => os_lib_path.to_string_lossy().into_owned(),
                    None => {
                        let mut os_lib_path = PathBuf::from(home_dir.as_str());
                        os_lib_path.push("lib");
                        os_lib_path.to_string_lossy().into_owned()
                    },
                }
            },
        };
        Some(Home { home_dir, backend_config_file, history_file, lib_path: new_lib_path, })
    }
    
    pub fn home_dir(&self) -> &str
    { self.home_dir.as_str() }

    pub fn backend_config_file(&self) -> &str
    { self.backend_config_file.as_str() }

    pub fn history_file(&self) -> &str
    { self.history_file.as_str() }

    pub fn lib_path(&self) -> &str
    { self.lib_path.as_str() }
}
