//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::mpsc::Receiver;
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

pub fn receiver_recv<T>(receiver: &Receiver<T>) -> Result<T>
{
    match receiver.recv() {
        Ok(object) => Ok(object),
        Err(_) => Err(Error::Recv),
    }
}

fn matrix_res_backend_name() -> matrix::Result<&'static str>
{
    let frontend = Frontend::new()?;
    Ok(frontend.backend().name())
}

pub fn matrix_backend_name() -> Result<&'static str>
{
    match matrix_res_backend_name() {
        Ok(s) => Ok(s),
        Err(err) => Err(Error::Matrix(err)),
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
    let b = unsafe { frontend.create_matrix(a.col_count(), a.row_count())? };
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

fn matrix_res_abs(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.abs(a, &b)?;
    Ok(b)
}

pub fn matrix_abs(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_abs(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_pow(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.pow(a, b, &c)?;
    Ok(c)
}

pub fn matrix_pow(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_pow(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_pow_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.pow_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_pow_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_pow_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_rpow_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.rpow_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_rpow_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rpow_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_exp(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.exp(a, &b)?;
    Ok(b)
}

pub fn matrix_exp(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_exp(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_ln(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.ln(a, &b)?;
    Ok(b)
}

pub fn matrix_ln(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_ln(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_log2(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.log2(a, &b)?;
    Ok(b)
}

pub fn matrix_log2(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_log2(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_log10(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.log10(a, &b)?;
    Ok(b)
}

pub fn matrix_log10(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_log10(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sin(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sin(a, &b)?;
    Ok(b)
}

pub fn matrix_sin(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_sin(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_cos(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.cos(a, &b)?;
    Ok(b)
}

pub fn matrix_cos(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_cos(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_tan(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.tan(a, &b)?;
    Ok(b)
}

pub fn matrix_tan(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_tan(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_asin(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.asin(a, &b)?;
    Ok(b)
}

pub fn matrix_asin(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_asin(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_acos(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.acos(a, &b)?;
    Ok(b)
}

pub fn matrix_acos(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_acos(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_atan(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.atan(a, &b)?;
    Ok(b)
}

pub fn matrix_atan(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_atan(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_atan2(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.atan2(a, b, &c)?;
    Ok(c)
}

pub fn matrix_atan2(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_atan2(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_atan2_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.atan2_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_atan2_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_atan2_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_ratan2_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.ratan2_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_ratan2_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_ratan2_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_sinh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.sinh(a, &b)?;
    Ok(b)
}

pub fn matrix_sinh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_sinh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_cosh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.cosh(a, &b)?;
    Ok(b)
}

pub fn matrix_cosh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_cosh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_asinh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.asinh(a, &b)?;
    Ok(b)
}

pub fn matrix_asinh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_asinh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_acosh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.acosh(a, &b)?;
    Ok(b)
}

pub fn matrix_acosh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_acosh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_atanh(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.atanh(a, &b)?;
    Ok(b)
}

pub fn matrix_atanh(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_atanh(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_signum(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.signum(a, &b)?;
    Ok(b)
}

pub fn matrix_signum(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_signum(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_ceil(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.ceil(a, &b)?;
    Ok(b)
}

pub fn matrix_ceil(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_ceil(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_floor(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.floor(a, &b)?;
    Ok(b)
}

pub fn matrix_floor(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_floor(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_round(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.round(a, &b)?;
    Ok(b)
}

pub fn matrix_round(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_round(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_trunc(a: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let b = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.trunc(a, &b)?;
    Ok(b)
}

pub fn matrix_trunc(a: &Matrix) -> Result<Matrix>
{
    match matrix_res_trunc(a) {
        Ok(b) => Ok(b),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_max(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.max(a, b, &c)?;
    Ok(c)
}

pub fn matrix_max(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_max(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_max_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.max_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_max_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_max_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_min(a: &Matrix, b: &Matrix) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.min(a, b, &c)?;
    Ok(c)
}

pub fn matrix_min(a: &Matrix, b: &Matrix) -> Result<Matrix>
{
    match matrix_res_min(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}

fn matrix_res_min_for_scalar(a: &Matrix, b: f32) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    let c = unsafe { frontend.create_matrix(a.row_count(), a.col_count())? };
    frontend.min_for_scalar(a, b, &c)?;
    Ok(c)
}

pub fn matrix_min_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_min_for_scalar(a, b) {
        Ok(c) => Ok(c),
        Err(err) => Err(Error::Matrix(err)),
    }
}
