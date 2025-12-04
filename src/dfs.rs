//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashSet;
use std::hash::Hash;

pub fn dfs<T: Clone + Eq + Hash, U, F, G, E>(start: T, data: &mut U, mut f: F, mut g: G) -> Result<bool, E>
    where F: FnMut(&T, &mut U) -> Result<Vec<T>, E>,
          G: FnMut(&T, &mut U) -> Result<(), E>
{
    let mut stack: Vec<(T, Vec<T>)> = Vec::new();
    let mut visiteds: HashSet<T> = HashSet::new();
    let mut processeds: HashSet<T> = HashSet::new();
    processeds.insert(start.clone());
    let mut us = f(&start, data)?;
    us.reverse();
    stack.push((start.clone(), us));
    visiteds.insert(start);
    loop {
        match stack.last_mut() {
            Some((u, vs)) => {
                let v = loop {
                    match vs.pop() {
                        Some(w) if !visiteds.contains(&w) => break Some(w),
                        Some(_) => (),
                        None => break None,
                    }
                };
                match v {
                    Some(v) => {
                        processeds.insert(v.clone());
                        let mut ws = f(&v, data)?;
                        if ws.iter().any(|w| processeds.contains(w)) {
                            return Ok(false);
                        }
                        ws.reverse();
                        stack.push((v.clone(), ws));
                        visiteds.insert(v);
                    },
                    None => {
                        g(u, data)?;
                        processeds.remove(u);
                        stack.pop();
                    },
                }
            },
            None => break,
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests;
