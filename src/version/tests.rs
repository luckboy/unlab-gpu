//
// Copyright (c) 2026 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_version_parse_parses_versions()
{
    match Version::parse("1.2.3") {
        Ok(version) => {
            assert_eq!(&[1, 2, 3], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("4.5") {
        Ok(version) => {
            assert_eq!(&[4, 5], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("1") {
        Ok(version) => {
            assert_eq!(&[1], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_parse_parses_versions_with_pre_releases()
{
    match Version::parse("1.2.3-alpha.4") {
        Ok(version) => {
            assert_eq!(&[1, 2, 3], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Alphanumeric(String::from("alpha")), PreReleaseIdent::Numeric(4)].as_slice()), version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("4.5-2.beta.3") {
        Ok(version) => {
            assert_eq!(&[4, 5], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Numeric(2), PreReleaseIdent::Alphanumeric(String::from("beta")), PreReleaseIdent::Numeric(3)].as_slice()), version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("1-2.x-y.3") {
        Ok(version) => {
            assert_eq!(&[1], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Numeric(2), PreReleaseIdent::Alphanumeric(String::from("x-y")), PreReleaseIdent::Numeric(3)].as_slice()), version.pre_release_idents());
            assert_eq!(None, version.build_idents());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_parse_parses_versions_with_builds()
{
    match Version::parse("1.2.3+build.12345") {
        Ok(version) => {
            assert_eq!(&[1, 2, 3], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(Some([String::from("build"), String::from("12345")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("4.5+12345.sha.67890") {
        Ok(version) => {
            assert_eq!(&[4, 5], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(Some([String::from("12345"), String::from("sha"), String::from("67890")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("1+2345.x-y.3456") {
        Ok(version) => {
            assert_eq!(&[1], version.numeric_idents());
            assert_eq!(None, version.pre_release_idents());
            assert_eq!(Some([String::from("2345"), String::from("x-y"), String::from("3456")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_parse_parses_versions_with_pre_releases_and_builds()
{
    match Version::parse("1.2.3-alpha.4+build.12345") {
        Ok(version) => {
            assert_eq!(&[1, 2, 3], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Alphanumeric(String::from("alpha")), PreReleaseIdent::Numeric(4)].as_slice()), version.pre_release_idents());
            assert_eq!(Some([String::from("build"), String::from("12345")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("4.5-2.beta.3+12345.sha.67890") {
        Ok(version) => {
            assert_eq!(&[4, 5], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Numeric(2), PreReleaseIdent::Alphanumeric(String::from("beta")), PreReleaseIdent::Numeric(3)].as_slice()), version.pre_release_idents());
            assert_eq!(Some([String::from("12345"), String::from("sha"), String::from("67890")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
    match Version::parse("1-2.x-y.3+2345.x-y.3456") {
        Ok(version) => {
            assert_eq!(&[1], version.numeric_idents());
            assert_eq!(Some([PreReleaseIdent::Numeric(2), PreReleaseIdent::Alphanumeric(String::from("x-y")), PreReleaseIdent::Numeric(3)].as_slice()), version.pre_release_idents());
            assert_eq!(Some([String::from("2345"), String::from("x-y"), String::from("3456")].as_slice()), version.build_idents());
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_parse_complains_on_invalid_version()
{
    match Version::parse("1.a.3") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3-1..2") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3-ab/cd") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3-abcd.1.e\\f") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3+1..2") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3+ab/cd") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
    match Version::parse("1.2.3+abcd.1.e\\f") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_version_cmp_compares_versions()
{
    let version = Version::parse("1.2.3").unwrap();
    let version2 = Version::parse("1.2.3").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3").unwrap();
    let version2 = Version::parse("1.2.4").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.4").unwrap();
    let version2 = Version::parse("1.2.3").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
    let version = Version::parse("1.2").unwrap();
    let version2 = Version::parse("1.2.0").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.0").unwrap();
    let version2 = Version::parse("1.2").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2").unwrap();
    let version2 = Version::parse("1.2.3").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.3").unwrap();
    let version2 = Version::parse("1.2").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
}

#[test]
fn test_version_cmp_compares_versions_with_pre_releases()
{
    let version = Version::parse("1.2.3-alpha.4").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.5").unwrap();
    let version2 = Version::parse("1.2.4-alpha.4").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.4-alpha.4").unwrap();
    let version2 = Version::parse("1.2.3-alpha.5").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.4").unwrap();
    let version2 = Version::parse("1.2.3-alpha.5").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.5").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha").unwrap();
    let version2 = Version::parse("1.2.3-beta").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.3-beta").unwrap();
    let version2 = Version::parse("1.2.3-alpha").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
    let version = Version::parse("1.2.3-4").unwrap();
    let version2 = Version::parse("1.2.3-alpha").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha").unwrap();
    let version2 = Version::parse("1.2.3-4").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.4").unwrap();
    let version2 = Version::parse("1.2.3").unwrap();
    assert_eq!(Ordering::Less, version.cmp(&version2));
    let version = Version::parse("1.2.3").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4").unwrap();
    assert_eq!(Ordering::Greater, version.cmp(&version2));
}

#[test]
fn test_version_cmp_compares_versions_with_builds()
{
    let version = Version::parse("1.2.3+build.12345").unwrap();
    let version2 = Version::parse("1.2.3+sha.67890").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3+sha.67890").unwrap();
    let version2 = Version::parse("1.2.3+build.12345").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3+build.12345").unwrap();
    let version2 = Version::parse("1.2.3").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3").unwrap();
    let version2 = Version::parse("1.2.3+build.12345").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
}

#[test]
fn test_version_cmp_compares_versions_with_pre_releases_and_builds()
{
    let version = Version::parse("1.2.3-alpha.4+build.12345").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4+sha.67890").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.4+sha.67890").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4+build.12345").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.4+build.12345").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
    let version = Version::parse("1.2.3-alpha.4").unwrap();
    let version2 = Version::parse("1.2.3-alpha.4+build.12345").unwrap();
    assert_eq!(Ordering::Equal, version.cmp(&version2));
}

#[test]
fn test_version_fmt_formats_versions()
{
    assert_eq!(String::from("1.2.3"), format!("{}", Version::parse("1.2.3").unwrap()));
    assert_eq!(String::from("4.5"), format!("{}", Version::parse("4.5").unwrap()));
    assert_eq!(String::from("1"), format!("{}", Version::parse("1").unwrap()));
}

#[test]
fn test_version_fmt_formats_versions_with_pre_releases()
{
    assert_eq!(String::from("1.2.3-alpha.4"), format!("{}", Version::parse("1.2.3-alpha.4").unwrap()));
    assert_eq!(String::from("4.5-2.beta.3"), format!("{}", Version::parse("4.5-2.beta.3").unwrap()));
    assert_eq!(String::from("1-2.x-y.3"), format!("{}", Version::parse("1-2.x-y.3").unwrap()));
}

#[test]
fn test_version_fmt_formats_versions_with_builds()
{
    assert_eq!(String::from("1.2.3+build.12345"), format!("{}", Version::parse("1.2.3+build.12345").unwrap()));
    assert_eq!(String::from("4.5+12345.sha.67890"), format!("{}", Version::parse("4.5+12345.sha.67890").unwrap()));
    assert_eq!(String::from("1+2345.x-y.3456"), format!("{}", Version::parse("1+2345.x-y.3456").unwrap()));
}

#[test]
fn test_version_fmt_formats_versions_with_pre_releases_and_builds()
{
    assert_eq!(String::from("1.2.3-alpha.4+build.12345"), format!("{}", Version::parse("1.2.3-alpha.4+build.12345").unwrap()));
    assert_eq!(String::from("4.5-2.beta.3+12345.sha.67890"), format!("{}", Version::parse("4.5-2.beta.3+12345.sha.67890").unwrap()));
    assert_eq!(String::from("1-2.x-y.3+2345.x-y.3456"), format!("{}", Version::parse("1-2.x-y.3+2345.x-y.3456").unwrap()));
}
