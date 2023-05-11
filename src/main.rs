use clap::Parser;
use colorgen_nvim::{cli::ColorgenArgs, Template};
use std::{error, fs::read_to_string, path::Path};

fn main() {
    match inner() {
        Ok(_) => {}
        Err(err) => println!("{err}"),
    }
}

fn inner() -> Result<(), Box<dyn error::Error>> {
    let template: Template = toml::from_str(&read_to_string(ColorgenArgs::parse().filename)?)?;

    template.generate(Path::new(""))?;
    Ok(())
}
