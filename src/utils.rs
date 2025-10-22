//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use crate::error::*;

#[derive(Clone)]
pub struct PushbackIter<T: Iterator>
{
    iter: T,
    pushed_items: Vec<T::Item>,
}

impl<T: Iterator> PushbackIter<T>
{
    pub fn new(iter: T) -> Self
    { PushbackIter { iter, pushed_items: Vec::new(), } }

    pub fn iter(&self) -> &T
    { &self.iter }
    
    pub fn iter_mut(&mut self) -> &mut T
    { &mut self.iter }

    pub fn undo(&mut self, item: T::Item)
    { self.pushed_items.push(item); }
}

impl<T: Iterator> Iterator for PushbackIter<T>
{
    type Item = T::Item;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.pushed_items.pop() {
            Some(item) => Some(item),
            None => self.iter.next(),
        }
    }
}

pub fn str_without_crnl(s: &str) -> &str
{
    if s.ends_with('\n') {
        let s2 = &s[0..(s.len() - 1)];
        if s2.ends_with('\r') {
            &s2[0..(s2.len() - 1)]
        } else {
            s2
        }
    } else {
        s
    }
}

pub fn rw_lock_read<T>(rw_lock: &RwLock<T>) -> Result<RwLockReadGuard<'_, T>>
{
    match rw_lock.read() {
        Ok(guard) => Ok(guard),
        Err(_) => Err(Error::RwLockRead),
    }
}

pub fn rw_lock_write<T>(rw_lock: &RwLock<T>) -> Result<RwLockWriteGuard<'_, T>>
{
    match rw_lock.write() {
        Ok(guard) => Ok(guard),
        Err(_) => Err(Error::RwLockRead),
    }
}
