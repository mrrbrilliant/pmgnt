use lib::{
    enums::{Architectures, BuildOptions, Licenses},
    pi_statics::{CONF_DIR, PI_DIR, ROOT},
    pkgbuild_statics::{BASEDIR, MANIFEST, PKGDATA, PKGDIR, SRCDIR},
    structs::*,
    utils::{archive::create_archive, prepare::prepare_bases},
};

#[tokio::main]
async fn main() {
    prepare_bases(vec![
        #[cfg(debug_assertions)]
        BASEDIR.to_path_buf(),
        SRCDIR.to_path_buf(),
        PKGDIR.to_path_buf(),
    ])
    .unwrap();
    let target_package = PKGDATA.as_ref().unwrap();
    target_package.pull_all().await;
    target_package.build();
    target_package.to_manifest().write().unwrap();
    target_package.create_package();
}
