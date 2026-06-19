# Backend configuration

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Backend configuration file

A backend configuration is the `backend.toml` file that is in the Unlab-gpu home directory.

## Backend configuration file format

A backend configuration file format is based on the [TOML](https://en.wikipedia.org/wiki/TOML) format.
The structure of backend configuration file format is:

- `backend` - the backend name (`"OpenCL"` or `"CUDA"`)
- `ordinal` - the ordinal number for the CUDA backend (default: `0`)
- `platform` - the platform index for the OpenCL backend (default: `0`)
- `device` - the device index for the OpenCL backend (default: `0`)
- `cublas` - if this field is `true`, the CUDA backend uses the cuBLAS library (default: `true`)
- `mma` - if this field is `true`, the CUDA backend uses the mma instruction (default: `false`)

The default value of the `backend` field is `"CUDA"` if Unlab-gpu is compiled with the `cuda` feature,
otherwise `"OpenCL"`.
