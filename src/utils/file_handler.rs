use serde_json;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, error, info};

use crate::models::connection::Connection;

// Define a custom error type that can represent errors from different sources
#[derive(Debug, Error)]
pub enum FileHandlerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Path not set")]
    PathNotSet,
}

pub struct FileHandler {
    file_path: Option<PathBuf>,
}

impl FileHandler {
    pub fn new(file_path: Option<PathBuf>) -> Self {
        Self { file_path }
    }

    pub fn load_wires(&self) -> Result<Vec<Connection>, FileHandlerError> {
        let file_path = self
            .file_path
            .as_ref()
            .ok_or(FileHandlerError::PathNotSet)?;

        let mut file = File::open(file_path).map_err(FileHandlerError::from)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(FileHandlerError::from)?;

        // Log the contents of the file to verify what's being read
        debug!("Raw JSON contents: {}", contents);

        info!(contents);

        let connections: Vec<Connection> = serde_json::from_str(&contents).map_err(|e| {
            error!(
                "Error deserializing JSON: {}. Contents were: {}",
                e, contents
            );
            FileHandlerError::from(e)
        })?;

        Ok(connections)
    }

    pub fn save(&self, data: &serde_json::Value) -> Result<(), FileHandlerError> {
        let file_path = self
            .file_path
            .as_ref()
            .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "File path not set"))?;
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }

    pub fn save_to_path(
        &self,
        data: &serde_json::Value,
        file_path: &PathBuf,
    ) -> Result<(), FileHandlerError> {
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }
}
