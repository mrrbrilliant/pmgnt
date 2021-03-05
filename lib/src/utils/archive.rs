use super::compress::compress_zstd;
use crate::pi_statics::LOCAL_DIR;
use crate::pkgbuild_statics::*;
use crate::structs::Package;
use std::{
    fs::{remove_file, File},
    io::Result,
    path::{Path, PathBuf},
};
use tar::Archive;

pub fn extract_archive(arg_file: &str, dest: &str) -> Result<()> {
    let filename = Path::new(arg_file.trim_end_matches(SUFFIX.as_str()));

    let file = File::open(filename)?;
    let mut a = Archive::new(file);

    match a.entries() {
        Ok(all_entries) => Ok(for file in all_entries {
            let mut f = file?;
            let p: String = f.path()?.clone().to_str().unwrap().to_string();
            match p.as_str() {
                ".files" => f.unpack(LOCAL_DIR.as_path().join(&p))?,
                _ => f.unpack(Path::new(dest).join(&p))?,
            };
        }),
        Err(e) => return Err(e),
    }
}

pub fn create_archive(data: &Package, path: PathBuf) {
    let archive_name = data.archive_name();
    let pkgf = File::create(&archive_name).unwrap();
    let mut tar = tar::Builder::new(pkgf);
    tar.append_dir_all(".", path).unwrap();
    compress_zstd(&archive_name).unwrap();
    remove_file(&archive_name).unwrap();
}
