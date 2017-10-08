use std;
use std::fs::{self, DirBuilder, File};
use std::os::unix::fs::DirBuilderExt;
use std::io::prelude::*;

pub type Token = String;

pub fn drop_token() -> Result<(), std::io::Error> {
    ensure_config_dir();
    fs::remove_file(get_config_filename())
}

pub fn get_anonymous_token() -> Result<Token, std::io::Error> {
    Ok(Token::new())
}

pub fn read_token() -> Result<Token, std::io::Error> {
    ensure_config_dir();
    let file = File::open(get_config_filename());

    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            Ok(format!("token {}", contents.trim()))
        }
        Err(e) => Err(e),
    }
}

pub fn write_token(token: String) -> Result<(), std::io::Error> {
    ensure_config_dir();
    let mut file = File::create(get_config_filename()).unwrap();
    match file.write(token.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn get_config_filename() -> String {
    get_config_dir_name().to_string() + "/token.txt"
}

fn get_config_dir_name() -> String {
    env!("HOME").to_string() + "/.config/rust-gist"
}

fn ensure_config_dir() {
    let exists = fs::metadata(get_config_dir_name());

    match exists {
        Ok(meta) => if !meta.is_dir() {
            println!("Directory exists, but is not directory. WTF? {:?}", meta);
            create_config_dir()
        },
        Err(_) => create_config_dir(),
    }
}

fn create_config_dir() {
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(get_config_dir_name())
        .unwrap();
}
