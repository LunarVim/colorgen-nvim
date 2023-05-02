use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

pub use rgb_color::{RgbColor, RgbParsingError};

mod rgb_color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette(pub HashMap<String, RgbColor>);

impl Display for Palette {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "local colors = {{")?;
        for (key, val) in &self.0 {
            write!(f, "\n  {key} = \"{val}\",")?;
        }
        write!(f, "\n}}\n\nreturn colors")
    }
}
