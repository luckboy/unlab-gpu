//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::io::BufRead;
use crate::error::*;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Token
{
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LDotBracket,
    RDotBracket,
    Star,
    Slash,
    DotStar,
    DotSlash,
    Plus,
    Minus,
    Lt,
    GtEq,
    Gt,
    LtEq,
    Eq,
    EqEq,
    Apos,
    Dot,
    Colon,
    ColonColon,
    Comma,
    Nl,
    And,
    End,
    Else,
    False,
    For,
    Function,
    If,
    In,
    Module,
    Not,
    Or,
    Return,
    True,
    While,
    Int(i64),
    Float(f32),
    String(String),
    Ident(String),
}

pub struct Lexer<'a>
{
    pos: Pos,
    reader: &'a mut dyn BufRead,
    line_tokens: Vec<Result<(Token, Pos)>>,
    line_token_index: usize,
    keywords: HashMap<String, Token>,
}
