use lib::pi_statics::{CONF_DIR, CONF_FILE, LOCAL_DIR, PI_DIR, ROOT, SYNC_DIR};
use lib::structs::PiConf;
use lib::utils::{archive::extract_archive, decompress::decompress_zstd, prepare::prepare_bases};
use std::{env, fs::File};

fn main() {
    prepare_bases(vec![
        #[cfg(debug_assertions)]
        ROOT.to_path_buf(),
        PI_DIR.to_path_buf(),
        LOCAL_DIR.to_path_buf(),
        SYNC_DIR.to_path_buf(),
        CONF_DIR.to_path_buf(),
    ])
    .unwrap();

    if !CONF_FILE.as_path().exists() {
        let mut file = File::create(CONF_FILE.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &PiConf::gen()).unwrap()
    }

    let args: Vec<String> = env::args().collect();
    let arg_file = &args[1];
    let dest = ROOT.to_path_buf();
    decompress_zstd(arg_file).unwrap();
    extract_archive(arg_file, &dest.to_str().unwrap()).unwrap();
}
