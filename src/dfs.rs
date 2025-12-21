//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub enum DfsResult<T>
{
    Success,
    Cycle(Vec<T>),
}

fn find_cycle<T: Clone + Eq + Hash>(u: &T, vs: &[T], stack: &[(T, Vec<T>)], processeds: &HashSet<T>) -> DfsResult<T>
{
    match vs.iter().find(|v| processeds.contains(v)) {
        Some(v) => {
            let mut cycle: Vec<T> = stack.iter().map(|p| p.0.clone()).collect();
            cycle.push(u.clone());
            cycle.push(v.clone());
            DfsResult::Cycle(cycle)
        },
        None => DfsResult::Success,
    }
}

pub fn dfs<T: Clone + Eq + Hash, U, F, G, E>(start: &T, visiteds: &mut HashSet<T>, data: &mut U, mut f: F, mut g: G) -> Result<DfsResult<T>, E>
    where F: FnMut(&T, &mut U) -> Result<Vec<T>, E>,
          G: FnMut(&T, &mut U) -> Result<(), E>
{
    let mut stack: Vec<(T, Vec<T>)> = Vec::new();
    let mut processeds: HashSet<T> = HashSet::new();
    processeds.insert(start.clone());
    let mut us = f(start, data)?;
    match find_cycle(start, us.as_slice(), stack.as_slice(), &processeds) {
        DfsResult::Success => (),
        res @ DfsResult::Cycle(_) => return Ok(res),
    }
    us.reverse();
    stack.push((start.clone(), us));
    visiteds.insert(start.clone());
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
                        match find_cycle(&v, ws.as_slice(), stack.as_slice(), &processeds) {
                            DfsResult::Success => (),
                            res @ DfsResult::Cycle(_) => return Ok(res),
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
    Ok(DfsResult::Success)
}

#[cfg(test)]
mod tests;
