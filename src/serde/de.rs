use serde::{
    de::{self, Visitor},
    Deserialize,
};
use std::fmt;

// SemVer requirement deserializer
#[derive(Clone, Debug, PartialEq)]
pub struct VersionReq(semver::VersionReq);
struct VersionReqVisitor;

impl<'de> Visitor<'de> for VersionReqVisitor {
    type Value = VersionReq;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("A valid SemVer version requirement")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let inner = semver::VersionReq::parse(v).map_err(|it| {
            de::Error::invalid_value(de::Unexpected::Str(&format!("{}", it)), &self)
        })?;
        Ok(VersionReq(inner))
    }
}

impl<'de> Deserialize<'de> for VersionReq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionReqVisitor)
    }
}
