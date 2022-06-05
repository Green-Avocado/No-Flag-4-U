use std::env;

fn get_config_path() -> String {
    env::var("CARGO_PKG_NAME").unwrap()
}

pub fn read_config() {
    println!("PLACEHOLDER: read config");
}
