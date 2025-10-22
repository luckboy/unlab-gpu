//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub trait DocIterator: Iterator
{
    fn take_doc(&mut self) -> Option<String>;
}

pub struct DocIter<T: Iterator>
{
    iter: T,
}

impl<T: Iterator> DocIter<T>
{
    pub fn new(iter: T) -> Self
    { DocIter { iter, } }

    pub fn iter(&self) -> &T
    { &self.iter }
    
    pub fn iter_mut(&mut self) -> &mut T
    { &mut self.iter }
}

impl<T: Iterator> Iterator for DocIter<T>
{
    type Item = T::Item;
    
    fn next(&mut self) -> Option<Self::Item>
    { self.iter.next() }
}

impl<T: Iterator> DocIterator for DocIter<T>
{
    fn take_doc(&mut self) -> Option<String>
    { None }
}
