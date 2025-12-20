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
    
    pub fn parse(s: &str) -> Option<Self>
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
                Err(_) => return None,
            }
        }
        let pre_release_idents = if !s.is_empty() {
            let mut tmp_pre_release_idents: Vec<PreReleaseIdent> = Vec::new();
            for t in pre_release.split('.') {
                match t.parse::<u32>() {
                    Ok(n) => tmp_pre_release_idents.push(PreReleaseIdent::Numeric(n)),
                    Err(_) => tmp_pre_release_idents.push(PreReleaseIdent::Alphanumeric(String::from(t))),
                }
            }
            Some(tmp_pre_release_idents)
        } else {
            None
        };
        let build_idents = if !s.is_empty() {
            let mut tmp_build_idents: Vec<String> = Vec::new();
            for t in build.split('.') {
                tmp_build_idents.push(String::from(t));
            }
            Some(tmp_build_idents)
        } else {
            None
        };
        Some(Self::new(numeric_idents, pre_release_idents, build_idents))
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
