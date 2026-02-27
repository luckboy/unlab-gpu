//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs;
use std::io::ErrorKind;
use std::io::Read;
use sealed_test::prelude::*;
use crate::builtins::add_std_builtin_funs;
use super::*;

#[sealed_test]
fn test_tester_load_loads_tests()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    match tester.load() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_load_loads_tests_with_script()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")
run(\"script.un\")
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let script_content = "
module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    fs::write(path_buf, &script_content[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    match tester.load() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_with_success()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_with_failure()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_bad_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_bad_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                Some((Error::Assert(Some(msg), Some((Value::Int(4), Value::Int(5)))), stack_trace)) => {
                    assert_eq!(String::from("left isn't equal to right"), *msg);
                    assert_eq!(1, stack_trace.len());
                    match &stack_trace[0] {
                        (Some(fun_value), pos) => {
                            assert_eq!(String::from("pl_jan_nowak_abc_tests::test_bad_add"), format!("{}", fun_value));
                            let mut path_buf = PathBuf::from("tests");
                            path_buf.push("pl.jan.nowak");
                            path_buf.push("abc");
                            path_buf.push("tests.un");
                            assert_eq!(Pos::new(Arc::new(path_buf.to_string_lossy().into_owned()), 12, 9), *pos);
                        },
                        (_, _) => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_with_stdout_data()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        println(\"abc\")
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let mut cursor_g = cursor.write().unwrap();
                    cursor_g.set_position(0);
                    let mut s = String::new();
                    cursor_g.read_to_string(&mut s).unwrap();
                    assert_eq!(String::from("abc\n"), s); 
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_with_stderr_data()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        eprintln(\"abc\")
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let mut cursor_g = cursor.write().unwrap();
                    cursor_g.set_position(0);
                    let mut s = String::new();
                    cursor_g.read_to_string(&mut s).unwrap();
                    assert_eq!(String::from("abc\n"), s); 
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_without_stdout_cursor()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), false, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_runs_test_without_stderr_cursor()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, false);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(1, tester.test_results().len());
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_complains_on_module_is_not_test_suite()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add")) {
        Err(Error::Tester(msg)) => assert_eq!(String::from("module isn't test suite"), msg),
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_test_complains_on_undefined_test_function()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_test(&vec![String::from("pl_jan_nowak_abc_tests")], &String::from("test_add2")) {
        Err(Error::Tester(msg)) => assert_eq!(String::from("undefined test function"), msg),
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_tests_in_test_suite_runs_tests()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_tests_in_test_suite(&vec![String::from("pl_jan_nowak_abc_tests")]) {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(2, tester.test_results().len());
            // test_add
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            // test_bad_add
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_bad_add")), tester.test_results()[1].0);
            match tester.test_results()[1].1.error_pair() {
                Some((Error::Assert(Some(msg), Some((Value::Int(4), Value::Int(5)))), stack_trace)) => {
                    assert_eq!(String::from("left isn't equal to right"), *msg);
                    assert_eq!(1, stack_trace.len());
                    match &stack_trace[0] {
                        (Some(fun_value), pos) => {
                            assert_eq!(String::from("pl_jan_nowak_abc_tests::test_bad_add"), format!("{}", fun_value));
                            let mut path_buf = PathBuf::from("tests");
                            path_buf.push("pl.jan.nowak");
                            path_buf.push("abc");
                            path_buf.push("tests.un");
                            assert_eq!(Pos::new(Arc::new(path_buf.to_string_lossy().into_owned()), 12, 9), *pos);
                        },
                        (_, _) => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match tester.test_results()[1].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[1].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_tests_in_test_suite_complains_on_module_is_not_test_suite()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_tests_in_test_suite(&vec![String::from("pl_jan_nowak_abc_tests")]) {
        Err(Error::Tester(msg)) => assert_eq!(String::from("module isn't test suite"), msg),
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_tester_run_all_tests_runs_all_tests()
{
    let lib_content = "
module pl_jan_nowak_abc
    function add(x, y)
        x + y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content[1..]).unwrap();
    let tests_content = "
uselib(\"abc\")

module pl_jan_nowak_abc_tests
    tests()
    usevars(\"pl_jan_nowak_abc\")

    function test_add()
        asserteq(4, add(2, 2))
    end

    function test_bad_add()
        asserteq(4, add(2, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.jan.nowak");
    path_buf.push("abc");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content[1..]).unwrap();
    let lib_content2 = "
module pl_nowakowski_def
    function sub(x, y)
        x - y
    end
end
";
    let mut path_buf = PathBuf::from("lib");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("lib.un");
    fs::write(path_buf, &lib_content2[1..]).unwrap();
    let tests_content2 = "
uselib(\"def\")

module pl_nowakowski_def_tests
    tests()
    usevars(\"pl_nowakowski_def\")

    function test_sub()
        asserteq(2, sub(4, 2))
    end

    function test_bad_sub()
        asserteq(2, sub(4, 3))
    end
end
";
    let mut path_buf = PathBuf::from("tests");
    path_buf.push("pl.nowakowski");
    path_buf.push("def");
    fs::create_dir_all(path_buf.as_path()).unwrap();
    path_buf.push("tests.un");
    fs::write(path_buf, &tests_content2[1..]).unwrap();
    let mut root_mod: ModNode<Value, ()> = ModNode::new(());
    add_std_builtin_funs(&mut root_mod);
    let mut tester = Tester::new(Arc::new(RwLock::new(root_mod)), OsString::from("lib"), OsString::from("doc"), Arc::new(EmptyPrinter::new()), true, true);
    tester.load().unwrap();
    match tester.run_all_tests() {
        Ok(()) => {
            let mut test_dir = PathBuf::from("work");
            test_dir.push("test");
            match fs::metadata(test_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            assert_eq!(4, tester.test_results().len());
            // test_add
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_add")), tester.test_results()[0].0);
            match tester.test_results()[0].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[0].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[0].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            // test_bad_add
            assert_eq!((vec![String::from("pl_jan_nowak_abc_tests")], String::from("test_bad_add")), tester.test_results()[1].0);
            match tester.test_results()[1].1.error_pair() {
                Some((Error::Assert(Some(msg), Some((Value::Int(4), Value::Int(5)))), stack_trace)) => {
                    assert_eq!(String::from("left isn't equal to right"), *msg);
                    assert_eq!(1, stack_trace.len());
                    match &stack_trace[0] {
                        (Some(fun_value), pos) => {
                            assert_eq!(String::from("pl_jan_nowak_abc_tests::test_bad_add"), format!("{}", fun_value));
                            let mut path_buf = PathBuf::from("tests");
                            path_buf.push("pl.jan.nowak");
                            path_buf.push("abc");
                            path_buf.push("tests.un");
                            assert_eq!(Pos::new(Arc::new(path_buf.to_string_lossy().into_owned()), 12, 9), *pos);
                        },
                        (_, _) => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match tester.test_results()[1].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[1].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            // test_bad_sub
            assert_eq!((vec![String::from("pl_nowakowski_def_tests")], String::from("test_bad_sub")), tester.test_results()[2].0);
            match tester.test_results()[2].1.error_pair() {
                Some((Error::Assert(Some(msg), Some((Value::Int(2), Value::Int(1)))), stack_trace)) => {
                    assert_eq!(String::from("left isn't equal to right"), *msg);
                    assert_eq!(1, stack_trace.len());
                    match &stack_trace[0] {
                        (Some(fun_value), pos) => {
                            assert_eq!(String::from("pl_nowakowski_def_tests::test_bad_sub"), format!("{}", fun_value));
                            let mut path_buf = PathBuf::from("tests");
                            path_buf.push("pl.nowakowski");
                            path_buf.push("def");
                            path_buf.push("tests.un");
                            assert_eq!(Pos::new(Arc::new(path_buf.to_string_lossy().into_owned()), 12, 9), *pos);
                        },
                        (_, _) => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match tester.test_results()[2].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[2].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            // test_sub
            assert_eq!((vec![String::from("pl_nowakowski_def_tests")], String::from("test_sub")), tester.test_results()[3].0);
            match tester.test_results()[3].1.error_pair() {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match tester.test_results()[3].1.stdout() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
            match tester.test_results()[3].1.stderr() {
                Some(cursor) => {
                    let cursor_g = cursor.read().unwrap();
                    assert_eq!(true, cursor_g.get_ref().is_empty())
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
