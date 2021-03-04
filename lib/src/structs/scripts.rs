use crate::pkgbuild_statics::*;
use crate::utils::run_script::run;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Script {
    pub commands: Vec<String>,
}

impl Script {
    pub fn exec(&self) -> Result<(), Box<dyn Error>> {
        // Commands to  execute
        let mut commands = self.commands.clone();
        commands.push(String::from("exit"));
        let cmd = &self.commands.join("\n").to_string();

        // Envronment varialbes
        let basedir = BASEDIR.to_str().unwrap();
        let srcdir = SRCDIR.to_str().unwrap();
        let pkgdir = PKGDIR.to_str().unwrap();
        let pkgname = &PKGDATA.as_ref().unwrap().pkgname;
        let pkgver = &PKGDATA.as_ref().unwrap().pkgver;
        let pkgrel = PKGDATA.as_ref().unwrap().pkgrel;

        // execute commands
        run(cmd, basedir, srcdir, pkgdir, pkgname, pkgver, pkgrel)
            .map(|output| println!("{}", output))
    }
}
