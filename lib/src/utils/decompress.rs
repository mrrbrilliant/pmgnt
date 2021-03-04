use crate::pkgbuild_statics::*;
use std::{
    fs::File,
    io::{copy, Result},
    str,
};
use zstd::Decoder;

pub fn decompress_zstd(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        Decoder::new(file)?
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}
