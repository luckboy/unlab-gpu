//
// Copyright (c) 2025 Łukasz Szpakowski
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
use std::fs::rename;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::result;
use std::sync::Arc;
use crate::jammdb;
use crate::jammdb::DB;
use crate::serde::de;
use crate::serde::de::Visitor;
use crate::serde::Deserialize;
use crate::serde::Deserializer;
use crate::serde::Serialize;
use crate::serde::Serializer;
use crate::dfs::*;
use crate::error::*;
use crate::fs::*;
use crate::version::*;

pub trait Source
{
    fn update(&mut self) -> Result<()>;
    
    fn versions(&mut self) -> Result<&BTreeSet<Version>>;
    
    fn set_current_version(&mut self, version: &Version);
    
    fn dir(&mut self) -> Result<&Path>;
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
    Renamed(String),
    #[serde(rename = "versions")]
    Versions(BTreeMap<Version, VersionSrcInfo>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest
{
    pub package: PkgInfo,
    pub dependencies: Option<HashMap<PkgName, VersionReq>>,
    pub constraints: Option<Arc<HashMap<PkgName, VersionReq>>>,
    pub sources: Option<Arc<HashMap<PkgName, Arc<SrcInfo>>>>,
}

impl Manifest
{
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

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    {
        match File::create(path) {
            Ok(mut file) => self.write(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

pub fn read_dependents(r: &mut dyn Read) -> Result<HashMap<PkgName, VersionReq>>
{
    let mut s = String::new();
    match r.read_to_string(&mut s) {
        Ok(_) => {
            match toml::from_str::<HashMap<PkgName, VersionReq>>(s.as_str()) {
                Ok(dependents) => Ok(dependents),
                Err(err) => Err(Error::TomlDe(err)),
            }
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn write_dependents(w: &mut dyn Write, dependents: &HashMap<PkgName, VersionReq>) -> Result<()>
{
    match toml::to_string(dependents) {
        Ok(s) => {
            match write!(w, "{}", s) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Io(err)),
            }
        },
        Err(err) => Err(Error::TomlSer(err)),
    }
}

pub fn load_dependents<P: AsRef<Path>>(path: P) -> Result<HashMap<PkgName, VersionReq>>
{
    match File::open(path) {
        Ok(mut file) => read_dependents(&mut file),
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn save_dependents<P: AsRef<Path>>(path: P, dependents: &HashMap<PkgName, VersionReq>) -> Result<()>
{
    match File::create(path) {
        Ok(mut file) => write_dependents(&mut file, dependents),
        Err(err) => Err(Error::Io(err)),
    }
}

#[derive(Clone, Debug)]
pub struct Pkg
{
    dir: Option<PathBuf>,
    info_dir: Option<PathBuf>,
    new_part_info_dir: Option<PathBuf>,
}

impl Pkg
{
    fn new() -> Self
    {
        Pkg {
            dir: Some(PathBuf::from(".")),
            info_dir: None,
            new_part_info_dir: None,
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
        match fs::metadata(info_dir) {
            Ok(_) => {
                copy(src_dependents_file, dst_dependents_file)?;
            },
            Err(err) if err.kind() == ErrorKind::NotFound => {
                let _file = File::create(dst_dependents_file)?;
            }
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
    
    fn new_with_copy(dir: Option<PathBuf>, info_dir: PathBuf, new_part_info_dir: PathBuf) -> Result<Self>
    {
        Self::copy_info_files(&dir, &info_dir, &new_part_info_dir)?;
        Ok(Pkg {
                dir,
                info_dir: Some(info_dir),
                new_part_info_dir: Some(new_part_info_dir),
        })
    }

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
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut manifest_file = new_part_info_dir.clone();
                manifest_file.push("manifest.toml");
                Manifest::load(manifest_file)
            },
            None => {
                match &self.info_dir {
                    Some(info_dir) => {
                        let mut manifest_file = info_dir.clone();
                        manifest_file.push("manifest.toml");
                        Manifest::load(manifest_file)
                    },
                    None => {
                        match &self.dir {
                            Some(dir) => {
                                let mut manifest_file = dir.clone();
                                manifest_file.push("Unlab.toml");
                                Manifest::load(manifest_file)
                            },
                            None => Err(Error::Pkg(String::from("no manifest file"))),
                        }
                    },
                }
            },
        }
    }

    fn dependents(&self) -> Result<HashMap<PkgName, VersionReq>>
    {
        match &self.new_part_info_dir {
            Some(new_part_info_dir) => {
                let mut dependents_file = new_part_info_dir.clone();
                dependents_file.push("dependents.toml");
                load_dependents(dependents_file)
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
                save_dependents(dependents_file, dependents)
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

#[derive(Clone)]
pub struct PkgManager
{
    pkg_db: DB,
    home_var_dir: PathBuf,
    var_dir: PathBuf,
    tmp_dir: PathBuf,
    bin_dir: PathBuf,
    lib_dir: PathBuf,
    pkgs: HashMap<PkgName, Pkg>,
    locks: HashMap<PkgName, Version>,
    constraints: Arc<HashMap<PkgName, VersionReq>>,
    sources: Arc<HashMap<PkgName, Arc<SrcInfo>>>,
}

impl PkgManager
{
    pub fn new(home_var_dir: PathBuf, var_dir: PathBuf, tmp_dir: PathBuf, bin_dir: PathBuf, lib_dir: PathBuf) -> Result<Self>
    {
        let mut pkg_db_file = var_dir.clone();
        pkg_db_file.push("pkg.db");
        let pkg_db = match DB::open(pkg_db_file) {
            Ok(tmp_pkg_db) => tmp_pkg_db,
            Err(err) => return Err(Error::Jammdb(err)),
        };
        Ok(PkgManager {
                pkg_db,
                home_var_dir,
                var_dir,
                tmp_dir,
                bin_dir,
                lib_dir,
                pkgs: HashMap::new(),
                locks: HashMap::new(),
                constraints: Arc::new(HashMap::new()),
                sources: Arc::new(HashMap::new()),
        })
    }
    
    fn create_source(&self, name: &PkgName) -> Result<Box<dyn Source + Send + Sync>>
    { Err(Error::Pkg(String::from("no source"))) }
    
    pub fn home_var_dir(&self) -> &Path
    { self.home_var_dir.as_path() }

    pub fn var_dir(&self) -> &Path
    { self.var_dir.as_path() }
    
    pub fn tmp_dir(&self) -> &Path
    { self.tmp_dir.as_path() }
    
    pub fn info_dir(&self) -> PathBuf
    {
        let mut dir = self.var_dir.clone();
        dir.push("info");
        dir
    }

    pub fn new_part_info_dir(&self) -> PathBuf
    {
        let mut dir = self.var_dir.clone();
        dir.push("info.new.part");
        dir
    }
    
    pub fn new_info_dir(&self) -> PathBuf
    {
        let mut dir = self.var_dir.clone();
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

    fn pkg_versions(&self, bucket_name: &str) -> Result<Vec<(PkgName, Version)>>
    {
        match self.pkg_db.tx(false) {
            Ok(tx) => {
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
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }
    
    fn pkg_version(&self, bucket_name: &str, name: &PkgName) -> Result<Option<Version>>
    {
        match self.pkg_db.tx(false) {
            Ok(tx) => {
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
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }

    fn add_pkg_version(&self, bucket_name: &str, name: &PkgName, version: &Version) -> Result<()>
    {
        match self.pkg_db.tx(true) {
            Ok(tx) => {
                match tx.get_or_create_bucket(bucket_name) {
                    Ok(version_bucket) => {
                        match version_bucket.put(name.name(), format!("{}", version)) {
                            Ok(_) => {
                                match tx.commit() {
                                    Ok(()) => Ok(()),
                                    Err(err) => Err(Error::Jammdb(err)),
                                }
                            },
                            Err(err) => Err(Error::Jammdb(err)),
                        }
                    },
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }    
    
    fn move_pkg_versions(&self, src_bucket_name: &str, dst_bucket_name: &str) -> Result<()>
    { 
        match self.pkg_db.tx(true) {
            Ok(tx) => {
                {
                    let src_version_bucket = match tx.get_bucket(src_bucket_name) {
                        Ok(tmp_src_version_bucket) => tmp_src_version_bucket,
                        Err(jammdb::Error::BucketMissing) => return Ok(()),
                        Err(err) => return Err(Error::Jammdb(err)),
                    };
                    let dst_version_bucket = match tx.get_or_create_bucket(dst_bucket_name) {
                        Ok(tmp_dst_version_bucket) => tmp_dst_version_bucket,
                        Err(err) => return Err(Error::Jammdb(err)),
                    };
                    for data in src_version_bucket.cursor() {
                        match dst_version_bucket.put(data.kv().key().to_vec(), data.kv().value().to_vec()) {
                            Ok(_) => (),
                            Err(err) => return Err(Error::Jammdb(err)),
                        }
                    }
                }
                match tx.delete_bucket(src_bucket_name) {
                    Ok(()) => (),
                    Err(err) => return Err(Error::Jammdb(err)),
                }
                match tx.commit() {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }

    fn max_pkg_version(versions: &BTreeSet<Version>, version_reqs: &[VersionReq], locked_version: Option<&Version>) -> Option<Version>
    {
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
    
    fn prepare_new_part_infos_for_pre_install(&mut self, name: &PkgName, visiteds: &mut HashSet<PkgName>, is_update: bool, is_force: bool) -> Result<()>
    {
        if visiteds.contains(name) {
            return Ok(());
        }
        let res = dfs(name, visiteds, self, |name, data| {
                let pkg = match data.pkgs.get(name) {
                    Some(tmp_pkg) => tmp_pkg,
                    None => {
                        let mut src = data.create_source(name)?;
                        let old_version = data.pkg_version("versions", name)?;
                        let tmp_new_version = data.pkg_version("new_versions", name)?;
                        let version = tmp_new_version.clone().or(old_version.clone());
                        let new_version = match &version {
                            Some(tmp_version) if !is_update || tmp_new_version.is_some() => Some(tmp_version.clone()),
                            _ => {
                                if old_version.is_some() {
                                    if is_update {
                                        src.update()?;
                                    }
                                    let versions = src.versions()?;
                                    let mut old_dependents_file = data.pkg_info_dir(name);
                                    old_dependents_file.push("dependents.toml");
                                    let old_dependants = load_dependents(old_dependents_file)?;
                                    let mut version_reqs: Vec<VersionReq> = old_dependants.values().map(|r| r.clone()).collect();
                                    match data.constraints.get(name) {
                                        Some(constraint) => version_reqs.push(constraint.clone()),
                                        None => (),
                                    }
                                    Self::max_pkg_version(&versions, version_reqs.as_slice(), data.locks.get(name))
                                } else {
                                    old_version.clone()
                                }
                            },
                        };
                        match &new_version {
                            Some(new_version) => {
                                src.set_current_version(new_version);
                                if tmp_new_version.is_none() {
                                    data.add_pkg_version("new_versions", name, &new_version)?;
                                }
                                let dir = if is_force || old_version.as_ref().map(|ov| ov != new_version).unwrap_or(true) {
                                    Some(PathBuf::from(src.dir()?))
                                } else {
                                    None
                                };
                                data.pkgs.insert(name.clone(), Pkg::new_with_copy(dir, data.pkg_info_dir(name), data.pkg_new_part_info_dir(name))?);
                                data.pkgs.get(name).unwrap()
                            },
                            None => return Err(Error::PkgName(name.clone(), String::from("each version isn't matched to version requirement"))),
                        }
                    },
                };
                let manifest = pkg.manifest()?;
                match &manifest.dependencies {
                    Some(deps) => {
                        for (dep_name, dep_version_req) in deps {
                            let mut dep_src = data.create_source(name)?;
                            if is_update {
                                dep_src.update()?;
                            }
                            let versions = dep_src.versions()?;
                            let mut version_reqs = vec![dep_version_req.clone()];
                            match data.constraints.get(name) {
                                Some(constraint) => version_reqs.push(constraint.clone()),
                                None => (),
                            }
                            let dep_version = Self::max_pkg_version(&versions, version_reqs.as_slice(), data.locks.get(name));
                            match &dep_version {
                                Some(dep_version) => {
                                    let dep_new_version = data.pkg_version("new_versions", dep_name)?;
                                    if dep_new_version.as_ref().map(|dnv| dnv == dep_version).unwrap_or(true) {
                                        if dep_new_version.is_none() {
                                            data.add_pkg_version("new_versions", dep_name, dep_version)?;
                                        }
                                    } else {
                                        return Err(Error::PkgName(name.clone(), String::from("version requirements of dependents are contradictory")));
                                    }
                                },
                                None => return Err(Error::PkgName(name.clone(), String::from("each version isn't matched to version requirements"))),
                            }
                        }
                        Ok(deps.keys().map(|dn| dn.clone()).collect())
                    },
                    None => Ok(Vec::new()),
                }
        }, |name, data| {
                let pkg = match data.pkgs.get(name) {
                    Some(tmp_pkg) => tmp_pkg,
                    None => return Err(Error::PkgName(name.clone(), String::from("no package"))),
                };
                let old_manifest = pkg.old_manifest()?;
                match old_manifest {
                    Some(old_manifest) => {
                        match &old_manifest.dependencies {
                            Some(old_deps) => {
                                for old_dep_name in old_deps.keys() {
                                    match data.pkgs.get(old_dep_name) {
                                        Some(old_dep_pkg) => {
                                            let mut depentents = old_dep_pkg.dependents()?;
                                            depentents.remove(name);
                                            pkg.save_dependents(&depentents)?;
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
                                    pkg.save_dependents(&depentents)?;
                                },
                                None => return Err(Error::PkgName(dep_name.clone(), String::from("no package"))),
                            }
                        }
                    },
                    None => (),
                }
                Ok(())
        })?;
        match res {
            DfsResult::Success => Ok(()),
            DfsResult::Cycle(names) => Err(Error::PkgDepCycle(names)),
        }
    }
    
    fn check_dependent_version_reqs(&self) -> Result<()>
    {
        let new_versions = self.pkg_versions("new_versions")?;
        for (name, new_version) in &new_versions {
            match self.pkgs.get(name) {
                Some(pkg) => {
                    let mut src = self.create_source(name)?;
                    let versions = src.versions()?;
                    let dependents = pkg.dependents()?;
                    let mut version_reqs: Vec<VersionReq> = dependents.values().map(|r| r.clone()).collect();
                    match self.constraints.get(name) {
                        Some(constraint) => version_reqs.push(constraint.clone()),
                        None => (),
                    }
                    let max_version = Self::max_pkg_version(&versions, version_reqs.as_slice(), self.locks.get(name));
                    match &max_version { 
                        Some(max_version) => {
                            if max_version != new_version {
                                return Err(Error::PkgName(name.clone(), String::from("version requirements of dependents are contradictory")));
                            }
                        },
                        None => return Err(Error::PkgName(name.clone(), String::from("each version isn't matched to version requirement"))),
                    }
                },
                None => return Err(Error::PkgName(name.clone(), String::from("no package"))),
            }
        }
        Ok(())
    }

    fn pkg_is_to_install(&self, name: &PkgName) -> Result<bool>
    {
        let mut manifest_file = self.pkg_new_part_info_dir(name);
        manifest_file.push("manifest.toml");
        match fs::metadata(manifest_file) {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
            Err(err) => Err(Error::Io(err)),
        }
    }

    fn find_path_conflicts(&self) -> Result<()>
    {
        let new_versions = self.pkg_versions("new_versions")?;
        let mut ignored_bin_paths: HashSet<PathBuf> = HashSet::new();
        let mut ignored_lib_paths: HashSet<PathBuf> = HashSet::new();
        for (name, _) in &new_versions {
            if self.pkg_is_to_install(name)? {
                let mut old_paths_file = self.pkg_info_dir(name);
                old_paths_file.push("paths.toml");
                let paths = Paths::load(old_paths_file)?;
                for bin_path in &paths.bin {
                    ignored_bin_paths.insert(PathBuf::from(bin_path));
                }
                for lib_path in &paths.lib {
                    ignored_lib_paths.insert(PathBuf::from(lib_path));
                }
            }
        }
        for (name, new_version) in &new_versions {
            if self.pkg_is_to_install(name)? {
                let mut src = self.create_source(name)?;
                src.set_current_version(new_version);
                let mut pkg_bin_dir = PathBuf::from(src.dir()?);
                pkg_bin_dir.push("bin");
                let bin_paths = match conflicts(pkg_bin_dir, self.bin_dir.as_path(), &ignored_bin_paths, Some(1)) {
                    Ok((conflict_paths, paths)) => {
                        if conflict_paths.is_empty() {
                            paths
                        } else {
                            return Err(Error::PkgPathConflict(name.clone(), None, conflict_paths, PkgPathConflict::Bin));
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                };
                let mut pkg_lib_dir = PathBuf::from(src.dir()?);
                pkg_lib_dir.push("lib");
                let lib_paths = match conflicts(pkg_lib_dir, self.lib_dir.as_path(), &ignored_lib_paths, Some(2)) {
                    Ok((conflict_paths, paths)) => {
                        if conflict_paths.is_empty() {
                            paths
                        } else {
                            return Err(Error::PkgPathConflict(name.clone(), None, conflict_paths, PkgPathConflict::Lib));
                        }
                    },
                    Err(err) => return Err(Error::Io(err)),
                };
                let mut bin_paths2: Vec<String> = Vec::new();
                for bin_path in &bin_paths {
                    match bin_path.to_str() {
                        Some(s) => bin_paths2.push(String::from(s)),
                        None => return Err(Error::PkgName(name.clone(), String::from("path contains invalid UTF-8 character"))),
                    }
                }
                let mut lib_paths2: Vec<String> = Vec::new();
                for lib_path in &lib_paths {
                    match lib_path.to_str() {
                        Some(s) => lib_paths2.push(String::from(s)),
                        None => return Err(Error::PkgName(name.clone(), String::from("path contains invalid UTF-8 character"))),
                    }
                }
                let paths = Paths::new(bin_paths2, lib_paths2);
                let mut paths_file = self.pkg_new_part_info_dir(name);
                paths_file.push("paths.toml");
                paths.save(paths_file)?;
            }
        }
        for (i, (name, new_version)) in new_versions.iter().enumerate() {
            for (name2, new_version2) in &new_versions[(i + 1)..] {
                if self.pkg_is_to_install(name)? && self.pkg_is_to_install(name2)? {
                    let mut src = self.create_source(name)?;
                    src.set_current_version(new_version);
                    let mut src2 = self.create_source(name2)?;
                    src2.set_current_version(new_version2);
                    let mut pkg_bin_dir = PathBuf::from(src.dir()?);
                    pkg_bin_dir.push("bin");
                    let mut pkg_bin_dir2 = PathBuf::from(src2.dir()?);
                    pkg_bin_dir2.push("bin");
                    match conflicts(pkg_bin_dir, pkg_bin_dir2, &HashSet::new(), Some(1)) {
                        Ok((conflict_paths, _)) => {
                            if !conflict_paths.is_empty() {
                                return Err(Error::PkgPathConflict(name.clone(), Some(name2.clone()), conflict_paths, PkgPathConflict::Bin));
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
                                return Err(Error::PkgPathConflict(name.clone(), Some(name2.clone()), conflict_paths, PkgPathConflict::Lib));
                            }
                        },
                        Err(err) => return Err(Error::Io(err)),
                    }
                }
            }
        }
        Ok(())
    }

    fn check_new_part_infos_for_pre_install(&self) -> Result<()>
    {
        self.check_dependent_version_reqs()?;
        self.find_path_conflicts()?;
        match rename(self.new_part_info_dir(), self.new_info_dir()) {
           Ok(()) => Ok(()),
           Err(err) => Err(Error::Io(err)),
        }
    }
}
