//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
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
