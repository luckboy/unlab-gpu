//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! An error module.
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::result;
use std::sync::Arc;
use crate::ctrlc;
use crate::curl;
use crate::serde_json;
use crate::toml;
use crate::matrix;
use crate::pkg::PkgName;
use crate::value::Value;

/// A structure of file position.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Pos
{
    /// A path to the file.
    pub path: Arc<String>,
    /// A line in the file
    pub line: u64,
    /// A column in the file.
    pub column: usize,
}

impl Pos
{
    /// Creates a file position.
    pub fn new(path: Arc<String>, line: u64, column: usize) -> Self
    { Pos { path, line, column, } }
}

/// An enumeration of parser EOF flag.
///
/// The parser EOF flag determines whether an interpreter should read next line if parsing of
/// lines ended an EOF error.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ParserEofFlag
{
    /// No a repetition, e.i. next line isn't read.
    NoRepetition,
    /// A repetition, e.i. next line is read.
    Repetition,
}

/// An enumeration of package path conflicts.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PkgPathConflict
{
    /// A bin directory.
    Bin,
    /// A lib directory.
    Lib,
    /// A doc directory.
    Doc,
}

/// An enumeration of stop type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Stop
{
    /// A stop by a break statement.
    Break,
    /// A stop by a continue statement.
    Continue,
    /// A stop by a return statement.
    Return,
    /// A stop by a quit statement.
    Quit,
    /// A stop by an error propagation.
    ErrorPropagation,
    /// A stop by an exit built-in function. 
    Exit(i32),
}

/// An error enumeration.
#[derive(Debug)]
pub enum Error
{
    /// A parser I/O error.
    ParserIo(Arc<String>, io::Error),
    /// A parser EOF error.
    ParserEof(Arc<String>, ParserEofFlag),
    /// A parser error.
    Parser(Pos, String),
    /// An interpreter error.
    Interp(String),
    /// A package error.
    Pkg(String),
    /// A package error with package name.
    PkgName(PkgName, String),
    /// An error of package depenency cycle.
    PkgDepCycle(Vec<PkgName>),
    /// An error of package path conflicts.
    PkgPathConflicts(PkgName, Option<PkgName>, Vec<PathBuf>, PkgPathConflict),
    ///  A tester error.
    Tester(String),
    /// A matrix error.
    Matrix(matrix::Error),
    /// A mutex can't be locked.
    Mutex,
    /// A reader-writer lock can't be read.
    RwLockRead,
    /// A reader-writer lock can't be written.
    RwLockWrite,
    /// An object can't be received.
    Recv,
    /// A module node is already added.
    AlreadyAddedModNode,
    /// No a function module.
    NoFunMod,
    /// No a documentation module.
    NoDocMod,
    /// An I/O error.
    Io(io::Error),
    /// A ctrlc error.
    Ctrlc(ctrlc::Error),
    /// A toml error for deserialization.
    TomlDe(toml::de::Error),
    /// A toml error for Serialization.
    TomlSer(toml::ser::Error),
    /// A winit error.
    Winit(Box<dyn error::Error>),
    /// A jammdb error.
    Jammdb(Box<dyn error::Error>),
    /// A zip error.
    Zip(Box<dyn error::Error>),
    /// A curl error.
    Curl(curl::Error),
    /// A serde_json error.
    SerdeJson(serde_json::Error),
    /// A latex2mathml error.
    Latex2mathml(Box<dyn error::Error>),
    /// A markdown error.
    Markdown(String),
    /// An opener error.
    Opener(Box<dyn error::Error>),
    /// A version is invalid.
    InvalidVersion,
    /// A package name is invalid.
    InvalidPkgName,
    /// No an OpenCL backend.
    NoOpenClBackend,
    /// No a CUDA backend.
    NoCudaBackend,
    /// A stop error that is used by an interpreter.
    Stop(Stop),
    /// An interruption is occurred.
    Intr,
    /// An assertion error.
    Assert(Option<String>, Option<(Value, Value)>),
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
            Error::Pkg(msg) => write!(f, "{}", msg),
            Error::PkgName(name, msg) => write!(f, "{}: {}", name, msg),
            Error::PkgDepCycle(names) => {
                write!(f, "occurred cycle of dependencies: ")?;
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
            Error::PkgPathConflicts(name, name2, conflict_paths, conflict) => {
                let conflict_name = match conflict {
                    PkgPathConflict::Bin => "bin",
                    PkgPathConflict::Lib => "lib",
                    PkgPathConflict::Doc => "doc",
                };
                match name2 {
                    Some(name2) => write!(f, "occurred conflicts between {} and {} for directory {}:", name, name2, conflict_name)?,
                    None => write!(f, "occurred conflicts between {} and installed packages for directory {}:", name, conflict_name)?,
                }
                for conflict_path in conflict_paths {
                    write!(f, "\n{}", conflict_path.to_string_lossy().into_owned())?;
                }
                Ok(())
            },
            Error::Tester(msg) => write!(f, "{}", msg),
            Error::Matrix(err) => write!(f, "matrix error: {}", err),
            Error::Mutex => write!(f, "can't lock mutex"),
            Error::RwLockRead => write!(f, "can't read rw lock"),
            Error::RwLockWrite => write!(f, "can't write rw lock"),
            Error::Recv => write!(f, "can't receive object"),
            Error::AlreadyAddedModNode => write!(f, "already added module node"),
            Error::NoFunMod => write!(f, "no function module"),
            Error::NoDocMod => write!(f, "no documentation module"),
            Error::Io(err) => write!(f, "i/o error: {}", err),
            Error::Ctrlc(err) => write!(f, "ctrl-c error: {}", err),
            Error::TomlDe(err) => write!(f, "toml error: {}", err),
            Error::TomlSer(err) => write!(f, "toml error: {}", err),
            Error::Winit(err) => write!(f, "winit error: {}", err),
            Error::Jammdb(err) => write!(f, "jammdb error: {}", err),
            Error::Zip(err) => write!(f, "zip error: {}", err),
            Error::Curl(err) => write!(f, "curl error: {}", err),
            Error::SerdeJson(err) => write!(f, "serde_json error: {}", err),
            Error::Latex2mathml(err) =>  write!(f, "latex2mathml error: {}", err),
            Error::Markdown(msg) =>  write!(f, "markdown error: {}", msg),
            Error::Opener(err) => write!(f, "opener error: {}", err),
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
            Error::Assert(msg, pair) => {
                match msg {
                    Some(msg) => write!(f, "assertion failed: {}", msg)?,
                    None => write!(f, "assertion failed")?,
                }
                match pair {
                    Some((left, right)) => {
                        write!(f, "\nleft:   {}", left)?;
                        write!(f, "\nright:  {}", right)?;
                    },
                    None => (),
                }
                Ok(())
            },
        }
    }
}

/// A result type.
pub type Result<T> = result::Result<T, Error>;
