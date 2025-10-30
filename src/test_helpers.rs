//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub(crate) fn expected_unary_op<F>(a: &[f32], n: usize, m: usize, mut f: F) -> Vec<f32>
    where F: FnMut(f32) -> f32
{
    let mut b = vec![0.0f32; n * m];
    for i in 0..n {
        for j in 0..m {
            b[i * m + j] = f(a[i * m + j]);
        }
    }
    b
}

pub(crate) fn expected_bin_op<F>(a: &[f32], b: &[f32], n: usize, m: usize, mut f: F) -> Vec<f32>
    where F: FnMut(f32, f32) -> f32
{
    let mut c = vec![0.0f32; n * m];
    for i in 0..n {
        for j in 0..m {
            c[i * m + j] = f(a[i * m + j], b[i * m + j]);
        }
    }
    c
}

pub(crate) fn expected_mul(a: &[f32], b: &[f32], n: usize, m: usize, l: usize) -> Vec<f32>
{
    let mut c = vec![0.0f32; n * m];
    for i in 0..n {
        for j in 0..m {
            c[i * m + j] = 0.0f32;
            for k in 0..l {
                c[i * m + j] += a[i * l + k] * b[k * m + j];
            }
        }
    }
    c
}
