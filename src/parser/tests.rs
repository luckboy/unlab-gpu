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
