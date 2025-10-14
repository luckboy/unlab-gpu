//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
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
