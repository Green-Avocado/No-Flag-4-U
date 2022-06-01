use std::{env, path::PathBuf};

fn main() {
    let mut test_lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    test_lib_path.push("target");
    test_lib_path.push(env::var("PROFILE").unwrap());
    test_lib_path.push("deps");
    test_lib_path
        .push("lib".to_owned() + &env::var("CARGO_PKG_NAME").unwrap().replace('-', "_") + ".so");

    println!(
        "cargo:rustc-env=INLINE_C_RS_LD_PRELOAD={path}",
        path = test_lib_path.as_path().to_string_lossy()
    );
}
