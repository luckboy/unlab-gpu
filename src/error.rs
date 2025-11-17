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
use crate::ctrlc;
use crate::ini;
use crate::matrix;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParserEofFlag
{
    NoRepetition,
    Repetition,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Stop
{
    Break,
    Continue,
    Return,
    Quit,
    ErrorPropagation,
    Exit(i32),
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
    Io(io::Error),
    Ctrlc(ctrlc::Error),
    Ini(ini::ParseError),
    InvalidIniField(String),
    NoOpenClBackend,
    NoCudaBackend,
    Stop(Stop),
    Intr,
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
            Error::Io(err) => write!(f, "i/o error: {}", err),
            Error::Ctrlc(err) => write!(f, "ctrl-c error: {}", err),
            Error::Ini(err) => write!(f, "ini error: {}", err),
            Error::InvalidIniField(field_name) => write!(f, "invalid ini field {}", field_name),
            Error::NoOpenClBackend => write!(f, "no OpenCL backend"),
            Error::NoCudaBackend => write!(f, "no CUDA backend"),
            Error::Stop(Stop::Break) => write!(f, "stopped by break"),
            Error::Stop(Stop::Continue) => write!(f, "stopped by continue"),
            Error::Stop(Stop::Return) => write!(f, "stopped by return"),
            Error::Stop(Stop::Quit) => write!(f, "stopped by quit"),
            Error::Stop(Stop::ErrorPropagation) => write!(f, "stopped by error propagation"),
            Error::Stop(Stop::Exit(code)) => write!(f, "stopped by exit with code {}", code),
            Error::Intr => write!(f, "interrupted"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
