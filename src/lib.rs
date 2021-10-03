pub mod cipher;
mod utils;

use clap::{App, Arg};

/// Config struct that holds encryption information
#[derive(Debug)]
pub struct Config {
    pub mode: String,
    pub file_path: String,
    pub secret: String,
    pub output_dir: String,
}

/// Initialize CLI
pub fn init_cli() -> Config {
    let matches = App::new("EasyCipher")
        .version("0.1.0")
        .about("An easy-to-use CLI-based cipher suite.")
        .arg(Arg::new("MODE")
            .about("Sets a mode (e: encryption | d: decryption)")
            .required(true)
            .index(1))
        .arg(Arg::new("FILE")
            .about("Sets an input file path")
            .required(true)
            .index(2))
        .arg(Arg::new("secret")
            .short('s')
            .long("secret")
            .value_name("SECRET")
            .about("Sets a secret")
            .takes_value(true))
        .arg(Arg::new("output_dir")
            .short('o')
            .long("output")
            .value_name("OUTPUT DIR")
            .about("Sets an output directory")
            .takes_value(true))
        .get_matches();

    let mut config = Config {
        mode: "".to_string(),
        file_path: "".to_string(),
        secret: "".to_string(),
        output_dir: "./".to_string(),
    };

    if let Some(mode) = matches.value_of("MODE") {
        config.mode = String::from(mode);
    }

    if let Some(file_path) = matches.value_of("FILE") {
        config.file_path = String::from(file_path);
    }

    if let Some(secret) = matches.value_of("secret") {
        config.secret = String::from(secret);
    }

    if let Some(output_dir) = matches.value_of("output_dir") {
        config.output_dir = String::from(output_dir);
    }

    config
}