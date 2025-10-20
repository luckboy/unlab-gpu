//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use super::*;

#[test]
fn test_lexer_next_returns_tokens()
{
    let s = "+";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::Plus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_skips_comments()
{
    let s = "
# first text
+ % second text
# third text
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 13), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Plus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 16), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 13), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_returns_newline_token_for_semicolon()
{
    let s = "
+; -
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::Plus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Minus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 4), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_returns_interpuction_tokens()
{
    let s = "()[]{}.[.]?*/+-.*./.+.-<>=><== ==!='.: ::";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::LParen, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::RParen, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 2), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::LBracket, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 3), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::RBracket, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 4), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::LBrace, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::RBrace, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 6), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotLBracket, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 7), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotRBracket, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 9), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Ques, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 11), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Star, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 12), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Slash, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 13), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Plus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 14), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Minus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 15), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotStar, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 16), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotSlash, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 18), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotPlus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 20), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::DotMinus, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 22), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Lt, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 24), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::GtEq, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 25), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Gt, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 27), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::LtEq, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 28), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Eq, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 30), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::EqEq, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 32), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::ExEq, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 34), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Apos, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 36), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Dot, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 37), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Colon, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 38), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::ColonColon, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 40), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 42), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_returns_keyboard_tokens()
{
    let s = "and break by continue else end false fill for function if in inf module nan none not or return root to true while";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::And, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Break, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::By, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 11), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Continue, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 14), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Else, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 23), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::End, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 28), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::False, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 32), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Fill, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 38), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::For, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 43), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Function, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 47), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::If, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 56), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::In, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 59), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Inf, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 62), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Module, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 66), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Nan, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 73), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::None, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 77), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Not, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 82), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Or, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 86), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Return, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 89), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Root, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 96), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::To, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 101), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::True, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 104), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::While, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 109), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 114), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_returns_integer_token()
{
    let s = "
1234
0x1234abcf
0XABCF1234
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::Int(1234), pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Int(0x1234abcf), pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 11), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Int(0xabcf1234), pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 11), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_lexer_next_returns_float_token()
{
    let s = "
12.3456
1.5e12
2e10
3E10
4e+10
5e-10
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut lexer = Lexer::new(Arc::new(String::from("test.un")), &mut cursor);
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(12.3456, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 1, 8), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(1.5e12, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 2, 7), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(2e10, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 3, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(3e10, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 4, 5), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(4e10, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 5, 6), pos),
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Float(n), pos))) => {
            assert_eq!(5e-10, n);
            assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 1), pos);
        },
        _ => assert!(false),
    }
    match lexer.next() {
        Some(Ok((Token::Newline, pos))) => assert_eq!(Pos::new(Arc::new(String::from("test.un")), 6, 6), pos),
        _ => assert!(false),
    }
}
