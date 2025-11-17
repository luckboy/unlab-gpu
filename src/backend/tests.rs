//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use super::*;

#[test]
fn test_backend_config_read_reads_fields()
{
    let s = "
backend = OpenCL
platform = 1234
device = 4567
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match BackendConfig::read(&mut cursor) {
        Ok(config) => {
            assert_eq!(Some(Backend::OpenCl), config.backend);
            assert_eq!(None, config.ordinal);
            assert_eq!(Some(1234), config.platform);
            assert_eq!(Some(4567), config.device);
            assert_eq!(None, config.cublas);
            assert_eq!(None, config.mma);
        },
        Err(_) => assert!(false), 
    }
}

#[test]
fn test_backend_config_read_reads_fields_for_cuda()
{
    let s = "
backend = CUDA
ordinal = 1234
cublas = false
mma = true
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match BackendConfig::read(&mut cursor) {
        Ok(config) => {
            assert_eq!(Some(Backend::Cuda), config.backend);
            assert_eq!(Some(1234), config.ordinal);
            assert_eq!(None, config.platform);
            assert_eq!(None, config.device);
            assert_eq!(Some(false), config.cublas);
            assert_eq!(Some(true), config.mma);
        },
        Err(_) => assert!(false), 
    }
}
