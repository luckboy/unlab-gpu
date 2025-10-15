//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::error::*;

#[derive(Clone, Debug)]
pub struct Tree(pub Vec<Node>);

#[derive(Clone, Debug)]
pub enum Node
{
    Def(Def),
    Stat(Stat),
}

#[derive(Clone, Debug)]
pub enum Def
{
    Mod(String, Arc<Mod>, Pos),
    Fun(String, Arc<Fun>, Pos),
}

#[derive(Clone, Debug)]
pub struct Mod(pub Vec<Node>);

#[derive(Clone, Debug)]
pub struct Fun(pub Vec<String>, pub Vec<Arc<Stat>>);

#[derive(Clone, Debug)]
pub enum Stat
{
    Expr(Arc<Expr>, Pos),
    Assign(Name, Arc<Expr>, Pos),
    If(Arc<Expr>, Vec<Arc<Stat>>, Vec<(Arc<Expr>, Vec<Arc<Stat>>)>, Option<Vec<Arc<Stat>>>, Pos),
    For(String, Arc<Expr>, Vec<Stat>, Pos),
    While(Arc<Expr>, Vec<Stat>, Pos),
    Return(Option<Arc<Expr>>, Pos),
}

#[derive(Clone, Debug)]
pub enum Expr
{
    Lit(Lit, Pos),
    Var(Name, Pos),
    UnaryOp(UnaryOp, Arc<Expr>, Pos),
    BinOp(BinOp, Arc<Expr>, Arc<Expr>, Pos),
    Field(Arc<Expr>, String, Pos),
    Range(Arc<Expr>, Arc<Expr>, Option<Arc<Expr>>, Pos),
}

#[derive(Clone, Debug)]
pub enum Lit
{
    Bool(bool),
    Int(i64),
    Float(f32),
    String(String),
    Matrix(Vec<MatrixRow>),
    FilledMatrix(MatrixRow, Arc<Expr>),
    Array(Vec<Arc<Expr>>),
    FilledArray(Arc<Expr>, Arc<Expr>),
    Struct(Vec<(String, Arc<Expr>)>),
}

#[derive(Clone, Debug)]
pub enum MatrixRow
{
    Row(Vec<Arc<Expr>>),
    Filled(Arc<Expr>, Arc<Expr>),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum UnaryOp
{
    Neg,
    DotNeg,
    Not,
    Transpose,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum BinOp
{
    Mul,
    DotMul,
    Div,
    DotDiv,
    Add,
    DotAdd,
    Sub,
    DotSub,
    Lt,
    Ge,
    Gt,
    Le,
    Eq,
    Ne,
    And,
    Or,
    Index,
}

#[derive(Clone, Debug)]
pub enum Name
{
    AbsGlobal(Vec<String>, String),
    RelGlobal(Vec<String>, String),
    Local(String),
}
