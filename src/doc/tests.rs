//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs;
use std::io::Cursor;
use sealed_test::prelude::*;
use crate::lexer::*;
use crate::parser::*;
use super::*;

#[sealed_test]
fn test_doc_tree_gen_generate_generates_documentation_tree()
{
    let s = "
%% Some text.
module a
    %% Some text2.
    %% Some text3.
    function f(X)
        X + 1
    end

    %% Some text4.
    X = 1
end

%% Some text5.
function g(X)
    X + 2
end

%% Some text6.
Y = 2
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new(parser.doc_root_mod().unwrap().clone());
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(Some(&String::from("Some text.\n")), a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(2, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("f"), (&Sig::Fun(vec![String::from("X")]), Some(&String::from("Some text2.\nSome text3.\n"))))));
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("X"), (&Sig::Var, Some(&String::from("Some text4.\n"))))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(2, var_desc_pairs.len());
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("g"), (&Sig::Fun(vec![String::from("X")]), Some(&String::from("Some text5.\n"))))));
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("Y"), (&Sig::Var, Some(&String::from("Some text6.\n"))))));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_generates_documentation_tree_without_documentation()
{
    let s = "
module a
    function f(X)
        X + 1
    end

    X = 1
end

function g(X)
    X + 2
end

Y = 2
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new(parser.doc_root_mod().unwrap().clone());
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(None, a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(2, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("f"), (&Sig::Fun(vec![String::from("X")]), None))));
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("X"), (&Sig::Var, None))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(2, var_desc_pairs.len());
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("g"), (&Sig::Fun(vec![String::from("X")]), None))));
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("Y"), (&Sig::Var, None))));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_generates_documentation_tree_with_included_files()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
%% Some text11.
module b
    %% Some text12.
    function f1(X, Y)
        X + Y
    end
end
%% Some text13.
X1 = 2
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let mut path_buf = PathBuf::from("scripts");
    path_buf.push("script2.un");
    let script2_content = "
%% Some text21.
module c
    %% Some text22.
    function f2(Y, Z)
        Y * Z
    end
end
%% Some text23.
X2 = 2
";
    fs::write(path_buf.as_path(), &script2_content[1..]).unwrap();
    let s = "
%% Some text.
module a
    runwithdoc(\"script.un\")
    runwithdoc(\"script2.un\")
end

%% Some text2.
function g(X)
    X + 2
end

%% Some text3.
Y = 2
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(Some(&String::from("Some text.\n")), a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(2, a_subtrees.len());
                    match a_subtrees.iter().find(|p| p.0 == &String::from("b")).map(|p| p.1.clone()) {
                        Some(a_b_subtree) => {
                            let a_b_subtree_g = a_b_subtree.read().unwrap();
                            assert_eq!(Some(&String::from("Some text11.\n")), a_b_subtree_g.desc());
                            let a_b_subtrees = a_b_subtree_g.subtrees();
                            assert_eq!(true, a_b_subtrees.is_empty());
                            let a_b_var_desc_pairs = a_b_subtree_g.var_desc_pairs();
                            assert_eq!(1, a_b_var_desc_pairs.len());
                            assert_eq!(true, a_b_var_desc_pairs.contains(&(&String::from("f1"), (&Sig::Fun(vec![String::from("X"), String::from("Y")]), Some(&String::from("Some text12.\n"))))));
                        },
                        None => assert!(false),
                    }
                    match a_subtrees.iter().find(|p| p.0 == &String::from("c")).map(|p| p.1.clone()) {
                        Some(a_c_subtree) => {
                            let a_c_subtree_g = a_c_subtree.read().unwrap();
                            assert_eq!(Some(&String::from("Some text21.\n")), a_c_subtree_g.desc());
                            let a_c_subtrees = a_c_subtree_g.subtrees();
                            assert_eq!(true, a_c_subtrees.is_empty());
                            let a_c_var_desc_pairs = a_c_subtree_g.var_desc_pairs();
                            assert_eq!(1, a_c_var_desc_pairs.len());
                            assert_eq!(true, a_c_var_desc_pairs.contains(&(&String::from("f2"), (&Sig::Fun(vec![String::from("Y"), String::from("Z")]), Some(&String::from("Some text22.\n"))))));
                        },
                        None => assert!(false),
                    }
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(2, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("X1"), (&Sig::Var, Some(&String::from("Some text13.\n"))))));
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("X2"), (&Sig::Var, Some(&String::from("Some text23.\n"))))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(2, var_desc_pairs.len());
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("g"), (&Sig::Fun(vec![String::from("X")]), Some(&String::from("Some text2.\n"))))));
            assert_eq!(true, var_desc_pairs.contains(&(&String::from("Y"), (&Sig::Var, Some(&String::from("Some text3.\n"))))));
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_generates_documentation_tree_with_nested_included_file()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
%% Some text3.
module c
    %% Some text4.
    function f(X, Y)
        X + Y
    end
end
%% Some text5.
X = 2
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let mut path_buf = PathBuf::from("scripts");
    path_buf.push("script2.un");
    let script2_content = "
%% Some text2.
module b
    runwithdoc(\"script.un\")
end
%% Some text6.
Y = 2
";
    fs::write(path_buf.as_path(), &script2_content[1..]).unwrap();
    let s = "
%% Some text.
module a
    runwithdoc(\"script2.un\")
end
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(Some(&String::from("Some text.\n")), a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(1, a_subtrees.len());
                    match a_subtrees.iter().find(|p| p.0 == &String::from("b")).map(|p| p.1.clone()) {
                        Some(a_b_subtree) => {
                            let a_b_subtree_g = a_b_subtree.read().unwrap();
                            assert_eq!(Some(&String::from("Some text2.\n")), a_b_subtree_g.desc());
                            let a_b_subtrees = a_b_subtree_g.subtrees();
                            match a_b_subtrees.iter().find(|p| p.0 == &String::from("c")).map(|p| p.1.clone()) {
                                Some(a_b_c_subtree) => {
                                    let a_b_c_subtree_g = a_b_c_subtree.read().unwrap();
                                    assert_eq!(Some(&String::from("Some text3.\n")), a_b_c_subtree_g.desc());
                                    let a_b_c_subtrees = a_b_c_subtree_g.subtrees();
                                    assert_eq!(true, a_b_c_subtrees.is_empty());
                                    let a_b_c_var_desc_pairs = a_b_c_subtree_g.var_desc_pairs();
                                    assert_eq!(1, a_b_c_var_desc_pairs.len());
                                    assert_eq!(true, a_b_c_var_desc_pairs.contains(&(&String::from("f"), (&Sig::Fun(vec![String::from("X"), String::from("Y")]), Some(&String::from("Some text4.\n"))))));
                                },
                                None => assert!(false),
                            }
                            let a_b_var_desc_pairs = a_b_subtree_g.var_desc_pairs();
                            assert_eq!(1, a_b_var_desc_pairs.len());
                            assert_eq!(true, a_b_var_desc_pairs.contains(&(&String::from("X"), (&Sig::Var, Some(&String::from("Some text5.\n"))))));
                        },
                        None => assert!(false),
                    }
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(1, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("Y"), (&Sig::Var, Some(&String::from("Some text6.\n"))))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(true, var_desc_pairs.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_includes_file_for_runwithdoc_absolute_name()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
X = 1
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let s = "
module a
    root::runwithdoc(\"script.un\")
end
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(None, a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(1, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("X"), (&Sig::Var, None))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(true, var_desc_pairs.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_does_not_include_file_for_defined_runwithdoc_function()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
X = 1
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let s = "
module a
    function runwithdoc(X)
        X + 1
    end
    runwithdoc(\"script.un\")
end
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(None, a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(1, a_var_desc_pairs.len());
                    assert_eq!(true, a_var_desc_pairs.contains(&(&String::from("runwithdoc"), (&Sig::Fun(vec![String::from("X")]), None))));
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(true, var_desc_pairs.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_does_not_include_file_for_runwithdoc_relative_name()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
X = 1
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let s = "
module a
    ::runwithdoc(\"script.un\")
end
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(None, a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(true, a_var_desc_pairs.is_empty());
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(true, var_desc_pairs.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_doc_tree_gen_generate_does_not_include_file_for_a_runwithdoc_absolute_name()
{
    let mut path_buf = PathBuf::from("scripts");
    fs::create_dir(path_buf.as_path()).unwrap();
    path_buf.push("script.un");
    let script_content = "
X = 1
";
    fs::write(path_buf.as_path(), &script_content[1..]).unwrap();
    let s = "
module a
    root::a::runwithdoc(\"script.un\")
end
";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new_with_doc_flag(Arc::new(String::from("test.un")), &mut cursor, true);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new_with_doc_root_mod(path, tokens, Some(Arc::new(RwLock::new(ModNode::new(None)))));
    let tree = parser.parse().unwrap();
    let mut doc_tree_gen = DocTreeGen::new_with_script_dir(parser.doc_root_mod().unwrap().clone(), PathBuf::from("scripts"));
    match doc_tree_gen.generate(&tree) {
        Ok(doc_tree) => {
            let doc_tree_g = doc_tree.read().unwrap();
            assert_eq!(None, doc_tree_g.desc());
            let subtrees = doc_tree_g.subtrees();
            assert_eq!(1, subtrees.len());
            match subtrees.iter().find(|p| p.0 == &String::from("a")).map(|p| p.1.clone()) {
                Some(a_subtree) => {
                    let a_subtree_g = a_subtree.read().unwrap();
                    assert_eq!(None, a_subtree_g.desc());
                    let a_subtrees = a_subtree_g.subtrees();
                    assert_eq!(true, a_subtrees.is_empty());
                    let a_var_desc_pairs = a_subtree_g.var_desc_pairs();
                    assert_eq!(true, a_var_desc_pairs.is_empty());
                },
                None => assert!(false),
            }
            let var_desc_pairs = doc_tree_g.var_desc_pairs();
            assert_eq!(true, var_desc_pairs.is_empty());
        },
        Err(_) => assert!(false),
    }
}
