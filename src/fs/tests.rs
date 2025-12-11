//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs;
use sealed_test::prelude::*;
use super::*;

#[sealed_test]
fn test_recursively_copy_copies_file()
{
    fs::write("test.txt", "some text").unwrap();
    match recursively_copy("test.txt", "test2.txt") {
        Ok(()) => {
            assert_eq!(String::from("some text"), fs::read_to_string("test.txt").unwrap());
            assert_eq!(String::from("some text"), fs::read_to_string("test2.txt").unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_copies_directory()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    match recursively_copy("test", "test2") {
        Ok(()) => {
            assert_eq!(true, fs::metadata("test").unwrap().is_dir());
            let mut path_buf = PathBuf::from("test2");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("a");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("b");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test.txt");
            assert_eq!(String::from("some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.push("test2.txt");
            assert_eq!(String::from("second some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.push("test3.txt");
            assert_eq!(String::from("third some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.push("c");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test4.txt");
            assert_eq!(String::from("fourth some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.push("test5.txt");
            assert_eq!(String::from("fifth some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_copies_directory_for_existent_directory()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    match recursively_copy("test", "test2") {
        Ok(()) => {
            assert_eq!(true, fs::metadata("test").unwrap().is_dir());
            let mut path_buf = PathBuf::from("test2");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("a");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test.txt");
            assert_eq!(String::from("some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_creates_destination_directories()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    path_buf.push("a");
    match recursively_copy("test", path_buf.as_path()) {
        Ok(()) => {
            assert_eq!(true, fs::metadata("test").unwrap().is_dir());
            let mut path_buf = PathBuf::from("test2");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("a");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test.txt");
            assert_eq!(String::from("some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_overwrites_file()
{
    fs::write("test.txt", "some text").unwrap();
    fs::write("test2.txt", "second some text").unwrap();
    match recursively_copy("test.txt", "test2.txt") {
        Ok(()) => {
            assert_eq!(String::from("some text"), fs::read_to_string("test.txt").unwrap());
            assert_eq!(String::from("some text"), fs::read_to_string("test2.txt").unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_complains_on_destination_directory_can_not_be_in_source_directory()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    match recursively_copy("test", path_buf.as_path()) {
        Err(err) => {
            assert_eq!(ErrorKind::Other, err.kind());
            assert_eq!(String::from("destination directory can't be in source directory"), format!("{}", err));
        },
        Ok(()) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_complains_on_not_a_directory()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    match recursively_copy("test", "test2") {
        Err(_) => assert!(true),
        Ok(()) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_remove_removes_file()
{
    fs::write("test.txt", "some text").unwrap();
    fs::write("test2.txt", "second some text").unwrap();
    match recursively_remove("test.txt") {
        Ok(()) => {
            match fs::metadata("test.txt") {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(String::from("second some text"), fs::read_to_string("test2.txt").unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_remove_removes_directory()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    fs::write("test2.txt", "second some text").unwrap();
    match recursively_remove("test") {
        Ok(()) => {
            match fs::metadata("test") {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(String::from("second some text"), fs::read_to_string("test2.txt").unwrap());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_conflicts_returns_conflict_paths_and_paths()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    match conflicts("test", "test2", None) {
        Ok((conflict_paths, paths)) => {
            assert_eq!(5, conflict_paths.len());
            let mut path_buf = PathBuf::from("a");
            path_buf.push("b");
            path_buf.push("test2.txt");
            assert_eq!(true, conflict_paths.contains(&path_buf));
            path_buf.pop();
            path_buf.pop();
            let mut path_buf = PathBuf::from("c");
            path_buf.push("test4.txt");
            assert_eq!(true, conflict_paths.contains(&path_buf));
            path_buf.pop();
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("d")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("e")));
            assert_eq!(6, paths.len());
            let mut path_buf = PathBuf::from("a");
            path_buf.push("b");
            path_buf.push("test.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            path_buf.push("test2.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            path_buf.pop();
            path_buf.push("test3.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            let mut path_buf = PathBuf::from("c");
            path_buf.push("test4.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            assert_eq!(true, paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, paths.contains(&PathBuf::from("e")));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_conflicts_returns_conflict_paths_and_paths_for_depth_2()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    match conflicts("test", "test2", Some(2)) {
        Ok((conflict_paths, paths)) => {
            assert_eq!(5, conflict_paths.len());
            let mut path_buf = PathBuf::from("a");
            path_buf.push("b");
            assert_eq!(true, conflict_paths.contains(&path_buf));
            path_buf.pop();
            let mut path_buf = PathBuf::from("c");
            path_buf.push("test4.txt");
            assert_eq!(true, conflict_paths.contains(&path_buf));
            path_buf.pop();
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("d")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("e")));
            assert_eq!(5, paths.len());
            let mut path_buf = PathBuf::from("a");
            path_buf.push("b");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            path_buf.push("test3.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            let mut path_buf = PathBuf::from("c");
            path_buf.push("test4.txt");
            assert_eq!(true, paths.contains(&path_buf));
            path_buf.pop();
            assert_eq!(true, paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, paths.contains(&PathBuf::from("e")));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_conflicts_returns_conflict_paths_and_paths_for_depth_1()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    let mut path_buf = PathBuf::from("test2");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("d");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("e");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.pop();
    match conflicts("test", "test2", Some(1)) {
        Ok((conflict_paths, paths)) => {
            assert_eq!(5, conflict_paths.len());
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("a")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("c")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("d")));
            assert_eq!(true, conflict_paths.contains(&PathBuf::from("e")));
            assert_eq!(5, paths.len());
            assert_eq!(true, paths.contains(&PathBuf::from("a")));
            assert_eq!(true, paths.contains(&PathBuf::from("c")));
            assert_eq!(true, paths.contains(&PathBuf::from("test5.txt")));
            assert_eq!(true, paths.contains(&PathBuf::from("d")));
            assert_eq!(true, paths.contains(&PathBuf::from("e")));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_copy_paths_in_dir_copies_directories()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("d");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("e");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("f");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("test6.txt");
    fs::write(path_buf.as_path(), "sixth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut path_buf = PathBuf::from("a");
    path_buf.push("b");
    paths.push(path_buf);
    let mut path_buf = PathBuf::from("c");
    path_buf.push("d");
    paths.push(path_buf);
    match recursively_copy_paths_in_dir("test", "test2", paths.as_slice()) {
        Ok(()) => {
            let mut path_buf = PathBuf::from("test2");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("a");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("b");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test.txt");
            assert_eq!(String::from("some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.push("test2.txt");
            assert_eq!(String::from("second some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.pop();
            path_buf.push("c");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("d");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test3.txt");
            assert_eq!(String::from("third some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.push("test4.txt");
            assert_eq!(String::from("fourth some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.pop();
            path_buf.push("e");
            match fs::metadata(path_buf.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_recursively_remove_paths_in_dir_removes_directories()
{
    let mut path_buf = PathBuf::from("test");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("a");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("b");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test.txt");
    fs::write(path_buf.as_path(), "some text").unwrap();
    path_buf.pop();
    path_buf.push("test2.txt");
    fs::write(path_buf.as_path(), "second some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("c");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("d");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test3.txt");
    fs::write(path_buf.as_path(), "third some text").unwrap();
    path_buf.pop();
    path_buf.push("test4.txt");
    fs::write(path_buf.as_path(), "fourth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    path_buf.push("e");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("f");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("test5.txt");
    fs::write(path_buf.as_path(), "fifth some text").unwrap();
    path_buf.pop();
    path_buf.push("test6.txt");
    fs::write(path_buf.as_path(), "sixth some text").unwrap();
    path_buf.pop();
    path_buf.pop();
    path_buf.pop();
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut path_buf = PathBuf::from("a");
    path_buf.push("b");
    paths.push(path_buf);
    let mut path_buf = PathBuf::from("c");
    path_buf.push("d");
    paths.push(path_buf);
    match recursively_remove_paths_in_dir("test", paths.as_slice()) {
        Ok(()) => {
            let mut path_buf = PathBuf::from("test");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("a");
            match fs::metadata(path_buf.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            path_buf.pop();
            path_buf.push("c");
            match fs::metadata(path_buf.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            path_buf.pop();
            path_buf.push("e");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("f");
            assert_eq!(true, fs::metadata(path_buf.as_path()).unwrap().is_dir());
            path_buf.push("test5.txt");
            assert_eq!(String::from("fifth some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.push("test6.txt");
            assert_eq!(String::from("sixth some text"), fs::read_to_string(path_buf.as_path()).unwrap());
            path_buf.pop();
            path_buf.pop();
            path_buf.pop();
        },
        Err(_) => assert!(false),
    }
}
