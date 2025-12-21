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
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Success) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 2, 3, 4, 5], data.0);
    assert_eq!(vec![3, 4, 2, 1, 5, 0], data.1);
    let mut expected_visiteds: HashSet<usize> = HashSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    expected_visiteds.insert(4);
    expected_visiteds.insert(5);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_traverses_graph_from_two_vertices()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![2, 3], // 0
        vec![2, 4], // 1
        vec![5, 6], // 2
        Vec::new(), // 3
        vec![6, 7], // 4
        Vec::new(), // 5
        Vec::new(), // 6
        Vec::new() // 7
    ];
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Success) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 2, 5, 6, 3], data.0);
    assert_eq!(vec![5, 6, 2, 3, 0], data.1);
    let mut expected_visiteds: HashSet<usize> = HashSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    expected_visiteds.insert(5);
    expected_visiteds.insert(6);
    assert_eq!(expected_visiteds, visiteds);
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&1, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Success) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![1, 4, 7], data.0);
    assert_eq!(vec![7, 4, 1], data.1);
    let mut expected_visiteds: HashSet<usize> = HashSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    expected_visiteds.insert(4);
    expected_visiteds.insert(5);
    expected_visiteds.insert(6);
    expected_visiteds.insert(7);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_traverses_graph_with_little_cycle()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1], // 0
        vec![1] // 1
    ];
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Cycle(cycle)) => assert_eq!(vec![0, 1, 1], cycle),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1], data.0);
    assert_eq!(true, data.1.is_empty());
    let mut expected_visiteds: HashSet<usize> = HashSet::new();
    expected_visiteds.insert(0);
    assert_eq!(expected_visiteds, visiteds);
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
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Cycle(cycle)) => assert_eq!(vec![0, 1, 2, 4, 1], cycle),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 2, 3, 4], data.0);
    assert_eq!(vec![3], data.1);
    let mut expected_visiteds: HashSet<usize> = HashSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_traverses_graph_with_little_cycle_for_one_node()
{
    let graph: Vec<Vec<usize>> = vec![vec![0]];
    let mut visiteds: HashSet<usize> = HashSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs(&0, &mut visiteds, &mut data, |u, data| {
            data.0.push(*u);
            Ok::<Vec<usize>, ()>(graph.get(*u).map(|vs| vs.clone()).unwrap_or(Vec::new()))
    }, |u, data| {
            data.1.push(*u);
            Ok::<(), ()>(())
    });
    match res {
        Ok(DfsResult::Cycle(cycle)) => assert_eq!(vec![0, 0], cycle),
        _ => assert!(false),
    }
    assert_eq!(vec![0], data.0);
    assert_eq!(true, data.1.is_empty());
    assert_eq!(true, visiteds.is_empty());
}
