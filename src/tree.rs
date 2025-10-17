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
pub struct Tree(pub Arc<Mod>);

#[derive(Clone, Debug)]
pub struct Mod(pub Vec<Node>);

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

impl Def
{
    pub fn pos(&self) -> &Pos
    {
        match &self {
            Def::Mod(_, _, pos) => pos,
            Def::Fun(_, _, pos) => pos,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Fun(pub Vec<String>, pub Vec<Box<Stat>>);

#[derive(Clone, Debug)]
pub enum Stat
{
    Expr(Box<Expr>, Pos),
    Assign(Box<Expr>, Box<Expr>, Pos),
    If(Box<Expr>, Vec<Box<Stat>>, Vec<(Box<Expr>, Vec<Box<Stat>>)>, Option<Vec<Box<Stat>>>, Pos),
    For(String, Box<Expr>, Vec<Stat>, Pos),
    While(Box<Expr>, Vec<Box<Stat>>, Pos),
    Return(Option<Box<Expr>>, Pos),
}

impl Stat
{
    pub fn pos(&self) -> &Pos
    {
        match &self {
            Stat::Expr(_, pos) => pos,
            Stat::Assign(_, _, pos) => pos,
            Stat::If(_, _, _, _, pos) => pos,
            Stat::For(_, _, _, pos) => pos,
            Stat::While(_, _, pos) => pos,
            Stat::Return(_, pos) => pos,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr
{
    Lit(Lit, Pos),
    Var(Name, Pos),
    App(Box<Expr>, Vec<Box<Expr>>, Pos),
    UnaryOp(UnaryOp, Box<Expr>, Pos),
    BinOp(BinOp, Box<Expr>, Box<Expr>, Pos),
    Field(Box<Expr>, String, Pos),
    Range(Box<Expr>, Box<Expr>, Option<Box<Expr>>, Pos),
}

impl Expr
{
    pub fn pos(&self) -> &Pos
    {
        match &self {
            Expr::Lit(_, pos) => pos,
            Expr::Var(_, pos) => pos,
            Expr::App(_, _, pos) => pos,
            Expr::UnaryOp(_, _, pos) => pos,
            Expr::BinOp(_, _, _, pos) => pos,
            Expr::Field(_, _, pos) => pos,
            Expr::Range(_, _, _, pos) => pos,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Lit
{
    Bool(bool),
    Int(i64),
    Float(f32),
    String(String),
    Matrix(Vec<MatrixRow>),
    FilledMatrix(MatrixRow, Box<Expr>),
    Array(Vec<Box<Expr>>),
    FilledArray(Box<Expr>, Box<Expr>),
    Struct(Vec<FieldPair>),
}

#[derive(Clone, Debug)]
pub enum MatrixRow
{
    Row(Vec<Box<Expr>>),
    FilledRow(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct FieldPair(pub String, pub Box<Expr>, pub Pos);

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
    Index,
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
}

#[derive(Clone, Debug)]
pub enum Name
{
    Abs(Vec<String>, String),
    Rel(Vec<String>, String),
    Var(String),
}
