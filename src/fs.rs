//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashSet;
use std::fs::copy;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::remove_dir;
use std::fs::remove_file;
use std::fs::symlink_metadata;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

fn recursively_copy_with_path_bufs(src_path_buf: &mut PathBuf, dst_path_buf: &mut PathBuf) -> Result<()>
{
    let metadata = symlink_metadata(src_path_buf.as_path())?;
    if metadata.is_dir() {
        match create_dir(dst_path_buf.as_path()) {
            Ok(()) => (),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => (),
            Err(err) => return Err(err),
        }
        let canon_src_path_buf = src_path_buf.canonicalize()?;
        let canon_dst_path_buf = dst_path_buf.canonicalize()?;
        if canon_src_path_buf == canon_dst_path_buf {
            return Ok(());
        }
        if canon_dst_path_buf.starts_with(canon_src_path_buf) {
            remove_dir(dst_path_buf.as_path())?;
            return Err(Error::new(ErrorKind::Other, "destination directory can't be in source directory"));
        }
        let entries = read_dir(src_path_buf.as_path())?;
        for entry in entries {
            let tmp_entry = entry?;
            src_path_buf.push(tmp_entry.file_name());
            dst_path_buf.push(tmp_entry.file_name());
            recursively_copy_with_path_bufs(src_path_buf, dst_path_buf)?;
            dst_path_buf.pop();
            src_path_buf.pop();
        }
    } else if metadata.is_file() {
        let canon_src_path_buf = src_path_buf.canonicalize()?;
        match dst_path_buf.canonicalize() {
            Ok(canon_dst_path_buf) => {
                if canon_src_path_buf == canon_dst_path_buf {
                    return Ok(());
                }
            },
            Err(err) if err.kind() == ErrorKind::NotFound => (),
            Err(err) => return Err(err),
        }
        copy(src_path_buf.as_path(), dst_path_buf.as_path())?;
    } else if metadata.is_symlink() {
        return Err(Error::new(ErrorKind::Other, "can't copy symbolic link"));
    } else {
        return Err(Error::new(ErrorKind::Other, "can't copy device file"));
    }
    Ok(())
}

pub fn recursively_copy<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()>
{
    let mut src_path_buf = PathBuf::from(src.as_ref());
    let mut dst_path_buf = PathBuf::from(dst.as_ref());
    let mut dst_parent_path_buf = dst_path_buf.clone();
    dst_parent_path_buf.pop();
    if dst_parent_path_buf != PathBuf::from("") {
        create_dir_all(dst_parent_path_buf)?;
    }
    recursively_copy_with_path_bufs(&mut src_path_buf, &mut dst_path_buf)
}

fn recursively_remove_with_path_buf(path_buf: &mut PathBuf) -> Result<()>
{
    let metadata = symlink_metadata(path_buf.as_path())?;
    if metadata.is_dir() {
        let entries = read_dir(path_buf.as_path())?;
        for entry in entries {
            let tmp_entry = entry?;
            path_buf.push(tmp_entry.file_name());
            recursively_remove_with_path_buf(path_buf)?;
            path_buf.pop();
        }
        remove_dir(path_buf.as_path())?;
    } else {
        remove_file(path_buf.as_path())?;
    }
    Ok(())
}

pub fn recursively_remove<P: AsRef<Path>>(path: P) -> Result<()>
{
    let mut path_buf = PathBuf::from(path.as_ref());
    recursively_remove_with_path_buf(&mut path_buf)
}

fn get_dir_paths_and_paths_in_dir(path: &Path, suffix_path_buf: &mut PathBuf, dir_paths: &mut HashSet<PathBuf>, paths: &mut HashSet<PathBuf>, path_vec: &mut Option<Vec<PathBuf>>, depth: Option<usize>) -> Result<()>
{
    let mut path_buf = PathBuf::from(path);
    if suffix_path_buf != &PathBuf::from("") {
        path_buf.push(suffix_path_buf.as_path());
    }
    let metadata = match symlink_metadata(path_buf.as_path()) {
        Ok(tmp_metadata) => tmp_metadata,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err),
    };
    if metadata.is_dir() && depth.map(|d| d > 0).unwrap_or(true) {
        match read_dir(path_buf.as_path()) {
            Ok(entries) => {
                dir_paths.insert(suffix_path_buf.clone());
                for entry in entries {
                    let tmp_entry = entry?;
                    suffix_path_buf.push(tmp_entry.file_name());
                    get_dir_paths_and_paths_in_dir(path, suffix_path_buf, dir_paths, paths, path_vec, depth.map(|d| d - 1))?;
                    suffix_path_buf.pop();
                }
            },
            Err(err) if err.kind() == ErrorKind::NotFound => (),
            Err(err) => return Err(err),
        }
    } else {
        paths.insert(suffix_path_buf.clone());
        match path_vec {
            Some(path_vec) => path_vec.push(suffix_path_buf.clone()),
            None => (),
        }
    }
    Ok(())
}

pub fn conflicts<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q, depth: Option<usize>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)>
{
    let mut paths: Option<Vec<PathBuf>> = Some(Vec::new());
    let mut src_dir_paths: HashSet<PathBuf> = HashSet::new();
    let mut src_paths: HashSet<PathBuf> = HashSet::new();
    let mut src_suffix_path_buf = PathBuf::from("");
    get_dir_paths_and_paths_in_dir(src.as_ref(), &mut src_suffix_path_buf, &mut src_dir_paths, &mut src_paths, &mut paths, depth)?;
    let mut dst_dir_paths: HashSet<PathBuf> = HashSet::new();
    let mut dst_paths: HashSet<PathBuf> = HashSet::new();
    let mut dst_suffix_path_buf = PathBuf::from("");
    get_dir_paths_and_paths_in_dir(dst.as_ref(), &mut dst_suffix_path_buf, &mut dst_dir_paths, &mut dst_paths, &mut None, depth)?;
    let mut conflict_paths: Vec<PathBuf> = src_dir_paths.intersection(&dst_paths).map(|p| p.clone()).collect();
    let mut conflict_paths2: Vec<PathBuf> = src_paths.intersection(&dst_dir_paths).map(|p| p.clone()).collect();
    let mut conflict_paths3: Vec<PathBuf> = src_paths.intersection(&dst_paths).map(|p| p.clone()).collect();
    conflict_paths.append(&mut conflict_paths2);
    conflict_paths.append(&mut conflict_paths3);
    Ok((conflict_paths, paths.unwrap_or(Vec::new())))
}

pub fn recursively_copy_paths_in_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q, paths: &[PathBuf]) -> Result<()>
{
    for suffix_path_buf in paths {
        let mut src_path_buf = PathBuf::from(src.as_ref());
        src_path_buf.push(suffix_path_buf.as_path());
        let mut dst_path_buf = PathBuf::from(dst.as_ref());
        dst_path_buf.push(suffix_path_buf.as_path());
        recursively_copy(src_path_buf.as_path(), dst_path_buf.as_path())?;
    }
    Ok(())
}

pub fn recursively_remove_paths_in_dir<P: AsRef<Path>>(path: P, paths: &[PathBuf]) -> Result<()>
{
    for suffix_path_buf in paths {
        let mut path_buf = PathBuf::from(path.as_ref());
        path_buf.push(suffix_path_buf.as_path());
        recursively_remove(path_buf.as_path())?;
        let mut tmp_suffix_path_buf = suffix_path_buf.clone();
        if tmp_suffix_path_buf != PathBuf::from("") {
            tmp_suffix_path_buf.pop();
            while tmp_suffix_path_buf != PathBuf::from("") {
                let mut dir_path_buf = PathBuf::from(path.as_ref());
                dir_path_buf.push(tmp_suffix_path_buf.as_path());
                match remove_dir(dir_path_buf.as_path()) {
                    Ok(()) => (),
                    Err(_) => break,
                }
                tmp_suffix_path_buf.pop();
            }
        }
    }
    Ok(())
}
