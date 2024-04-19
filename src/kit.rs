use std::path::Path;

use xshell::{cmd, Shell};

use crate::error::{Error, Result};

pub fn report_size(sh: &Shell, path: impl AsRef<Path>) -> Result<usize> {
    let path = path.as_ref();

    if !path.try_exists()? {
        return Err(Error::FileNotExist(format!("{}", path.display())));
    }

    let ret = cmd!(sh, "du -sb {path}").read()?;

    let size: usize = ret.split('\t').collect::<Vec<&str>>()[0].parse()?;

    Ok(size)
}
