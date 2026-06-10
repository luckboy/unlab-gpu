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

The directory with the binaries can contain scripts in the Unlab scripting language which can be
ran by Unix shell.

The directory with the libraries can contain the domain directories which can contain the library
directories. The library directory contains the `lib.un` file and the script files which are used in
the `lib.un` file.

Also, the directory with the tests for libraries can contain the domain directory which can contain
the library directory. The library directory contains the `tests.un` file instead of the `lib.un`
file.

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

The version requirements of section of constraints can only limit the versions of packages. The
packages of section of constraints aren't depedents. The constraints are only used in the current
package.

## Sources

The section of sources contain fields with keys which are package names. The value of this field is
a structure with one field of many fields. These fields of the union are:

- `versions` - keys are package version and values are version structure with one field of many field
- `renamed` - old package name

The fields of version structure are:

- `dir` - packge directory
- `file` - package archive
- `url` - URL to package archive

The section of sources contain the custom sources. The custom sources can refer the package
directories and/or the package archives for package versions. The field of section of sources can be
used to rename package where the `renamed` field is old package name and the key of this section is
new package name. Also, the sources are only used in the current package.

## Package names

A package name can have two or more components which are separated by the `/` character. If the
package name has three components, the package name can specify default source. The default source
allows to download the package from the repository. The first component specifies the git hosting
service. The second component is an user name. The third component is a repository name. The first
components for the git hosting services are:

- `github.com` - [GitHub](https://github.com)
- with `gitlab.` prefix - [GitLab](https://about.gitlab.com)
- `bitbucket.org` - [Bitbucket](https://bitbucket.org)

## Versions

Versions are compatible with the [SemVer](https://semver.org) format. The version also can have less
or more numeric identifiers than in the [SemVer](https://semver.org) format.

## Version requirements

A single version requirement can have the operator with the version, the version or the `*` character.
The operators of single version requirement are:

- `=` - is equal to requirement version
- `!=` - isn't equal to requirement version
- `<` - is less than requirement version
- `>=` - is greater than or equal to requirement version
- `>` - is greater than requirement version
- `<=` - is less than or equal to requirement version
- `^` - default operator
- `~` - tilde operator

The default operator compares with the requirement version. If the version is greater than or equal to
the requirement version and the zero numeric identifiers or the zero numeric identifiers with the
first non-zero numeric identifier are equal, the version is matched. The number of zero numeric
identifiers comes from the requirement version.

The tilde operator compares with the requirement version. If the version is greater than or equal to
the requirement version and the tilde numeric identifiers are equal, the version is matched. The
number of tilde numeric identifiers is two if the number of numeric identifier is greater than or
equal to two, otherwise the number of numeric identifiers. The number of numeric identifiers comes
from the requirement version.

If the numeric identifier doesn't exits, zero is used as the numeric identifier.

If single version requirement hasn't operator, the default operator is used. Any version is matched if
the single version requirement is the `*` character.

The version requirement can have many single version requirements which are separeted the comma
character.
