use crate::config::settings::Settings;
use crate::models::connection::Connection;
use crate::utils::file_handler::{FileHandler, FileHandlerError};

use csv::Writer;
use serde::de::Error;
use std::io::Cursor;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum ConnectionManagerError {
    #[error("No file path given")]
    NoFilePathGiven,
    #[error("Malformed data")]
    MalformedData,
    #[error("Duplicate Connection")]
    DuplicateConnection,
    #[error("Connection not found")]
    ConnectionNotFoundError,
    // Include errors related to file handling
    #[error("File error: {0}")]
    FileOperationError(#[from] std::io::Error),
    // Potentially other errors, e.g., JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    JsonSerializationError(#[from] serde_json::Error),
    #[error("File handler error: {0}")]
    FileHandlerError(#[from] FileHandlerError),
}

pub trait Observer {
    fn update_connection_list(&self /* parameters */);
}

struct GuiObserver;

impl Observer for GuiObserver {
    fn update_connection_list(&self /* parameters */) {
        // implementation
    }
}

pub struct ConnectionManager {
    pub connections: Vec<Connection>,
    observers: Vec<Box<dyn Observer>>, // Observers pattern implementation might be different based on your application's architecture
    wire_label_path_name: PathBuf,
    settings: Settings,
    file_handler: FileHandler,
    output_file_name: PathBuf,
    settings_file_name: PathBuf,
}

impl ConnectionManager {
    pub fn new(
        wire_label_path_name: Option<PathBuf>,
        output_file_name: PathBuf,
        settings_file_name: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let settings = Settings::new(settings_file_name.clone())?;

        let file_handler = FileHandler::new(wire_label_path_name.clone());

        let mut manager = Self {
            connections: Vec::new(),
            observers: Vec::new(),
            wire_label_path_name: wire_label_path_name.clone().unwrap_or_default(),
            settings,
            file_handler,
            output_file_name,
            settings_file_name,
        };

        // Attempt to load connections if a path is provided
        if let Some(path) = wire_label_path_name {
            match manager.file_handler.load_wires() {
                Ok(loaded_connections) => manager.connections = loaded_connections,
                Err(e) => {
                    error!("Failed to load connections from {:?}: {}", path, e);
                    // Consider how you want to handle this error.
                    // For simplicity, we're just logging it, but you might want to return an error
                    // or take other actions
                }
            }
        }
        Ok(manager)
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer_index: usize) {
        self.observers.remove(observer_index); // Simplified for example
    }

    pub fn notify_observers(&self /* args */) {
        for observer in &self.observers {
            observer.update_connection_list(/* args */);
        }
    }

    // Method to populate connections from a list of dictionaries
    // This method assumes you have a way to construct `Connection` instances from a
    // `serde_json::Value`
    pub fn populate_connections(
        &mut self,
        conn_dicts: Vec<serde_json::Value>,
    ) -> Result<(), ConnectionManagerError> {
        // Clear existing connections to repopulate
        self.connections.clear();

        for conn_dict in conn_dicts {
            match Connection::from_json_value(conn_dict.clone()) {
                Ok(connection) => {
                    // Check for Duplicate connections before adding
                    if !self.connections.contains(&connection) {
                        self.connections.push(connection);
                    } else {
                        return Err(ConnectionManagerError::DuplicateConnection);
                    }
                }
                Err(_) => {
                    return Err(ConnectionManagerError::JsonSerializationError(
                        serde_json::Error::custom("Error deserializing connection"),
                    ));
                }
            }
        }
        self.save_json_to_file()?;
        Ok(())
    }

    pub fn delete_connection(
        &mut self,
        connection_to_delete: &Connection,
    ) -> Result<(), ConnectionManagerError> {
        if let Some(pos) = self
            .connections
            .iter()
            .position(|x| x == connection_to_delete)
        {
            self.connections.remove(pos);
            self.save_json_to_file()?;
            Ok(())
        } else {
            Err(ConnectionManagerError::MalformedData)
        }
    }

    pub fn get_connection_tuple(
        self,
        connection: &Connection,
    ) -> Result<(String, String), ConnectionManagerError> {
        if self.connections.contains(&connection) {
            Ok(connection.to_tuple())
        } else {
            Err(ConnectionManagerError::ConnectionNotFoundError)
        }
    }

    pub fn get_connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn add_connection(
        &mut self,
        src_component: String,
        src_terminal_block: String,
        src_terminal: String,
        dst_component: String,
        dst_terminal_block: String,
        dst_terminal: String,
    ) -> Result<Connection, crate::models::connection_manager::ConnectionManagerError> {
        let connection = Connection::new(
            src_component,
            src_terminal_block,
            src_terminal,
            dst_component,
            dst_terminal_block,
            dst_terminal,
        );
        if self.connections.contains(&connection) {
            return Err(ConnectionManagerError::DuplicateConnection);
        }
        self.connections.push(connection.clone());
        self.save_json_to_file()?;
        Ok(connection)
    }

    pub fn generate_csv_string(&self) -> Result<String, csv::Error> {
        let mut wtr = Writer::from_writer(Cursor::new(Vec::new()));
        for conn in &self.connections {
            wtr.serialize(conn)?;
        }
        wtr.flush()?;
        let cursor = wtr.into_inner().map_err(|e| {
            csv::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.into_error(),
            ))
        })?;
        let data = cursor.into_inner();

        // Convert Vec<u8> to String, handling Potential UTF-8 conversion errors
        String::from_utf8(data)
            .map_err(|e| csv::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    }

    pub fn save_json_to_file(&self) -> Result<(), ConnectionManagerError> {
        let data: Vec<_> = self.connections.iter().map(|c| c.to_dict()).collect();
        let json_data = serde_json::to_value(data)?;

        self.file_handler
            .save_to_path(&json_data, &self.output_file_name)?;
        info!("Saved connections to {}", self.output_file_name.display());

        Ok(())
    }

    pub fn print_connections(&self) {
        if self.connections.is_empty() {
            info!("No connections available.");
        } else {
            for (index, connection) in self.connections.iter().enumerate() {
                info!(
                    "{}: {}-{}-{} | {}-{}-{}",
                    index + 1,
                    connection.src_component,
                    connection.src_terminal_block,
                    connection.src_terminal,
                    connection.dst_component,
                    connection.dst_terminal_block,
                    connection.dst_terminal,
                );
            }
        }
    }
}
