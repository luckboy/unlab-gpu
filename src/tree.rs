//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A module of syntax tree.
use std::fmt;
use std::sync::Arc;
use crate::error::*;

/// A structure of syntax tree.
///
/// The syntax tree creates from tokens by a parser. An interpreter can take the syntax tree to
/// interpretation.
#[derive(Clone, Debug)]
pub struct Tree(pub Vec<Node>);

/// A node enumeration.
#[derive(Clone, Debug)]
pub enum Node
{
    /// A definition.
    Def(Box<Def>),
    /// A statement.
    Stat(Box<Stat>),
}

/// A definition enumeration.
#[derive(Clone, Debug)]
pub enum Def
{
    /// A module.
    Mod(String, Box<Mod>, Pos),
    /// A function.
    Fun(String, Arc<Fun>, Pos),
}

impl Def
{
    /// Returns the file position.
    pub fn pos(&self) -> &Pos
    {
        match self {
            Def::Mod(_, _, pos) => pos,
            Def::Fun(_, _, pos) => pos,
        }
    }

    /// Sets the file position.
    pub fn set_pos(&mut self, pos: Pos)
    {
        match self {
            Def::Mod(_, _, pos2) => *pos2 = pos,
            Def::Fun(_, _, pos2) => *pos2 = pos,
        }
    }
}

/// A module structure.
#[derive(Clone, Debug)]
pub struct Mod(pub Vec<Node>);

/// A function structure.
#[derive(Clone, Debug)]
pub struct Fun(pub Vec<Arg>, pub Vec<Box<Stat>>);

/// An argument structure.
#[derive(Clone, Debug)]
pub struct Arg(pub String, pub Pos);

impl Arg
{
    /// Returns the file position.
    pub fn pos(&self) -> &Pos
    { &self.1 }
    
    /// Sets the file position.
    pub fn set_pos(&mut self, pos: Pos)
    { self.1 = pos; }
}

/// A statement enumeration.
#[derive(Clone, Debug)]
pub enum Stat
{
    /// An expression statement.
    Expr(Box<Expr>, Pos),
    /// An assignment statement.
    Assign(Box<Expr>, Box<Expr>, Pos),
    /// An `if` statement.
    If(Box<Expr>, Vec<Box<Stat>>, Vec<(Box<Expr>, Vec<Box<Stat>>)>, Option<Vec<Box<Stat>>>, Pos),
    /// A `for` statement.
    For(String, Box<Expr>, Vec<Box<Stat>>, Pos),
    /// A `while` statement.
    While(Box<Expr>, Vec<Box<Stat>>, Pos),
    /// A `break` statement.
    Break(Pos),
    /// A `continue` statement.
    Continue(Pos),
    /// A `return` statement.
    Return(Option<Box<Expr>>, Pos),
    /// A `quit` statement.
    Quit(Pos),
}

impl Stat
{
    /// Returns the file position.
    pub fn pos(&self) -> &Pos
    {
        match self {
            Stat::Expr(_, pos) => pos,
            Stat::Assign(_, _, pos) => pos,
            Stat::If(_, _, _, _, pos) => pos,
            Stat::For(_, _, _, pos) => pos,
            Stat::While(_, _, pos) => pos,
            Stat::Break(pos) => pos,
            Stat::Continue(pos) => pos,
            Stat::Return(_, pos) => pos,
            Stat::Quit(pos) => pos,
        }
    }

    /// Sets the file position.
    pub fn set_pos(&mut self, pos: Pos)
    {
        match self {
            Stat::Expr(_, pos2) => *pos2 = pos,
            Stat::Assign(_, _, pos2) => *pos2 = pos,
            Stat::If(_, _, _, _, pos2) => *pos2 = pos,
            Stat::For(_, _, _, pos2) => *pos2 = pos,
            Stat::While(_, _, pos2) => *pos2 = pos,
            Stat::Break(pos2) => *pos2 = pos,
            Stat::Continue(pos2) => *pos2 = pos,
            Stat::Return(_, pos2) => *pos2 = pos,
            Stat::Quit(pos2) => *pos2 = pos,
        }
    }
}

/// An expression.
#[derive(Clone, Debug)]
pub enum Expr
{
    /// A literal.
    Lit(Lit, Pos),
    /// A variable.
    Var(Name, Pos),
    /// A function application.
    App(Box<Expr>, Vec<Box<Expr>>, Pos),
    /// An expression of unary operator.
    UnaryOp(UnaryOp, Box<Expr>, Pos),
    /// An expression of binary operator.
    BinOp(BinOp, Box<Expr>, Box<Expr>, Pos),
    /// A logical AND expression.
    And(Box<Expr>, Box<Expr>, Pos),
    /// A logical OR expression.
    Or(Box<Expr>, Box<Expr>, Pos),
    /// A field access.
    Field(Box<Expr>, String, Pos),
    /// A range.
    Range(Box<Expr>, Box<Expr>, Option<Box<Expr>>, Pos),
    /// An error propagation.
    PropagateError(Box<Expr>, Pos),
}

impl Expr
{
    /// Returns the file position.
    pub fn pos(&self) -> &Pos
    {
        match self {
            Expr::Lit(_, pos) => pos,
            Expr::Var(_, pos) => pos,
            Expr::App(_, _, pos) => pos,
            Expr::UnaryOp(_, _, pos) => pos,
            Expr::BinOp(_, _, _, pos) => pos,
            Expr::And(_, _, pos) => pos,
            Expr::Or(_, _, pos) => pos,
            Expr::Field(_, _, pos) => pos,
            Expr::Range(_, _, _, pos) => pos,
            Expr::PropagateError(_, pos) => pos,
        }
    }

    /// Sets the file position.
    pub fn set_pos(&mut self, pos: Pos)
    {
        match self {
            Expr::Lit(_, pos2) => *pos2 = pos,
            Expr::Var(_, pos2) => *pos2 = pos,
            Expr::App(_, _, pos2) => *pos2 = pos,
            Expr::UnaryOp(_, _, pos2) => *pos2 = pos,
            Expr::BinOp(_, _, _, pos2) => *pos2 = pos,
            Expr::And(_, _, pos2) => *pos2 = pos,
            Expr::Or(_, _, pos2) => *pos2 = pos,
            Expr::Field(_, _, pos2) => *pos2 = pos,
            Expr::Range(_, _, _, pos2) => *pos2 = pos,
            Expr::PropagateError(_, pos2) => *pos2 = pos,
        }
    }
}

/// A literal.
#[derive(Clone, Debug)]
pub enum Lit
{
    /// A none literal.
    None,
    /// A boolean literal
    Bool(bool),
    /// An integer number literal.
    Int(i64),
    /// A floating-point number literal.
    Float(f32),
    /// A stribg literal.
    String(String),
    /// A matrix literal.
    Matrix(Vec<MatrixRow>),
    /// A filled matrix literal.
    FilledMatrix(MatrixRow, Box<Expr>),
    /// An array literal.
    Array(Vec<Box<Expr>>),
    /// A filled array literal with one value.
    FilledArray(Box<Expr>, Box<Expr>),
    /// A structure literal.
    Struct(Vec<FieldPair>),
}

/// An enumeration of matrix row.
#[derive(Clone, Debug)]
pub enum MatrixRow
{
    /// An matrix row.
    Row(Vec<Box<Expr>>),
    /// A filled matrix row with one value.
    FilledRow(Box<Expr>, Box<Expr>),
}

/// A structure of field pair. 
#[derive(Clone, Debug)]
pub struct FieldPair(pub String, pub Box<Expr>, pub Pos);

impl FieldPair
{
    /// Returns the file position.
    pub fn pos(&self) -> &Pos
    { &self.2 }
    
    /// Sets the file position.
    pub fn set_pos(&mut self, pos: Pos)
    { self.2 = pos; }
}

/// An enumeration of unary operator.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum UnaryOp
{
    /// A negation.
    Neg,
    /// A dot negation.
    DotNeg,
    /// A logical NOT.
    Not,
    /// A transpose.
    Transpose,
}

/// An enumeration of binary operator.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum BinOp
{
    /// An index operator.
    Index,
    /// A multiplication.
    Mul,
    /// A dot multiplication.
    DotMul,
    /// A division.
    Div,
    /// A dot division.
    DotDiv,
    /// An addition.
    Add,
    /// A dot addition.
    DotAdd,
    /// A subtraction.
    Sub,
    /// A dot subtraction.
    DotSub,
    /// Less than.
    Lt,
    /// Greater than or equal to.
    Ge,
    /// Greater than.
    Gt,
    /// Less than or equal to.
    Le,
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
}

/// An enumeration of variable name.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Name
{
    /// An absolute name with identifiers of modules and a variable identifier.
    Abs(Vec<String>, String),
    /// A relative name with identifiers of modules and a variable identifier.
    Rel(Vec<String>, String),
    /// A name with one identifier.
    Var(String),
}

impl fmt::Display for Name
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Name::Abs(idents, ident) => {
                write!(f, "root")?;
                for ident2 in idents {
                    write!(f, "::{}", ident2)?;
                }
                write!(f, "::{}", ident)
            },
            Name::Rel(idents, ident) => {
                let mut is_first = true;
                for ident2 in idents {
                    if !is_first {
                        write!(f, "::{}", ident2)?;
                    } else {
                        write!(f, "{}", ident2)?;
                    }
                    is_first = false;
                }
                write!(f, "::{}", ident)
            },
            Name::Var(ident) => write!(f, "{}", ident),
        }
    }
}
