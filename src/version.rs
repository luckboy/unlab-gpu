//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::Ordering;
use std::cmp::max;
use std::fmt;
use std::result;
use crate::serde::de;
use crate::serde::de::Visitor;
use crate::serde::Deserialize;
use crate::serde::Deserializer;
use crate::serde::Serialize;
use crate::serde::Serializer;
use crate::error::*;

#[derive(Clone, Debug)]
pub enum PreReleaseIdent
{
    Numeric(u32),
    Alphanumeric(String),
}

impl Eq for PreReleaseIdent
{}

impl PartialEq for PreReleaseIdent
{
    fn eq(&self, other: &Self) -> bool
    { self.cmp(other) == Ordering::Equal }
}

impl Ord for PreReleaseIdent
{
    fn cmp(&self, other: &Self) -> Ordering 
    {
        match (self, other) {
            (PreReleaseIdent::Numeric(n), PreReleaseIdent::Numeric(m)) => n.cmp(&m),
            (PreReleaseIdent::Alphanumeric(_), PreReleaseIdent::Numeric(_)) => Ordering::Greater,
            (PreReleaseIdent::Numeric(_), PreReleaseIdent::Alphanumeric(_)) => Ordering::Less,
            (PreReleaseIdent::Alphanumeric(s), PreReleaseIdent::Alphanumeric(t)) => s.cmp(&t),
        }
    }
}

impl PartialOrd for PreReleaseIdent
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    { Some(self.cmp(other)) }
}

impl fmt::Display for PreReleaseIdent
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            PreReleaseIdent::Numeric(n) => write!(f, "{}", n),
            PreReleaseIdent::Alphanumeric(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Version
{
    numeric_idents: Vec<u32>,
    pre_release_idents: Option<Vec<PreReleaseIdent>>,
    build_idents: Option<Vec<String>>,
}

impl Version
{
    pub fn new(numeric_idents: Vec<u32>, pre_release_idents: Option<Vec<PreReleaseIdent>>, build_idents: Option<Vec<String>>) -> Self
    { Version { numeric_idents, pre_release_idents, build_idents, } }
    
    pub fn parse(s: &str) -> Result<Self>
    {
        let (pair_s, build) = match s.split_once('+') {
            Some(pair) => pair,
            None => (s, ""),
        };
        let (version_core, pre_release) = match pair_s.split_once('-') {
            Some(pair) => pair,
            None => (s, ""),
        };
        let mut numeric_idents: Vec<u32> = Vec::new();
        for t in version_core.split('.') {
            match t.parse::<u32>() {
                Ok(n) => numeric_idents.push(n),
                Err(_) => return Err(Error::InvalidVersion),
            }
        }
        let pre_release_idents = if !s.is_empty() {
            let mut tmp_pre_release_idents: Vec<PreReleaseIdent> = Vec::new();
            for t in pre_release.split('.') {
                match t.parse::<u32>() {
                    Ok(n) => tmp_pre_release_idents.push(PreReleaseIdent::Numeric(n)),
                    Err(_) => {
                        if t.is_empty() || t.contains('/') || t.contains('\\') {
                            return Err(Error::InvalidVersion);
                        }
                        tmp_pre_release_idents.push(PreReleaseIdent::Alphanumeric(String::from(t)))
                    },
                }
            }
            Some(tmp_pre_release_idents)
        } else {
            None
        };
        let build_idents = if !s.is_empty() {
            let mut tmp_build_idents: Vec<String> = Vec::new();
            for t in build.split('.') {
                if t.is_empty() || t.contains('/') || t.contains('\\') {
                    return Err(Error::InvalidVersion);
                }
                tmp_build_idents.push(String::from(t));
            }
            Some(tmp_build_idents)
        } else {
            None
        };
        Ok(Self::new(numeric_idents, pre_release_idents, build_idents))
    }
    
    pub fn numeric_idents(&self) -> &[u32]
    { self.numeric_idents.as_slice() }

    pub fn pre_release_idents(&self) -> Option<&[PreReleaseIdent]>
    {
        match &self.pre_release_idents {
            Some(pre_release_idents) => Some(pre_release_idents.as_slice()),
            None => None,
        }
    }

    pub fn build_idents(&self) -> Option<&[String]>
    {
        match &self.build_idents {
            Some(build_idents) => Some(build_idents.as_slice()),
            None => None,
        }
    }

    pub fn eq_numeric_idents(&self, version: &Version, count: usize) -> bool
    {
        for i in 0..count {
            let n = if i < self.numeric_idents.len() {
                self.numeric_idents[i]
            } else {
                0
            };
            let m = if i < version.numeric_idents.len() {
                version.numeric_idents[i]
            } else {
                0
            };
            if n != m {
                return false;
            }
        }
        true
    }
}

impl Eq for Version
{}

impl PartialEq for Version
{
    fn eq(&self, other: &Self) -> bool
    { self.cmp(other) == Ordering::Equal }
}

impl Ord for Version
{
    fn cmp(&self, other: &Self) -> Ordering 
    {
        let len = max(self.numeric_idents.len(), other.numeric_idents.len());
        for i in 0..len {
            let n = if i < self.numeric_idents.len() {
                self.numeric_idents[i]
            } else {
                0
            };
            let m = if i < other.numeric_idents.len() {
                other.numeric_idents[i]
            } else {
                0
            };
            match n.cmp(&m) {
                Ordering::Equal => (),
                ordering => return ordering,
            }
        }
        match (&self.pre_release_idents, &other.pre_release_idents) {
            (Some(pre_release_idents), Some(pre_release_idents2)) => pre_release_idents.cmp(&pre_release_idents2),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Version
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    { Some(self.cmp(other)) }
}

impl fmt::Display for Version
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut is_first = true;
        for numeric_ident in &self.numeric_idents {
            if !is_first {
                write!(f, ".")?;
            }
            write!(f, "{}", numeric_ident)?;
            is_first = false;
        }
        match &self.pre_release_idents {
            Some(pre_release_idents) => {
                write!(f, "-")?;
                is_first = true;
                for pre_release_ident in pre_release_idents {
                    if !is_first {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", pre_release_ident)?;
                    is_first = false;
                }
            },
            None => (),
        }
        match &self.build_idents {
            Some(build_idents) => {
                write!(f, "+")?;
                is_first = true;
                for build_ident in build_idents {
                    if !is_first {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", build_ident)?;
                    is_first = false;
                }
            },
            None => (),
        }
        Ok(())
    }
}

impl Serialize for Version
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    { serializer.serialize_str(format!("{}", self).as_str()) }
}

struct VersionVisitor;

impl<'de> Visitor<'de> for VersionVisitor
{
    type Value = Version;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    { write!(formatter, "a version") }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
        where E: de::Error
    {
        match Version::parse(v) {
            Ok(version) => Ok(version),
            Err(err) => Err(E::custom(format!("{}", err))),
        }
    }
}

impl<'de> Deserialize<'de> for Version
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    { deserializer.deserialize_str(VersionVisitor) }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VersionOp
{
    Eq,
    Ne,
    Lt,
    Ge,
    Gt,
    Le,
    Default,
    Tilde,
}

impl fmt::Display for VersionOp
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            VersionOp::Eq => write!(f, "="),
            VersionOp::Ne => write!(f, "!="),
            VersionOp::Lt => write!(f, "<"),
            VersionOp::Ge => write!(f, ">="),
            VersionOp::Gt => write!(f, ">"),
            VersionOp::Le => write!(f, "<="),
            VersionOp::Default => write!(f, "^"),
            VersionOp::Tilde => write!(f, "~"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SingleVersionReq
{
    Wildcard,
    Pair(VersionOp, Version),
}

impl SingleVersionReq
{
    pub fn parse(s: &str) -> Result<Self>
    {
        let trimmed_s = s.trim();
        if trimmed_s != "*" {
            let (op, t) = if trimmed_s.starts_with("=") {
                (VersionOp::Eq, &trimmed_s[1..])
            } else if trimmed_s.starts_with("!=") {
                (VersionOp::Ne, &trimmed_s[2..])
            } else if trimmed_s.starts_with("<=") {
                (VersionOp::Le, &trimmed_s[2..])
            } else if trimmed_s.starts_with("<") {
                (VersionOp::Lt, &trimmed_s[1..])
            } else if trimmed_s.starts_with(">=") {
                (VersionOp::Ge, &trimmed_s[2..])
            } else if trimmed_s.starts_with(">") {
                (VersionOp::Gt, &trimmed_s[1..])
            } else if trimmed_s.starts_with("^") {
                (VersionOp::Default, &trimmed_s[1..])
            } else if trimmed_s.starts_with("~") {
                (VersionOp::Tilde, &trimmed_s[1..])
            } else {
                (VersionOp::Default, trimmed_s)
            };
            let trimmed_t = t.trim();
            let version = Version::parse(trimmed_t)?;
            Ok(SingleVersionReq::Pair(op, version))
        } else {
            Ok(SingleVersionReq::Wildcard)
        }
    }
    
    pub fn matches(&self, version: &Version) -> bool
    {
        match self {
            SingleVersionReq::Wildcard => true,
            SingleVersionReq::Pair(op, version2) => {
                match op {
                    VersionOp::Eq => version == version2,
                    VersionOp::Ne => version != version2,
                    VersionOp::Lt => version < version2,
                    VersionOp::Ge => version >= version2,
                    VersionOp::Gt => version > version2,
                    VersionOp::Le => version <= version2,
                    VersionOp::Default => {
                        let mut count = 0usize;
                        if !version2.numeric_idents.is_empty() {
                            count += 1;
                            for i in 0..version2.numeric_idents.len() {
                                match version2.numeric_idents.get(i) {
                                    Some(0) if version2.numeric_idents.len() >= i + 2 => count += 1,
                                    _ => break,
                                }
                            }
                        }
                        version >= version2 && version.eq_numeric_idents(version2, count)
                    },
                    VersionOp::Tilde => {
                        let count = if !version2.numeric_idents.is_empty() {
                            if version2.numeric_idents.len() >= 2 {
                                2
                            } else {
                                1
                            }
                        } else {
                            0
                        };
                        version >= version2 && version.eq_numeric_idents(version2, count)
                    },
                }
            },
        }
    }
}

impl fmt::Display for SingleVersionReq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            SingleVersionReq::Wildcard => write!(f, "*"),
            SingleVersionReq::Pair(op, version) => write!(f, "{}{}", op, version),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VersionReq
{
    single_reqs: Vec<SingleVersionReq>,
}

impl VersionReq
{
    pub fn new(single_reqs: Vec<SingleVersionReq>) -> Self
    { VersionReq { single_reqs, } }
    
    pub fn parse(s: &str) -> Result<Self>
    {
        let mut single_reqs: Vec<SingleVersionReq> = Vec::new();
        for t in s.split(',') {
            single_reqs.push(SingleVersionReq::parse(t)?);
        }
        Ok(VersionReq::new(single_reqs))
    }
    
    pub fn single_reqs(&self) -> &[SingleVersionReq]
    { self.single_reqs.as_slice() }

    pub fn matches(&self, version: &Version) -> bool
    { self.single_reqs.iter().all(|sr| sr.matches(version)) }
}

impl fmt::Display for VersionReq
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut is_first = true;
        for single_req in &self.single_reqs {
            if !is_first {
                write!(f, ",")?;
            }
            write!(f, "{}", single_req)?;
            is_first = false;
        }
        Ok(())
    }
}

impl Serialize for VersionReq
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    { serializer.serialize_str(format!("{}", self).as_str()) }
}

struct VersionReqVisitor;

impl<'de> Visitor<'de> for VersionReqVisitor
{
    type Value = VersionReq;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    { write!(formatter, "a version requirement") }

    fn visit_str<E>(self, v: &str) -> result::Result<Self::Value, E>
        where E: de::Error
    {
        match VersionReq::parse(v) {
            Ok(req) => Ok(req),
            Err(err) => Err(E::custom(format!("{}", err))),
        }
    }
}

impl<'de> Deserialize<'de> for VersionReq
{
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: Deserializer<'de>
    { deserializer.deserialize_str(VersionReqVisitor) }
}
