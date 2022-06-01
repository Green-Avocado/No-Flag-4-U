use std::{env, path::PathBuf};

fn main() {
    let mut lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    lib_path.push("target");
    lib_path.push(env::var("PROFILE").unwrap());
    lib_path.push("deps");
    lib_path
        .push("lib".to_owned() + &env::var("CARGO_PKG_NAME").unwrap().replace('-', "_") + ".so");

    println!(
        "cargo:rustc-env=INLINE_C_RS_LD_PRELOAD={path}",
        path = lib_path.as_path().to_string_lossy()
    );
}
