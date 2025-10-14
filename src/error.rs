//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use std::io;
use std::result;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Pos
{
    pub path: Arc<String>,
    pub line: u64,
    pub column: usize,
}

impl Pos
{
    pub fn new(path: Arc<String>, line: u64, column: usize) -> Self
    { Pos { path, line, column, } }
}

#[derive(Debug)]
pub enum Error
{
    ParserIo(Arc<String>, io::Error),
    ParserEof(Pos),
    Parser(Pos, String),
}

impl error::Error for Error
{}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Error::ParserIo(path, err) => write!(f, "{}: i/o error: {}", path, err),
            Error::ParserEof(pos) => write!(f, "{}: {}.{}: end of file", pos.path, pos.line, pos.column),
            Error::Parser(pos, msg) => write!(f, "{}: {}.{}: {}", pos.path, pos.line, pos.column, msg),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
