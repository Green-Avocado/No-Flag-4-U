use std::{env, fs, path::PathBuf};

/// Contains the configuration.
pub struct Config {
    /// The port that the log server listens on.
    /// Defaults to `40000`.
    pub port: u16,

    /// The host the log server is running on.
    /// Defaults to `127.0.0.1`
    pub host: String,
}

/// Default config options.
impl Default for Config {
    /// Returns the default configuration.
    fn default() -> Config {
        Config {
            port: 40000,
            host: String::from("127.0.0.1"),
        }
    }
}

/// Returns a string containing the path to the config file.
fn get_config_path() -> String {
    let mut config_path = PathBuf::new();
    config_path.push("/etc/");
    config_path.push(env::var("CARGO_PKG_NAME").unwrap());
    config_path.push(env::var("CARGO_PKG_NAME").unwrap() + ".conf");

    String::from(config_path.as_path().to_str().unwrap())
}

/// Returns the current configuration.
pub fn read_config() -> Config {
    let mut conf = Config {
        ..Default::default()
    };

    if let Ok(contents) = fs::read_to_string(get_config_path()) {
        for row in contents.lines() {
            let mut columns = row.split_whitespace();

            match columns.next() {
                Some(col) if col == "PORT" => {
                    conf.port = columns
                        .next()
                        .expect("missing PORT value")
                        .parse::<u16>()
                        .expect("failed parsing port");
                }
                Some(col) if col == "HOST" => {
                    conf.host = String::from(columns.next().expect("missing HOST string"));
                }
                Some(col) if col.starts_with('#') => (), // comment
                Some(col) => panic!("unrecognized config option: {}", col), // unrecognized option
                None => (),                              // empty line
            }
        }
    }

    conf
}
