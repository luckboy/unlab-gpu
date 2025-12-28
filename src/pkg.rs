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
use crate::version::*;

pub trait Source
{
    fn update(&mut self) -> Result<()>;
    
    fn versions(&mut self) -> Result<&BTreeSet<Version>>;
    
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

#[derive(Clone)]
pub struct PkgManager
{
    pkg_db: DB,
    index_dir: PathBuf,
    cache_dir: PathBuf,
    info_dir: PathBuf,
    new_part_info_dir: PathBuf,
    new_info_dir: PathBuf,
    tmp_dir: PathBuf,
}

impl PkgManager
{
    pub fn new<P: AsRef<Path>>(pkg_db_file: P, index_dir: PathBuf, cache_dir: PathBuf, info_dir: PathBuf, new_part_info_dir: PathBuf, new_info_dir: PathBuf, tmp_dir: PathBuf) -> Result<Self>
    {
        let pkg_db = match DB::open(pkg_db_file) {
            Ok(tmp_pkg_db) => tmp_pkg_db,
            Err(err) => return Err(Error::Jammdb(err)),
        };
        Ok(PkgManager {
                pkg_db,
                index_dir,
                cache_dir,
                info_dir,
                new_part_info_dir,
                new_info_dir,
                tmp_dir,
        })
    }
    
    fn create_source(&self, name: &PkgName) -> Result<Box<dyn Source + Send + Sync>>
    { Err(Error::Pkg(String::from("no source"))) }
    
    fn pkg_version(&self, name: &PkgName, bucket_name: &str) -> Result<Option<Version>>
    {
        match self.pkg_db.tx(false) {
            Ok(tx) => {
                match tx.get_bucket("versions") {
                    Ok(version_bucket) => {
                        match version_bucket.get(name.name()) {
                            Some(data) => {
                                match String::from_utf8(data.kv().value().to_vec()) {
                                    Ok(s) => Ok(Some(Version::parse(s.as_str())?)),
                                    Err(err) => Err(Error::Pkg(format!("invalid version data"))),
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

    fn move_new_pkg_versions_to_pkg_versions(&self, name: &PkgName, version: &Version) -> Result<()>
    { 
        match self.pkg_db.tx(true) {
            Ok(tx) => {
                {
                    let new_version_bucket = match tx.get_bucket("new_version") {
                        Ok(tmp_new_version_bucket) => tmp_new_version_bucket,
                        Err(jammdb::Error::BucketMissing) => return Ok(()),
                        Err(err) => return Err(Error::Jammdb(err)),
                    };
                    let version_bucket = match tx.get_or_create_bucket("version") {
                        Ok(tmp_version_bucket) => tmp_version_bucket,
                        Err(err) => return Err(Error::Jammdb(err)),
                    };
                    for data in new_version_bucket.cursor() {
                        match version_bucket.put(data.kv().key().to_vec(), data.kv().value().to_vec()) {
                            Ok(_) => (),
                            Err(err) => return Err(Error::Jammdb(err)),
                        }
                    }
                }
                match tx.delete_bucket("new_version") {
                    Ok(()) => Ok(()),
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }

    fn new_pkg_versions(&self, name: &PkgName, bucket_name: &str) -> Result<Vec<(PkgName, Version)>>
    {
        match self.pkg_db.tx(false) {
            Ok(tx) => {
                match tx.get_bucket("new_versions") {
                    Ok(new_version_bucket) => {
                        let mut pairs: Vec<(PkgName, Version)> = Vec::new();
                        for data in new_version_bucket.cursor() {
                            let name = match String::from_utf8(data.kv().key().to_vec()) {
                                Ok(s) => PkgName::parse(s.as_str())?,
                                Err(err) => return Err(Error::Pkg(format!("invalid package name data"))),
                            };
                            let version = match String::from_utf8(data.kv().value().to_vec()) {
                                Ok(s) => Version::parse(s.as_str())?,
                                Err(err) => return Err(Error::Pkg(format!("invalid version data"))),
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
    
    fn add_new_pkg_version(&self, name: &PkgName, version: &Version) -> Result<()>
    {
        match self.pkg_db.tx(true) {
            Ok(tx) => {
                match tx.get_or_create_bucket("new_version") {
                    Ok(version_bucket) => {
                        match version_bucket.put(name.name(), format!("{}", version)) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(Error::Jammdb(err)),
                        }
                    },
                    Err(err) => Err(Error::Jammdb(err)),
                }
            },
            Err(err) => Err(Error::Jammdb(err)),
        }
    }
        
    fn prepare_new_part_infos_for_pre_install(&mut self, name: &PkgName, visiteds: &mut HashSet<PkgName>) -> Result<()>
    {
        let res = dfs(name, visiteds, self, |name, data| {
                Ok(Vec::new())
        }, |name, data| {
                Ok(())
        })?;
        match res {
            DfsResult::Success => Ok(()),
            DfsResult::Cycle(names) => Err(Error::PkgDepCycle(names)),
        }
    }
}
