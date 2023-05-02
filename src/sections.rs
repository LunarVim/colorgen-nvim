use crate::palette::Palette;
use crate::sections::color_spec::{parser::Color, ColorFormat};
use color_spec::ColorSpec;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

mod color_spec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sections(pub HashMap<String, Section>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Section(pub HashMap<String, ColorSpec>);

impl Sections {
    pub fn check_colors(&self, palette: &Palette) -> Result<(), MissingPaletteColor> {
        for (section_name, section) in &self.0 {
            for (hl_group, color_spec) in &section.0 {
                match color_spec {
                    ColorSpec::Color(ColorFormat {
                        fg, bg, special, ..
                    }) => {
                        check_palette_ref(fg, palette, section_name, hl_group)?;
                        check_palette_ref(bg, palette, section_name, hl_group)?;
                        check_palette_ref(special, palette, section_name, hl_group)?;
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }
}

pub fn check_palette_ref(
    color: &Option<Color>,
    palette: &Palette,
    section_name: &str,
    hl_group: &str,
) -> Result<(), MissingPaletteColor> {
    match color {
        Some(Color::PaletteRef(palette_ref)) => {
            if palette.0.contains_key(palette_ref) {
                Ok(())
            } else {
                Err(MissingPaletteColor {
                    section_name: section_name.to_string(),
                    highlight_group: hl_group.to_string(),
                    color_ref: palette_ref.to_string(),
                })
            }
        }
        _ => Ok(()),
    }
}

#[derive(Debug, thiserror::Error)]
#[error("In highlight group {section_name}.{highlight_group} the palette color {color_ref} could not be found")]
pub struct MissingPaletteColor {
    section_name: String,
    highlight_group: String,
    color_ref: String,
}

pub struct SectionsFormatter<'a> {
    pub theme_name: &'a str,
    pub sections: &'a Sections,
}

impl<'a> Display for SectionsFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
local c = require('{theme_name}.palette')

local hl = vim.api.nvim_set_hl
local theme = {{}}

theme.set_highlights = function()",
            theme_name = self.theme_name
        )?;

        for (table_name, section) in &self.sections.0 {
            write!(f, "\n\n  -- {table_name}")?;
            for (hl_group, color_spec) in &section.0 {
                write!(f, "\n  hl(0, \"{hl_group}\", {color_spec})")?;
            }
        }

        write!(f, "\n\nend\n\nreturn theme")
    }
}
