use std::{collections::HashMap, io, path::Path, fmt::Display};
use serde::{Deserialize, Serialize};
use tokio::fs;
use toml::{Table, Value};
use uuid::Uuid;

#[derive(Debug)]
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO Error: {}", e),
            Self::TomlErrorDe(e) => write!(f, "Toml deserializing error: {}", e),
            Self::TomlErrorSer(e) => write!(f, "Toml serializing error: {}", e),
        }
    }
}

impl std::error::Error for Error {}


pub type AlbumMap = HashMap<String, Album>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Album {
    pub name: String,
    pub keywords: Option<Vec<String>>,
    pub people: Option<Vec<String>>,
    pub path: Option<String>,
}

impl Album {
    pub fn new(name: &str) -> Album {
        Album { name: name.to_string(), keywords: None, people: None, path: None }
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
pub async fn load_albums(toml_path: &Path) -> Result<AlbumMap, Error> {
    
    let str = fs::read_to_string(toml_path).await?;
    
    let mut album_map = toml::from_str::<AlbumMap>(&str)?;
    album_map.retain(|k, _| Uuid::parse_str(k).is_ok() );

    Ok(album_map)
}

pub async fn save_albums(album_map: &AlbumMap, toml_path: &Path) -> Result<(), Error> {
    let str = toml::to_string(album_map)?;
    fs::write(toml_path, str).await?;

    Ok(())
}