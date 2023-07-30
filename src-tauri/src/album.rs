use std::{collections::HashMap, io};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Clone)]
pub struct Album {
    name: String,
    keywords: Option<Vec<String>>,
    people: Option<Vec<String>>,
}

impl Album {
    pub fn new(name: &str) -> Album {
        Album { name: name.to_string(), keywords: None, people: None }
    }

    pub fn with_keywords(mut self, keywords: &[String]) -> Album {
        self.keywords = Some(keywords.to_vec());
        self
    }

    pub fn with_people(mut self, keywords: &[String]) -> Album {
        self.people = Some(keywords.to_vec());
        self
    }
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