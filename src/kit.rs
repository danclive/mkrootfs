use std::{io::Result, path::Path};

use xshell::{cmd, Shell};

pub fn report_size(sh: &Shell, path: impl AsRef<Path>) -> Result<usize> {
    let path = path.as_ref();

    if !path.try_exists()? {
        // todo: return error
    }

    let ret = cmd!(sh, "du -sb {path}").read().unwrap();
    // todo: resolve error

    let size: usize = ret.split('\t').collect::<Vec<&str>>()[0].parse().unwrap();
    // todo: resolve error

    Ok(size)
}
