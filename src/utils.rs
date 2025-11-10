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
use crate::matrix;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::error::*;
use crate::value::*;

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

fn matrix_res_create_and_set_zeros(row_count: usize, col_count: usize) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    Ok(frontend.create_matrix_and_set_zeros(row_count, col_count)?)
}

pub fn matrix_create_and_set_zeros(row_count: usize, col_count: usize) -> Result<Matrix>
{
    match matrix_res_create_and_set_zeros(row_count, col_count) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_create_and_set_elems(row_count: usize, col_count: usize, elems: &[f32]) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    Ok(frontend.create_matrix_and_set_elems(row_count, col_count, elems)?)
}

pub fn matrix_create_and_set_elems(row_count: usize, col_count: usize, elems: &[f32]) -> Result<Matrix>
{
    match matrix_res_create_and_set_elems(row_count, col_count, elems) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_elems_and_transpose_flag(a: &Matrix) -> matrix::Result<(Vec<f32>, bool)>
{
    let frontend = Frontend::new()?;
    Ok(frontend.elems_and_transpose_flag(a)?)
}

pub fn matrix_elems_and_transpose_flag(a: &Matrix) -> Result<(Vec<f32>, bool)>
{
    match matrix_res_elems_and_transpose_flag(a) {
        Ok(pair) => Ok(pair),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_add(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add(a, b, &c)?;
    Ok(c)
}

pub fn matrix_add(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_add(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sub(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub(a, b, &c)?;
    Ok(c)
}

pub fn matrix_sub(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_sub(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = if  frontend.backend().has_cublas() {
        frontend.create_matrix_and_set_zeros(a.row_count(), b.col_count())?
    } else {
        unsafe { frontend.create_matrix(a.row_count(), b.col_count())? }
    };
    frontend.mul(a, b, &c)?;
    Ok(c)
}

pub fn matrix_mul(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_mul(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_elems(a, b, &c)?;
    Ok(c)
}

pub fn matrix_mul_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_mul_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_div_elems(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_elems(a, b, &c)?;
    Ok(c)
}

pub fn matrix_div_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_div_elems(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_add_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.add_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_add_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_add_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sub_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_sub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_sub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_rsub_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rsub_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_rsub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rsub_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_mul_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.mul_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_mul_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_mul_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_div_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.div_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_div_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_div_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_rdiv_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rdiv_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_rdiv_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rdiv_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sigmoid(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sigmoid(a, &b)?;
    Ok(b)
}

pub fn matrix_sigmoid(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_sigmoid(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_tanh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.tanh(a, &b)?;
    Ok(b)
}

pub fn matrix_tanh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_tanh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_swish(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.swish(a, &b)?;
    Ok(b)
}

pub fn matrix_swish(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_swish(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_softmax(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.softmax(a, &b)?;
    Ok(b)
}

pub fn matrix_softmax(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_softmax(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sqrt(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sqrt(a, &b)?;
    Ok(b)
}

pub fn matrix_sqrt(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_sqrt(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_really_transpose(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.really_transpose(a, &b)?;
    Ok(b)
}

pub fn matrix_really_transpose(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_really_transpose(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_repeat(a: &Matrix, n: usize) -> matrix::Result<Option<Matrix>>
{
    if a.col_count() != 1 && a.row_count() != 1 {
        return Ok(None);
    }
    let frontend = Frontend::new()?;
    let b = if a.col_count() == 1 {
        unsafe { frontend.create_matrix(a.row_count(), n)? }
    } else {
        unsafe { frontend.create_matrix(n, a.col_count())? }
    };
    frontend.repeat(a, &b)?;
    Ok(Some(b))
}

pub fn matrix_repeat(a: &Matrix, n: usize) -> Result<Option<Matrix>>
{
    match matrix_res_repeat(a, n) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_to_matrix_array(a: &Matrix) -> matrix::Result<Object>
{
    let frontend = Frontend::new()?;
    let xs = frontend.elems_and_transpose_flag(a)?.0;
    let transpose_flag = if a.is_transposed() {
        TransposeFlag::Transpose
    } else {
        TransposeFlag::NoTranspose
    };
    Ok(Object::MatrixArray(a.row_count(), a.col_count(), transpose_flag, xs))
}

pub fn matrix_to_matrix_array(a: &Matrix) -> Result<Object>
{
    match matrix_res_to_matrix_array(a) {
        Ok(object) => Ok(object),
        Err(err) => Err(Error::Matrix(err)),
    }
}
