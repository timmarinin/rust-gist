use std;
use super::errors::*;
use std::fs::{self, DirBuilder, File};
use std::os::unix::fs::DirBuilderExt;
use std::io::prelude::*;

pub type Token = String;

pub fn drop_token() -> Result<()> {
    ensure_config_dir()?;
    fs::remove_file(get_config_filename()).chain_err(|| ErrorKind::SaveToken)
}

pub fn get_anonymous_token() -> Result<Token> {
    Ok(Token::new())
}

pub fn read_token() -> Result<Token> {
    ensure_config_dir()?;
    let mut f = File::open(get_config_filename()).chain_err(|| ErrorKind::LoadToken)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents).chain_err(|| ErrorKind::LoadToken)?;
    Ok(format!("token {}", contents.trim()))
}

pub fn write_token(token: String) -> Result<()> {
    ensure_config_dir()?;
    let mut file = File::create(get_config_filename()).chain_err(|| ErrorKind::SaveToken)?;
    file.write(token.as_bytes())
        .map(|_| ())
        .chain_err(|| ErrorKind::SaveToken)
}

fn get_config_filename() -> String {
    get_config_dir_name().to_string() + "/token.txt"
}

fn get_config_dir_name() -> String {
    env!("HOME").to_string() + "/.config/rust-gist"
}

fn ensure_config_dir() -> Result<()> {
    let exists = fs::metadata(get_config_dir_name());

    match exists {
        Ok(meta) => {
            if meta.is_dir() {
                Ok(())
            } else {
                println!("Directory exists, but is not a directory. WTF? {:?}", meta);
                create_config_dir()
            }
        },
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => create_config_dir(),
                _ => bail!(ErrorKind::SaveToken)
            }
        }
    }
}

fn create_config_dir() -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(get_config_dir_name())
        .chain_err(|| ErrorKind::SaveToken)
}
