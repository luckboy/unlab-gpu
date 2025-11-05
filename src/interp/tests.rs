//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::doc::*;
use crate::lexer::*;
use crate::mod_node::*;
use crate::parser::*;
use super::*;

#[test]
fn test_interp_interprets_expression()
{
    let s = "X = 1 + 2";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_variable_expression()
{
    let s = "
X = 1
Y = X + 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_application_expressions()
{
    let s = "
function f()
    1
end
function g(X)
    X + 1
end
function h(X, Y, Z)
    X + Y + Z
end
X = f()
Y = g(2)
Z = h(1, 2, 3)
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Z")) {
                Some(Value::Int(6)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_unary_operator_expressions()
{
    let s = "
X = -1
Y = not true
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(-1)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_binary_operator_expressions()
{
    let s = "
X = 1 + 2
Y = 2 * 3 + 4
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(3)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Int(10)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_and_expressions()
{
    let s = "
A = false
function f()
    ::A = true
    1
end
B = false
function g()
    ::B = true
    1
end
X = true and f()
Y = false and g()
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("A")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("B")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_or_expressions()
{
    let s = "
A = false
function f()
    ::A = true
    1
end
B = false
function g()
    ::B = true
    1
end
X = true or f()
Y = false or g()
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("A")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("B")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("X")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Int(1)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_field_expressions()
{
    let s = "
X = { a: 1; b: 2.5; c: false; }
Y = X.b
Z = X.c
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(2.5));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_value, *value);
                },
                None => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(Value::Float(n)) => assert_eq!(2.5, *n),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Z")) {
                Some(Value::Bool(false)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_range_expressions()
{
    let s = "
X = 2 to 3
Y = 1 to 4 by 2
Z = 1.5 to 3
W = 1 to 2.5 by 0.5
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::IntRange(2, 3, 1)));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::IntRange(1, 4, 2)));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Z")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::FloatRange(1.5, 3.0, 1.0)));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("W")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::FloatRange(1.0, 2.5, 0.5)));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_error_propagation_expressions()
{
    let s = "
function f(X)
    X?
    2
end
X = f(A)
Y = f(B)
Z = f(C)
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut root_mod = ModNode::new(());
            root_mod.add_var(String::from("A"), Value::None);
            root_mod.add_var(String::from("B"), Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def")))));
            root_mod.add_var(String::from("C"), Value::Int(1));
            let mut env = Env::new(Arc::new(RwLock::new(root_mod)));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(Value::None) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Y")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::Error(String::from("abc"), String::from("def"))));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("Z")) {
                Some(Value::Int(2)) => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_simple_literals()
{
    let s = "
A = none
B = true
C = 1234
D = 12.45
E = \"abcdef\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("A")) {
                Some(Value::None) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("B")) {
                Some(Value::Bool(true)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("C")) {
                Some(Value::Int(1234)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("D")) {
                Some(Value::Float(n)) => assert_eq!(12.45, *n),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("E")) {
                Some(value) => {
                    let expected_value = Value::Object(Arc::new(Object::String(String::from("abcdef"))));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_matrix_literal()
{
    let s = "
X = [
    1, 2, 3
    4, 5, 6
    7, 8, 9
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let a = vec![
                        1.0, 2.0, 3.0,
                        4.0, 5.0, 6.0,
                        7.0, 8.0, 9.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, a.clone()));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_matrix_literal_with_filled_rows()
{
    let s = "
I = 0
function f()
    ::I = ::I + 1
    ::I
end
X = [
    f() fill 3
    7, 8, 9
    f() fill 3
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("I")) {
                Some(Value::Int(6)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let a = vec![
                        1.0, 2.0, 3.0,
                        7.0, 8.0, 9.0,
                        4.0, 5.0, 6.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(3, 3, TransposeFlag::NoTranspose, a.clone()));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_filled_matrix_literal()
{
    let s = "
I = 0
function f()
    ::I = ::I + 1
    ::I
end
X = [1, f(), 1; fill 4]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("I")) {
                Some(Value::Int(4)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let a = vec![
                        1.0, 1.0, 1.0,
                        1.0, 2.0, 1.0,
                        1.0, 3.0, 1.0,
                        1.0, 4.0, 1.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a.clone()));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_filled_matrix_literal_with_filled_rows()
{
    let s = "
I = 0
function f()
    ::I = ::I + 1
    ::I
end
X = [f() fill 3; fill 4]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("I")) {
                Some(Value::Int(12)) => assert!(true),
                _ => assert!(false),
            }
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let a = vec![
                        1.0, 2.0, 3.0,
                        4.0, 5.0, 6.0,
                        7.0, 8.0, 9.0,
                        10.0, 11.0, 12.0
                    ];
                    let matrix_array = Arc::new(Object::MatrixArray(4, 3, TransposeFlag::NoTranspose, a.clone()));
                    assert_eq!(Value::Object(matrix_array), value.to_matrix_array().unwrap());
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_array_literal()
{
    let s = "
X = .[ 1, 2.5, false .]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Float(2.5), Value::Bool(false)]))));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_filled_array_literal()
{
    let s = "
I = 0
function f()
    ::I = ::I + 1
    ::I
end
X = .[ f() fill 3 .]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))));
                    assert_eq!(expected_value, *value);
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_interp_interprets_structure_literal()
{
    let s = "
X = { a: 1; b: 2.5; c: false; }
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn DocIterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(tree) => {
            let mut env = Env::new(Arc::new(RwLock::new(ModNode::new(()))));
            let mut interp = Interp::new();
            match interp.interpret(&mut env, &tree) {
                Ok(()) => assert!(true),
                Err(_) => assert!(false),
            }
            assert_eq!(true, interp.stack_trace().is_empty());
            let root_mod_g = env.root_mod().read().unwrap();
            match root_mod_g.var(&String::from("X")) {
                Some(value) => {
                    let mut expected_fields: BTreeMap<String, Value> = BTreeMap::new();
                    expected_fields.insert(String::from("a"), Value::Int(1));
                    expected_fields.insert(String::from("b"), Value::Float(2.5));
                    expected_fields.insert(String::from("c"), Value::Bool(false));
                    let expected_value = Value::Ref(Arc::new(RwLock::new(MutObject::Struct(expected_fields))));
                    assert_eq!(expected_value, *value);
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
