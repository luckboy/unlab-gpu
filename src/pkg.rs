//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::path;
use std::path::PathBuf;
use std::result;
use std::sync::Arc;
use crate::serde::de;
use crate::serde::de::Visitor;
use crate::serde::Deserialize;
use crate::serde::Deserializer;
use crate::serde::Serialize;
use crate::serde::Serializer;
use crate::error::*;
use crate::version::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PkgName
{
    name: String,
}

impl PkgName
{
    pub fn new(name: String) -> Self
    { PkgName { name, } }

    pub fn parse(name: &str) -> Result<Self>
    {
        if name.split('/').count() < 2 {
            return Err(Error::InvalidPkgName);
        }
        let ss = name.split('/');
        for s in ss {
            if s.contains('\\') || s == "." || s == ".." {
                return Err(Error::InvalidPkgName);
            }
        }
        Ok(Self::new(String::from(name)))
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
    pub sources: Option<Arc<HashMap<PkgName, SrcInfo>>>,
}
