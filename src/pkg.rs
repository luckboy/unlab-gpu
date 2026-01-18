//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::copy;
use std::fs::create_dir_all;
use std::fs::remove_dir;
use std::fs::rename;
use std::io;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::io::stdout;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::result;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use jammdb::Bucket;
use jammdb::DB;
use jammdb::KVPair;
use jammdb::ToBytes;
use jammdb::Tx;
use liblzma::read::XzDecoder;
use zip::read::ZipArchive;
use crate::curl;
use crate::serde::de;
use crate::serde::de::Visitor;
use crate::serde::Deserialize;
use crate::serde::Deserializer;
use crate::serde::Serialize;
use crate::serde::Serializer;
use crate::dfs::*;
use crate::error::*;
use crate::fs::*;
use crate::utils::*;
use crate::version::*;

pub trait Print
{
    fn print_pre_installing(&self);
    
    fn print_installing(&self);

    fn print_pre_removing(&self);

    fn print_removing(&self);

    fn print_updating_pkg_versions(&self, name: &PkgName, is_done: bool);
    
    fn print_downloading_pkg_file(&self, name: &PkgName, is_done: bool);

    fn print_downloading_pkg_file_with_progress(&self, name: &PkgName, byte_count: f64, total_byte_count: f64);
    
    fn print_extracting_pkg_file(&self, name: &PkgName, is_done: bool);
    
    fn print_checking_dependent_version_reqs(&self, is_done: bool);

    fn print_searching_path_conflicts(&self, is_done: bool);

    fn print_documenting_pkg(&self, name: &PkgName, is_done: bool);
    
    fn print_installing_pkg(&self, name: &PkgName, is_done: bool);

    fn print_removing_pkg(&self, name: &PkgName, is_done: bool);

    fn print_cleaning_after_install(&self, is_done: bool);

    fn print_cleaning_before_removal(&self, is_done: bool);

    fn print_cleaning_after_error(&self, is_done: bool);

    fn print_cleaning(&self, is_done: bool);
    
    fn print_nl_for_error(&self);
    
    fn eprint_error(&self, err: &Error);
}

#[derive(Copy, Clone, Debug)]
pub struct EmptyPrinter;

impl EmptyPrinter
{
    pub fn new() -> Self
    { EmptyPrinter }
}

impl Print for EmptyPrinter
{
    fn print_pre_installing(&self)
    {}
    
    fn print_installing(&self)
    {}

    fn print_pre_removing(&self)
    {}

    fn print_removing(&self)
    {}

    fn print_updating_pkg_versions(&self, _name: &PkgName, _is_done: bool)
    {}

    fn print_downloading_pkg_file(&self, _name: &PkgName, _is_done: bool)
    {}

    fn print_downloading_pkg_file_with_progress(&self, _name: &PkgName, _byte_count: f64, _total_byte_count: f64)
    {}
    
    fn print_extracting_pkg_file(&self, _name: &PkgName, _is_done: bool)
    {}
    
    fn print_checking_dependent_version_reqs(&self, _is_done: bool)
    {}

    fn print_searching_path_conflicts(&self, _is_done: bool)
    {}

    fn print_documenting_pkg(&self, _name: &PkgName, _is_done: bool)
    {}
    
    fn print_installing_pkg(&self, _name: &PkgName, _is_done: bool)
    {}

    fn print_removing_pkg(&self, _name: &PkgName, _is_done: bool)
    {}

    fn print_cleaning_after_install(&self, _is_done: bool)
    {}

    fn print_cleaning_before_removal(&self, _is_done: bool)
    {}

    fn print_cleaning_after_error(&self, _is_done: bool)
    {}

    fn print_cleaning(&self, _is_done: bool)
    {}
    
    fn print_nl_for_error(&self)
    {}
    
    fn eprint_error(&self, _err: &Error)
    {}
}

#[derive(Debug)]
pub struct StdPrinter
{
    byte_count: Mutex<f64>,
    has_nl_for_error: AtomicBool,
}

impl StdPrinter
{
    pub fn new() -> Self
    { StdPrinter { byte_count: Mutex::new(0.0), has_nl_for_error: AtomicBool::new(false), } }
}

impl Print for StdPrinter
{
    fn print_pre_installing(&self)
    { println!("Pre-installing:"); }
    
    fn print_installing(&self)
    { println!("Installing:"); }

    fn print_pre_removing(&self)
    { println!("Pre-removing:"); }

    fn print_removing(&self)
    { println!("Removing:"); }

    fn print_updating_pkg_versions(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Updating {} ...", name);
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }
    
    fn print_downloading_pkg_file(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            let byte_count = {
                let byte_count_g = self.byte_count.lock().unwrap();
                *byte_count_g
            };
            println!("  progress: {}KiB (100%)", (byte_count / 1024.0).ceil());
        } else {
            {
                let mut byte_count_g = self.byte_count.lock().unwrap();
                *byte_count_g = 0.0;
            }
            println!("Downloading {} ...", name);
        }
        self.has_nl_for_error.store(false, Ordering::SeqCst);
    }

    fn print_downloading_pkg_file_with_progress(&self, _name: &PkgName, byte_count: f64, total_byte_count: f64)
    {
        if total_byte_count != 0.0 {
            print!("  progress: {}KiB ({}%)\r", (byte_count / 1024.0).ceil(), ((byte_count * 100.0) / total_byte_count).floor());
        } else {
            print!("  progress: {}KiB (?%)\r", (byte_count / 1024.0).ceil());
        }
        let _res = stdout().flush();
        {
            let mut byte_count_g = self.byte_count.lock().unwrap();
            *byte_count_g = byte_count;
        }
        self.has_nl_for_error.store(true, Ordering::SeqCst);
    }
    
    fn print_extracting_pkg_file(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Extracting {} ...", name);
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }
    
    fn print_checking_dependent_version_reqs(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Checking dependent version requirements ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_searching_path_conflicts(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Searching path conflicts ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_documenting_pkg(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            println!("done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Documenting {} ...", name);
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }
    
    fn print_installing_pkg(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Installing {} ...", name);
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_removing_pkg(&self, name: &PkgName, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Removing {} ...", name);
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_cleaning_after_install(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Cleaning after installation ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_cleaning_before_removal(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Cleaning before removal ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }
    
    fn print_cleaning_after_error(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            self.print_nl_for_error();
            print!("Cleaning after error ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }

    fn print_cleaning(&self, is_done: bool)
    {
        if is_done {
            println!(" done");
            self.has_nl_for_error.store(false, Ordering::SeqCst);
        } else {
            print!("Cleaning ...");
            let _res = stdout().flush();
            self.has_nl_for_error.store(true, Ordering::SeqCst);
        }
    }
    
    fn print_nl_for_error(&self)
    {
        if self.has_nl_for_error.swap(false, Ordering::SeqCst) {
            println!("");
        }
    }
    
    fn eprint_error(&self, err: &Error)
    {
        self.print_nl_for_error();
        eprintln!("{}", err);
    }
}

pub trait Source
{
    fn update(&mut self) -> Result<()>;
    
    fn versions(&mut self) -> Result<&BTreeSet<Version>>;
    
    fn set_current_version(&mut self, version: Version);
    
    fn dir(&mut self) -> Result<&Path>;
}

pub trait SourceCreate
{
    fn create(&self, name: PkgName, new_name: Option<PkgName>, home_dir: PathBuf, work_dir: PathBuf, printer: Arc<dyn Print + Send + Sync>) -> Option<Box<dyn Source + Send + Sync>>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PkgName
{
    name: String,
}

impl PkgName
{
    pub fn new(name: String) -> Self
    { PkgName { name, } }

    pub fn parse(s: &str) -> Result<Self>
    {
        if s.split('/').count() < 2 {
            return Err(Error::InvalidPkgName);
        }
        let ss = s.split('/');
        for t in ss {
            if t.is_empty() || t.contains('\\') || t == "." || t == ".." {
                return Err(Error::InvalidPkgName);
            }
        }
        Ok(Self::new(String::from(s)))
    }
    
    pub fn name(&self) -> &str
    { self.name.as_str() }
    
    pub fn to_path_buf(&self) -> PathBuf
    { PathBuf::from(self.name.replace('/', path::MAIN_SEPARATOR_STR)) }
}

impl fmt::Display for PkgName
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}", self.name) }
}

impl Serialize for PkgName
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    { serializer.serialize_str(format!("{}", self).as_str()) }
}

struct PkgNameVisitor;

impl<'de> Visitor<'de> for PkgNameVisitor
{
    type Value = PkgName;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    { write!(formatter, "a package name") }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
        where E: de::Error
    {
        match PkgName::parse(v) {
            Ok(pkg_name) => Ok(pkg_name),
            Err(err) => Err(E::custom(format!("{}", err))),
        }
    }
}

impl<'de> Deserialize<'de> for PkgName
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    { deserializer.deserialize_str(PkgNameVisitor) }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PkgInfo
{
    pub name: PkgName,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VersionSrcInfo
{
    #[serde(rename = "dir")]
    Dir(String),
    #[serde(rename = "file")]
    File(String),
    #[serde(rename = "url")]
    Url(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SrcInfo
{
    #[serde(rename = "renamed")]
    Renamed(PkgName),
    #[serde(rename = "versions")]
    Versions(Arc<BTreeMap<Version, VersionSrcInfo>>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest
{
    pub package: PkgInfo,
    pub dependencies: Option<HashMap<PkgName, VersionReq>>,
    pub constraints: Option<Arc<HashMap<PkgName, VersionReq>>>,
    pub sources: Option<Arc<HashMap<PkgName, SrcInfo>>>,
}

impl Manifest
{
    pub fn new(name: PkgName) -> Self
    {
        Manifest {
            package: PkgInfo {
                name,
                description: None,
                authors: None,
                license: None,
            },
            dependencies: Some(HashMap::new()),
            constraints: None,
            sources: None,
        }
    }
    
    pub fn read(r: &mut dyn Read) -> Result<Self>
    {
        let mut s = String::new();
        match r.read_to_string(&mut s) {
            Ok(_) => {
                match toml::from_str(s.as_str()) {
                    Ok(manifest) => Ok(manifest),
                    Err(err) => Err(Error::TomlDe(err)),
                }
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn write(&self, w: &mut dyn Write) -> Result<()>
    {
        match toml::to_string(self) {
            Ok(s) => {
                match write!(w, "{}", s) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => Err(Error::TomlSer(err)),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn load_opt<P: AsRef<Path>>(path: P) -> Result<Option<Self>>
    {
        match File::open(path) {
            Ok(mut file) => Ok(Some(Self::read(&mut file)?)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None), 
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    {
        match File::create(path) {
            Ok(mut file) => self.write(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paths
{
    bin: Vec<String>,
    lib: Vec<String>,
}

impl Paths
{
    pub fn new(bin: Vec<String>, lib: Vec<String>) -> Self
    { Paths { bin, lib, } }
    
    pub fn read(r: &mut dyn Read) -> Result<Self>
    {
        let mut s = String::new();
        match r.read_to_string(&mut s) {
            Ok(_) => {
                match toml::from_str(s.as_str()) {
                    Ok(paths) => Ok(paths),
                    Err(err) => Err(Error::TomlDe(err)),
                }
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn write(&self, w: &mut dyn Write) -> Result<()>
    {
        match toml::to_string(self) {
            Ok(s) => {
                match write!(w, "{}", s) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => Err(Error::TomlSer(err)),
        }
    }
    
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn load_opt<P: AsRef<Path>>(path: P) -> Result<Option<Self>>
    {
        match File::open(path) {
            Ok(mut file) => Ok(Some(Self::read(&mut file)?)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None), 
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    {
        match File::create(path) {
            Ok(mut file) => self.write(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Versions
{
    versions: BTreeSet<Version>,
}

impl Versions
{
    pub fn new(versions: BTreeSet<Version>) -> Self
    { Versions { versions, } }
    
    pub fn read(r: &mut dyn Read) -> Result<Self>
    {
        let mut s = String::new();
        match r.read_to_string(&mut s) {
            Ok(_) => {
                match toml::from_str(s.as_str()) {
                    Ok(versions) => Ok(versions),
                    Err(err) => Err(Error::TomlDe(err)),
                }
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn write(&self, w: &mut dyn Write) -> Result<()>
    {
        match toml::to_string(self) {
            Ok(s) => {
                match write!(w, "{}", s) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => Err(Error::TomlSer(err)),
        }
    }
    
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn load_opt<P: AsRef<Path>>(path: P) -> Result<Option<Self>>
    {
        match File::open(path) {
            Ok(mut file) => Ok(Some(Self::read(&mut file)?)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None), 
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    {
        match File::create(path) {
            Ok(mut file) => self.write(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PkgConfig
{
    account: Option<String>,
    domain: Option<String>,
}

impl PkgConfig
{
    pub fn new(account: Option<String>, domain: Option<String>) -> Self
    { PkgConfig { account, domain, } }
    
    pub fn read(r: &mut dyn Read) -> Result<Self>
    {
        let mut s = String::new();
        match r.read_to_string(&mut s) {
            Ok(_) => {
                match toml::from_str(s.as_str()) {
                    Ok(config) => Ok(config),
                    Err(err) => Err(Error::TomlDe(err)),
                }
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn write(&self, w: &mut dyn Write) -> Result<()>
    {
        match toml::to_string(self) {
            Ok(s) => {
                match write!(w, "{}", s) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => Err(Error::TomlSer(err)),
        }
    }
    
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn load_opt<P: AsRef<Path>>(path: P) -> Result<Option<Self>>
    {
        match File::open(path) {
            Ok(mut file) => Ok(Some(Self::read(&mut file)?)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None), 
            Err(err) => Err(Error::Io(err)),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    {
        match File::create(path) {
            Ok(mut file) => self.write(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

pub fn read_versions(r: &mut dyn Read) -> Result<HashMap<PkgName, Version>>
{
    let mut s = String::new();
    match r.read_to_string(&mut s) {
        Ok(_) => {
            match toml::from_str::<HashMap<PkgName, Version>>(s.as_str()) {
                Ok(src_infos) => Ok(src_infos),
                Err(err) => Err(Error::TomlDe(err)),
            }
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn write_versions(w: &mut dyn Write, versions: &HashMap<PkgName, Version>) -> Result<()>
{
    match toml::to_string(versions) {
        Ok(s) => {
            match write!(w, "{}", s) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Io(err)),
            }
        },
        Err(err) => Err(Error::TomlSer(err)),
    }
}

pub fn load_versions<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, Version>>
{
    match File::open(path) {
        Ok(mut file) => read_versions(&mut file),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_opt_versions<P: AsRef<Path>>(path: P) -> Result<Option<HashMap<PkgName, Version>>>
{
    match File::open(path) {
        Ok(mut file) => Ok(Some(read_versions(&mut file)?)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_versions_or_empty<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, Version>>
{
    match File::open(path) {
        Ok(mut file) => read_versions(&mut file),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(HashMap::new()),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn save_versions<P: AsRef<Path>>(path: P, versions: &HashMap<PkgName, Version>) -> Result<()>
{
    match File::create(path) {
        Ok(mut file) => write_versions(&mut file, versions),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn read_version_reqs(r: &mut dyn Read) -> Result<HashMap<PkgName, VersionReq>>
{
    let mut s = String::new();
    match r.read_to_string(&mut s) {
        Ok(_) => {
            match toml::from_str::<HashMap<PkgName, VersionReq>>(s.as_str()) {
                Ok(version_reqs) => Ok(version_reqs),
                Err(err) => Err(Error::TomlDe(err)),
            }
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn write_version_reqs(w: &mut dyn Write, version_reqs: &HashMap<PkgName, VersionReq>) -> Result<()>
{
    match toml::to_string(version_reqs) {
        Ok(s) => {
            match write!(w, "{}", s) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Io(err)),
            }
        },
        Err(err) => Err(Error::TomlSer(err)),
    }
}

pub fn load_version_reqs<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, VersionReq>>
{
    match File::open(path) {
        Ok(mut file) => read_version_reqs(&mut file),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_opt_version_reqs<P: AsRef<Path>>(path: P) -> Result<Option<HashMap<PkgName, VersionReq>>>
{
    match File::open(path) {
        Ok(mut file) => Ok(Some(read_version_reqs(&mut file)?)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_version_reqs_or_empty<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, VersionReq>>
{
    match File::open(path) {
        Ok(mut file) => read_version_reqs(&mut file),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(HashMap::new()),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn save_version_reqs<P: AsRef<Path>>(path: P, version_reqs: &HashMap<PkgName, VersionReq>) -> Result<()>
{
    match File::create(path) {
        Ok(mut file) => write_version_reqs(&mut file, version_reqs),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn read_src_infos(r: &mut dyn Read) -> Result<HashMap<PkgName, SrcInfo>>
{
    let mut s = String::new();
    match r.read_to_string(&mut s) {
        Ok(_) => {
            match toml::from_str::<HashMap<PkgName, SrcInfo>>(s.as_str()) {
                Ok(src_infos) => Ok(src_infos),
                Err(err) => Err(Error::TomlDe(err)),
            }
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn write_src_infos(w: &mut dyn Write, src_infos: &HashMap<PkgName, SrcInfo>) -> Result<()>
{
    match toml::to_string(src_infos) {
        Ok(s) => {
            match write!(w, "{}", s) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Io(err)),
            }
        },
        Err(err) => Err(Error::TomlSer(err)),
    }
}

pub fn load_src_infos<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, SrcInfo>>
{
    match File::open(path) {
        Ok(mut file) => read_src_infos(&mut file),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_opt_src_infos<P: AsRef<Path>>(path: P) -> Result<Option<HashMap<PkgName, SrcInfo>>>
{
    match File::open(path) {
        Ok(mut file) => Ok(Some(read_src_infos(&mut file)?)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn load_src_infos_or_empty<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, SrcInfo>>
{
    match File::open(path) {
        Ok(mut file) => read_src_infos(&mut file),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(HashMap::new()),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn save_src_infos<P: AsRef<Path>>(path: P, src_infos: &HashMap<PkgName, SrcInfo>) -> Result<()>
{
    match File::create(path) {
        Ok(mut file) => write_src_infos(&mut file, src_infos),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn index_dir<P: AsRef<Path>>(home_dir: P) -> PathBuf
{
    let mut dir = PathBuf::from(home_dir.as_ref());
    dir.push("index");
    dir
}

pub fn cache_dir<P: AsRef<Path>>(home_dir: P) -> PathBuf
{
    let mut dir = PathBuf::from(home_dir.as_ref());
    dir.push("cache");
    dir
}

pub fn tmp_dir<P: AsRef<Path>>(work_dir: P) -> PathBuf
{
    let mut dir = PathBuf::from(work_dir.as_ref());
    dir.push("tmp");
    dir
}

pub fn pkg_index_dir<P: AsRef<Path>>(home_dir: P, name: &PkgName) -> PathBuf
{
    let mut dir = index_dir(home_dir);
    dir.push(name.to_path_buf());
    dir
}

pub fn pkg_cache_dir<P: AsRef<Path>>(home_dir: P, name: &PkgName, version: &Version) -> PathBuf
{
    let mut dir = cache_dir(home_dir);
    dir.push(name.to_path_buf());
    dir.push(format!("{}", version).as_str());
    dir
}

pub fn pkg_tmp_dir<P: AsRef<Path>>(work_dir: P, name: &PkgName, version: &Version) -> PathBuf
{
    let mut dir = tmp_dir(work_dir);
    dir.push(name.to_path_buf());
    dir.push(format!("{}", version).as_str());
    dir
}

pub fn pkg_part_dir<P: AsRef<Path>>(work_dir: P, name: &PkgName, version: &Version) -> PathBuf
{
    let mut dir = pkg_tmp_dir(work_dir, name, version);
    dir.push("dir.part");
    dir
}

pub fn pkg_dir<P: AsRef<Path>>(work_dir: P, name: &PkgName, version: &Version) -> PathBuf
{
    let mut dir = pkg_tmp_dir(work_dir, name, version);
    dir.push("dir");
    dir
}

fn res_remove_and_rename_for_updated_pkg_versions(new_part_path: &Path, new_path: &Path, path: &Path) -> io::Result<()>
{
    recursively_remove(new_path, true)?;
    rename(new_part_path, new_path)?;
    recursively_remove(path, true)?;
    rename(new_path, path)?;
    Ok(())
}

fn res_remove_and_rename_for_unupdated_pkg_versions(new_part_path: &Path, new_path: &Path, path: &Path) -> io::Result<()>
{
    recursively_remove(new_part_path, true)?;
    match fs::metadata(new_path) {
        Ok(_) => {
            recursively_remove(path, true)?;
            rename(new_path, path)?;
            Ok(())
        },
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()), 
        Err(err) => Err(err),
    }
}

pub fn update_pkg_versions<P: AsRef<Path>, F, G>(name: &PkgName, home_dir: P, is_update: bool, printer: &Arc<dyn Print + Send + Sync>, f: F, g: G) -> Result<BTreeSet<Version>>
    where F: FnOnce() -> result::Result<curl::easy::Easy, curl::Error>,
        G: FnOnce(&[u8]) -> Result<BTreeSet<Version>>
{
    let path_buf = pkg_index_dir(home_dir.as_ref(), name);
    let mut new_part_versions_path_buf = path_buf.clone();
    new_part_versions_path_buf.push("versions.toml.new.part");
    let mut new_versions_path_buf = path_buf.clone();
    new_versions_path_buf.push("versions.toml.new");
    let mut versions_path_buf = path_buf.clone();
    versions_path_buf.push("versions.toml");
    let is_to_update = match fs::metadata(versions_path_buf.as_path()) {
        Ok(_) => is_update,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            match fs::metadata(new_versions_path_buf.as_path()) {
                Ok(_) => is_update,
                Err(err) if err.kind() == ErrorKind::NotFound => true,
                Err(err) => return Err(Error::Io(err)),
            }
        },
        Err(err) => return Err(Error::Io(err)),
    };
    if is_to_update {
        printer.print_updating_pkg_versions(name, false);
        match recursively_remove(new_part_versions_path_buf.as_path(), true) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        let data: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let data2 = data.clone();
        let mut easy = match f() {
            Ok(tmp_easy) => tmp_easy,
            Err(err) => return Err(Error::Curl(err)),
        };
        match easy.fail_on_error(true) {
            Ok(()) => (),
            Err(err) => return Err(Error::Curl(err)),
        }
        match easy.write_function(move |buf| {
                let mut data2_g = data2.lock().unwrap();
                data2_g.extend_from_slice(buf);
                Ok(buf.len())
        }) {
            Ok(()) => (),
            Err(err) => return Err(Error::Curl(err)),
        }
        match easy.perform() {
            Ok(()) => (),
            Err(err) => return Err(Error::Curl(err)),
        }
        let versions = {
            let mut data_g = mutex_lock(&data)?;
            let res = g(data_g.as_slice());
            data_g.clear();
            Versions::new(res?)
        };
        match create_dir_all(path_buf.as_path()) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        versions.save(new_part_versions_path_buf.as_path())?;
        match res_remove_and_rename_for_updated_pkg_versions(new_part_versions_path_buf.as_path(), new_versions_path_buf.as_path(), versions_path_buf.as_path()) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        printer.print_updating_pkg_versions(name, true);
    } else {
        match res_remove_and_rename_for_unupdated_pkg_versions(new_part_versions_path_buf.as_path(), new_versions_path_buf.as_path(), versions_path_buf.as_path()) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
    }
    Ok(Versions::load(versions_path_buf)?.versions)
}

fn res_download_pkg_file(name: &PkgName, url: &str, part_file_path: &Path, printer: &Arc<dyn Print + Send + Sync>) -> result::Result<(), curl::Error>
{
    let mut easy = curl::easy::Easy::new();
    easy.url(url)?;
    easy.follow_location(true)?;
    easy.fail_on_error(true)?;
    easy.progress(true)?;
    let name2 = name.clone();
    let printer2 = printer.clone();
    easy.progress_function(move |total_byte_count, byte_count, _, _| {
            printer2.print_downloading_pkg_file_with_progress(&name2, byte_count, total_byte_count);
            true
    })?;
    let part_file_path_buf = PathBuf::from(part_file_path);
    let printer2 = printer.clone();
    easy.write_function(move |buf| {
            match File::options().create(true).append(true).open(part_file_path_buf.as_path()) {
                Ok(mut file) => {
                    match file.write_all(buf) {
                        Ok(()) => (),
                        Err(err) => printer2.eprint_error(&Error::Io(err)),
                    }
                },
                Err(err) => printer2.eprint_error(&Error::Io(err)),
            }
            Ok(buf.len())
    })?;
    easy.perform()
}

pub fn download_pkg_file<P: AsRef<Path>>(name: &PkgName, version: &Version, url: &str, home_dir: P, printer: &Arc<dyn Print + Send + Sync>) -> Result<PathBuf>
{
    let path_buf = pkg_cache_dir(home_dir.as_ref(), name, version);
    match create_dir_all(path_buf.as_path()) {
        Ok(()) => (),
        Err(err) => return Err(Error::Io(err)),
    }
    let first_url_part = match url.split_once('?') {
        Some((tmp_first_url_part, _)) => tmp_first_url_part,
        None => url,
    };
    let (part_file_name, file_name) = if first_url_part.ends_with(".zip") {
        ("file.zip.part", "file.zip")
    } else if first_url_part.ends_with(".tar.gz") {
        ("file.tar.gz.part", "file.tar.gz")
    } else if first_url_part.ends_with(".tar.bz2") {
        ("file.tar.bz2.part", "file.tar.bz2")
    } else if first_url_part.ends_with(".tar.xz") {
        ("file.tar.xz.part", "file.tar.xz")
    } else if first_url_part.ends_with(".tar") {
        ("file.tar.part", "file.tar")
    } else {
        ("file.part", "file")
    };
    let mut part_file_path_buf = path_buf.clone();
    part_file_path_buf.push(part_file_name);
    let mut file_path_buf = path_buf.clone();
    file_path_buf.push(file_name);
    match fs::metadata(file_path_buf.as_path()) {
        Ok(_) => (),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            printer.print_downloading_pkg_file(name, false);
            match recursively_remove(part_file_path_buf.as_path(), true) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            match res_download_pkg_file(name, url, part_file_path_buf.as_path(), printer) {
                Ok(()) => (),
                Err(err) => return Err(Error::Curl(err)),
            }
            match rename(part_file_path_buf.as_path(), file_path_buf.as_path()) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            printer.print_downloading_pkg_file(name, true);
        },
        Err(err) => return Err(Error::Io(err)),
    }
    Ok(file_path_buf)
}

pub fn extract_pkg_file<P: AsRef<Path>, F>(name: &PkgName, version: &Version, work_dir: P, printer: &Arc<dyn Print + Send + Sync>, f: F) -> Result<PathBuf>
    where F: FnOnce() -> Result<PathBuf>
{
    let part_path_buf = pkg_part_dir(work_dir.as_ref(), name, version);
    let path_buf = pkg_dir(work_dir.as_ref(), name, version);
    match fs::metadata(path_buf.as_path()) {
        Ok(_) => (),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let archive_path_buf = f()?;
            printer.print_extracting_pkg_file(name, false);
            match recursively_remove(part_path_buf.as_path(), true) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            match create_dir_all(part_path_buf.as_path()) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            if archive_path_buf.to_string_lossy().into_owned().ends_with(".zip") {
                match File::open(archive_path_buf) {
                    Ok(mut file) => {
                        let mut br = BufReader::new(&mut file); 
                        let mut archive = match ZipArchive::new(&mut br) {
                            Ok(tmp_archive) => tmp_archive,
                            Err(err) => return Err(Error::Zip(Box::new(err))),
                        };
                        match archive.extract(part_path_buf.as_path()) {
                            Ok(()) => (),
                            Err(err) => return Err(Error::Zip(Box::new(err))),
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                }
            } else if archive_path_buf.to_string_lossy().into_owned().ends_with(".tar.gz") {
                match File::open(archive_path_buf) {
                    Ok(mut file) => {
                        let mut br = BufReader::new(&mut file); 
                        let mut decoder = GzDecoder::new(&mut br);
                        let mut archive = tar::Archive::new(&mut decoder);
                        match archive.unpack(part_path_buf.as_path()) {
                            Ok(()) => (),
                            Err(err) => return Err(Error::Io(err)),
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                }
            } else if archive_path_buf.to_string_lossy().into_owned().ends_with(".tar.bz2") {
                match File::open(archive_path_buf) {
                    Ok(mut file) => {
                        let mut br = BufReader::new(&mut file); 
                        let mut decoder = BzDecoder::new(&mut br);
                        let mut archive = tar::Archive::new(&mut decoder);
                        match archive.unpack(part_path_buf.as_path()) {
                            Ok(()) => (),
                            Err(err) => return Err(Error::Io(err)),
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                }
            } else if archive_path_buf.to_string_lossy().into_owned().ends_with(".tar.xz") {
                match File::open(archive_path_buf) {
                    Ok(mut file) => {
                        let mut br = BufReader::new(&mut file); 
                        let mut decoder = XzDecoder::new(&mut br);
                        let mut archive = tar::Archive::new(&mut decoder);
                        match archive.unpack(part_path_buf.as_path()) {
                            Ok(()) => (),
                            Err(err) => return Err(Error::Io(err)),
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                }
            } else {
                match File::open(archive_path_buf) {
                    Ok(mut file) => {
                        let mut br = BufReader::new(&mut file); 
                        let mut archive = tar::Archive::new(&mut br);
                        match archive.unpack(part_path_buf.as_path()) {
                            Ok(()) => (),
                            Err(err) => return Err(Error::Io(err)),
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                }
            }
            match rename(part_path_buf.as_path(), path_buf.as_path()) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            printer.print_extracting_pkg_file(name, true);
        },
        Err(err) => return Err(Error::Io(err)),
    }
    match only_one_dir_in_dir(path_buf.as_path()) {
        Ok(Some(only_one_dir)) => Ok(only_one_dir),
        Ok(None) => Ok(path_buf),
        Err(err) => Err(Error::Io(err)),
    }
}

#[derive(Clone)]
pub struct CustomSrc
{
    name: PkgName,
    home_dir: PathBuf,
    work_dir: PathBuf,
    version_src_infos: Arc<BTreeMap<Version, VersionSrcInfo>>,
    versions: Arc<BTreeSet<Version>>,
    printer: Arc<dyn Print + Send + Sync>,
    current_version: Option<Version>,
    dir: Option<PathBuf>,
}

impl CustomSrc
{
    pub fn new(name: PkgName, home_dir: PathBuf, work_dir: PathBuf, version_src_infos: Arc<BTreeMap<Version, VersionSrcInfo>>, printer: Arc<dyn Print + Send + Sync>) -> Self
    {
        let versions: Arc<BTreeSet<Version>> = Arc::new(version_src_infos.keys().map(|v| v.clone()).collect()); 
        CustomSrc {
            name,
            home_dir,
            work_dir,
            version_src_infos,
            printer,
            versions,
            current_version: None,
            dir: None,
        }
    }
    
    pub fn name(&self) -> &PkgName
    { &self.name }

    pub fn home_dir(&self) -> &Path
    { self.home_dir.as_path() }

    pub fn work_dir(&self) -> &Path
    { self.work_dir.as_path() }

    pub fn version_src_infos(&self) -> &Arc<BTreeMap<Version, VersionSrcInfo>>
    { &self.version_src_infos }

    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }

    pub fn current_version(&self) -> Option<&Version>
    { 
        match &self.current_version {
            Some(current_version) => Some(current_version),
            None => None,
        }
    }
}

impl Source for CustomSrc
{
    fn update(&mut self) -> Result<()>
    { Ok(()) }
    
    fn versions(&mut self) -> Result<&BTreeSet<Version>>
    { Ok(&self.versions) }
    
    fn set_current_version(&mut self, version: Version)
    { self.current_version = Some(version); }
    
    fn dir(&mut self) -> Result<&Path>
    {
        let dir = if self.dir.is_none() {
            match &self.current_version {
                Some(current_version) => {
                    match self.version_src_infos.get(current_version) {
                        Some(version_src_info) => {
                            match version_src_info {
                                VersionSrcInfo::Dir(tmp_dir) => Some(PathBuf::from(tmp_dir.replace('/', path::MAIN_SEPARATOR_STR))),
                                VersionSrcInfo::File(file) => Some(extract_pkg_file(&self.name, current_version, &self.work_dir, &self.printer, || Ok(PathBuf::from(file.replace('/', path::MAIN_SEPARATOR_STR))))?),
                                VersionSrcInfo::Url(url) => {
                                    Some(extract_pkg_file(&self.name, current_version, &self.work_dir, &self.printer, || {
                                            download_pkg_file(&self.name, current_version, url, &self.home_dir, &self.printer)
                                    })?)
                                },
                            }
                        },
                        None => return Err(Error::PkgName(self.name.clone(), String::from("not found version"))),
                    }
                },
                None => return Err(Error::PkgName(self.name.clone(), String::from("no current version"))),
            }
        } else {
            None
        };
        if dir.is_some() {
            self.dir = dir;
        }
        Ok(self.dir.as_ref().unwrap().as_path())
    }
}

#[derive(Clone, Debug)]
pub struct Pkg
{
    dir: Option<PathBuf>,
    info_dir: Option<PathBuf>,
    new_part_info_dir: Option<PathBuf>,
    is_added_by_dependent: bool,
    has_new_version_from_bucket: bool,
}

impl Pkg
{
    fn new() -> Self
    {
        Pkg {
            dir: Some(PathBuf::from(".")),
            info_dir: None,
            new_part_info_dir: None,
            is_added_by_dependent: false,
            has_new_version_from_bucket: true,
        }
    }

    fn res_copy_info_files(dir: &Option<PathBuf>, info_dir: &PathBuf, new_part_info_dir: &PathBuf) -> io::Result<()>
    {
        create_dir_all(new_part_info_dir)?;
        match dir {
            Some(dir) => {
                let mut src_manifest_file = dir.clone();
                src_manifest_file.push("Unlab.toml");
                let mut dst_manifest_file = new_part_info_dir.clone();
                dst_manifest_file.push("manifest.toml");
                copy(src_manifest_file, dst_manifest_file)?;
            },
            None => (),
        }
        let mut src_dependents_file = info_dir.clone();
        src_dependents_file.push("dependents.toml");
        let mut dst_dependents_file = new_part_info_dir.clone();
        dst_dependents_file.push("dependents.toml");
        match fs::metadata(dst_dependents_file.as_path()) {
            Ok(_) => (),
            Err(err) if err.kind() == ErrorKind::NotFound => {
                match fs::metadata(info_dir) {
                    Ok(_) => {
                        copy(src_dependents_file, dst_dependents_file)?;
                    },
                    Err(err) if err.kind() == ErrorKind::NotFound => fs::write(dst_dependents_file, "\n")?,
                    Err(err) => return Err(err),
                }
            },
            Err(err) => return Err(err),
        }
        Ok(())
    }

    fn copy_info_files(dir: &Option<PathBuf>, info_dir: &PathBuf, new_part_info_dir: &PathBuf) -> Result<()>
    {
        match Self::res_copy_info_files(dir, info_dir, new_part_info_dir) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::Io(err)),
        }
    }
    
    fn new_with_copying_and_flags(dir: Option<PathBuf>, info_dir: PathBuf, new_part_info_dir: PathBuf, is_added_by_dependent: bool, is_new_version_from_bucket: bool) -> Result<Self>
    {
        Self::copy_info_files(&dir, &info_dir, &new_part_info_dir)?;
        Ok(Pkg {
                dir,
                info_dir: Some(info_dir),
                new_part_info_dir: Some(new_part_info_dir),
                is_added_by_dependent,
                has_new_version_from_bucket: is_new_version_from_bucket,
        })
    }

    fn new_with_copying(dir: Option<PathBuf>, info_dir: PathBuf, new_part_info_dir: PathBuf) -> Result<Self>
    { Self::new_with_copying_and_flags(dir, info_dir, new_part_info_dir, false, true) }

    fn new_without_copying_with_flags(info_dir: PathBuf, is_added_by_dependent: bool, is_new_version_from_bucket: bool) -> Self
    {
        Pkg {
            dir: None,
            info_dir: Some(info_dir),
            new_part_info_dir: None,
            is_added_by_dependent,
            has_new_version_from_bucket: is_new_version_from_bucket,
        }
    }

    fn new_without_copying(info_dir: PathBuf) -> Self
    { Self::new_without_copying_with_flags(info_dir, false, true) }
    
    fn old_manifest(&self) -> Result<Option<Manifest>>
    {
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                match &self.info_dir {
                    Some(info_dir) => {
                        let mut new_manifest_file = new_part_info_dir.clone();
                        new_manifest_file.push("manifest.toml");
                        let is_new_manifest = match fs::metadata(new_manifest_file) {
                            Ok(_) => true,
                            Err(err) if err.kind() == ErrorKind::NotFound => false,
                            Err(err) => return Err(Error::Io(err)),
                        };
                        if is_new_manifest {
                            let mut old_manifest_file = info_dir.clone();
                            old_manifest_file.push("manifest.toml");
                            match Manifest::load(old_manifest_file) {
                                Ok(tmp_old_manifest) => Ok(Some(tmp_old_manifest)),
                                Err(Error::Io(io_err)) if io_err.kind() == ErrorKind::NotFound => Ok(None),
                                Err(err) => Err(err),
                            }
                        } else {
                            Ok(None)
                        }
                    },
                    None => Ok(None),
                }
            },
            None => Ok(None),
        }
    }

    fn manifest(&self) -> Result<Manifest>
    {
        let mut manifest = match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut manifest_file = new_part_info_dir.clone();
                manifest_file.push("manifest.toml");
                Manifest::load_opt(manifest_file)?
            },
            None => None
        };
        if manifest.is_none() {
            manifest = match &self.info_dir {
                Some(info_dir) => {
                    let mut manifest_file = info_dir.clone();
                    manifest_file.push("manifest.toml");
                    Manifest::load_opt(manifest_file)?
                },
                None => None,
            };
        }
        if manifest.is_none() {
            match &self.dir {
                Some(dir) => {
                    let mut manifest_file = dir.clone();
                    manifest_file.push("Unlab.toml");
                    match Manifest::load_opt(manifest_file)? {
                        Some(manifest) => Ok(manifest),
                        None => Err(Error::Pkg(String::from("no manifest file"))),
                    }
                },
                None => Err(Error::Pkg(String::from("no manifest file"))),
            }
        } else {
            match manifest {
                Some(manifest) => Ok(manifest),
                None => Err(Error::Pkg(String::from("no manifest file"))),
            }
        }
    }

    fn dependents(&self) -> Result<HashMap<PkgName, VersionReq>>
    {
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut dependents_file = new_part_info_dir.clone();
                dependents_file.push("dependents.toml");
                load_version_reqs_or_empty(dependents_file)
            },
            None => Ok(HashMap::new()),
        }
    }

    fn save_dependents(&self, dependents: &HashMap<PkgName, VersionReq>) -> Result<()>
    {
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut dependents_file = new_part_info_dir.clone();
                dependents_file.push("dependents.toml");
                save_version_reqs(dependents_file, dependents)
            },
            None => Ok(()),
        }
    }

    fn is_to_install(&self) -> Result<bool>
    {
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut manifest_file = new_part_info_dir.clone();
                manifest_file.push("manifest.toml");
                match fs::metadata(manifest_file) {
                    Ok(_) => Ok(true),
                    Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            None => Ok(false),
        }
    }
}

fn db_tx(db: &DB, writable: bool) -> Result<Tx<'_>>
{
    match db.tx(writable) {
        Ok(tx) => Ok(tx),
        Err(err) => Err(Error::Jammdb(Box::new(err))),
    }
}

fn tx_get_or_create_bucket<'b, 'tx, T: ToBytes<'tx>>(tx: &'b Tx<'tx>, name: T) -> Result<Bucket<'b, 'tx>>
{
    match tx.get_or_create_bucket(name) {
        Ok(bucket) => Ok(bucket),
        Err(err) => Err(Error::Jammdb(Box::new(err))),
    }
}

fn tx_delete_bucket<'tx, T: ToBytes<'tx>>(tx: &Tx<'tx>, name: T) -> Result<()>
{
    match tx.delete_bucket(name) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Jammdb(Box::new(err))),
    }
}

fn tx_commit<'tx>(tx: Tx<'tx>) -> Result<()>
{
    match tx.commit() {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Jammdb(Box::new(err))),
    }
}

fn bucket_put<'a, 'b, 'tx, T: ToBytes<'tx>, S: ToBytes<'tx>>(bucket: &'a Bucket<'b, 'tx>, key: T, value: S) -> Result<Option<KVPair<'b, 'tx>>>
{
    match bucket.put(key, value) {
        Ok(kv_pair) => Ok(kv_pair),
        Err(err) => Err(Error::Jammdb(Box::new(err))),
    }
}

#[derive(Clone)]
pub struct PkgManager
{
    pkg_db: DB,
    home_dir: PathBuf,
    work_dir: PathBuf,
    bin_dir: PathBuf,
    lib_dir: PathBuf,
    doc_dir: PathBuf,
    pkgs: HashMap<PkgName, Pkg>,
    locks: HashMap<PkgName, Version>,
    constraints: Arc<HashMap<PkgName, VersionReq>>,
    sources: Arc<HashMap<PkgName, SrcInfo>>,
    src_factories: Vec<Arc<dyn SourceCreate + Send + Sync>>,
    printer: Arc<dyn Print + Send + Sync>,
}

impl PkgManager
{
    pub fn new(home_dir: PathBuf, work_dir: PathBuf, bin_dir: PathBuf, lib_dir: PathBuf, doc_dir: PathBuf, src_factories: Vec<Arc<dyn SourceCreate + Send + Sync>>, printer: Arc<dyn Print + Send + Sync>) -> Result<Self>
    {
        let mut work_var_dir = work_dir.clone();
        work_var_dir.push("var");
        match create_dir_all(work_var_dir.as_path()) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        let mut pkg_db_file = work_var_dir.clone();
        pkg_db_file.push("pkg.db");
        let pkg_db = match DB::open(pkg_db_file) {
            Ok(tmp_pkg_db) => tmp_pkg_db,
            Err(err) => return Err(Error::Jammdb(Box::new(err))),
        };
        Ok(PkgManager {
                pkg_db,
                home_dir,
                work_dir,
                bin_dir,
                lib_dir,
                doc_dir,
                pkgs: HashMap::new(),
                locks: HashMap::new(),
                constraints: Arc::new(HashMap::new()),
                sources: Arc::new(HashMap::new()),
                src_factories,
                printer,
        })
    }
    
    pub fn home_dir(&self) -> &Path
    { self.home_dir.as_path() }

    pub fn work_dir(&self) -> &Path
    { self.work_dir.as_path() }
    
    pub fn bin_dir(&self) -> &Path
    { self.bin_dir.as_path() }

    pub fn lib_dir(&self) -> &Path
    { self.lib_dir.as_path() }
    
    pub fn doc_dir(&self) -> &Path
    { self.doc_dir.as_path() }

    pub fn locks(&self) -> &HashMap<PkgName, Version>
    { &self.locks }

    pub fn set_locks(&mut self, locks: HashMap<PkgName, Version>)
    { self.locks = locks; }

    pub fn load_locks(&mut self) -> Result<()>
    {
        self.locks = load_versions_or_empty("Unlab.lock")?;
        Ok(())
    }

    pub fn save_locks(&self) -> Result<()>
    { save_versions("Unlab.lock", &self.locks) }
    
    pub fn save_locks_from_pkg_versions(&self) -> Result<()>
    {
        let mut locks: HashMap<PkgName, Version> = HashMap::new();
        self.pkg_versions_for_bucket_in("versions", |name, version| {
                locks.insert(name.clone(), version.clone());
                Ok(())
        })?;
        save_versions("Unlab.lock", &locks)
    }
    
    pub fn constraints(&self) -> &Arc<HashMap<PkgName, VersionReq>>
    { &self.constraints }

    pub fn set_constraints(&mut self, constraints: Arc<HashMap<PkgName, VersionReq>>)
    { self.constraints = constraints; }

    pub fn load_constraints(&mut self) -> Result<()>
    {
        self.constraints = Arc::new(load_version_reqs_or_empty(self.constraints_file())?);
        Ok(())
    }
    
    pub fn sources(&self) -> &Arc<HashMap<PkgName, SrcInfo>>
    { &self.sources }

    pub fn set_sources(&mut self, sources: Arc<HashMap<PkgName, SrcInfo>>)
    { self.sources = sources; }

    pub fn load_sources(&mut self) -> Result<()>
    {
        self.sources = Arc::new(load_src_infos_or_empty(self.sources_file())?);
        Ok(())
    }

    pub fn src_factories(&self) -> &[Arc<dyn SourceCreate + Send + Sync>]
    { self.src_factories.as_slice() }
    
    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }
    
    pub fn manifest() -> Result<Manifest>
    { Manifest::load("Unlab.toml") }

    pub fn save_manifest(manifest: &Manifest) -> Result<()>
    { manifest.save("Unlab.toml") }

    pub fn pkg_config(&self) -> Result<Option<PkgConfig>>
    { PkgConfig::load_opt(self.pkg_config_file()) }
    
    pub fn save_pkg_config(&self, config: &PkgConfig) -> Result<()>
    { config.save(self.pkg_config_file()) }

    pub fn reset(&mut self)
    { self.pkgs.clear(); }
    
    pub fn pkg_config_file(&self) -> PathBuf
    {
        let mut file = self.home_dir.clone();
        file.push("pkg.toml");
        file
    }

    pub fn constraints_file(&self) -> PathBuf
    {
        let mut file = self.home_dir.clone();
        file.push("constraints.toml");
        file
    }

    pub fn sources_file(&self) -> PathBuf
    {
        let mut file = self.home_dir.clone();
        file.push("sources.toml");
        file
    }
    
    pub fn home_var_dir(&self) -> PathBuf
    {
        let mut dir = self.home_dir.clone();
        dir.push("var");
        dir
    }

    pub fn work_var_dir(&self) -> PathBuf
    {
        let mut dir = self.work_dir.clone();
        dir.push("var");
        dir
    }    

    pub fn work_tmp_dir(&self) -> PathBuf
    {
        let mut dir = self.work_dir.clone();
        dir.push("tmp");
        dir
    }    
    
    pub fn info_dir(&self) -> PathBuf
    {
        let mut dir = self.work_var_dir();
        dir.push("info");
        dir
    }

    pub fn new_part_info_dir(&self) -> PathBuf
    {
        let mut dir = self.work_var_dir();
        dir.push("info.new.part");
        dir
    }
    
    pub fn new_info_dir(&self) -> PathBuf
    {
        let mut dir = self.work_var_dir();
        dir.push("info.new");
        dir
    }

    pub fn pkg_info_dir(&self, name: &PkgName) -> PathBuf
    {
        let mut dir = self.info_dir();
        dir.push(name.to_path_buf());
        dir
    }

    pub fn pkg_new_part_info_dir(&self, name: &PkgName) -> PathBuf
    {
        let mut dir = self.new_part_info_dir();
        dir.push(name.to_path_buf());
        dir
    }
    
    pub fn pkg_new_info_dir(&self, name: &PkgName) -> PathBuf
    {
        let mut dir = self.new_info_dir();
        dir.push(name.to_path_buf());
        dir
    }
    
    pub fn pkg_tmp_doc_dir(&self, name: &PkgName, version: &Version) -> PathBuf
    {
        let mut dir = self.work_tmp_dir();
        dir.push(name.to_path_buf());
        dir.push(format!("{}", version));
        dir.push("doc");
        dir
    }
    
    pub fn create_source(&self, name: &PkgName) -> Result<Box<dyn Source + Send + Sync>>
    {
        match self.sources.get(name) {
            Some(src_info) => {
                match src_info {
                    SrcInfo::Renamed(old_name) => {
                        for src_factory in &self.src_factories {
                            match src_factory.create(old_name.clone(), Some(name.clone()), self.home_dir.clone(), self.work_dir.clone(), self.printer.clone()) {
                                Some(src) => return Ok(src),
                                None => (),
                            }
                        }
                        Err(Error::PkgName(name.clone(), String::from("unrecognized source for renamed package")))
                    },
                    SrcInfo::Versions(version_src_infos) => Ok(Box::new(CustomSrc::new(name.clone(), self.home_dir.clone(), self.work_dir.clone(), version_src_infos.clone(), self.printer.clone()))),
                }
            },
            None => {
                for src_factory in &self.src_factories {
                    match src_factory.create(name.clone(), None, self.home_dir.clone(), self.work_dir.clone(), self.printer.clone()) {
                        Some(src) => return Ok(src),
                        None => (),
                    }
                }
                Err(Error::PkgName(name.clone(), String::from("unrecognized source for package")))
            },
        }
    }
    
    fn has_bucket(&self, bucket_name: &str) -> Result<bool>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(_) => Ok(true),
            Err(jammdb::Error::BucketMissing) => Ok(false),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }

    fn remove_bucket(&self, bucket_name: &str) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, true)?;
        match tx.delete_bucket(bucket_name) {
            Ok(()) => (),
            Err(jammdb::Error::BucketMissing) => (),
            Err(err) => return Err(Error::Jammdb(Box::new(err))),
        }
        tx_commit(tx)?;
        Ok(())
    }
    
    fn pkg_versions_for_bucket(&self, bucket_name: &str) -> Result<Vec<(PkgName, Version)>>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(version_bucket) => {
                let mut pairs: Vec<(PkgName, Version)> = Vec::new();
                for data in version_bucket.cursor() {
                    let name = match String::from_utf8(data.kv().key().to_vec()) {
                        Ok(s) => PkgName::parse(s.as_str())?,
                        Err(_) => return Err(Error::Pkg(format!("invalid package name data"))),
                    };
                    let version = match String::from_utf8(data.kv().value().to_vec()) {
                        Ok(s) => Version::parse(s.as_str())?,
                        Err(_) => return Err(Error::Pkg(format!("invalid version data"))),
                    };
                    pairs.push((name, version));
                }
                Ok(pairs)
            },
            Err(jammdb::Error::BucketMissing) => Ok(Vec::new()),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }

    fn pkg_versions_for_bucket_in<F>(&self, bucket_name: &str, mut f: F) -> Result<()>
        where F: FnMut(&PkgName, &Version) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(version_bucket) => {
                for data in version_bucket.cursor() {
                    let name = match String::from_utf8(data.kv().key().to_vec()) {
                        Ok(s) => PkgName::parse(s.as_str())?,
                        Err(_) => return Err(Error::Pkg(format!("invalid package name data"))),
                    };
                    let version = match String::from_utf8(data.kv().value().to_vec()) {
                        Ok(s) => Version::parse(s.as_str())?,
                        Err(_) => return Err(Error::Pkg(format!("invalid version data"))),
                    };
                    f(&name, &version)?;
                }
                Ok(())
            },
            Err(jammdb::Error::BucketMissing) => Ok(()),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }
    
    fn pkg_version_for_bucket(&self, bucket_name: &str, name: &PkgName) -> Result<Option<Version>>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(version_bucket) => {
                match version_bucket.get(name.name()) {
                    Some(data) => {
                        match String::from_utf8(data.kv().value().to_vec()) {
                            Ok(s) => Ok(Some(Version::parse(s.as_str())?)),
                            Err(_) => Err(Error::Pkg(format!("invalid version data"))),
                        }
                    },
                    None => Ok(None),
                }
            },
            Err(jammdb::Error::BucketMissing) => Ok(None),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }

    fn add_pkg_version_for_bucket(&self, bucket_name: &str, name: &PkgName, version: &Version) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, true)?;
        let version_bucket = tx_get_or_create_bucket(&tx, bucket_name)?;
        bucket_put(&version_bucket, name.name(), format!("{}", version))?;
        tx_commit(tx)?;
        Ok(())
    }
    
    fn move_pkg_versions_for_buckets(&self, src_bucket_name: &str, dst_bucket_name: &str) -> Result<()>
    { 
        let tx = db_tx(&self.pkg_db, true)?;
        {
            let src_version_bucket = match tx.get_bucket(src_bucket_name) {
                Ok(tmp_src_version_bucket) => tmp_src_version_bucket,
                Err(jammdb::Error::BucketMissing) => return Ok(()),
                Err(err) => return Err(Error::Jammdb(Box::new(err))),
            };
            let dst_version_bucket = tx_get_or_create_bucket(&tx, dst_bucket_name)?;
            for data in src_version_bucket.cursor() {
                bucket_put(&dst_version_bucket, data.kv().key().to_vec(), data.kv().value().to_vec())?;
            }
        }
        tx_delete_bucket(&tx, src_bucket_name)?;
        tx_commit(tx)?;
        Ok(())
    }

    fn pkg_names_for_bucket(&self, bucket_name: &str) -> Result<Vec<PkgName>>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(version_bucket) => {
                let mut names: Vec<PkgName> = Vec::new();
                for data in version_bucket.cursor() {
                    let name = match String::from_utf8(data.kv().key().to_vec()) {
                        Ok(s) => PkgName::parse(s.as_str())?,
                        Err(_) => return Err(Error::Pkg(format!("invalid package name data"))),
                    };
                    names.push(name);
                }
                Ok(names)
            },
            Err(jammdb::Error::BucketMissing) => Ok(Vec::new()),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }

    fn has_pkg_names_for_bucket(&self, bucket_name: &str, name: &PkgName) -> Result<bool>
    {
        let tx = db_tx(&self.pkg_db, false)?;
        match tx.get_bucket(bucket_name) {
            Ok(name_bucket) => {
                match name_bucket.get(name.name()) {
                    Some(_) => Ok(true),
                    None => Ok(false),
                }
            },
            Err(jammdb::Error::BucketMissing) => Ok(false),
            Err(err) => Err(Error::Jammdb(Box::new(err))),
        }
    }
    
    fn add_pkg_names_for_bucket(&self, bucket_name: &str, name: &PkgName) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, true)?;
        let name_bucket = tx_get_or_create_bucket(&tx, bucket_name)?;
        bucket_put(&name_bucket, name.name(), "t")?;
        tx_commit(tx)?;
        Ok(())
    }

    fn add_pkg_names_for_bucket_and_removing(&self, bucket_name: &str, names: &[PkgName]) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, true)?;
        let name_bucket = tx_get_or_create_bucket(&tx, bucket_name)?;
        for name in names {
            let mut dependents_file = self.pkg_info_dir(&name);
            dependents_file.push("dependents.toml");
            let dependents = load_opt_version_reqs(dependents_file)?;
            match dependents {
                Some(dependents) => {
                    if dependents.is_empty() {
                        bucket_put(&name_bucket, name.name(), "t")?;
                    } else {
                        return Err(Error::PkgName(name.clone(), String::from("can't remove package")));
                    }
                },
                None => return Err(Error::PkgName(name.clone(), String::from("package isn't installed"))),
            }
        }
        tx_commit(tx)?;
        Ok(())
    }    

    fn add_pkg_names_for_buckets_and_autoremoving(&self, bucket_name: &str, version_bucket_name: &str, visiteds: &HashSet<PkgName>) -> Result<()>
    {
        let tx = db_tx(&self.pkg_db, true)?;
        {
            let version_bucket = match tx.get_bucket(version_bucket_name) {
                Ok(tmp_version_bucket) => tmp_version_bucket,
                Err(jammdb::Error::BucketMissing) => return Ok(()),
                Err(err) => return Err(Error::Jammdb(Box::new(err))),
            };
            let name_bucket = tx_get_or_create_bucket(&tx, bucket_name)?;
            for data in version_bucket.cursor() {
                let name = match String::from_utf8(data.kv().key().to_vec()) {
                    Ok(s) => PkgName::parse(s.as_str())?,
                    Err(_) => return Err(Error::Pkg(format!("invalid package name data"))),
                };
                if !visiteds.contains(&name) {
                    bucket_put(&name_bucket, data.kv().key().to_vec(), "t")?;
                }
            }
        }
        tx_commit(tx)?;
        Ok(())
    }    
    
    fn remove_pkg_versions_for_buckets(&self, removal_bucket_name: &str, bucket_name: &str) -> Result<()>
    { 
        let tx = db_tx(&self.pkg_db, true)?;
        {
            let removal_bucket = match tx.get_bucket(removal_bucket_name) {
                Ok(tmp_removal_bucket) => tmp_removal_bucket,
                Err(jammdb::Error::BucketMissing) => return Ok(()),
                Err(err) => return Err(Error::Jammdb(Box::new(err))),
            };
            let version_bucket = tx_get_or_create_bucket(&tx, bucket_name)?;
            for data in removal_bucket.cursor() {
                match version_bucket.delete(data.kv().key()) {
                    Ok(_) => (),
                    Err(err) => return Err(Error::Jammdb(Box::new(err))),
                }
            }
        }
        tx_delete_bucket(&tx, removal_bucket_name)?;
        tx_commit(tx)?;
        Ok(())
    }
    
    pub fn pkg_versions(&self) -> Result<Vec<(PkgName, Version)>>
    { self.pkg_versions_for_bucket("versions") }

    pub fn pkg_versions_in<F>(&self, f: F) -> Result<()>
        where F: FnMut(&PkgName, &Version) -> Result<()>
    { self.pkg_versions_for_bucket_in("versions", f) }

    pub fn pkg_version(&self, name: &PkgName) -> Result<Option<Version>>
    { self.pkg_version_for_bucket("versions", name) }
    
    pub fn pkg_manifest(&self, name: &PkgName) -> Result<Option<Manifest>>
    {
        let mut manifest_file = self.pkg_info_dir(name);
        manifest_file.push("manifest.toml");
        Manifest::load_opt(manifest_file)
    }

    pub fn pkg_dependents(&self, name: &PkgName) -> Result<Option<HashMap<PkgName, VersionReq>>>
    {
        let mut dependents_file = self.pkg_info_dir(name);
        dependents_file.push("dependents.toml");
        load_opt_version_reqs(dependents_file)
    }

    pub fn pkg_paths(&self, name: &PkgName) -> Result<Option<Paths>>
    {
        let mut paths_file = self.pkg_info_dir(name);
        paths_file.push("paths.toml");
        Paths::load_opt(paths_file)
    }    
    
    fn max_pkg_version(versions: &BTreeSet<Version>, version_req: Option<&VersionReq>, constraint: Option<&VersionReq>, locked_version: Option<&Version>) -> Option<Version>
    {
        let mut version_reqs: Vec<&VersionReq> = Vec::new();
        match version_req {
            Some(version_req) => version_reqs.push(version_req),
            None => (),
        }
        match constraint {
            Some(constraint) => version_reqs.push(constraint),
            None => (),
        }
        let mut max_version: Option<Version> = None;
        for version in versions {
            if version_reqs.iter().all(|r| r.matches(version)) {
                if locked_version.map(|lv| lv == version).unwrap_or(true) { 
                    max_version = Some(version.clone());
                }
            }
        }
        max_version
    }
    
    fn res_remove_dirs_for_cleaning(&self) -> io::Result<()>
    {
        recursively_remove(self.work_tmp_dir(), true)?;
        recursively_remove(self.new_part_info_dir(), true)?;
        Ok(())
    }
    
    fn clean_after_error(&self) -> Result<()>
    {
        self.printer.print_cleaning_after_error(false);
        self.remove_bucket("new_versions")?;
        self.remove_bucket("pkgs_to_remove")?;
        self.remove_bucket("pkgs_to_change")?;
        match self.res_remove_dirs_for_cleaning() {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        self.printer.print_cleaning_after_error(true);
        Ok(())
    }

    fn prepare_new_part_infos_for_pre_installing_without_reset(&mut self, name: &PkgName, visiteds: &mut HashSet<PkgName>, is_update: bool, is_force: bool) -> Result<()>
    {
        if visiteds.contains(name) {
            return Ok(());
        }
        let res = dfs(name, visiteds, self, |name, data| {
                let pkg = match data.pkgs.get(name) {
                    Some(tmp_pkg) if !tmp_pkg.is_added_by_dependent => tmp_pkg.clone(),
                    _ => {
                        let mut src = data.create_source(name)?;
                        let old_version = data.pkg_version_for_bucket("versions", name)?;
                        let is_new_version_from_bucket = match data.pkgs.get_mut(name) {
                            Some(tmp_pkg) => {
                                let tmp_is_new_version_from_bucket = tmp_pkg.has_new_version_from_bucket;
                                tmp_pkg.has_new_version_from_bucket = true;
                                tmp_is_new_version_from_bucket
                            },
                            None => true,
                        };
                        let new_version_from_bucket = if is_new_version_from_bucket {
                            data.pkg_version_for_bucket("new_versions", name)?
                        } else {
                            None
                        };
                        let new_version = match &new_version_from_bucket {
                            Some(tmp_new_version) => Some(tmp_new_version.clone()),
                            None => {
                                if is_update {
                                    src.update()?;
                                }
                                let versions = src.versions()?;
                                let old_dependants = if old_version.is_some() {
                                    let mut old_dependents_file = data.pkg_info_dir(name);
                                    old_dependents_file.push("dependents.toml");
                                    load_version_reqs(old_dependents_file)?
                                } else {
                                    HashMap::new()
                                };
                                let mut tmp_new_version: Option<Version> = None; 
                                for old_version_req in old_dependants.values() {
                                    let max_version = Self::max_pkg_version(&versions, Some(old_version_req), data.constraints.get(name), data.locks.get(name));
                                    match &max_version {
                                        Some(max_version) => {
                                            match &tmp_new_version {
                                                Some(tmp_new_version) => {
                                                    if tmp_new_version != max_version {
                                                        return Err(Error::PkgName(name.clone(), format!("version requirements indicate two different package versions: {}, {}", tmp_new_version, max_version)));
                                                    }
                                                },
                                                None => tmp_new_version = Some(max_version.clone()),
                                            }
                                        },
                                        None => return Err(Error::PkgName(name.clone(), String::from("each package version isn't matched to version requirement"))),
                                    }
                                }
                                match tmp_new_version {
                                    Some(tmp_new_version) => Some(tmp_new_version),
                                    None => Self::max_pkg_version(&versions, None, data.constraints.get(name), data.locks.get(name)),
                                }
                            },
                        };
                        match &new_version {
                            Some(new_version) => {
                                src.set_current_version(new_version.clone());
                                if new_version_from_bucket.is_none() {
                                    data.add_pkg_version_for_bucket("new_versions", name, &new_version)?;
                                }
                                let dir = if is_force || old_version.as_ref().map(|ov| ov != new_version).unwrap_or(true) {
                                    Some(PathBuf::from(src.dir()?))
                                } else {
                                    None
                                };
                                data.pkgs.insert(name.clone(), Pkg::new_with_copying(dir, data.pkg_info_dir(name), data.pkg_new_part_info_dir(name))?);
                                data.pkgs.get(name).unwrap().clone()
                            },
                            None => return Err(Error::PkgName(name.clone(), String::from("each package version isn't matched to version requirement"))),
                        }
                    },
                };
                let manifest = pkg.manifest()?;
                match &manifest.dependencies {
                    Some(deps) => {
                        for (dep_name, dep_version_req) in deps {
                            let mut dep_src = data.create_source(dep_name)?;
                            if is_update {
                                dep_src.update()?;
                            }
                            let versions = dep_src.versions()?;
                            let max_version = Self::max_pkg_version(&versions, Some(dep_version_req), data.constraints.get(dep_name), data.locks.get(dep_name));
                            match &max_version {
                                Some(max_version) => {
                                    let is_new_version_from_bucket = match data.pkgs.get_mut(dep_name) {
                                        Some(tmp_pkg) => {
                                            let tmp_is_new_version_from_bucket = tmp_pkg.has_new_version_from_bucket;
                                            tmp_pkg.has_new_version_from_bucket = true;
                                            tmp_is_new_version_from_bucket
                                        },
                                        None => true,
                                    };
                                    let dep_new_version_from_bucket = if is_new_version_from_bucket {
                                        data.pkg_version_for_bucket("new_versions", dep_name)?
                                    } else {
                                        None
                                    };
                                    match &dep_new_version_from_bucket {
                                        Some(dep_new_version_from_bucket) => {
                                            if dep_new_version_from_bucket != max_version {
                                                return Err(Error::PkgName(dep_name.clone(), format!("version requirements indicate two different package versions: {}, {}", dep_new_version_from_bucket, max_version)));
                                            }
                                        },
                                        None => data.add_pkg_version_for_bucket("new_versions", dep_name, max_version)?,
                                    }
                                },
                                None => return Err(Error::PkgName(dep_name.clone(), String::from("each package version isn't matched to version requirement"))),
                            }
                        }
                        Ok(deps.keys().map(|dn| dn.clone()).collect())
                    },
                    None => Ok(Vec::new()),
                }
        }, |name, data| {
                let pkg = match data.pkgs.get(name) {
                    Some(tmp_pkg) => tmp_pkg.clone(),
                    None => return Err(Error::PkgName(name.clone(), String::from("no package"))),
                };
                if pkg.is_to_install()? {
                    let old_manifest = pkg.old_manifest()?;
                    match old_manifest {
                        Some(old_manifest) => {
                            match &old_manifest.dependencies {
                                Some(old_deps) => {
                                    for old_dep_name in old_deps.keys() {
                                        if data.pkg_version_for_bucket("new_versions", old_dep_name)?.is_none() {
                                            match data.pkg_version_for_bucket("versions", old_dep_name)? {
                                                Some(version) => {
                                                    data.add_pkg_version_for_bucket("new_versions", old_dep_name, &version)?;
                                                    data.pkgs.insert(old_dep_name.clone(), Pkg::new_with_copying_and_flags(None, data.pkg_info_dir(old_dep_name), data.pkg_new_part_info_dir(old_dep_name), true, false)?);
                                                },
                                                None => return Err(Error::PkgName(old_dep_name.clone(), String::from("no version"))),
                                            }
                                        }
                                        match data.pkgs.get(old_dep_name) {
                                            Some(old_dep_pkg) => {
                                                let mut depentents = old_dep_pkg.dependents()?;
                                                depentents.remove(name);
                                                old_dep_pkg.save_dependents(&depentents)?;
                                            },
                                            None => return Err(Error::PkgName(old_dep_name.clone(), String::from("no package"))),
                                        }
                                    }
                                },
                                None => (),
                            }
                        },
                        None => (),
                    }
                    let manifest = pkg.manifest()?;
                    match &manifest.dependencies {
                        Some(deps) => {
                            for (dep_name, dep_version_req) in deps {
                                match data.pkgs.get(dep_name) {
                                    Some(dep_pkg) => {
                                        let mut depentents = dep_pkg.dependents()?;
                                        depentents.insert(name.clone(), dep_version_req.clone());
                                        dep_pkg.save_dependents(&depentents)?;
                                    },
                                    None => return Err(Error::PkgName(dep_name.clone(), String::from("no package"))),
                                }
                            }
                        },
                        None => (),
                    }
                }
                Ok(())
        })?;
        match res {
            DfsResult::Success => Ok(()),
            DfsResult::Cycle(names) => Err(Error::PkgDepCycle(names)),
        }
    }

    fn prepare_new_part_infos_for_pre_installing(&mut self, name: &PkgName, visiteds: &mut HashSet<PkgName>, is_update: bool, is_force: bool) -> Result<()>
    {
        let res = self.prepare_new_part_infos_for_pre_installing_without_reset(name, visiteds, is_update, is_force);
        match res {
            Ok(()) => Ok(()),
            Err(err) => {
                self.pkgs.clear();
                self.clean_after_error()?;
                Err(err)
            },
        }
    }
    
    fn check_dependent_version_reqs(&self) -> Result<()>
    {
        self.printer.print_checking_dependent_version_reqs(false);
        let new_versions = self.pkg_versions_for_bucket("new_versions")?;
        for (name, new_version) in &new_versions {
            match self.pkgs.get(name) {
                Some(pkg) => {
                    let mut src = self.create_source(name)?;
                    let versions = src.versions()?;
                    let dependents = pkg.dependents()?;
                    for version_req in dependents.values() {
                        let max_version = Self::max_pkg_version(&versions, Some(version_req), self.constraints.get(name), self.locks.get(name));
                        match &max_version {
                            Some(max_version) => {
                                if new_version != max_version {
                                    return Err(Error::PkgName(name.clone(), format!("version requirements indicate two different package versions: {}, {}", new_version, max_version)));
                                }
                            },
                            None => return Err(Error::PkgName(name.clone(), String::from("each package version isn't matched to version requirement"))),
                        }
                    }
                },
                None => return Err(Error::PkgName(name.clone(), String::from("no package"))),
            }
        }
        self.printer.print_checking_dependent_version_reqs(true);
        Ok(())
    }

    fn pkg_is_to_install_for_pre_install(&self, name: &PkgName) -> Result<bool>
    {
        let mut manifest_file = self.pkg_new_part_info_dir(name);
        manifest_file.push("manifest.toml");
        match fs::metadata(manifest_file) {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(err) => Err(Error::Io(err)),
        }
    }
    
    fn check_dir(path: &Path, err_msg: &str) -> Result<()>
    {
        match fs::metadata(path) {
            Ok(metadata) if metadata.is_dir() => Ok(()),
            Ok(_) => Err(Error::Pkg(String::from(err_msg))),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(Error::Io(err)),
        }
    }

    fn check_dir_for_pkg(path: &Path, name: &PkgName, err_msg: &str) -> Result<()>
    {
        match fs::metadata(path) {
            Ok(metadata) if metadata.is_dir() => Ok(()),
            Ok(_) => Err(Error::PkgName(name.clone(), String::from(err_msg))),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(Error::Io(err)),
        }
    }
    
    fn search_path_conflicts(&self) -> Result<()>
    {
        self.printer.print_searching_path_conflicts(false);
        Self::check_dir(self.bin_dir.as_path(), "bin isn't directory")?;
        Self::check_dir(self.lib_dir.as_path(), "lib isn't directory")?;
        Self::check_dir(self.doc_dir.as_path(), "doc isn't directory")?;
        let new_versions = self.pkg_versions_for_bucket("new_versions")?;
        let mut ignored_bin_paths: HashSet<PathBuf> = HashSet::new();
        let mut ignored_lib_paths: HashSet<PathBuf> = HashSet::new();
        for (name, _) in &new_versions {
            if self.pkg_is_to_install_for_pre_install(name)? {
                let mut old_paths_file = self.pkg_info_dir(name);
                old_paths_file.push("paths.toml");
                match Paths::load_opt(old_paths_file)? {
                    Some(paths) => {
                        for bin_path in &paths.bin {
                            ignored_bin_paths.insert(PathBuf::from(bin_path));
                        }
                        for lib_path in &paths.lib {
                            ignored_lib_paths.insert(PathBuf::from(lib_path));
                        }
                    },
                    None => (),
                }
            }
        }
        for (name, new_version) in &new_versions {
            if self.pkg_is_to_install_for_pre_install(name)? {
                let mut src = self.create_source(name)?;
                src.set_current_version(new_version.clone());
                let mut pkg_bin_dir = PathBuf::from(src.dir()?);
                pkg_bin_dir.push("bin");
                Self::check_dir_for_pkg(pkg_bin_dir.as_path(), name, "bin in package isn't directory")?;
                let bin_paths = match conflicts(pkg_bin_dir, self.bin_dir.as_path(), &ignored_bin_paths, Some(1)) {
                    Ok((conflict_paths, paths)) => {
                        if conflict_paths.is_empty() {
                            paths
                        } else {
                            return Err(Error::PkgPathConflicts(name.clone(), None, conflict_paths, PkgPathConflict::Bin));
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                };
                let mut pkg_lib_dir = PathBuf::from(src.dir()?);
                pkg_lib_dir.push("lib");
                Self::check_dir_for_pkg(pkg_lib_dir.as_path(), name, "lib in package isn't directory")?;
                let lib_paths = match conflicts(pkg_lib_dir, self.lib_dir.as_path(), &ignored_lib_paths, Some(2)) {
                    Ok((conflict_paths, paths)) => {
                        if conflict_paths.is_empty() {
                            paths
                        } else {
                            return Err(Error::PkgPathConflicts(name.clone(), None, conflict_paths, PkgPathConflict::Lib));
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                };
                let mut bin: Vec<String> = Vec::new();
                for bin_path in &bin_paths {
                    match bin_path.to_str() {
                        Some(s) => bin.push(String::from(s)),
                        None => return Err(Error::PkgName(name.clone(), String::from("bin path contains invalid UTF-8 character"))),
                    }
                }
                let mut lib: Vec<String> = Vec::new();
                for lib_path in &lib_paths {
                    match lib_path.to_str() {
                        Some(s) => lib.push(String::from(s)),
                        None => return Err(Error::PkgName(name.clone(), String::from("lib path contains invalid UTF-8 character"))),
                    }
                }
                let paths = Paths::new(bin, lib);
                let mut paths_file = self.pkg_new_part_info_dir(name);
                paths_file.push("paths.toml");
                paths.save(paths_file)?;
            }
        }
        for (i, (name, new_version)) in new_versions.iter().enumerate() {
            for (name2, new_version2) in &new_versions[(i + 1)..] {
                if self.pkg_is_to_install_for_pre_install(name)? && self.pkg_is_to_install_for_pre_install(name2)? {
                    let mut src = self.create_source(name)?;
                    src.set_current_version(new_version.clone());
                    let mut src2 = self.create_source(name2)?;
                    src2.set_current_version(new_version2.clone());
                    let mut pkg_bin_dir = PathBuf::from(src.dir()?);
                    pkg_bin_dir.push("bin");
                    let mut pkg_bin_dir2 = PathBuf::from(src2.dir()?);
                    pkg_bin_dir2.push("bin");
                    match conflicts(pkg_bin_dir, pkg_bin_dir2, &HashSet::new(), Some(1)) {
                        Ok((conflict_paths, _)) => {
                            if !conflict_paths.is_empty() {
                                return Err(Error::PkgPathConflicts(name.clone(), Some(name2.clone()), conflict_paths, PkgPathConflict::Bin));
                            }
                        },
                        Err(err) => return Err(Error::Io(err)),
                    }
                    let mut pkg_lib_dir = PathBuf::from(src.dir()?);
                    pkg_lib_dir.push("lib");
                    let mut pkg_lib_dir2 = PathBuf::from(src2.dir()?);
                    pkg_lib_dir2.push("lib");
                    match conflicts(pkg_lib_dir, pkg_lib_dir2, &HashSet::new(), Some(2)) {
                        Ok((conflict_paths, _)) => {
                            if !conflict_paths.is_empty() {
                                return Err(Error::PkgPathConflicts(name.clone(), Some(name2.clone()), conflict_paths, PkgPathConflict::Lib));
                            }
                        },
                        Err(err) => return Err(Error::Io(err)),
                    }
                }
            }
        }
        self.printer.print_searching_path_conflicts(true);
        Ok(())
    }

    fn generate_pkg_doc(&self, name: &PkgName, new_version: &Version) -> Result<()>
    {
        if self.pkg_is_to_install_for_pre_install(name)? {
            self.printer.print_documenting_pkg(name, false);
            let doc_dir = self.pkg_tmp_doc_dir(name, &new_version);
            match create_dir_all(doc_dir.as_path()) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            // Line for documentation generation.
            self.printer.print_documenting_pkg(name, true);
        }
        Ok(())
    }
    
    fn generate_docs(&self) -> Result<()>
    {
        let new_versions = self.pkg_versions_for_bucket("new_versions")?;
        for (name, new_version) in &new_versions {
            self.generate_pkg_doc(name, new_version)?;
        }
        Ok(())
    }
    
    fn check_new_part_infos_and_generate_docs_for_pre_installing_without_reset(&self, is_doc: bool) -> Result<()>
    {
        self.check_dependent_version_reqs()?;
        self.search_path_conflicts()?;
        if is_doc {
            self.generate_docs()?;
        }
        match rename(self.new_part_info_dir(), self.new_info_dir()) {
           Ok(()) => Ok(()),
           Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
           Err(err) => Err(Error::Io(err)),
        }
    }

    fn check_new_part_infos_and_generate_docs_for_pre_installing(&mut self, is_doc: bool) -> Result<()>
    {
        let res = self.check_new_part_infos_and_generate_docs_for_pre_installing_without_reset(is_doc);
        self.pkgs.clear();
        match res {
            Ok(()) => Ok(()),
            Err(err) => {
                self.clean_after_error()?;
                Err(err)
            },
        }
    }
    
    fn prepare_new_part_infos_for_pre_removing_without_reset(&mut self) -> Result<()>
    {
        let names = self.pkg_names_for_bucket("pkgs_to_remove")?;
        for name in &names {
            let pkg = Pkg::new_without_copying(self.pkg_info_dir(name));
            let manifest = pkg.manifest()?;
            match &manifest.dependencies {
                Some(deps) => {
                    for dep_name in deps.keys() {
                        if !self.has_pkg_names_for_bucket("pkgs_to_change", dep_name)? {
                            if self.pkg_version_for_bucket("versions", dep_name)?.is_some() {
                                self.add_pkg_names_for_bucket("pkgs_to_change", dep_name)?;
                                self.pkgs.insert(dep_name.clone(), Pkg::new_with_copying_and_flags(None, self.pkg_info_dir(dep_name), self.pkg_new_part_info_dir(dep_name), true, false)?);
                            } else {
                                return Err(Error::PkgName(dep_name.clone(), String::from("no package2")));
                            }
                        }
                        match self.pkgs.get(dep_name) {
                            Some(dep_pkg) => {
                                let mut depentents = dep_pkg.dependents()?;
                                depentents.remove(name);
                                dep_pkg.save_dependents(&depentents)?;
                            },
                            None => return Err(Error::PkgName(dep_name.clone(), String::from("no package3"))),
                        }
                    }
                },
                None => (),
            }
        }
        Ok(())
    }

    fn prepare_new_infos_for_pre_removing(&mut self) -> Result<()>
    {
        let res = self.prepare_new_part_infos_for_pre_removing_without_reset();
        self.pkgs.clear();
        match res {
            Ok(()) => {
                match rename(self.new_part_info_dir(), self.new_info_dir()) {
                    Ok(()) => Ok(()),
                    Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => {
                self.clean_after_error()?;
                Err(err)
            },
        }
    }
    
    fn res_install_pkg(&self, name: &PkgName, new_version: &Version, dir: &Path, paths: &Paths, is_doc: bool) -> io::Result<()>
    {
        let mut src_bin_dir = PathBuf::from(dir);
        src_bin_dir.push("bin");
        let dst_bin_dir = self.bin_dir.clone();
        let bin_paths: Vec<PathBuf> = paths.bin.iter().map(|s| PathBuf::from(s)).collect();
        recursively_copy_paths_in_dir(src_bin_dir, dst_bin_dir, bin_paths.as_slice())?;
        let mut src_lib_dir = PathBuf::from(dir);
        src_lib_dir.push("lib");
        let dst_lib_dir = self.lib_dir.clone();
        let lib_paths: Vec<PathBuf> = paths.lib.iter().map(|s| PathBuf::from(s)).collect();
        recursively_copy_paths_in_dir(src_lib_dir, dst_lib_dir, lib_paths.as_slice())?;
        if is_doc {
            let src_doc_dir = self.pkg_tmp_doc_dir(name, new_version);
            let dst_doc_dir = self.doc_dir.clone();
            recursively_copy_paths_in_dir(src_doc_dir, dst_doc_dir, lib_paths.as_slice())?;
        }
        create_dir_all(self.pkg_info_dir(name))?;
        let mut src_manifest_file = self.pkg_new_info_dir(name);
        src_manifest_file.push("manifest.toml");
        let mut dst_manifest_file = self.pkg_info_dir(name);
        dst_manifest_file.push("manifest.toml");
        copy(src_manifest_file, dst_manifest_file)?;
        let mut src_dependents_file = self.pkg_new_info_dir(name);
        src_dependents_file.push("dependents.toml");
        let mut dst_dependents_file = self.pkg_info_dir(name);
        dst_dependents_file.push("dependents.toml");
        copy(src_dependents_file, dst_dependents_file)?;
        let mut src_paths_file = self.pkg_new_info_dir(name);
        src_paths_file.push("paths.toml");
        let mut dst_paths_file = self.pkg_info_dir(name);
        dst_paths_file.push("paths.toml");
        rename(src_paths_file, dst_paths_file)?;
        Ok(())
    }

    fn res_copy_dependents_file(&self, name: &PkgName) -> io::Result<()>
    {
        create_dir_all(self.pkg_info_dir(name))?;
        let mut src_dependents_file = self.pkg_new_info_dir(name);
        src_dependents_file.push("dependents.toml");
        let mut dst_dependents_file = self.pkg_info_dir(name);
        dst_dependents_file.push("dependents.toml");
        match copy(src_dependents_file, dst_dependents_file) {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        }
    }
    
    fn install_pkg(&self, name: &PkgName, new_version: &Version, is_doc: bool) -> Result<()>
    {
        let mut paths_file = self.pkg_new_info_dir(name);
        paths_file.push("paths.toml");
        match Paths::load(paths_file) {
            Ok(paths) => {
                self.printer.print_installing_pkg(name, false);
                let mut src = self.create_source(name)?;
                src.set_current_version(new_version.clone());
                match self.res_install_pkg(name, new_version, src.dir()?, &paths, is_doc) {
                    Ok(()) => (),
                    Err(err) => return Err(Error::Io(err)),
                }
                self.printer.print_installing_pkg(name, true);
                Ok(())
            },
            Err(Error::Io(io_err)) if io_err.kind() == ErrorKind::NotFound => {
                match self.res_copy_dependents_file(name) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) => Err(err),
        }
    }
    
    fn change_pkg(&self, name: &PkgName) -> Result<()>
    {
        let mut dependents_file = self.pkg_new_info_dir(name);
        dependents_file.push("dependents.toml");
        match fs::metadata(dependents_file) {
            Ok(_) => {
                match self.res_copy_dependents_file(name) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Io(err)),
                }
            },
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(Error::Io(err)),
        }
    }
    
    fn res_remove_pkg(&self, name: &PkgName, paths: &Paths) -> io::Result<()>
    {
        let bin_dir = self.bin_dir.clone();
        let bin_paths: Vec<PathBuf> = paths.bin.iter().map(|s| PathBuf::from(s)).collect();
        recursively_remove_paths_in_dir(bin_dir, bin_paths.as_slice(), true)?;
        let lib_dir = self.lib_dir.clone();
        let lib_paths: Vec<PathBuf> = paths.lib.iter().map(|s| PathBuf::from(s)).collect();
        recursively_remove_paths_in_dir(lib_dir, lib_paths.as_slice(), true)?;
        let doc_dir = self.doc_dir.clone();
        recursively_remove_paths_in_dir(doc_dir, lib_paths.as_slice(), true)?;
        let mut manifest_file = self.pkg_info_dir(name);
        manifest_file.push("manifest.toml");
        recursively_remove(manifest_file, true)?;
        let mut dependents_file = self.pkg_info_dir(name);
        dependents_file.push("dependents.toml");
        recursively_remove(dependents_file, true)?;
        let mut paths_file = self.pkg_info_dir(name);
        paths_file.push("paths.toml");
        recursively_remove(paths_file, true)?;
        let mut tmp_suffix_path_buf = name.to_path_buf();
        tmp_suffix_path_buf.pop();
        while tmp_suffix_path_buf != PathBuf::from("") {
            let mut dir_path_buf = self.info_dir();
            dir_path_buf.push(tmp_suffix_path_buf.as_path());
            match remove_dir(dir_path_buf.as_path()) {
                Ok(()) => (),
                Err(_) => break,
            }
            tmp_suffix_path_buf.pop();
        }
        Ok(())
    }

    fn remove_pkg(&self, name: &PkgName) -> Result<()>
    {
        let mut paths_file = self.pkg_info_dir(name);
        paths_file.push("paths.toml");
        match Paths::load(paths_file) {
            Ok(paths) => {
                self.printer.print_removing_pkg(name, false);
                match self.res_remove_pkg(name, &paths) {
                    Ok(()) => (),
                    Err(err) => return Err(Error::Io(err)),
                }
                self.printer.print_removing_pkg(name, true);
                Ok(())
            },
            Err(Error::Io(io_err)) if io_err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn pkg_is_to_install(&self, name: &PkgName) -> Result<bool>
    {
        let mut paths_file = self.pkg_new_info_dir(name);
        paths_file.push("paths.toml");
        match fs::metadata(paths_file) {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(err) => Err(Error::Io(err)),
        }
    }

    fn install_pkgs(&self, is_doc: bool) -> Result<()>
    {
        let new_versions = self.pkg_versions_for_bucket("new_versions")?;
        for (name, _) in &new_versions {
            if self.pkg_is_to_install(name)? {
                self.remove_pkg(name)?;
            }
        }
        for (name, new_version) in &new_versions {
            self.install_pkg(name, new_version, is_doc)?;
        }
        self.printer.print_cleaning_after_install(false);
        match recursively_remove(self.work_tmp_dir(), true) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        self.move_pkg_versions_for_buckets("new_versions", "versions")?;
        match recursively_remove(self.new_info_dir(), true) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        self.printer.print_cleaning_after_install(true);
        Ok(())
    }

    fn change_pkgs(&self) -> Result<()>
    {
        let names = self.pkg_names_for_bucket("pkgs_to_change")?;
        for name in &names {
            self.change_pkg(name)?;
        }
        self.remove_bucket("pkgs_to_change")?;
        self.printer.print_cleaning_before_removal(false);
        match recursively_remove(self.new_info_dir(), true) {
            Ok(()) => (),
            Err(err) => return Err(Error::Io(err)),
        }
        self.printer.print_cleaning_before_removal(true);
        Ok(())
    }
    
    fn remove_pkgs(&self) -> Result<()>
    {
        let names = self.pkg_names_for_bucket("pkgs_to_remove")?;
        for name in &names {
            self.remove_pkg(name)?;
        }
        self.remove_pkg_versions_for_buckets("pkgs_to_remove", "versions")
    }
    
    pub fn update(&self, names: &[PkgName]) -> Result<()>
    {
        for name in names {
            let mut src = self.create_source(name)?;
            src.update()?;
        }
        Ok(())
    }
    
    pub fn install(&mut self, names: &[PkgName], is_update: bool, is_force: bool, is_doc: bool) -> Result<()>
    {
        self.printer.print_pre_installing();
        let mut visiteds: HashSet<PkgName> = HashSet::new();
        for name in names {
            self.prepare_new_part_infos_for_pre_installing(name, &mut visiteds, is_update, is_force)?;
        }
        self.check_new_part_infos_and_generate_docs_for_pre_installing(is_doc)?;
        self.printer.print_installing();
        self.install_pkgs(is_doc)?;
        Ok(())
    }
    
    pub fn install_deps(&mut self, is_update: bool, is_force: bool, is_doc: bool) -> Result<()>
    {
        self.printer.print_pre_installing();
        let mut visiteds: HashSet<PkgName> = HashSet::new();
        let current_pkg = Pkg::new();
        let manifest = current_pkg.manifest()?;
        let start_name = manifest.package.name.clone();
        self.constraints = manifest.constraints.map(|cs| cs.clone()).unwrap_or(Arc::new(HashMap::new()));
        self.sources = manifest.sources.map(|ss| ss.clone()).unwrap_or(Arc::new(HashMap::new()));
        self.pkgs.insert(start_name.clone(), current_pkg);
        self.prepare_new_part_infos_for_pre_installing(&start_name, &mut visiteds, is_update, is_force)?;
        self.add_pkg_names_for_buckets_and_autoremoving("pkgs_to_remove", "versions", &visiteds)?;
        self.check_new_part_infos_and_generate_docs_for_pre_installing(is_doc)?;
        self.printer.print_installing();
        self.install_pkgs(is_doc)?;
        self.printer.print_removing();
        self.remove_pkgs()?;
        Ok(())
    }
    
    pub fn remove(&mut self, names: &[PkgName]) -> Result<()>
    {
        self.printer.print_pre_removing();
        self.add_pkg_names_for_bucket_and_removing("pkgs_to_remove", names)?;
        self.prepare_new_infos_for_pre_removing()?;
        self.printer.print_removing();
        self.change_pkgs()?;
        self.remove_pkgs()?;
        Ok(())
    }
    
    pub fn check_last_op(&self, are_deps: bool) -> Result<()>
    {
        let is_new_part_info_dir = match fs::metadata(self.new_part_info_dir()) {
            Ok(_) => true,
            Err(err) if err.kind() == ErrorKind::NotFound => false,
            Err(err) => return Err(Error::Io(err)),
        };
        if is_new_part_info_dir {
            if are_deps {
                return Err(Error::Pkg(String::from("Last operation was interrupted while preparation. Please execute clean-deps command to clean.")));
            } else {
                return Err(Error::Pkg(String::from("Last operation was interrupted while preparation. Please execute clean command to clean.")));
            }
        }
        let is_new_info_dir = match fs::metadata(self.new_info_dir()) {
            Ok(_) => true,
            Err(err) if err.kind() == ErrorKind::NotFound => false,
            Err(err) => return Err(Error::Io(err)),
        };
        if (is_new_info_dir && self.has_bucket("new_versions")?) || is_new_info_dir || self.has_bucket("pkgs_to_remove")? {
            if are_deps {
                return Err(Error::Pkg(String::from("Last operation is incompleted. Please execute continue-deps command to complete operation.")));
            } else {
                return Err(Error::Pkg(String::from("Last operation is incompleted. Please execute continue command to complete operation.")));
            }
        }
        Ok(())
    }

    pub fn cont(&self, is_doc: bool, are_deps: bool) -> Result<()>
    {
        let is_new_info_dir = match fs::metadata(self.new_info_dir()) {
            Ok(_) => true,
            Err(err) if err.kind() == ErrorKind::NotFound => false,
            Err(err) => return Err(Error::Io(err)),
        };
        if is_new_info_dir && !self.has_bucket("pkgs_to_remove")? {
            self.printer.print_installing();
            self.install_pkgs(is_doc)?;
        } else if is_new_info_dir && (are_deps || !self.has_bucket("pkgs_to_remove")?) {
            self.printer.print_installing();
            self.printer.print_cleaning_after_install(false);
            match recursively_remove(self.new_info_dir(), true) {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            self.printer.print_cleaning_after_install(true);
        }
        if self.has_bucket("pkgs_to_remove")? {
            self.printer.print_removing();
            if !are_deps {
                if is_new_info_dir && self.has_bucket("pkgs_to_change")? {
                    self.change_pkgs()?;
                } else if is_new_info_dir {
                    self.printer.print_cleaning_before_removal(false);
                    match recursively_remove(self.new_info_dir(), true) {
                        Ok(()) => (),
                        Err(err) => return Err(Error::Io(err)),
                    }
                    self.printer.print_cleaning_before_removal(true);
                }
            }
            self.remove_pkgs()?;
        }
        Ok(())
    }

    pub fn clean(&self) -> Result<()>
    {
        let is_new_part_info_dir = match fs::metadata(self.new_part_info_dir()) {
            Ok(_) => true,
            Err(err) if err.kind() == ErrorKind::NotFound => false,
            Err(err) => return Err(Error::Io(err)),
        };
        if is_new_part_info_dir {
            self.printer.print_cleaning(false);
            self.remove_bucket("new_versions")?;
            self.remove_bucket("pkgs_to_remove")?;
            self.remove_bucket("pkgs_to_change")?;
            match self.res_remove_dirs_for_cleaning() {
                Ok(()) => (),
                Err(err) => return Err(Error::Io(err)),
            }
            self.printer.print_cleaning(true);
        }
        Ok(())
    }
    
    pub fn update_all(&self) -> Result<()>
    {
        let mut names: Vec<PkgName> = Vec::new();
        self.pkg_versions_in(|name, _| {
                names.push(name.clone());
                Ok(())
        })?;
        self.update(names.as_slice())
    }

    pub fn install_all(&mut self, is_update: bool, is_force: bool, is_doc: bool) -> Result<()>
    {
        let mut names: Vec<PkgName> = Vec::new();
        self.pkg_versions_in(|name, _| {
                let dependents = self.pkg_dependents(name)?;
                if dependents.map(|ds| ds.is_empty()).unwrap_or(true) {
                    names.push(name.clone());
                }
                Ok(())
        })?;
        self.install(names.as_slice(), is_update, is_force, is_doc)
    }
}

#[cfg(test)]
mod tests;
