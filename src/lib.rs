use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum Error<ParseError: std::error::Error> {
    #[error("Failed to read config file: {0:#}")]
    Io(#[from] io::Error),
    #[error("Failed to parse config: {0:#}")]
    Parse(ParseError),
    #[error("Failed to load config from paths")]
    NoPath,
}

pub fn load<Config, E: std::error::Error>(
    name: &str,
    parse: impl Fn(&str) -> Result<Config, E> + Copy,
) -> Result<Config, Error<E>> {
    if let Ok(path) = env::var(format!("{}_CONFIG", name.to_uppercase())) {
        if let Some(path) = path_with_home_dir(&path) {
            match load_from_path(path, parse) {
                Err(Error::Io(e)) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
                v => return v,
            }
        }
    }
    let paths = [
        format!("~/.config/{name}/config.toml"),
        format!("/etc/{name}/config.toml"),
        format!("/usr/local/etc/{name}/config.toml"),
        format!("~/Library/Preferences/{name}/config.toml"),
        format!("/Library/Preferences/{name}/config.toml"),
    ];
    for path in paths {
        if let Some(path) = path_with_home_dir(&path) {
            match load_from_path(path, parse) {
                Err(Error::Io(e)) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
                v => return v,
            }
        }
    }
    error!("Failed to load config");
    Err(Error::NoPath)
}

fn load_from_path<Config, E: std::error::Error>(
    path: impl AsRef<Path>,
    parse: impl Fn(&str) -> Result<Config, E>,
) -> Result<Config, Error<E>> {
    info!("Loading config from {}", path.as_ref().to_string_lossy());
    parse(&fs::read_to_string(path)?).map_err(Error::Parse)
}

fn path_with_home_dir(path: &str) -> Option<PathBuf> {
    if path.starts_with("~/") {
        dirs::home_dir().map(|v| v.join(&path[2..]))
    } else {
        Some(PathBuf::from(path))
    }
}
