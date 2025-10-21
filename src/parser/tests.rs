//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::lexer::*;
use super::*;

#[test]
fn test_parser_parse_parses_expression()
{
    let s = "X + 1";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Add, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            assert_eq!(Name::Var(String::from("X")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression1()
{
    let s = "true and true or false and false";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Or, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::And, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Bool(true), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Bool(true), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::And, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 18), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Bool(false), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 18), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Bool(false), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 28), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression2()
{
    let s = "1 == 2 and 3 != 4";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::And, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::Eq, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 6), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::Ne, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 12), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 12), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 17), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression3()
{
    let s = "1 < 2 == 3 > 4";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Eq, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::Lt, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::Gt, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 14), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression4()
{
    let s = "1 to 2 by 3 < 4 to 5";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Lt, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Range(expr4, expr5, Some(expr6), pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 6), *pos),
                                                _ => assert!(false),                                                          
                                            }
                                            match &**expr6 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Range(expr4, expr5, None, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 15), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 15), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(5), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 20), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression5()
{
    let s = "1 + 2 to 3 - 4";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Range(expr2, expr3, None, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::Add, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::Sub, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 14), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression6()
{
    let s = "1 * 2 + 3 / 4";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Add, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::Mul, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::Div, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 9), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 9), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 13), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression7()
{
    let s = "-1 * not true";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Mul, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::UnaryOp(UnaryOp::Neg, expr4, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::UnaryOp(UnaryOp::Not, expr4, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 6), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Bool(true), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 10), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression8()
{
    let s = "-1?";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::UnaryOp(UnaryOp::Neg, expr2, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::PropagateError(expr3, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), *pos);
                                            match &**expr3 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression_with_parenthesis()
{
    let s = "(1 + 2) * (3 - 4)";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Mul, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::BinOp(BinOp::Add, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 6), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::BinOp(BinOp::Sub, expr4, expr5, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 11), *pos);
                                            match &**expr4 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 12), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr5 {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 16), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_application_expressions()
{
    let s = "
f()
g(1)
h(2, 3, 4)
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(3, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::App(expr2, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            assert_eq!(Name::Var(String::from("f")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, args.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[1] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                            match &**expr {
                                Expr::App(expr2, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                                            assert_eq!(Name::Var(String::from("g")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, args.len());
                                    match &*args[0] {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 3), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[2] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                            match &**expr {
                                Expr::App(expr2, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                                            assert_eq!(Name::Var(String::from("h")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(3, args.len());
                                    match &*args[0] {
                                        Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 3), *pos),
                                        _ => assert!(false),
                                    }
                                    match &*args[1] {
                                        Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 6), *pos),
                                        _ => assert!(false),
                                    }
                                    match &*args[2] {
                                        Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 9), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_index_expression()
{
    let s = "X[1]";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Index, expr2, expr3, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            assert_eq!(Name::Var(String::from("X")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 3), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_field_expression()
{
    let s = "X.a";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Field(expr2, ident, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            assert_eq!(Name::Var(String::from("X")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("a"), *ident);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_simple_literals()
{
    let s = "
none
false
true
1234
12.45
inf
nan
\"abcdef\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(8, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::None, pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[1] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Bool(false), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[2] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Bool(true), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[3] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Int(1234), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 1), *pos),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[4] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Float(n), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 1), *pos);
                                    assert_eq!(12.45, *n);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[5] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Float(n), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 1), *pos);
                                    assert_eq!(f32::INFINITY, *n);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[6] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 7, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Float(n), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 7, 1), *pos);
                                    assert_eq!(true, n.is_nan());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[7] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 8, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::String(t), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 8, 1), *pos);
                                    assert_eq!(String::from("abcdef"), *t);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_matrix_literal()
{
    let s = "
[
    1, 2, 3

    4, 5, 6

    7, 8, 9
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Matrix(matrix_rows), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(3, matrix_rows.len());
                                    match &matrix_rows[0] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[1] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(5), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(6), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[2] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(7), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(8), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(9), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_matrix_literal_with_one_row()
{
    let s = "
[
    1, 2, 3
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Matrix(matrix_rows), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(1, matrix_rows.len());
                                    match &matrix_rows[0] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_matrix_literal_with_one_column()
{
    let s = "
[
    1
    2
    3
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Matrix(matrix_rows), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(3, matrix_rows.len());
                                    match &matrix_rows[0] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[1] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[2] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 5), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_matrix_literal_with_filled_rows()
{
    let s = "
[
    1 fill 3
    2, 3, 4
    5 fill n
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Matrix(matrix_rows), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(3, matrix_rows.len());
                                    match &matrix_rows[0] {
                                        MatrixRow::FilledRow(expr2, expr3) => {
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr3 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 12), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[1] {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &matrix_rows[2] {
                                        MatrixRow::FilledRow(expr2, expr3) => {
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(5), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr3 {
                                                Expr::Var(name, pos) => {
                                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 12), *pos);
                                                    assert_eq!(Name::Var(String::from("n")), *name);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_filled_matrix_literal()
{
    let s = "
[
    1, 2, 3

    fill 4
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::FilledMatrix(matrix_row, expr2), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match matrix_row {
                                        MatrixRow::Row(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos),
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 11), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr2 {
                                        Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 10), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_filled_matrix_literal_with_filled_rows()
{
    let s = "
[
    1 fill 3; fill 4
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::FilledMatrix(matrix_row, expr2), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &matrix_row {
                                        MatrixRow::FilledRow(expr2, expr3) => {
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                                _ => assert!(false),
                                            }
                                            match &**expr3 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 12), *pos),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr2 {
                                        Expr::Lit(Lit::Int(4), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 20), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_empty_matrix_literal()
{
    let s = "
[
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Matrix(matrix_rows), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(true, matrix_rows.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_array_literal()
{
    let s = "
.[
    1, 2, 3
.]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Array(exprs), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(3, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos),
                                        _ => assert!(false),
                                    }
                                    match &*exprs[2] {
                                        Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 11), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_array_literal_with_one_element()
{
    let s = "
.[
    1
.]";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Array(exprs), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(1, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn test_parser_parse_parses_filled_array_literal()
{
    let s = "
.[
    1 fill 3
.]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::FilledArray(expr2, expr3), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos),
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 12), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn test_parser_parse_parses_empty_array_literal()
{
    let s = "
.[
.]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Array(exprs), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(true, exprs.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn test_parser_parse_parses_structure_literal()
{
    let s = "
{
    a: 1
    
    b: 2

    c: 3
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Struct(field_pairs), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(3, field_pairs.len());
                                    match &field_pairs[0] {
                                        FieldPair(ident, expr2, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos);
                                            assert_eq!(String::from("a"), *ident);
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos),
                                                _ => assert!(false),
                                            }
                                       },
                                    }
                                    match &field_pairs[1] {
                                        FieldPair(ident, expr2, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 5), *pos);
                                            assert_eq!(String::from("b"), *ident);
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(2), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 8), *pos),
                                                _ => assert!(false),
                                            }
                                       },
                                    }
                                    match &field_pairs[2] {
                                        FieldPair(ident, expr2, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 5), *pos);
                                            assert_eq!(String::from("c"), *ident);
                                            match &**expr2 {
                                                Expr::Lit(Lit::Int(3), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 8), *pos),
                                                _ => assert!(false),
                                            }
                                       },
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn test_parser_parse_parses_empty_structure_literal()
{
    let s = "
{
}
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Struct(field_pairs), pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(true, field_pairs.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn test_parser_parse_parses_names()
{
    let s = "
X
a::b::Y
root::c::d::Z
::X2
::a2::b2::Y2
::root::c2::d2::Z2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(6, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(Name::Var(String::from("X")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[1] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                                    assert_eq!(Name::Rel(vec![String::from("a"), String::from("b")], String::from("Y")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[2] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                                    assert_eq!(Name::Abs(vec![String::from("c"), String::from("d")], String::from("Z")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[3] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 1), *pos);
                                    assert_eq!(Name::Rel(Vec::new(), String::from("X2")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[4] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 1), *pos);
                                    assert_eq!(Name::Rel(vec![String::from("a2"), String::from("b2")], String::from("Y2")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[5] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 1), *pos);
                                    assert_eq!(Name::Abs(vec![String::from("c2"), String::from("d2")], String::from("Z2")), *name);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression_statement()
{
    let s = "
f()
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Expr(expr, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::App(expr2, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    match &**expr2 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                            assert_eq!(Name::Var(String::from("f")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, args.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_assigment_statements()
{
    let s = "
X = f()
Y[1] = g()
Z.a = h()
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(3, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Assign(expr, expr2, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Var(name, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                                    assert_eq!(Name::Var(String::from("X")), *name);
                                },
                                _ => assert!(false),
                            }
                            match &**expr2 {
                                Expr::App(expr3, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos);
                                    match &**expr3 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), *pos);
                                            assert_eq!(Name::Var(String::from("f")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, args.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[1] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Assign(expr, expr2, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                            match &**expr {
                                Expr::BinOp(BinOp::Index, expr3, expr4, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                                    match &**expr3 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), *pos);
                                            assert_eq!(Name::Var(String::from("Y")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr4 {
                                        Expr::Lit(Lit::Int(1), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 3), *pos),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &**expr2 {
                                Expr::App(expr3, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos);
                                    match &**expr3 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 8), *pos);
                                            assert_eq!(Name::Var(String::from("g")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, args.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match &nodes[2] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::Assign(expr, expr2, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                            match &**expr {
                                Expr::Field(expr3, ident, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                                    match &**expr3 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), *pos);
                                            assert_eq!(Name::Var(String::from("Z")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("a"), *ident);
                                },
                                _ => assert!(false),
                            }
                            match &**expr2 {
                                Expr::App(expr3, args, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 7), *pos);
                                    match &**expr3 {
                                        Expr::Var(name, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 7), *pos);
                                            assert_eq!(Name::Var(String::from("h")), *name);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, args.is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_if_statement()
{
    let s = "
if true
    f()
    g()
end
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    let path = lexer.path().clone();
    let tokens: &mut dyn Iterator<Item = Result<(Token, Pos)>> = &mut lexer;
    let mut parser = Parser::new(path, tokens);
    match parser.parse() {
        Ok(Tree(nodes)) => {
            assert_eq!(1, nodes.len());
            match &nodes[0] {
                Node::Stat(stat) => {
                    match &**stat {
                        Stat::If(expr, stats, else_if_pairs, None, pos) => {
                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), *pos);
                            match &**expr {
                                Expr::Lit(Lit::Bool(true), pos) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 4), *pos),
                                _ => assert!(false),
                            }
                            assert_eq!(2, stats.len());
                            match &*stats[0] {
                                Stat::Expr(expr2, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos);
                                    match &**expr2 {
                                        Expr::App(expr3, args, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos);
                                            match &**expr3 {
                                                Expr::Var(name, pos) => {
                                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 5), *pos);
                                                    assert_eq!(Name::Var(String::from("f")), *name);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(true, args.is_empty());
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &*stats[1] {
                                Stat::Expr(expr2, pos) => {
                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), *pos);
                                    match &**expr2 {
                                        Expr::App(expr3, args, pos) => {
                                            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), *pos);
                                            match &**expr3 {
                                                Expr::Var(name, pos) => {
                                                    assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), *pos);
                                                    assert_eq!(Name::Var(String::from("g")), *name);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(true, args.is_empty());
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, else_if_pairs.is_empty());
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
