use crate::sections::color_spec::ColorSpec;
use serde::{de::Visitor, Deserialize, Serialize};

impl Serialize for ColorSpec {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for ColorSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorSpecVisitor)
    }
}

struct ColorSpecVisitor;

impl<'de> Visitor<'de> for ColorSpecVisitor {
    type Value = ColorSpec;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("Expected a str in the form of `foreground background style special blend`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}
