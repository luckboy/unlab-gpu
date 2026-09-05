//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::any::Any;
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use crate::env::*;
use crate::error::*;
use crate::interp::*;
use crate::value::Value;

pub struct UserFun(pub Box<dyn Fn(&mut Interp, &mut Env, &[Value]) -> Result<Value> + Send + Sync>);

impl UserFun
{
    pub fn new<F>(f: F) -> Self
        where F: Fn(&mut Interp, &mut Env, &[Value]) -> Result<Value> + Send + Sync + 'static
    { UserFun(Box::new(f)) }
}

impl fmt::Debug for UserFun
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "UserFun(...)") }
}

#[derive(Clone)]
pub struct UserObject(pub Arc<dyn Any + Send + Sync>);

impl UserObject
{
    pub fn new<T: Send + Sync + 'static>(data: T) -> Self
    { UserObject(Arc::new(data)) }
}

impl fmt::Debug for UserObject
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "UserObject(...)") }
}

#[derive(Clone)]
pub struct MutUserObject(pub Arc<RwLock<dyn Any + Send + Sync>>);

impl MutUserObject
{
    pub fn new<T: Send + Sync + 'static>(data: T) -> Self
    { MutUserObject(Arc::new(RwLock::new(data))) }
}

impl fmt::Debug for MutUserObject
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "MutUserObject(...)") }
}
