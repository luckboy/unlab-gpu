//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
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
                    println!("{}", t);
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
                    println!("{}", t);
                    assert_eq!(String::from(&expected_t[1..]), t);
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
