//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs;
use std::io::BufWriter;
use std::io::Cursor;
use bzip2::write::BzEncoder;
use flate2::write::GzEncoder;
use liblzma::write::XzEncoder;
use zip::write::SimpleFileOptions;
use zip::write::ZipWriter;
use zip::CompressionMethod;
use sealed_test::prelude::*;
use super::*;

#[test]
fn test_pkg_name_parse_parses_package_names()
{
    match PkgName::parse("abc/def") {
        Ok(pkg_name) => assert_eq!(String::from("abc/def"), pkg_name.name()),
        Err(_) => assert!(false),
    }
    match PkgName::parse("abc/def/ghi") {
        Ok(pkg_name) => assert_eq!(String::from("abc/def/ghi"), pkg_name.name()),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_name_parse_complains_on_invalid_package_name()
{
    match PkgName::parse("abc") {
        Err(Error::InvalidPkgName) => assert!(true),
        _ => assert!(true),
    }
    match PkgName::parse("abc//def") {
        Err(Error::InvalidPkgName) => assert!(true),
        _ => assert!(true),
    }
}

#[test]
fn test_pkg_name_fmt_formats_package_names()
{
    assert_eq!(String::from("abc/def"), format!("{}", PkgName::new(String::from("abc/def"))));
    assert_eq!(String::from("abc/def/ghi"), format!("{}", PkgName::new(String::from("abc/def/ghi"))));
}

#[test]
fn test_manifest_read_reads_manifest()
{
    let s = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"
\"example3.com/ghi\" = \"2.3.4\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match Manifest::read(&mut cursor) {
        Ok(manifest) => {
            assert_eq!(PkgName::new(String::from("example1.com/abc")), manifest.package.name);
            assert_eq!(None, manifest.package.description);
            assert_eq!(None, manifest.package.authors);
            assert_eq!(None, manifest.package.license);
            match &manifest.dependencies {
                Some(dependencies) => {
                    match dependencies.get(&PkgName::new(String::from("example2.com/def"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match dependencies.get(&PkgName::new(String::from("example3.com/ghi"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match manifest.constraints {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match manifest.sources {
                None => assert!(true),
                Some(_) => assert!(false),
            }
        },
        Err(_) => assert!(false), 
    }
}

#[test]
fn test_manifest_read_reads_manifest_with_package_fields()
{
    let s = "
[package]
name = \"example1.com/abc\"
description = \"Some text.\"
authors = [\"Jan Nowak\", \"Jacek Nowakowski\"]
license = \"MPL-2.0\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"
\"example3.com/ghi\" = \"2.3.4\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match Manifest::read(&mut cursor) {
        Ok(manifest) => {
            assert_eq!(PkgName::new(String::from("example1.com/abc")), manifest.package.name);
            assert_eq!(Some(String::from("Some text.")), manifest.package.description);
            assert_eq!(Some(vec![String::from("Jan Nowak"), String::from("Jacek Nowakowski")]), manifest.package.authors);
            assert_eq!(Some(String::from("MPL-2.0")), manifest.package.license);
            match &manifest.dependencies {
                Some(dependencies) => {
                    match dependencies.get(&PkgName::new(String::from("example2.com/def"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match dependencies.get(&PkgName::new(String::from("example3.com/ghi"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match manifest.constraints {
                None => assert!(true),
                Some(_) => assert!(false),
            }
            match manifest.sources {
                None => assert!(true),
                Some(_) => assert!(false),
            }
        },
        Err(_) => assert!(false), 
    }
}

#[test]
fn test_manifest_read_reads_manifest_with_constraints_and_sources()
{
    let s = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"
\"example3.com/ghi\" = \"2.3.4\"

[constraints]
\"example2.com/def\" = \"<=1.2.5\"
\"example3.com/ghi\" = \"<=2.3.7\"

[sources]
\"example2.com/def\".versions.\"1.2.1\".dir = \"../def\"
\"example2.com/def\".versions.\"1.2.2\".file = \"../def.tar.gz\"
\"example2.com/def\".versions.\"1.2.3\".url = \"https://example2.com/def.tar.gz\"
\"example3.com/ghi\".renamed = \"example4.com/ghi\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match Manifest::read(&mut cursor) {
        Ok(manifest) => {
            assert_eq!(PkgName::new(String::from("example1.com/abc")), manifest.package.name);
            assert_eq!(None, manifest.package.description);
            assert_eq!(None, manifest.package.authors);
            assert_eq!(None, manifest.package.license);
            match &manifest.dependencies {
                Some(dependencies) => {
                    match dependencies.get(&PkgName::new(String::from("example2.com/def"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match dependencies.get(&PkgName::new(String::from("example3.com/ghi"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match &manifest.constraints {
                Some(constraints) => {
                    match constraints.get(&PkgName::new(String::from("example2.com/def"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Le, version) => assert_eq!(Version::parse("1.2.5").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match constraints.get(&PkgName::new(String::from("example3.com/ghi"))) {
                        Some(version_req) => {
                            assert_eq!(1, version_req.single_reqs().len());
                            match &version_req.single_reqs()[0] {
                                SingleVersionReq::Pair(VersionOp::Le, version) => assert_eq!(Version::parse("2.3.7").unwrap(), *version),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match &manifest.sources {
                Some(sources) => {
                    match sources.get(&PkgName::new(String::from("example2.com/def"))) {
                        Some(SrcInfo::Versions(version_src_infos)) => {
                            match version_src_infos.get(&Version::parse("1.2.1").unwrap()) {
                                Some(VersionSrcInfo::Dir(dir)) => assert_eq!(String::from("../def"), *dir),
                                _ => assert!(false),
                            }
                            match version_src_infos.get(&Version::parse("1.2.2").unwrap()) {
                                Some(VersionSrcInfo::File(file)) => assert_eq!(String::from("../def.tar.gz"), *file),
                                _ => assert!(false),
                            }
                            match version_src_infos.get(&Version::parse("1.2.3").unwrap()) {
                                Some(VersionSrcInfo::Url(url)) => assert_eq!(String::from("https://example2.com/def.tar.gz"), *url),
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    match sources.get(&PkgName::new(String::from("example3.com/ghi"))) {
                        Some(SrcInfo::Renamed(pkg_name)) => assert_eq!(PkgName::new(String::from("example4.com/ghi")), *pkg_name),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false), 
    }
}

#[test]
fn test_manifest_write_writes_manifest_for_new_method()
{
    let manifest = Manifest::new(PkgName::new(String::from("example1.com/abc")));
    let mut cursor = Cursor::new(Vec::<u8>::new());
    match manifest.write(&mut cursor) {
        Ok(()) => {
            cursor.set_position(0);
            let mut s = String::new();
            match cursor.read_to_string(&mut s) {
                Ok(_) => {
                    let expected_s = "
[package]
name = \"example1.com/abc\"

[dependencies]
";
                    assert_eq!(String::from(&expected_s[1..]), s);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_manifest_write_writes_manifest()
{
    let s = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let manifest = Manifest::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match manifest.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"^1.2.3\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_manifest_write_writes_manifest_with_package_fields()
{
    let s = "
[package]
name = \"example1.com/abc\"
description = \"Some text.\"
authors = [\"Jan Nowak\", \"Jacek Nowakowski\"]
license = \"MPL-2.0\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let manifest = Manifest::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match manifest.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[package]
name = \"example1.com/abc\"
description = \"Some text.\"
authors = [\"Jan Nowak\", \"Jacek Nowakowski\"]
license = \"MPL-2.0\"

[dependencies]
\"example2.com/def\" = \"^1.2.3\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_manifest_write_writes_manifest_with_constraints_and_sources_for_first_case()
{
    let s = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"

[constraints]
\"example2.com/def\" = \"<=1.2.5\"

[sources]
\"example2.com/def\".versions.\"1.2.1\".dir = \"../def\"
\"example2.com/def\".versions.\"1.2.2\".file = \"../def.tar.gz\"
\"example2.com/def\".versions.\"1.2.3\".url = \"https://example2.com/def.tar.gz\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let manifest = Manifest::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match manifest.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"^1.2.3\"

[constraints]
\"example2.com/def\" = \"<=1.2.5\"

[sources.\"example2.com/def\".versions.\"1.2.1\"]
dir = \"../def\"

[sources.\"example2.com/def\".versions.\"1.2.2\"]
file = \"../def.tar.gz\"

[sources.\"example2.com/def\".versions.\"1.2.3\"]
url = \"https://example2.com/def.tar.gz\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_manifest_write_writes_manifest_with_constraints_and_sources_for_second_case()
{
    let s = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"1.2.3\"

[constraints]
\"example2.com/def\" = \"<=1.2.5\"

[sources]
\"example2.com/def\".renamed = \"example3.com/def\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let manifest = Manifest::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match manifest.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"^1.2.3\"

[constraints]
\"example2.com/def\" = \"<=1.2.5\"

[sources.\"example2.com/def\"]
renamed = \"example3.com/def\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_paths_read_reads_paths()
{
    let s = "
bin = [
    \"abc\",
    \"def\",
    \"ghi\"
]
lib = [
    \"abc/def\",
    \"def/ghi\",
    \"ghi/jkl\"
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match Paths::read(&mut cursor) {
        Ok(paths) => {
            assert_eq!(vec![String::from("abc"), String::from("def"), String::from("ghi")], paths.bin);
            assert_eq!(vec![String::from("abc/def"), String::from("def/ghi"), String::from("ghi/jkl")], paths.lib);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_paths_write_writes_paths()
{
    let s = "
bin = [
    \"abc\",
    \"def\",
    \"ghi\"
]
lib = [
    \"abc/def\",
    \"def/ghi\",
    \"ghi/jkl\"
]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let paths = Paths::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match paths.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
bin = [\"abc\", \"def\", \"ghi\"]
lib = [\"abc/def\", \"def/ghi\", \"ghi/jkl\"]
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_versions_read_reads_versions()
{
    let s = "
versions = [\"1.2.3\", \"2.3.4\", \"3.4.5\"]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match Versions::read(&mut cursor) {
        Ok(versions) => {
            let mut expected_versions: BTreeSet<Version> = BTreeSet::new();
            expected_versions.insert(Version::parse("1.2.3").unwrap());
            expected_versions.insert(Version::parse("2.3.4").unwrap());
            expected_versions.insert(Version::parse("3.4.5").unwrap());
            assert_eq!(expected_versions, versions.versions);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_versions_write_writes_versions()
{
    let s = "
versions = [\"1.2.3\", \"2.3.4\", \"3.4.5\"]
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let versions = Versions::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match versions.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
versions = [\"1.2.3\", \"2.3.4\", \"3.4.5\"]
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_read_reads_pkg_config_with_first_field()
{
    let s = "
account = \"example.com/abc\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match PkgConfig::read(&mut cursor) {
        Ok(config) => {
            assert_eq!(Some(String::from("example.com/abc")), config.account);
            assert_eq!(None, config.domain);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_read_reads_pkg_config_with_second_field()
{
    let s = "
domain = \"pl.jan.nowak\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match PkgConfig::read(&mut cursor) {
        Ok(config) => {
            assert_eq!(None, config.account);
            assert_eq!(Some(String::from("pl.jan.nowak")), config.domain);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_read_reads_pkg_config_with_all_fields()
{
    let s = "
account = \"example.com/abc\"
domain = \"pl.jan.nowak\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match PkgConfig::read(&mut cursor) {
        Ok(config) => {
            assert_eq!(Some(String::from("example.com/abc")), config.account);
            assert_eq!(Some(String::from("pl.jan.nowak")), config.domain);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_write_writes_pkg_config_with_first_field()
{
    let s = "
account = \"example.com/abc\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let config = PkgConfig::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match config.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
account = \"example.com/abc\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_write_writes_pkg_config_with_second_field()
{
    let s = "
domain = \"pl.jan.nowak\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let config = PkgConfig::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match config.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
domain = \"pl.jan.nowak\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pkg_config_write_writes_pkg_config_with_all_fields()
{
    let s = "
account = \"example.com/abc\"
domain = \"pl.jan.nowak\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let config = PkgConfig::read(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match config.write(&mut cursor2) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
account = \"example.com/abc\"
domain = \"pl.jan.nowak\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_read_versions_reads_versions()
{
    let s = "
\"example1.com/abc\" = \"1.2.3\"
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match read_versions(&mut cursor) {
        Ok(versions) => {
            assert_eq!(3, versions.len());
            let mut expected_versions: HashMap<PkgName, Version> = HashMap::new();
            expected_versions.insert(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap());
            expected_versions.insert(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap());
            expected_versions.insert(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap());
            assert_eq!(expected_versions, versions);
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_versions_writes_version()
{
    let s = "
\"example1.com/abc\" = \"1.2.3\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let versions = read_versions(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match write_versions(&mut cursor2, &versions) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
\"example1.com/abc\" = \"1.2.3\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_read_version_reqs_reads_version_requirements()
{
    let s = "
\"example1.com/abc\" = \"1.2.3\"
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match read_version_reqs(&mut cursor) {
        Ok(version_reqs) => {
            assert_eq!(3, version_reqs.len());
            match version_reqs.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match version_reqs.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match version_reqs.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_version_reqs_writes_version_requirement()
{
    let s = "
\"example1.com/abc\" = \"1.2.3\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let version_reqs = read_version_reqs(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match write_version_reqs(&mut cursor2, &version_reqs) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
\"example1.com/abc\" = \"^1.2.3\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_read_src_infos_reads_source_infos()
{
    let s = "
\"example1.com/abc\".versions.\"1.2.1\".dir = \"../abc\"
\"example1.com/abc\".versions.\"1.2.2\".file = \"../abc.tar.gz\"
\"example1.com/abc\".versions.\"1.2.3\".url = \"https://example1.com/abc.tar.gz\"
\"example2.com/def\".renamed = \"example3.com/def\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    match read_src_infos(&mut cursor) {
        Ok(src_infos) => {
            assert_eq!(2, src_infos.len());
            match src_infos.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(SrcInfo::Versions(version_src_infos)) => {
                    match version_src_infos.get(&Version::parse("1.2.1").unwrap()) {
                        Some(VersionSrcInfo::Dir(dir)) => assert_eq!(String::from("../abc"), *dir),
                        _ => assert!(false),
                    }
                    match version_src_infos.get(&Version::parse("1.2.2").unwrap()) {
                        Some(VersionSrcInfo::File(file)) => assert_eq!(String::from("../abc.tar.gz"), *file),
                        _ => assert!(false),
                    }
                    match version_src_infos.get(&Version::parse("1.2.3").unwrap()) {
                        Some(VersionSrcInfo::Url(url)) => assert_eq!(String::from("https://example1.com/abc.tar.gz"), *url),
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            match src_infos.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(SrcInfo::Renamed(pkg_name)) => assert_eq!(PkgName::new(String::from("example3.com/def")), *pkg_name),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_src_infos_writes_source_info_for_first_case()
{
    let s = "
\"example1.com/abc\".versions.\"1.2.1\".dir = \"../abc\"
\"example1.com/abc\".versions.\"1.2.2\".file = \"../abc.tar.gz\"
\"example1.com/abc\".versions.\"1.2.3\".url = \"https://example1.com/abc.tar.gz\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let src_infos = read_src_infos(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match write_src_infos(&mut cursor2, &src_infos) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[\"example1.com/abc\".versions.\"1.2.1\"]
dir = \"../abc\"

[\"example1.com/abc\".versions.\"1.2.2\"]
file = \"../abc.tar.gz\"

[\"example1.com/abc\".versions.\"1.2.3\"]
url = \"https://example1.com/abc.tar.gz\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_src_infos_writes_source_info_for_second_case()
{
    let s = "
\"example2.com/def\".renamed = \"example3.com/def\"
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let src_infos = read_src_infos(&mut cursor).unwrap();
    let mut cursor2 = Cursor::new(Vec::<u8>::new());
    match write_src_infos(&mut cursor2, &src_infos) {
        Ok(()) => {
            cursor2.set_position(0);
            let mut t = String::new();
            match cursor2.read_to_string(&mut t) {
                Ok(_) => {
                    let expected_t = "
[\"example2.com/def\"]
renamed = \"example3.com/def\"
";
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

fn create_pkg(path: &str, manifest: &str, bin: Option<(&str, &str)>, lib: Option<(&str, &str)>)
{
    let path_buf = PathBuf::from(path.replace('/', path::MAIN_SEPARATOR_STR));
    if path_buf != PathBuf::from(".") {
        fs::create_dir_all(path_buf.as_path()).unwrap();
    }
    let mut manifest_file = path_buf.clone();
    manifest_file.push("Unlab.toml");
    fs::write(manifest_file, manifest).unwrap();
    match bin {
        Some((bin_path, script_content)) => {
            let mut bin_file = path_buf.clone();
            bin_file.push(bin_path.replace('/', path::MAIN_SEPARATOR_STR));
            let mut bin_dir = bin_file.clone();
            bin_dir.pop();
            fs::create_dir_all(bin_dir).unwrap();
            fs::write(bin_file, script_content).unwrap();
        },
        None => (),
    }
    match lib {
        Some((lib_path, lib_content)) => {
            let mut lib_file = path_buf.clone();
            lib_file.push(lib_path.replace('/', path::MAIN_SEPARATOR_STR));
            let mut lib_dir = lib_file.clone();
            lib_dir.pop();
            fs::create_dir_all(lib_dir).unwrap();
            fs::write(lib_file, lib_content).unwrap();
        },
        None => (),
    }
}

fn dirs_from_path(path: &PathBuf) -> Vec<PathBuf>
{
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut path_buf = path.clone();
    while path_buf != PathBuf::from("") && path_buf != PathBuf::from(".") {
        dirs.push(path_buf.clone());
        path_buf.pop();
    }
    dirs.reverse();
    dirs
}

fn create_zip(archive_path: &str, path: &str, manifest: &str, bin: Option<(&str, &str)>, lib: Option<(&str, &str)>)
{
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::DEFLATE);
    let mut file = File::create(archive_path).unwrap();
    let mut bw = BufWriter::new(&mut file);
    let mut zip_writer = ZipWriter::new(&mut bw);
    let path_buf = PathBuf::from(path.replace('/', path::MAIN_SEPARATOR_STR));
    if path_buf != PathBuf::from(".") {
        let dirs = dirs_from_path(&path_buf);
        for dir in  &dirs {
            zip_writer.add_directory_from_path(dir.as_path(), options).unwrap();
        }
    }
    let mut manifest_file = path_buf.clone();
    manifest_file.push("Unlab.toml");
    zip_writer.start_file_from_path(manifest_file.as_path(), options).unwrap();
    write!(&mut zip_writer, "{}", manifest).unwrap();
    match bin {
        Some((bin_path, script_content)) => {
            let tmp_bin_file = PathBuf::from(bin_path.replace('/', path::MAIN_SEPARATOR_STR));
            let mut tmp_bin_dir = tmp_bin_file.clone();
            tmp_bin_dir.pop();
            let tmp_dirs = dirs_from_path(&tmp_bin_dir);
            for tmp_dir in  &tmp_dirs {
                let mut dir = path_buf.clone();
                dir.push(tmp_dir.as_path());
                zip_writer.add_directory_from_path(dir.as_path(), options).unwrap();
            }
            let mut bin_file = path_buf.clone();
            bin_file.push(tmp_bin_file.as_path());
            zip_writer.start_file_from_path(bin_file.as_path(), options).unwrap();
            write!(&mut zip_writer, "{}", script_content).unwrap();
        },
        None => (),
    }
    match lib {
        Some((lib_path, lib_content)) => {
            let tmp_lib_file = PathBuf::from(lib_path.replace('/', path::MAIN_SEPARATOR_STR));
            let mut tmp_lib_dir = tmp_lib_file.clone();
            tmp_lib_dir.pop();
            let tmp_dirs = dirs_from_path(&tmp_lib_dir);
            for tmp_dir in  &tmp_dirs {
                let mut dir = path_buf.clone();
                dir.push(tmp_dir.as_path());
                zip_writer.add_directory_from_path(dir.as_path(), options).unwrap();
            }
            let mut lib_file = path_buf.clone();
            lib_file.push(tmp_lib_file.as_path());
            zip_writer.start_file_from_path(lib_file.as_path(), options).unwrap();
            write!(&mut zip_writer, "{}", lib_content).unwrap();
        },
        None => (),
    }
    zip_writer.finish().unwrap();
}

fn create_tar_gz_from_pkg(archive_path: &str, path: &str)
{
    let mut file = File::create(archive_path).unwrap();
    let mut bw = BufWriter::new(&mut file);
    let mut encoder = GzEncoder::new(&mut bw, flate2::Compression::default());
    let mut builder = tar::Builder::new(&mut encoder);
    builder.append_dir_all(path, path).unwrap();
    builder.finish().unwrap();
}

fn create_tar_bz2_from_pkg(archive_path: &str, path: &str)
{
    let mut file = File::create(archive_path).unwrap();
    let mut bw = BufWriter::new(&mut file);
    let mut encoder = BzEncoder::new(&mut bw, bzip2::Compression::default());
    let mut builder = tar::Builder::new(&mut encoder);
    builder.append_dir_all(path, path).unwrap();
    builder.finish().unwrap();
}

fn create_tar_xz_from_pkg(archive_path: &str, path: &str)
{
    let mut file = File::create(archive_path).unwrap();
    let mut bw = BufWriter::new(&mut file);
    let mut encoder = XzEncoder::new(&mut bw, 6);
    let mut builder = tar::Builder::new(&mut encoder);
    builder.append_dir_all(path, path).unwrap();
    builder.finish().unwrap();
}

fn create_tar_from_pkg(archive_path: &str, path: &str)
{
    let mut file = File::create(archive_path).unwrap();
    let mut bw = BufWriter::new(&mut file);
    let mut builder = tar::Builder::new(&mut bw);
    builder.append_dir_all(path, path).unwrap();
    builder.finish().unwrap();
}

#[sealed_test]
fn test_pkg_manager_install_installs_package()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".dir = \"abc\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_with_dependencies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(4, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_with_dependencies_and_constraints()
{
    fs::create_dir("home").unwrap();
    let mut constraints_file = PathBuf::from("home");
    constraints_file.push("constraints.toml");
    let constraints_content = "
\"example1.com/jkl\" = \"<=4.5.6\"
";
    fs::write(constraints_file, &constraints_content[1..]).unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/jkl\".versions.\"5.6.7\".dir = \"jkl2\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"*\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
descritpion = \"Some text.\"

[dependencies]
";
    let lib_content42 = "
W = 5
";
    create_pkg("jkl2", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Wildcard => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(4, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_reinstalls_package_with_dependecies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file.clone(), &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest12 = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"3.2.1\"
\"example3.com/ghi\" = \"4.3.2\"
";
    let lib_content12 = "
X = 12
";
    create_pkg("abc2", &manifest12[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content12[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest22 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/mno\" = \"5.6.0\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def2", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi2", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example1.com/abc\".versions.\"1.2.4\".dir = \"abc2\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example2.com/def\".versions.\"3.2.1\".dir = \"def2\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example3.com/ghi\".versions.\"4.3.2\".dir = \"ghi2\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest12[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content12[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest22[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.2.1").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content22[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest32[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.3.2").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content32[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // mno
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("3.2.1").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("4.3.2").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_reinstalls_package_dependencies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file.clone(), &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest22 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/mno\" = \"5.6.0\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def2", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi2", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example2.com/def\".versions.\"2.3.5\".dir = \"def2\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example3.com/ghi\".versions.\"3.4.6\".dir = \"ghi2\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest22[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content22[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest32[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content32[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // mno
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_reinstalls_package_dependencies_and_nested_package_dependencies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file.clone(), &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest22 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/mno\" = \"5.6.0\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def2", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi2", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
description = \"Some text1.\"

[dependencies]
";
    let lib_content42 = "
W = 42
";
    create_pkg("jkl2", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let manifest52 = "
[package]
name = \"example1.com/mno\"
description = \" Some text2.\"

[dependencies]
";
    let lib_content52 = "
V = 52
";
    create_pkg("mno2", &manifest52[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content52[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example2.com/def\".versions.\"2.3.5\".dir = \"def2\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example3.com/ghi\".versions.\"3.4.6\".dir = \"ghi2\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/jkl\".versions.\"4.5.7\".dir = \"jkl2\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
\"example1.com/mno\".versions.\"5.6.8\".dir = \"mno2\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest22[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content22[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest32[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content32[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest42[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content42[1..]), fs::read_to_string(lib_file).unwrap());
            // mno
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest52[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content52[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.7").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.8").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_packages()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_packages_with_dependency()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def")), PkgName::new(String::from("example3.com/ghi"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_reinstalls_package_for_force_flag()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    recursively_remove("abc", false).unwrap();
    recursively_remove("def", false).unwrap();
    recursively_remove("ghi", false).unwrap();
    recursively_remove("jkl", false).unwrap();
    let manifest12 = "
[package]
name = \"example1.com/abc\"
description = \"Some text1.\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content12 = "
X = 12
";
    create_pkg("abc", &manifest12[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content12[1..])));
    let manifest22 = "
[package]
name = \"example2.com/def\"
description = \"Some text2.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"
description = \"Some text3.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
description = \"Some text4.\"

[dependencies]
";
    let lib_content42 = "
W = 42
";    
    create_pkg("jkl", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, true, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest12[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content12[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest22[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content22[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest32[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content32[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest42[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content42[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(4, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_after_removal()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".dir = \"abc\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false).unwrap();
    pkg_manager.remove(&[PkgName::new(String::from("example.com/abc"))]).unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_from_zip_archive()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".file = \"abc.zip\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_zip("abc.zip", "abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_from_tar_gz_archive()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".file = \"abc.tar.gz\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    create_tar_gz_from_pkg("abc.tar.gz", "abc");
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_from_tar_bz2_archive()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".file = \"abc.tar.bz2\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    create_tar_bz2_from_pkg("abc.tar.bz2", "abc");
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_from_tar_xz_archive()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".file = \"abc.tar.xz\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    create_tar_xz_from_pkg("abc.tar.xz", "abc");
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_installs_package_from_tar_archive()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".file = \"abc.tar\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    create_tar_from_pkg("abc.tar", "abc");
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_version_requirements_indicate_two_different_package_versions()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/jkl\".versions.\"5.6.7\".dir = \"jkl2\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"*\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
descritpion = \"Some text.\"

[dependencies]
";
    let lib_content42 = "
W = 5
";
    create_pkg("jkl2", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Err(Error::PkgName(name, msg)) => {
            assert_eq!(PkgName::new(String::from("example1.com/jkl")), name);
            assert_eq!(true, msg.starts_with("version requirements indicate two different package versions: "));
            let mut info_dir = PathBuf::from("work");
            info_dir.push("var");
            info_dir.push("info");
            match fs::metadata(info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut bin_dir = PathBuf::from("work");
            bin_dir.push("bin");
            match fs::metadata(bin_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_dir = PathBuf::from("work");
            lib_dir.push("lib");
            match fs::metadata(lib_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut doc_dir = PathBuf::from("work");
            doc_dir.push("doc");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_version_requirements_indicate_two_different_package_versions_for_second_case()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file.as_path(), &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest12 = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"5.4.3\"
";
    let lib_content12 = "
X = 12
";
    create_pkg("abc2", &manifest12[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content12[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
description = \"Some text.\"

[dependencies]
";
    let lib_content42 = "
W = 42
";
    create_pkg("jkl2", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false).unwrap();
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example1.com/abc\".versions.\"1.2.4\".dir = \"abc2\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/jkl\".versions.\"5.4.3\".dir = \"jkl2\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Err(Error::PkgName(name, msg)) => {
            assert_eq!(PkgName::new(String::from("example1.com/jkl")), name);
            assert_eq!(true, msg.starts_with("version requirements indicate two different package versions: "));
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_each_package_version_is_not_matched_to_version_requirement()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"4.3.2\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Err(Error::PkgName(name, msg)) => {
            assert_eq!(PkgName::new(String::from("example3.com/ghi")), name);
            assert_eq!(String::from("each package version isn't matched to version requirement"), msg);
            let mut info_dir = PathBuf::from("work");
            info_dir.push("var");
            info_dir.push("info");
            match fs::metadata(info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut bin_dir = PathBuf::from("work");
            bin_dir.push("bin");
            match fs::metadata(bin_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_dir = PathBuf::from("work");
            lib_dir.push("lib");
            match fs::metadata(lib_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut doc_dir = PathBuf::from("work");
            doc_dir.push("doc");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_occurred_cycle_of_dependencies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/abc\" = \"1.2.3\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false) {
        Err(Error::PkgDepCycle(names)) => {
            let expected_names = vec![
                PkgName::new(String::from("example1.com/abc")),
                PkgName::new(String::from("example2.com/def")),
                PkgName::new(String::from("example3.com/ghi")),
                PkgName::new(String::from("example1.com/abc"))
            ];
            assert_eq!(expected_names, names);
            let mut info_dir = PathBuf::from("work");
            info_dir.push("var");
            info_dir.push("info");
            match fs::metadata(info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut bin_dir = PathBuf::from("work");
            bin_dir.push("bin");
            match fs::metadata(bin_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_dir = PathBuf::from("work");
            lib_dir.push("lib");
            match fs::metadata(lib_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut doc_dir = PathBuf::from("work");
            doc_dir.push("doc");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_occurred_conflicts_between_package_and_installed_packages_for_directory_bin()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
";
    let script_content2 = "
#!/usr/bin/env unlab-gpu --
println(2 + 3)
";
    let lib_content2 = "
X = 2
";
    create_pkg("def", &manifest2[1..], Some(("bin/script.un", &script_content2[1..])), Some(("lib/pl.jan.nowak/def/lib.un", &lib_content2[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example2.com/def"))], false, false, false) {
        Err(Error::PkgPathConflicts(name, None, conflict_paths, PkgPathConflict::Bin)) => {
            assert_eq!(PkgName::new(String::from("example2.com/def")), name);
            let expected_conflict_path = PathBuf::from("script.un");
            assert_eq!(vec![expected_conflict_path], conflict_paths);
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_occurred_conflicts_between_package_and_installed_packages_for_directory_lib()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
";
    let script_content2 = "
#!/usr/bin/env unlab-gpu --
println(2 + 3)
";
    let lib_content2 = "
X = 2
";
    create_pkg("def", &manifest2[1..], Some(("bin/script2.un", &script_content2[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content2[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc"))], false, false, false).unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example2.com/def"))], false, false, false) {
        Err(Error::PkgPathConflicts(name, None, conflict_paths, PkgPathConflict::Lib)) => {
            assert_eq!(PkgName::new(String::from("example2.com/def")), name);
            let mut expected_conflict_path = PathBuf::from("pl.jan.nowak");
            expected_conflict_path.push("abc");
            assert_eq!(vec![expected_conflict_path], conflict_paths);
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            let bin = PathBuf::from("script.un");
            assert_eq!(vec![bin.to_string_lossy().into_owned()], paths.bin);
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            assert_eq!(String::from(&script_content[1..]), fs::read_to_string(script_file).unwrap());
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(1, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_occurred_conflicts_between_package_and_other_package_for_directory_bin()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
";
    let script_content2 = "
#!/usr/bin/env unlab-gpu --
println(2 + 3)
";
    let lib_content2 = "
X = 2
";
    create_pkg("def", &manifest2[1..], Some(("bin/script.un", &script_content2[1..])), Some(("lib/pl.jan.nowak/def/lib.un", &lib_content2[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false) {
        Err(Error::PkgPathConflicts(name, Some(name2), conflict_paths, PkgPathConflict::Bin)) => {
            assert!(PkgName::new(String::from("example1.com/abc")) == name || PkgName::new(String::from("example2.com/def")) == name);
            assert!(PkgName::new(String::from("example2.com/def")) == name2 || PkgName::new(String::from("example1.com/abc")) == name2);
            let expected_conflict_path = PathBuf::from("script.un");
            assert_eq!(vec![expected_conflict_path], conflict_paths);
            let mut info_dir = PathBuf::from("work");
            info_dir.push("var");
            info_dir.push("info");
            match fs::metadata(info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut bin_dir = PathBuf::from("work");
            bin_dir.push("bin");
            match fs::metadata(bin_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_dir = PathBuf::from("work");
            lib_dir.push("lib");
            match fs::metadata(lib_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut doc_dir = PathBuf::from("work");
            doc_dir.push("doc");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_complains_on_occurred_conflicts_between_package_and_other_package_for_directory_lib()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
";
    let script_content2 = "
#!/usr/bin/env unlab-gpu --
println(2 + 3)
";
    let lib_content2 = "
X = 2
";
    create_pkg("def", &manifest2[1..], Some(("bin/script2.un", &script_content2[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content2[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    match pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false) {
        Err(Error::PkgPathConflicts(name, Some(name2), conflict_paths, PkgPathConflict::Lib)) => {
            assert!(PkgName::new(String::from("example1.com/abc")) == name || PkgName::new(String::from("example2.com/def")) == name);
            assert!(PkgName::new(String::from("example2.com/def")) == name2 || PkgName::new(String::from("example1.com/abc")) == name2);
            let mut expected_conflict_path = PathBuf::from("pl.jan.nowak");
            expected_conflict_path.push("abc");
            assert_eq!(vec![expected_conflict_path], conflict_paths);
            let mut info_dir = PathBuf::from("work");
            info_dir.push("var");
            info_dir.push("info");
            match fs::metadata(info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut bin_dir = PathBuf::from("work");
            bin_dir.push("bin");
            match fs::metadata(bin_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_dir = PathBuf::from("work");
            lib_dir.push("lib");
            match fs::metadata(lib_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut doc_dir = PathBuf::from("work");
            doc_dir.push("doc");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_deps_installs_dependencies()
{
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    let lib_content = "
X = 1
";
    create_pkg(".", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    match pkg_manager.install_deps(false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(3, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_deps_installs_dependencies_with_lock_file()
{
    let locks = "
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    fs::write("Unlab.lock", &locks[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example2.com/def\".versions.\"2.3.5\".dir = \"def2\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example3.com/ghi\".versions.\"3.4.6\".dir = \"ghi2\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/jkl\".versions.\"4.5.7\".dir = \"jkl2\"
";
    let lib_content = "
X = 1
";
    create_pkg(".", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest22 = "
[package]
name = \"example2.com/def\"
description = \" Some text2.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def2", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"
description = \" Some text3.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi2", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
description = \" Some text4.\"

[dependencies]
";
    let lib_content42 = "
W = 42
";
    create_pkg("jkl2", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_locks().unwrap();
    match pkg_manager.install_deps(false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(3, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_deps_installs_dependencies_with_constraints()
{
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"

[constraints]
\"example1.com/jkl\" = \"<=4.5.6\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    let lib_content = "
X = 1
";
    create_pkg(".", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"*\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    match pkg_manager.install_deps(false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Wildcard => assert!(true),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(3, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_deps_reinstalls_dependencies_for_force_flag()
{
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
";
    let lib_content = "
X = 1
";
    create_pkg(".", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.install_deps(false, false, false).unwrap();
    recursively_remove("def", false).unwrap();
    recursively_remove("ghi", false).unwrap();
    recursively_remove("jkl", false).unwrap();
    let manifest22 = "
[package]
name = \"example2.com/def\"
description = \"Some text2.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content22 = "
Y = 22
";
    create_pkg("def", &manifest22[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content22[1..])));
    let manifest32 = "
[package]
name = \"example3.com/ghi\"
description = \"Some text3.\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
";
    let lib_content32 = "
Z = 32
";
    create_pkg("ghi", &manifest32[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content32[1..])));
    let manifest42 = "
[package]
name = \"example1.com/jkl\"
description = \"Some text4.\"

[dependencies]
";
    let lib_content42 = "
W = 42
";    
    create_pkg("jkl", &manifest42[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content42[1..])));
    match pkg_manager.install_deps(false, true, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest22[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content22[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest32[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content32[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest42[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example3.com/ghi"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content42[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(3, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_install_deps_removes_unused_packages()
{
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"
\"example3.com/ghi\" = \"3.4.5\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    let lib_content = "
X = 1
";
    create_pkg(".", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.install_deps(false, false, false).unwrap();
    fs::remove_file("Unlab.toml").unwrap();
    let manifest12 = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example2.com/def\" = \"2.3.4\"

[sources]
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write("Unlab.toml", &manifest12[1..]).unwrap();
    match pkg_manager.install_deps(false, false, false) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // mno
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(2, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_remove_removes_package()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example.com/abc\".versions.\"1.2.3\".dir = \"abc\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example.com/abc\"

[dependencies]
";
    let script_content = "
#!/usr/bin/env unlab-gpu --
println(1 + 2)
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], Some(("bin/script.un", &script_content[1..])), Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example.com/abc"))], false, false, false).unwrap();
    match pkg_manager.remove(&[PkgName::new(String::from("example.com/abc"))]) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut script_file = PathBuf::from("work");
            script_file.push("bin");
            script_file.push("script.un");
            match fs::metadata(script_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(true, versions.is_empty());
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_remove_removes_package_without_dependencies()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false).unwrap();
    match pkg_manager.remove(&[PkgName::new(String::from("example1.com/abc"))]) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(4, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_remove_removes_packages()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false).unwrap();
    match pkg_manager.remove(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))]) {
        Ok(()) => {
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            match fs::metadata(manifest_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            match fs::metadata(dependents_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            match fs::metadata(paths_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            match fs::metadata(lib_file) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(3, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        Err(_) => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_remove_complains_on_can_not_remove_package()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false).unwrap();
    match pkg_manager.remove(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example3.com/ghi"))]) {
        Err(Error::PkgName(name, msg)) => {
            assert_eq!(PkgName::new(String::from("example3.com/ghi")), name);
            assert_eq!(String::from("can't remove package"), msg);
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}

#[sealed_test]
fn test_pkg_manager_remove_complains_on_package_is_not_installed()
{
    fs::create_dir("home").unwrap();
    let mut sources_file = PathBuf::from("home");
    sources_file.push("sources.toml");
    let sources_content = "
\"example1.com/abc\".versions.\"1.2.3\".dir = \"abc\"
\"example2.com/def\".versions.\"2.3.4\".dir = \"def\"
\"example3.com/ghi\".versions.\"3.4.5\".dir = \"ghi\"
\"example1.com/jkl\".versions.\"4.5.6\".dir = \"jkl\"
\"example1.com/mno\".versions.\"5.6.7\".dir = \"mno\"
";
    fs::write(sources_file, &sources_content[1..]).unwrap();
    let manifest = "
[package]
name = \"example1.com/abc\"

[dependencies]
\"example3.com/ghi\" = \"3.4.5\"
\"example1.com/jkl\" = \"4.5.6\"
";
    let lib_content = "
X = 1
";
    create_pkg("abc", &manifest[1..], None, Some(("lib/pl.jan.nowak/abc/lib.un", &lib_content[1..])));
    let manifest2 = "
[package]
name = \"example2.com/def\"

[dependencies]
\"example1.com/jkl\" = \"4.5.0\"
\"example1.com/mno\" = \"5.6.7\"
";
    let lib_content2 = "
Y = 2
";
    create_pkg("def", &manifest2[1..], None, Some(("lib/pl.nowakowski/def/lib.un", &lib_content2[1..])));
    let manifest3 = "
[package]
name = \"example3.com/ghi\"

[dependencies]
";
    let lib_content3 = "
Z = 3
";
    create_pkg("ghi", &manifest3[1..], None, Some(("lib/pl.nowakowski/ghi/lib.un", &lib_content3[1..])));
    let manifest4 = "
[package]
name = \"example1.com/jkl\"

[dependencies]
";
    let lib_content4 = "
W = 4
";
    create_pkg("jkl", &manifest4[1..], None, Some(("lib/pl.jan.nowak/jkl/lib.un", &lib_content4[1..])));
    let manifest5 = "
[package]
name = \"example1.com/mno\"

[dependencies]
";
    let lib_content5 = "
V = 5
";
    create_pkg("mno", &manifest5[1..], None, Some(("lib/pl.jan.nowak/mno/lib.un", &lib_content5[1..])));
    let mut bin_dir = PathBuf::from("work");
    bin_dir.push("bin");
    let mut lib_dir = PathBuf::from("work");
    lib_dir.push("lib");
    let mut doc_dir = PathBuf::from("work");
    doc_dir.push("doc");
    let printer = EmptyPrinter::new();
    let mut pkg_manager = match PkgManager::new(PathBuf::from("home"), PathBuf::from("work"), bin_dir.clone(), lib_dir.clone(), doc_dir.clone(), Vec::new(), Arc::new(printer)) {
        Ok(tmp_pkg_manager) => tmp_pkg_manager, 
        Err(_) => {
            assert!(false);
            return;
        },
    };
    pkg_manager.load_constraints().unwrap();
    pkg_manager.load_sources().unwrap();
    pkg_manager.install(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example2.com/def"))], false, false, false).unwrap();
    match pkg_manager.remove(&[PkgName::new(String::from("example1.com/abc")), PkgName::new(String::from("example1.com/pqr"))]) {
        Err(Error::PkgName(name, msg)) => {
            assert_eq!(PkgName::new(String::from("example1.com/pqr")), name);
            assert_eq!(String::from("package isn't installed"), msg);
            let mut new_part_info_dir = PathBuf::from("work");
            new_part_info_dir.push("var");
            new_part_info_dir.push("info.new.part");
            match fs::metadata(new_part_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut new_info_dir = PathBuf::from("work");
            new_info_dir.push("var");
            new_info_dir.push("info.new");
            match fs::metadata(new_info_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            let mut tmp_dir = PathBuf::from("work");
            tmp_dir.push("tmp");
            match fs::metadata(tmp_dir.as_path()) {
                Err(err) => assert_eq!(ErrorKind::NotFound, err.kind()),
                Ok(_) => assert!(false),
            }
            // abc
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("abc");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("abc");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("abc");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content[1..]), fs::read_to_string(lib_file).unwrap());
            // def
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example2.com");
            pkg_info_dir.push("def");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest2[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(true, dependents.is_empty());
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("def");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("def");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content2[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example3.com");
            pkg_info_dir.push("ghi");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest3[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.nowakowski");
            lib.push("ghi");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.nowakowski");
            lib_file.push("ghi");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content3[1..]), fs::read_to_string(lib_file).unwrap());
            // jkl
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("jkl");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest4[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(2, dependents.len());
            match dependents.get(&PkgName::new(String::from("example1.com/abc"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("4.5.0").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("jkl");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("jkl");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content4[1..]), fs::read_to_string(lib_file).unwrap());
            // ghi
            let mut pkg_info_dir = PathBuf::from("work");
            pkg_info_dir.push("var");
            pkg_info_dir.push("info");
            pkg_info_dir.push("example1.com");
            pkg_info_dir.push("mno");
            let mut manifest_file = pkg_info_dir.clone();
            manifest_file.push("manifest.toml");
            assert_eq!(String::from(&manifest5[1..]), fs::read_to_string(manifest_file).unwrap());
            let mut dependents_file = pkg_info_dir.clone();
            dependents_file.push("dependents.toml");
            let dependents = load_version_reqs(dependents_file).unwrap();
            assert_eq!(1, dependents.len());
            match dependents.get(&PkgName::new(String::from("example2.com/def"))) {
                Some(version_req) => {
                    assert_eq!(1, version_req.single_reqs().len());
                    match &version_req.single_reqs()[0] {
                        SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let mut paths_file = pkg_info_dir.clone();
            paths_file.push("paths.toml");
            let paths = Paths::load(paths_file).unwrap();
            assert_eq!(true, paths.bin.is_empty());
            let mut lib = PathBuf::from("pl.jan.nowak");
            lib.push("mno");
            assert_eq!(vec![lib.to_string_lossy().into_owned()], paths.lib);
            let mut lib_file = PathBuf::from("work");
            lib_file.push("lib");
            lib_file.push("pl.jan.nowak");
            lib_file.push("mno");
            lib_file.push("lib.un");
            assert_eq!(String::from(&lib_content5[1..]), fs::read_to_string(lib_file).unwrap());
            let versions = pkg_manager.pkg_versions_for_bucket("versions").unwrap();
            assert_eq!(5, versions.len());
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/abc")), Version::parse("1.2.3").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example2.com/def")), Version::parse("2.3.4").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example3.com/ghi")), Version::parse("3.4.5").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/jkl")), Version::parse("4.5.6").unwrap())));
            assert_eq!(true, versions.contains(&(PkgName::new(String::from("example1.com/mno")), Version::parse("5.6.7").unwrap())));
            let new_versions = pkg_manager.pkg_versions_for_bucket("new_versions").unwrap();
            assert_eq!(true, new_versions.is_empty());
            let pkgs_to_remove = pkg_manager.pkg_names_for_bucket("pkgs_to_remove").unwrap();
            assert_eq!(true, pkgs_to_remove.is_empty());
            let pkgs_to_change = pkg_manager.pkg_names_for_bucket("pkgs_to_change").unwrap();
            assert_eq!(true, pkgs_to_change.is_empty());
        },
        _ => assert!(false),
    }
}
