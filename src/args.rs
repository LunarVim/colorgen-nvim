use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ColorgenArgs {
    /// The filename for your colorscheme
    pub filename: String,
}
