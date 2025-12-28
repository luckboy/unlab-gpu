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
use crate::jammdb;
use crate::toml;
use crate::matrix;
use crate::pkg::PkgName;

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
    Pkg(String),
    PkgDepCycle(Vec<PkgName>),
    Matrix(matrix::Error),
    RwLockRead,
    RwLockWrite,
    Recv,
    AlreadyAddedModNode,
    NoFunMod,
    Io(io::Error),
    Ctrlc(ctrlc::Error),
    Toml(toml::de::Error),
    Winit(Box<dyn error::Error>),
    Jammdb(jammdb::Error),
    InvalidVersion,
    InvalidPkgName,
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
            Error::Pkg(msg) => write!(f, "package error: {}", msg),
            Error::PkgDepCycle(names) => {
                write!(f, "package error: occurred cycle of dependencies: ")?;
                let mut is_first = true;
                for name in names {
                    if !is_first {
                        write!(f, " -> ")?;
                    }
                    write!(f, "{}", name)?;
                    is_first = false;
                }
                Ok(())
            },
            Error::Matrix(err) => write!(f, "matrix error: {}", err),
            Error::RwLockRead => write!(f, "can't read r/w lock"),
            Error::RwLockWrite => write!(f, "can't write r/w lock"),
            Error::Recv => write!(f, "can't receive object"),
            Error::AlreadyAddedModNode => write!(f, "already added module node"),
            Error::NoFunMod => write!(f, "no function module"),
            Error::Io(err) => write!(f, "i/o error: {}", err),
            Error::Ctrlc(err) => write!(f, "ctrl-c error: {}", err),
            Error::Toml(err) => write!(f, "toml error: {}", err),
            Error::Winit(err) => write!(f, "winit error: {}", err),
            Error::Jammdb(err) => write!(f, "jammdb error: {}", err),
            Error::InvalidVersion => write!(f, "invalid version"),
            Error::InvalidPkgName => write!(f, "invalid package name"),
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
