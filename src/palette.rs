use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

pub use rgb_color::{RgbColor, RgbParsingError};

mod rgb_color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette(pub LinkedHashMap<String, RgbColor>);

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "local colors = {{{palette}\n\nreturn colors",
            palette = InnerPalette {
                colors: &self.0,
                indent: "  "
            }
        )
    }
}

pub struct InnerPalette<'a> {
    pub colors: &'a LinkedHashMap<String, RgbColor>,
    pub indent: &'a str,
}

impl Display for InnerPalette<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (key, val) in self.colors {
            write!(f, "\n{indent}{key} = \"{val}\",", indent = self.indent)?;
        }
        write!(f, "\n}}")?;
        Ok(())
    }
}
