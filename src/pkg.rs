//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use crate::serde::Deserialize;
use crate::serde::Serialize;
use crate::version::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PkgInfo
{
    pub name: String,
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
    pub constraints: Option<Arc<HashMap<String, VersionReq>>>,
    pub dependencies: Option<HashMap<String, VersionReq>>,
    pub sources: Option<Arc<HashMap<String, SrcInfo>>>,
}
