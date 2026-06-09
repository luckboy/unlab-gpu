# Unlab-pkg reference

## Copyright and license

Copyright (c) 2026 Łukasz Szpakowski

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.

## Introduction

This document is the reference for the Unlab-pkg package manager. The Unlab-pkg package manager is a
package manager for the Unlab scripting language. This reference describes the structure of package
and some configuration files for this package manager.

## Package structure

The package structure is:

- `bin` - directory with binaries
- `lib` - directory with libraries
- `tests` - directory with tests for libraries
- `work` - working directory
- `Unlab.toml` - package manifest
- `Unlab.lock` - files with locked package versions

## Manifest format

A manifest format is based on the [TOML](https://en.wikipedia.org/wiki/TOML) format. The structure of
manifest format is:

- `[package]` - package section
    - `name` - package name
    - `description` - package description (optional)
    - `authors` - list of authors as string array (optional)
    - `license` - package license (optional)
    - `unlab-gpu-version` - version requirement of Unlab-gpu (optional)
- `[dependencies]` - section of dependencies (optional)
- `[constraints]` - section of constraints (optional)
- `[sources]` - seciton of sources (optional)

## Dependencies and constraints

The section of depedencies and the section of constraints can contain keys and values. The key is the
package name and the value are a version requirement for the package.

The packages of section of depedencies are depedents with version requirements.

The version requirements of section of constraints can only limit the versions of packages. The packages of section of constraints aren't depedents. The constraints are only used in current package.

## Sources

The section of sources contain fields with keys which are package names. The value of this field is
an union. These fields of the union are:

- `versions` - keys are package version and values are version unions
- `renamed` - old package name

The fields of version union are:

- `dir` - packge directory
- `file` - package archive
- `url` - URL to package archive

The section of sources can refer the package directories and/or the package archives for package versions. The field of section of sources can is used to rename package where the `renamed` field is
old package name and key of this section is new package name. Also, the sources are only used in
current package.
