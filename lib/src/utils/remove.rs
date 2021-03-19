use crate::statics::{PM_DIR_LOCAL, PM_DIR_ROOT};
use crate::structs::Manifest;
use std::error::Error;
use std::fs::{remove_dir_all, remove_file};
use std::path::PathBuf;

pub fn remove_one(app_name: &String) -> Result<(), Box<dyn Error>> {
    let app_dir: PathBuf = PM_DIR_LOCAL.to_owned().join(app_name);
    let app_path: PathBuf = app_dir.join("manifest.yml");
    // println!("{:?}", app_path.display());
    let app_data = Manifest::from_file(app_path).unwrap();
    let files = app_data.files.clone();

    // println!("{}", PM_DIR_ROOT.to_path_buf().join("/etc").display());

    files.iter().for_each(|file| {
        let f: Vec<String> = file.split(" ").into_iter().map(|a| a.to_string()).collect();
        let fi = f[0].clone();
        // println!("{}", PM_DIR_ROOT.to_path_buf().join(fi).display());
        // println!("{}", PM_DIR_ROOT.to_owned().join(&fi).display());
        remove_file(PM_DIR_ROOT.join(&fi)).unwrap()
    });

    remove_dir_all(app_dir).unwrap();

    // println!("Target files: \n{:#?}", files);

    Ok(())
}
