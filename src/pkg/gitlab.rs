//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::str;
use super::*;

pub const SERVICE_DOMAIN: &'static str = "gitlab.com";
pub const SERVICE_DOMAIN_PREFIX: &'static str = "gitlab.";

#[derive(Clone, Debug, Deserialize)]
struct Tag
{
    name: String,
}

pub struct GitLabSrc
{
    name: PkgName,
    old_name: Option<PkgName>,
    home_dir: PathBuf,
    work_dir: PathBuf,
    printer: Arc<dyn Print + Send + Sync>,
    versions: Option<BTreeSet<Version>>,
    current_version: Option<Version>,
    dir: Option<PathBuf>,
}

impl GitLabSrc
{
    pub fn new(name: PkgName, old_name: Option<PkgName>, home_dir: PathBuf, work_dir: PathBuf, printer: Arc<dyn Print + Send + Sync>) -> Option<Self>
    {
        let original_name = old_name.as_ref().unwrap_or(&name);
        if original_name.name().split('/').count() != 3 {
            return None;
        }
        match original_name.name().split_once('/') {
            Some((domain, _)) if domain.starts_with(SERVICE_DOMAIN_PREFIX) => {
                Some(GitLabSrc {
                        name,
                        old_name,
                        home_dir,
                        work_dir,
                        printer,
                        versions: None,
                        current_version: None,
                        dir: None,
                })
            },
            _ => None,
        }
    }
    
    pub fn name(&self) -> &PkgName
    { &self.name }

    pub fn old_name(&self) -> Option<&PkgName>
    { 
        match &self.old_name {
            Some(old_name) => Some(old_name),
            None => None,
        }
    }

    pub fn home_dir(&self) -> &Path
    { self.home_dir.as_path() }

    pub fn work_dir(&self) -> &Path
    { self.work_dir.as_path() }

    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }

    pub fn current_version(&self) -> Option<&Version>
    { 
        match &self.current_version {
            Some(current_version) => Some(current_version),
            None => None,
        }
    }

    fn update_versions(&self, is_update: bool) -> Result<BTreeSet<Version>>
    {
        let original_name = self.old_name.as_ref().unwrap_or(&self.name);
        let (service_domain, repo_path) = match original_name.name().split_once('/') {
            Some((tmp_service_domain, tmp_repo_path)) => (tmp_service_domain, tmp_repo_path),
            None => return Err(Error::PkgName(self.name.clone(), String::from("no service domain and package repository path"))),
        };
        update_pkg_versions(&self.name, &self.old_name, self.home_dir.as_path(), is_update, &self.printer, || {
                let mut easy = curl::easy::Easy::new();
                easy.url(format!("https://{}/api/v4/projects/{}/repository/tags", str_to_url_part(service_domain, false), str_to_url_part(repo_path, false)).as_str())?;
                easy.follow_location(true)?;
                Ok(easy)
        }, |data| {
                let s = match str::from_utf8(data) {
                    Ok(tmp_s) => tmp_s,
                    Err(_) => return Err(Error::PkgName(self.name.clone(), String::from("data contains invalid UTF-8 character"))),
                };
                let tags: Vec<Tag> = match serde_json::from_str(s) {
                    Ok(tmp_refs) => tmp_refs,
                    Err(err) => return Err(Error::SerdeJson(Box::new(err))),
                };
                let mut versions: BTreeSet<Version> = BTreeSet::new();
                for tag in &tags {
                    match tag_name_to_version(tag.name.as_str()) {
                        Some(version) => {
                            versions.insert(version);
                        },
                        None => (),
                    }
                }
                Ok(versions)
        })
    }
}

impl Source for GitLabSrc
{
    fn update(&mut self) -> Result<()>
    {
        self.versions = Some(self.update_versions(true)?);
        Ok(())
    }
    
    fn versions(&mut self) -> Result<&BTreeSet<Version>>
    {
        if self.versions.is_none() {
            self.versions = Some(self.update_versions(false)?);
        }
        match &self.versions {
            Some(versions) => Ok(versions),
            None => return Err(Error::PkgName(self.name.clone(), String::from("no package versions"))),
        }
    }
    
    fn set_current_version(&mut self, version: Version)
    { self.current_version = Some(version); }
    
    fn dir(&mut self) -> Result<&Path>
    {
        if self.dir.is_none() {
            match &self.current_version {
                Some(current_version) => {
                    self.dir = Some(extract_pkg_file(&self.name, current_version, &self.work_dir, &self.printer, || {
                            let original_name = self.old_name.as_ref().unwrap_or(&self.name);
                            let repo_name = match original_name.name().split_once('/') {
                                Some((_, repo_path)) => {
                                    match repo_path.split_once('/') {
                                        Some((_, tmp_repo_name)) => tmp_repo_name,
                                        None => return Err(Error::PkgName(self.name.clone(), String::from("no package repository name"))),
                                    }
                                },
                                None => return Err(Error::PkgName(self.name.clone(), String::from("no package repository path"))),
                            };
                            let tag_name = version_to_tag_name(current_version);
                            let url = format!("https://{}/-/archive/{}/{}-{}.tar.gz?ref_type=tags", str_to_url_part(original_name.name(), true), str_to_url_part(tag_name.as_str(), false), str_to_url_part(repo_name, false), str_to_url_part(tag_name.as_str(), false));
                            download_pkg_file(&self.name, &self.old_name, current_version, url.as_str(), &self.home_dir, &self.printer)
                    })?)
                },
                None => return Err(Error::PkgName(self.name.clone(), String::from("no current package version"))),
            }
        }
        match &self.dir {
            Some(versions) => Ok(versions),
            None => return Err(Error::PkgName(self.name.clone(), String::from("no package directory"))),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GitLabSrcFactory;

impl GitLabSrcFactory
{
    pub fn new() -> Self
    { GitLabSrcFactory }
}

impl SourceCreate for GitLabSrcFactory
{
    fn create(&self, name: PkgName, old_name: Option<PkgName>, home_dir: PathBuf, work_dir: PathBuf, printer: Arc<dyn Print + Send + Sync>) -> Option<Box<dyn Source + Send + Sync>>
    { 
        match GitLabSrc::new(name, old_name, home_dir, work_dir, printer) {
            Some(src) => Some(Box::new(src)),
            None => None,
        }
    }
}
