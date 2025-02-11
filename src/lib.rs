//! Load a config file by trying out default config file locations:
//!
//! - `{NAME_SCREAMING_SNAKE_CASE}_CONFIG` envitonment variable
//! - `~/.config/{name}/config.toml`
//! - `/etc/{name}/config.toml`
//! - `/usr/local/etc/{name}/config.toml`
//! - `~/Library/Preferences/{name}/config.toml`
//! - `/usr/local/etc/{name}/config.toml`
//!
//! ```no_run
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Deserialize, Serialize)]
//! struct Config {}
//!
//! let config: Config = config_dirs::load("my-app", toml::from_str).expect("Failed to load config");
//! ```

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use genawaiter::{stack::let_gen, yield_};
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
    if let Ok(path) = env::var(format!("{}_CONFIG", name.to_case(Case::ScreamingSnake))) {
        if let Some(path) = path_with_home_dir(&path) {
            match load_from_path(path, parse) {
                Err(Error::Io(e)) if matches!(e.kind(), io::ErrorKind::NotFound) => {}
                v => return v,
            }
        }
    }
    let_gen!(paths, {
        yield_!(format!("~/.config/{name}/config.toml"));
        yield_!(format!("/etc/{name}/config.toml"));
        yield_!(format!("/usr/local/etc/{name}/config.toml"));
        yield_!(format!("~/Library/Preferences/{name}/config.toml"));
        yield_!(format!("/Library/Preferences/{name}/config.toml"));
    });
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

pub fn load_from_path<Config, E: std::error::Error>(
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
