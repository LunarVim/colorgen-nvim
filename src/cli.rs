use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ColorgenArgs {
    /// The filename for your colorscheme
    pub filename: PathBuf,

    /// Write into a single file instead of writing a whole module
    #[arg(short, long, default_value_t = false)]
    pub single_file: bool,

    /// File/Directory to write the theme to.
    ///
    /// If used together with `--single-file` it will either write directly to the file, or if
    /// `--output` is a directory it will write into this directory under `<theme-name>.lua`.
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
