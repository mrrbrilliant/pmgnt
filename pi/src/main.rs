#![allow(unused_imports, dead_code)]

use lib::statics::{PM_DIR_CONF, PM_DIR_LIB, PM_DIR_LOCAL, PM_DIR_ROOT, PM_DIR_SYNC, PM_FILE_CONF};
use lib::structs::PiConf;
use lib::utils::{
    archive::extract_archive, decompress::decompress_zstd, install::is_installed,
    prepare::prepare_bases, remove::remove_one,
};
use solvent::DepGraph;
use std::{env, fs::File};
use text_diff::{diff, Difference};
use walkdir::WalkDir;
fn main() {
    prepare_bases(vec![
        #[cfg(debug_assertions)]
        PM_DIR_ROOT.to_path_buf(),
        PM_DIR_LIB.to_path_buf(),
        PM_DIR_LOCAL.to_path_buf(),
        PM_DIR_SYNC.to_path_buf(),
        PM_DIR_CONF.to_path_buf(),
    ])
    .unwrap();

    if !PM_FILE_CONF.as_path().exists() {
        let mut file = File::create(PM_FILE_CONF.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &PiConf::gen()).unwrap()
    }

    let args: Vec<String> = env::args().collect();
    let arg_file = &args[1..];
    let dest = PM_DIR_ROOT.to_path_buf();

    decompress_zstd(arg_file).unwrap();
    extract_archive(arg_file, &dest.to_str().unwrap()).unwrap();
    list_installed();

    // remove_one(&String::from("calamares")).unwrap();
    // println!("{}", is_installed("calamares"))
}

// fn list_installed() {
//     for entry in WalkDir::new(PM_DIR_LOCAL.as_path())
//         .min_depth(1)
//         .max_depth(1)
//         .sort_by(|a, b| a.file_name().cmp(b.file_name()))
//     {
//         println!(
//             "{}",
//             entry
//                 .unwrap()
//                 .path()
//                 .display()
//                 .to_string()
//                 .trim_start_matches(PM_DIR_LOCAL.to_str().unwrap())
//                 .trim_start_matches("/")
//         )
//     }
// }

// fn diff_removed() {
//     let left: Vec<String> = vec![
//         String::from("a"),
//         String::from("b"),
//         String::from("c"),
//         String::from("d"),
//         String::from("e"),
//     ];
//     let mut right: Vec<String> = Vec::with_capacity(left.len());

//     right.push(String::from("a"));
//     right.push(String::from("c"));
//     right.push(String::from("d"));

//     let mut removed: Vec<String> = Vec::new();

//     let rs = right.join("\n");
//     let ls = left.join("\n");

//     let res = diff(&ls, &rs, "\n");

//     for i in res.1.iter() {
//         match i {
//             Difference::Rem(s) => removed.push(s.clone()),
//             _ => {}
//         }
//     }
//     println!("{:?}", removed);
// }
