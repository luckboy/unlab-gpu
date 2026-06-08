# Unlab-pkg reference

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Introduction

This document is the reference for the Unlab-pkg package manager. The Unlab-pkg package manager is a
package manager for the Unlab scripting language. This reference describes the structure of package and
some configuration files for this package manager.

## Package structure

The package structure is:

- `lib` - directory with libraries
- `tests` - directory with tests for libraries
- `work` - working directory
- `Unlab.toml` - package manifest
- `Unlab.lock` - files with locked package versions

## Manifest format

A manifest format is based on [TOML](https://en.wikipedia.org/wiki/TOML) format. The manfest format
structure is:

- `[package]` - package section
    - `name` - package name
    - `description` - package description (optional)
    - `authors` - list of authors as string array (optional)
    - `license` - package license (optional)
    - `unlab-gpu-version` - version requirement of Unlab-gpu (optional)
- `[dependencies]` - section of dependencies (optional)
- `[constraints]` - section of constraints (optional)
- `[sources]` - seciton of sources (optional)
