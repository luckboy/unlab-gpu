//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::error::*;
use crate::value::*;

fn matrix_res_create_and_set_elems(row_count: usize, col_count: usize, elems: &[f32]) -> matrix::Result<Matrix>
{
    let frontend = Frontend::new()?;
    Ok(frontend.create_matrix_and_set_elems(row_count, col_count, elems)?)
}

pub(crate) fn matrix_create_and_set_elems(row_count: usize, col_count: usize, elems: &[f32]) -> Result<Matrix>
{
    match matrix_res_create_and_set_elems(row_count, col_count, elems) {
        Ok(c) => Ok(c),
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

pub(crate) fn matrix_add(a: &Matrix, b: &Matrix) -> Result<Matrix>
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

pub(crate) fn matrix_sub(a: &Matrix, b: &Matrix) -> Result<Matrix>
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

pub(crate) fn matrix_mul(a: &Matrix, b: &Matrix) -> Result<Matrix>
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

pub(crate) fn matrix_mul_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
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

pub(crate) fn matrix_div_elems(a: &Matrix, b: &Matrix) -> Result<Matrix>
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

pub(crate) fn matrix_add_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
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

pub(crate) fn matrix_sub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
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

pub(crate) fn matrix_rsub_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
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

pub(crate) fn matrix_mul_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
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

pub(crate) fn matrix_div_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
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

pub(crate) fn matrix_rdiv_for_scalar(a: &Matrix, b: f32) -> Result<Matrix>
{
    match matrix_res_rdiv_for_scalar(a, b) {
        Ok(c) => Ok(c),
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

pub(crate) fn matrix_to_matrix_array(a: &Matrix) -> Result<Object>
{
    match matrix_res_to_matrix_array(a) {
        Ok(object) => Ok(object),
        Err(err) => Err(Error::Matrix(err)),
    }
}
