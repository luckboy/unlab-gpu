//
// Copyright (c) 2025-2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
//! A backend module.
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
#[cfg(any(feature = "opencl", feature = "cuda"))]
use std::sync::Arc;
#[cfg(feature = "opencl")]
use crate::matrix;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::CL_DEVICE_TYPE_ALL;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::ClBackend;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::Context;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::Device;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::get_platforms;
#[cfg(feature = "cuda")]
use crate::matrix::cuda::CudaBackend;
#[cfg(any(feature = "opencl", feature = "cuda"))]
use crate::matrix::set_default_backend;
use crate::matrix::unset_default_backend;
use crate::serde::Deserialize;
use crate::serde::Serialize;
use crate::toml;
use crate::error::*;

/// A backend enumeration.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Backend
{
    /// An OpenCL backend.
    #[serde(rename = "OpenCL")]
    OpenCl,
    /// A CUDA backend.
    #[serde(rename = "CUDA")]
    Cuda,
}

/// A structure of backend configuration.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BackendConfig
{
    /// A backend.
    pub backend: Option<Backend>,
    /// An ordinal number for the CUDA backend.
    pub ordinal: Option<usize>,
    /// A platform index for the OpenCL backend.
    pub platform: Option<usize>,
    /// A device index for the OpenCL backend.
    pub device: Option<usize>,
    /// If this field is `true`, the CUDA backend uses the cuBLAS library.
    pub cublas: Option<bool>,
    /// If this field is `true`, the CUDA backend uses the mma instruction.
    pub mma: Option<bool>,
}

impl BackendConfig
{
    /// Reads a backend configuration from the reader.
    pub fn read(r: &mut dyn Read) -> Result<Self>
    {
        let mut s = String::new();
        match r.read_to_string(&mut s) {
            Ok(_) => {
                match toml::from_str(s.as_str()) {
                    Ok(config) => Ok(config),
                    Err(err) => Err(Error::TomlDe(err)),
                }
            },
            Err(err) => Err(Error::Io(err)),
        }
    }

    /// Loads a backend configuration from the file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match File::open(path) {
            Ok(mut file) => Self::read(&mut file),
            Err(err) => Err(Error::Io(err)),
        }
    }

    /// Loads a backend configuration from the file if the file exists, otherwise this method
    /// returns `None`.
    pub fn load_opt<P: AsRef<Path>>(path: P) -> Result<Option<Self>>
    {
        match File::open(path) {
            Ok(mut file) => Ok(Some(Self::read(&mut file)?)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

#[cfg(feature = "opencl")]
fn initialize_opencl_backend(platform_idx: usize, device_idx: usize) -> Result<()>
{
    let platforms = match get_platforms() {
        Ok(tmp_platforms) => tmp_platforms,
        Err(err) => return Err(Error::Matrix(matrix::Error::OpenCl(err))),
    };
    let platform = match platforms.get(platform_idx) {
        Some(tmp_platform) => tmp_platform,
        None => return Err(Error::Matrix(matrix::Error::NoPlatform)),
    };
    let device_ids = match platform.get_devices(CL_DEVICE_TYPE_ALL) {
        Ok(tmp_device_ids) => tmp_device_ids,
        Err(err) => return Err(Error::Matrix(matrix::Error::OpenCl(err))),
    };
    let device = match device_ids.get(device_idx) {
        Some(device_id) => Device::new(*device_id),
        None => return Err(Error::Matrix(matrix::Error::NoDevice)),
    };
    let context = match Context::from_device(&device) {
        Ok(tmp_context) => tmp_context,
        Err(err) => return Err(Error::Matrix(matrix::Error::OpenCl(err))),
    };
    match ClBackend::new_with_context(context) {
        Ok(backend) => {
            match set_default_backend(Arc::new(backend)) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Matrix(err)),
            }
        },
        Err(err) => Err(Error::Matrix(err)),
    }
}

#[cfg(not(feature = "opencl"))]
fn initialize_opencl_backend(_platform_idx: usize, _device_idx: usize) -> Result<()>
{ Err(Error::NoOpenClBackend) }

#[cfg(feature = "cuda")]
fn initialize_cuda_backend(ordinal: usize, is_cublas: bool, is_mma: bool) -> Result<()>
{
    match CudaBackend::new_with_ordinal_and_flags(ordinal, is_cublas, is_mma) {
        Ok(backend) => {
            match set_default_backend(Arc::new(backend)) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::Matrix(err)),
            }
        },
        Err(err) => Err(Error::Matrix(err)),        
    }
}

#[cfg(not(feature = "cuda"))]
fn initialize_cuda_backend(_ordinal: usize, _is_cublas: bool, _is_mma: bool) -> Result<()>
{ Err(Error::NoCudaBackend) }

/// Initializes a backend for matrices with the backend configuration.
///
/// If the backend configuration isn't passed, this method uses the default field values of
/// backend configuration.
pub fn initialize_backend_with_config(config: &Option<BackendConfig>) -> Result<()>
{
    #[cfg(feature = "cuda")]
    let mut backend = Backend::Cuda;
    #[cfg(not(feature = "cuda"))]
    let mut backend = Backend::OpenCl;
    let mut ordinal = 0usize;
    let mut platform_idx = 0usize;
    let mut device_idx = 0usize;
    let mut is_cublas = true;
    let mut is_mma = false;
    match config {
        Some(config) => {
            backend = config.backend.unwrap_or(backend);
            ordinal = config.ordinal.unwrap_or(ordinal);
            platform_idx = config.platform.unwrap_or(platform_idx);
            device_idx = config.device.unwrap_or(device_idx);
            is_cublas = config.cublas.unwrap_or(is_cublas);
            is_mma = config.mma.unwrap_or(is_mma);
        },
        None => (),
    }
    match backend {
        Backend::OpenCl => initialize_opencl_backend(platform_idx, device_idx),
        Backend::Cuda => initialize_cuda_backend(ordinal, is_cublas, is_mma),
    }
}

/// Initializes a backend for matrices with the file of backend configuration.
///
/// If the file of backend configuration doesn't exist, this method uses the default field values
/// of backend configuration.
pub fn initialize_backend<P: AsRef<Path>>(path: P) -> Result<()>
{
    let config = BackendConfig::load_opt(path)?;
    initialize_backend_with_config(&config)
}

/// Finalizes a backend for matrices.
pub fn finalize_backend() -> Result<()>
{
    match unset_default_backend() {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Matrix(err)),        
    }
}

#[cfg(test)]
mod tests;
