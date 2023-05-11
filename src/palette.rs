use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

pub use rgb_color::{RgbColor, RgbParsingError};

mod rgb_color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette(pub LinkedHashMap<String, RgbColor>);

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "local colors = {{")?;
        for (key, val) in &self.0 {
            write!(f, "\n  {key} = \"{val}\",")?;
        }
        write!(f, "\n}}\n\nreturn colors")
    }
}
