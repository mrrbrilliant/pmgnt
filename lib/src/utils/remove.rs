use crate::pi_statics::{LOCAL_DIR, ROOT};
use crate::structs::Manifest;
use std::error::Error;
use std::fs::remove_file;
use std::path::PathBuf;

pub fn remove_one(app_name: &String) -> Result<(), Box<dyn Error>> {
    let app_path: PathBuf = LOCAL_DIR.to_owned().join(app_name).join("manifest.yml");
    // println!("{:?}", app_path.display());
    let app_data = Manifest::from_file(app_path);
    let files = app_data.files.clone();

    // println!("{}", ROOT.to_path_buf().join("/etc").display());

    files.iter().for_each(|file| {
        let f: Vec<String> = file.split(" ").into_iter().map(|a| a.to_string()).collect();
        let fi = f[0].clone();
        // println!("{}", ROOT.to_path_buf().join(fi).display());
        // println!("{}", ROOT.to_owned().join(&fi).display());
        remove_file(ROOT.join(&fi)).unwrap()
    });
    // println!("Target files: \n{:#?}", files);

    Ok(())
}
