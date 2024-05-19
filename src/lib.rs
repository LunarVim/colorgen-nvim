use crate::{
    formatters::{InitLua, InitSetup, VimColorsFile},
    global::Global,
    information::Information,
    macros::write_file,
    palette::{InnerPalette, Palette},
    sections::{Sections, SectionsFormatter, ThemeHighlights},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

pub mod cli;
pub mod formatters;
pub mod global;
pub mod information;
pub(crate) mod macros;
pub mod palette;
pub mod sections;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub information: Information,
    pub palette: Palette,
    pub global: Option<Global>,
    #[serde(flatten)]
    pub sections: Sections,
}

impl Template {
    pub fn generate(&self, base_path: &Path) -> io::Result<()> {
        let name = Path::new(&self.information.name);
        self.setup_directories(base_path)?;

        write_file!([base_path, "lua", name, "init.lua"], self.generate_init(),)?;

        let init_lua = name.with_extension("lua");
        write_file!(
            [base_path, "colors", init_lua],
            self.generate_vim_colors_file(),
        )?;

        write_file!([base_path, "lua", name, "palette.lua"], &self.palette,)?;

        write_file!([base_path, "lua", name, "theme.lua"], self.generate_theme(),)?;

        Ok(())
    }

    pub fn setup_directories(&self, base_path: &Path) -> io::Result<()> {
        let name = Path::new(&self.information.name);
        fs::create_dir_all(
            [base_path, Path::new("lua"), name]
                .iter()
                .collect::<PathBuf>(),
        )?;
        fs::create_dir_all([base_path, Path::new("colors")].iter().collect::<PathBuf>())?;
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

    pub fn generate_single_file(&self) -> SingleFile {
        SingleFile {
            init_setup: InitSetup {
                name: &self.information.name,
                background: self.information.background,
                indent: "",
            },
            palette: InnerPalette {
                colors: &self.palette.0,
                indent: "  ",
            },
            global: self.global.as_ref(),
            theme: ThemeHighlights {
                theme_name: &self.information.name,
                sections: &self.sections,
                indent: "",
            },
        }
    }
}

pub struct SingleFile<'a> {
    pub init_setup: InitSetup<'a>,
    pub palette: InnerPalette<'a>,
    pub theme: ThemeHighlights<'a>,
    pub global: Option<&'a Global>,
}

impl SingleFile<'_> {
    pub fn write_to_file(&self, path: &Path) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);
        file.write_fmt(format_args!("{}", self))?;
        Ok(())
    }
}

impl Display for SingleFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(global) = &self.global {
            writeln!(f, "{}", global)?;
        }
        write!(f, "{}\n\n", self.init_setup)?;
        write!(f, "local c = {{")?;
        write!(f, "{}\n\n", self.palette)?;
        write!(f, "local hl = vim.api.nvim_set_hl")?;
        write!(f, "{}", self.theme)?;
        Ok(())
    }
}
