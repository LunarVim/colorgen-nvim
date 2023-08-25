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
    let args = ColorgenArgs::parse();
    let template: Template = toml::from_str(&read_to_string(&args.filename)?)?;

    match args.single_file {
        false => {
            template.generate(
                &args
                    .output
                    .as_deref()
                    .unwrap_or(Path::new(&template.information.name)),
            )?;
        }
        true => {
            let path = match args.output {
                Some(mut path) => {
                    if path.is_dir() {
                        path.push(&template.information.name);
                        path.as_mut_os_string().push(".lua");
                    }
                    path
                }
                None => Path::new(&template.information.name).with_extension("lua"),
            };

            template.generate_single_file().write_to_file(&path)?;
        }
    }

    Ok(())
}
