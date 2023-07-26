use std::{collections::HashMap, io};
use serde::Deserialize;
use tokio::fs;

pub enum Error {
    IoError(io::Error), // IO Error
    TomlErrorDe(toml::de::Error), // Toml deserializing error
    TomlErrorSer(toml::ser::Error), // Toml serializing error
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
} 
 
impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlErrorDe(value)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Self::TomlErrorSer(value)
    }
}

pub type AlbumMap = HashMap<String, Album>;

#[derive(Deserialize)]
pub struct Album {
    pub name: String,
    pub keywords: Option<Vec<String>>,
    pub people: Option<Vec<String>>,
}

/// Loads albums from a toml file and returns an AlbumMap
/// 
/// # Arguments
/// 
/// * 'toml_path' - The path to the toml file
/// 
/// # Examples
/// 
/// ```
/// let albums = load_albums("/librario/data.toml");
/// ```
pub async fn load_albums(toml_path: &str) -> Result<AlbumMap, Error> {
    let str = fs::read_to_string(toml_path).await?;
    Ok(toml::from_str::<AlbumMap>(&str)?)
}