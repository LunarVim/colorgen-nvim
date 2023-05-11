use hex::FromHex;
use serde::{de::Visitor, Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct RgbColor(pub [u8; 3]);

impl Display for RgbColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let RgbColor([r, g, b]) = self;
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl FromStr for RgbColor {
    type Err = RgbParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(number) = s.strip_prefix("#") {
            FromHex::from_hex(number)
                .map(|v: [u8; 3]| RgbColor(v))
                .map_err(|_| RgbParsingError)
        } else {
            Err(RgbParsingError)
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("failed to parse a hex color")]
pub struct RgbParsingError;

impl Serialize for RgbColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format_args!("{self}"))
    }
}

struct BaseIndexVisitor;

impl<'de> Deserialize<'de> for RgbColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BaseIndexVisitor)
    }
}

impl<'de> Visitor<'de> for BaseIndexVisitor {
    type Value = RgbColor;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a hex color in the form #ff0000")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}
