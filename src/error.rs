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
use crate::matrix;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ParserEofFlag
{
    NoRepetition,
    Repetition,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Stop
{
    Break,
    Continue,
    Return,
    Quit,
}

#[derive(Debug)]
pub enum Error
{
    ParserIo(Arc<String>, io::Error),
    ParserEof(Arc<String>, ParserEofFlag),
    Parser(Pos, String),
    Interp(String),
    Matrix(matrix::Error),
    RwLockRead,
    RwLockWrite,
    AlreadyAddedModNode,
    NoFunMod,
    Stop(Stop),
}

impl error::Error for Error
{}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Error::ParserIo(path, err) => write!(f, "{}: i/o error: {}", path, err),
            Error::ParserEof(path, _) => write!(f, "{}: end of file", path),
            Error::Parser(pos, msg) => write!(f, "{}: {}.{}: {}", pos.path, pos.line, pos.column, msg),
            Error::Interp(msg) => write!(f, "{}", msg),
            Error::Matrix(err) => write!(f, "matrix error: {}", err),
            Error::RwLockRead => write!(f, "can't read r/w lock"),
            Error::RwLockWrite => write!(f, "can't write r/w lock"),
            Error::AlreadyAddedModNode => write!(f, "already added module node"),
            Error::NoFunMod => write!(f, "no function module"),
            Error::Stop(Stop::Break) => write!(f, "stopped by break"),
            Error::Stop(Stop::Continue) => write!(f, "stopped by continue"),
            Error::Stop(Stop::Return) => write!(f, "stopped by return"),
            Error::Stop(Stop::Quit) => write!(f, "stopped by quit"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
