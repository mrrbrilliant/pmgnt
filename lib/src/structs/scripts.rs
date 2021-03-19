use crate::statics::*;
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
        let basedir = PB_DIR_CWD.to_str().unwrap();
        let srcdir = PB_DIR_SRC.to_str().unwrap();
        let pkgdir = PB_DIR_PKG.to_str().unwrap();
        let pkgname = &DATA_PACKAGE.as_ref().unwrap().pkgname;
        let pkgver = &DATA_PACKAGE.as_ref().unwrap().pkgver;
        let pkgrel = DATA_PACKAGE.as_ref().unwrap().pkgrel;

        // execute commands
        run(cmd, basedir, srcdir, pkgdir, pkgname, pkgver, pkgrel)
            .map(|output| println!("{}", output))
    }
}
