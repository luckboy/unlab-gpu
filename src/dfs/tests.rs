//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_dfs_traverses_graph()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2, 3, 5], // 0
        vec![2], // 1
        vec![3, 4], // 2
        Vec::new(), // 3
        Vec::new(), // 4
        Vec::new() // 5
    ];
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 2, 3, 4, 5], data.0);
    assert_eq!(vec![3, 4, 2, 1, 5, 0], data.1);
}

#[test]
fn test_dfs_traverses_graph_with_little_cycle()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1], // 0
        vec![1] // 1
    ];
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1], data.0);
    assert_eq!(true, data.1.is_empty());
}

#[test]
fn test_dfs_traverses_graph_with_cycle()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2, 3, 5], // 0
        vec![2], // 1
        vec![3, 4], // 2
        Vec::new(), // 3
        vec![1], // 4
        Vec::new() // 5
    ];
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 2, 3, 4], data.0);
    assert_eq!(vec![3], data.1);
}
