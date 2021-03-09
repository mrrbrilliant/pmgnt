use lib::pi_statics::SUFFIX;
use lib::structs::{Manifest, PackageRepo};
use lib::utils::{archive::extract_archive, decompress::decompress_zstd};
use serde_yaml::{from_reader, to_writer};
use std::{
    env,
    fs::{create_dir_all, File},
    io::{Error, ErrorKind, Read},
    path::{Path, PathBuf},
    result::Result,
};
use tar::Archive;

fn main() {
    let args: Vec<String> = env::args_os()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();

    let verb = &args[1];
    let repo: Option<&String> = args.get(2);

    let packages: Option<Vec<String>> = if &args.len() >= &(3 as usize) {
        Some(args[3..].to_vec())
    } else {
        None
    };

    match verb.as_ref() {
        "a" | "add" | "-a" | "--add" => {
            if let Some(rep) = repo {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    // pkgs.iter().for_each(|p| println!("{}", p))
                    add(rep, ps)
                } else {
                    println!("No packages was given");
                }
            } else {
                eprintln!("Repo name is require")
            }
        }
        "c" | "create" | "-c" | "--create" => {
            if let Some(rep) = repo {
                create(rep).unwrap();
            } else {
                eprintln!("Repo name is require")
            }
        }
        "r" | "remove" | "-r" | "--remove" => {
            if let Some(rep) = repo {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    remove(rep, ps)
                } else {
                    println!("No packages was given");
                }
            } else {
                eprintln!("Repo name is require")
            }
        }
        _ => help(),
    }
}

fn create(path: &str) -> Result<(), Error> {
    let p = Path::new(path);
    if let Some(p) = p.parent() {
        if !p.exists() {
            create_dir_all(p).unwrap();
        }
    }

    let mut file = File::create(path)?;
    let data = PackageRepo::default();
    match to_writer(file, &data) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn update_db(path: &str, data: &PackageRepo) -> Result<(), Error> {
    let p = Path::new(path);
    if let Some(p) = p.parent() {
        if !p.exists() {
            create_dir_all(p).unwrap();
        }
    }

    let mut file = File::create(path)?;
    // let data = PackageRepo::default();
    match to_writer(file, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn opendb(path: &str) -> Result<PackageRepo, Error> {
    let db_file = File::open(path)?;
    match from_reader(db_file) {
        Ok(db) => Ok(db),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn add(db_path: &str, pkg_files: Vec<PathBuf>) {
    let mut db: PackageRepo = opendb(db_path).unwrap();
    for pkg_file in pkg_files.iter() {
        let pkg_file_name = pkg_file.to_str().unwrap();

        let file_name: String = pkg_file_name
            .to_string()
            .trim_end_matches(SUFFIX.as_str())
            .to_string();

        decompress_zstd(pkg_file.to_str().unwrap()).unwrap();

        let mut arch = Archive::new(File::open(file_name).unwrap());
        let mut manif = arch.entries().unwrap().skip_while(|entry| {
            entry.as_ref().unwrap().path().unwrap().to_str().unwrap() != "manifest.yml"
        });
        let mut buf: String = String::new();
        manif
            .nth(0)
            .unwrap()
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let data: Manifest = serde_yaml::from_str(&buf).unwrap();
        let res = db.packages.iter().find(|&m| m.pkgname == data.pkgname);

        if let Some(_) = res {
            &db.packages.retain(|m| m.pkgname != data.pkgname);
            &db.packages.push(data.clone());
        } else {
            &db.packages.push(data.clone());
        }
    }
    update_db(db_path, &db);
}

fn remove(db_path: &str, pkg_files: Vec<PathBuf>) {
    let mut db: PackageRepo = opendb(db_path).unwrap();

    if !pkg_files.is_empty() {
        for pkg in pkg_files.iter() {
            &db.packages
                .retain(|m| m.pkgname != pkg.to_str().unwrap().to_string());
        }
    }

    update_db(db_path, &db);
}

fn help() {
    print!(
        r#"
USAGE:
repo <operation> <repo_name> [packages]

Operations:
    create <repo_name>              generation an empty repo with the given name.
    add <repo_name> [packages]      add the packages to that repo.
    remove <repo_name> [package]    remove the packages to that repo.
    truncate <repo_name>            remove all packages from the repo.
"#
    );
}
