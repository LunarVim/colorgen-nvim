use crate::{information::Information, palette::Palette, sections::Sections};
use formatters::{InitLua, VimColorsFile};
use macros::write_file;
use sections::SectionsFormatter;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub mod cli;
pub mod formatters;
pub mod information;
pub(crate) mod macros;
pub mod palette;
pub mod sections;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub information: Information,
    pub palette: Palette,
    #[serde(flatten)]
    pub sections: Sections,
}

impl Template {
    pub fn generate(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let name = Path::new(&self.information.name);
        self.setup_directories(base_path)?;

        write_file!(
            [base_path, name, "lua", name, "init.lua"],
            self.generate_init(),
        )?;

        let init_lua = name.with_extension("lua");
        write_file!(
            [base_path, name, "colors", init_lua],
            self.generate_vim_colors_file(),
        )?;

        write_file!([base_path, name, "lua", name, "palette.lua"], &self.palette,)?;

        write_file!(
            [base_path, name, "lua", name, "theme.lua"],
            self.generate_theme(),
        )?;

        Ok(())
    }

    pub fn setup_directories(&self, base_path: &Path) -> io::Result<()> {
        let name = Path::new(&self.information.name);
        fs::create_dir_all(
            [base_path, name, Path::new("lua"), name]
                .iter()
                .collect::<PathBuf>(),
        )?;
        fs::create_dir_all(
            [base_path, name, Path::new("colors")]
                .iter()
                .collect::<PathBuf>(),
        )?;
        Ok(())
    }

    pub fn generate_init(&self) -> InitLua {
        InitLua {
            name: &self.information.name,
            background: self.information.background,
        }
    }

    pub fn generate_vim_colors_file(&self) -> VimColorsFile {
        VimColorsFile {
            name: &self.information.name,
        }
    }

    pub fn generate_theme(&self) -> SectionsFormatter {
        SectionsFormatter {
            theme_name: &self.information.name,
            sections: &self.sections,
        }
    }
}
