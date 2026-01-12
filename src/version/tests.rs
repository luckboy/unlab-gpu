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

#[test]
fn test_version_req_parse_parses_version_requiremets()
{
    match VersionReq::parse("*") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Wildcard => assert!(true),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("=1.2.3") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Eq, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("!=2.3.4") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Ne, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("<3.4.5") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Lt, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse(">=4.5.6") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Ge, version) => assert_eq!(Version::parse("4.5.6").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse(">5.6.7") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Gt, version) => assert_eq!(Version::parse("5.6.7").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("<=6.7.8") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Le, version) => assert_eq!(Version::parse("6.7.8").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("^7.8.9") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("7.8.9").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("~8.9.10") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Tilde, version) => assert_eq!(Version::parse("8.9.10").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    match VersionReq::parse("9.10.11") {
        Ok(version_req) => {
            assert_eq!(1, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Default, version) => assert_eq!(Version::parse("9.10.11").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_req_parse_parses_version_requiremet_with_many_single_version_requirements()
{
    match VersionReq::parse(">=1.2.3,!=2.3.4,<=3.4.5") {
        Ok(version_req) => {
            assert_eq!(3, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Ge, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                _ => assert!(false),
            }
            match &version_req.single_reqs()[1] {
                SingleVersionReq::Pair(VersionOp::Ne, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                _ => assert!(false),
            }
            match &version_req.single_reqs()[2] {
                SingleVersionReq::Pair(VersionOp::Le, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_req_parse_parses_version_requiremets_with_many_single_version_requirements_and_spaces()
{
    match VersionReq::parse(" >= 1.2.3 ,  !=  2.3.4  ,   <=   3.4.5   ") {
        Ok(version_req) => {
            assert_eq!(3, version_req.single_reqs().len());
            match &version_req.single_reqs()[0] {
                SingleVersionReq::Pair(VersionOp::Ge, version) => assert_eq!(Version::parse("1.2.3").unwrap(), *version),
                _ => assert!(false),
            }
            match &version_req.single_reqs()[1] {
                SingleVersionReq::Pair(VersionOp::Ne, version) => assert_eq!(Version::parse("2.3.4").unwrap(), *version),
                _ => assert!(false),
            }
            match &version_req.single_reqs()[2] {
                SingleVersionReq::Pair(VersionOp::Le, version) => assert_eq!(Version::parse("3.4.5").unwrap(), *version),
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_version_req_parse_complains_on_invalid_version()
{
    match VersionReq::parse("1.a.3") {
        Err(Error::InvalidVersion) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_wildcard()
{
    let version_req = VersionReq::parse("*").unwrap();
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("2.3.4").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("3.4.5").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_eq_operator()
{
    let version_req = VersionReq::parse("=1.2.3").unwrap();
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.3.4").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("3.4.5").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_ne_operator()
{
    let version_req = VersionReq::parse("!=1.2.3").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("2.3.4").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("3.4.5").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_lt_operator()
{
    let version_req = VersionReq::parse("<1.2.3").unwrap();
    assert_eq!(true, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.2.4").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_ge_operator()
{
    let version_req = VersionReq::parse(">=1.2.3").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.4").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_gt_operator()
{
    let version_req = VersionReq::parse(">1.2.3").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.4").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremet_for_le_operator()
{
    let version_req = VersionReq::parse("<=1.2.3").unwrap();
    assert_eq!(true, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.2.4").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremets_for_default_operator()
{
    let version_req = VersionReq::parse("1.2.3").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.3.4").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0-alpha").unwrap()));
    let version_req = VersionReq::parse("0.1.2").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("0.1.1").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.1.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.1.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.2.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.2.0-alpha").unwrap()));
    let version_req = VersionReq::parse("0.0.1").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.0.1").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.0.2").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.0.2-alpha").unwrap()));
    let version_req = VersionReq::parse("1.2.0").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.1.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.3.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0-alpha").unwrap()));
    let version_req = VersionReq::parse("1.2").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.1.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.3.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0-alpha").unwrap()));
    let version_req = VersionReq::parse("1").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0-alpha").unwrap()));
    let version_req = VersionReq::parse("0.1").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.1.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.1.2").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.2.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("0.2.0-alpha").unwrap()));
    let version_req = VersionReq::parse("0").unwrap();
    assert_eq!(true, version_req.matches(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("0.1.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.0.0-alpha").unwrap()));
}

#[test]
fn test_version_req_matches_matches_versions_to_version_requiremets_for_tilde_operator()
{
    let version_req = VersionReq::parse("~1.2.3").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.3.4").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.3.0-alpha").unwrap()));
    let version_req = VersionReq::parse("~1.2").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.1.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.3.4").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("1.3.0-alpha").unwrap()));
    let version_req = VersionReq::parse("~1").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("0.0.0").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.0.0-alpha").unwrap()));
}

#[test]
fn test_version_req_matche_matches_versions_to_version_requiremet_with_many_single_version_requirements()
{
    let version_req = VersionReq::parse(">=1.2.3,!=2.3.4,<=3.4.5").unwrap();
    assert_eq!(false, version_req.matches(&Version::parse("1.2.2").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.3").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("1.2.4").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("2.3.4").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("2.3.5").unwrap()));
    assert_eq!(true, version_req.matches(&Version::parse("3.4.5").unwrap()));
    assert_eq!(false, version_req.matches(&Version::parse("3.4.6").unwrap()));
}

#[test]
fn test_version_req_fmt_formats_version_requiremets()
{
    assert_eq!(String::from("*"), format!("{}", VersionReq::parse("*").unwrap()));
    assert_eq!(String::from("=1.2.3"), format!("{}", VersionReq::parse("=1.2.3").unwrap()));
    assert_eq!(String::from("!=2.3.4"), format!("{}", VersionReq::parse("!=2.3.4").unwrap()));
    assert_eq!(String::from("<3.4.5"), format!("{}", VersionReq::parse("<3.4.5").unwrap()));
    assert_eq!(String::from(">=4.5.6"), format!("{}", VersionReq::parse(">=4.5.6").unwrap()));
    assert_eq!(String::from(">5.6.7"), format!("{}", VersionReq::parse(">5.6.7").unwrap()));
    assert_eq!(String::from("<=6.7.8"), format!("{}", VersionReq::parse("<=6.7.8").unwrap()));
    assert_eq!(String::from("^7.8.9"), format!("{}", VersionReq::parse("^7.8.9").unwrap()));
    assert_eq!(String::from("~8.9.10"), format!("{}", VersionReq::parse("~8.9.10").unwrap()));
}

#[test]
fn test_version_req_fmt_formats_version_requiremet_with_many_single_version_requirements()
{ assert_eq!(String::from(">=1.2.3,!=2.3.4,<=3.4.5"), format!("{}", VersionReq::parse(">=1.2.3,!=2.3.4,<=3.4.5").unwrap())); }
