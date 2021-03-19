use lib::{
    enums::{Architectures, BuildOptions, Licenses},
    statics::{DATA_PACKAGE, PB_DIR_CWD, PB_DIR_PKG, PB_DIR_SRC, PM_FILE_MANI},
    statics::{PM_DIR_CONF, PM_DIR_LIB, PM_DIR_ROOT},
    structs::*,
    utils::{archive::create_archive, prepare::prepare_bases},
};

#[tokio::main]
async fn main() {
    prepare_bases(vec![
        #[cfg(debug_assertions)]
        PB_DIR_CWD.to_path_buf(),
        PB_DIR_SRC.to_path_buf(),
        PB_DIR_PKG.to_path_buf(),
    ])
    .unwrap();

    let target_package = DATA_PACKAGE.as_ref().unwrap();
    target_package.check_makedepends();
    // target_package.pull_all().await;
    // target_package.build();
    // target_package.to_manifest().write().unwrap();
    // target_package.create_package();
}
