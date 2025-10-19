//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Weak;
use crate::matrix::Matrix;
use crate::tree::*;

#[derive(Clone, Debug)]
pub enum Value
{
    None,
    Bool(bool),
    Int(i64),
    Float(f32),
    Object(Arc<Object>),
    Ref(Arc<RwLock<MutObject>>),
    Weak(Weak<RwLock<MutObject>>),
}

#[derive(Clone, Debug)]
pub enum Object
{
    String(String),
    IntRange(i64, i64, i64),
    FloatRange(f32, f32, f32),
    Matrix(Matrix),
    Fun(Vec<String>, String, Arc<Fun>),
    MatrixArray(usize, usize, TransposeFlag, Vec<f32>),
    MatrixRowSlice(Arc<Object>, usize),
    Error(String, String),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TransposeFlag
{
    NoTranspose,
    Transpose,
}

#[derive(Clone, Debug)]
pub enum MutObject
{
    Array(Vec<Value>),
    Struct(BTreeMap<String, Value>),
}
