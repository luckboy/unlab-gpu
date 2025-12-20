//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::Ordering;
use std::cmp::max;

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

#[derive(Clone, Debug)]
pub struct Version
{
    numeric_idents: Vec<u32>,
    pre_release_idents: Option<Vec<PreReleaseIdent>>,
}

impl Version
{
    pub fn new(numeric_idents: Vec<u32>, pre_release_idents: Option<Vec<PreReleaseIdent>>) -> Self
    { Version { numeric_idents, pre_release_idents, } }
    
    pub fn parse(s: &str) -> Option<Self>
    {
        let (t, u) = match s.split_once('-') {
            Some(pair) => pair,
            None => (s, ""),
        };
        let mut numeric_idents: Vec<u32> = Vec::new();
        for v in t.split('.') {
            match v.parse::<u32>() {
                Ok(n) => numeric_idents.push(n),
                Err(_) => return None,
            }
        }
        let pre_release_idents = if !s.is_empty() {
            let mut tmp_pre_release_idents: Vec<PreReleaseIdent> = Vec::new();
            for w in u.split('.') {
                match w.parse::<u32>() {
                    Ok(n) => tmp_pre_release_idents.push(PreReleaseIdent::Numeric(n)),
                    Err(_) => tmp_pre_release_idents.push(PreReleaseIdent::Alphanumeric(String::from(w))),
                }
            }
            Some(tmp_pre_release_idents)
        } else {
            None
        };
        Some(Self::new(numeric_idents, pre_release_idents))
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
