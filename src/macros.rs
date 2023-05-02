use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

pub(crate) fn write_fmt<'a>(path: &Path, format: impl Display) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path.into_iter().collect::<PathBuf>())?);
    file.write_fmt(format_args!("{}", format))?;
    Ok(())
}

macro_rules! write_file {
    ([$($path:expr),*], $display:expr,) => {
        {
            let path = [$(
                {
                    let path: &Path = $path.as_ref();
                    path
                }),+].iter().collect::<PathBuf>();
            crate::macros::write_fmt(&path, $display)
        }
    };
}

pub(crate) use write_file;
