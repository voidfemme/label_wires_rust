use crate::models::connection::Connection;
use crate::models::connection_manager::{ConnectionManager, ConnectionManagerError};

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// AppState (refactor to another file?)
#[derive(Default)]
struct AppState {
    connections: Vec<Connection>,
    selected_connection_uuids: Vec<Uuid>,
}

#[derive(Debug)]
pub enum CommandError {
    DuplicateConnection,
    ConnectionNotDeleted,
    ConnectionNotFoundError,
    Other(String), // General error category for simplification
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CommandError::DuplicateConnection => write!(f, "Duplicate connection detected."),
            CommandError::ConnectionNotDeleted => {
                write!(f, "Connection was not successfully removed.")
            }
            CommandError::ConnectionNotFoundError => {
                write!(f, "Connection not found")
            }
            CommandError::Other(ref cause) => write!(f, "{}", cause),
        }
    }
}

impl std::error::Error for CommandError {}

pub trait Command {
    fn execute(&mut self) -> Result<(), CommandError>;
    fn undo(&mut self) -> Result<(), CommandError>;
    fn redo(&mut self) -> Result<(), CommandError>;
}

pub struct AddConnectionCommand {
    connection_manager: ConnectionManager,
    source: HashMap<String, String>,
    destination: HashMap<String, String>,
    connection: Option<Connection>,
}

impl AddConnectionCommand {
    pub fn new(
        connection_manager: ConnectionManager,
        source: HashMap<String, String>,
        destination: HashMap<String, String>,
    ) -> Self {
        AddConnectionCommand {
            connection_manager,
            source,
            destination,
            connection: None,
        }
    }
}

impl Command for AddConnectionCommand {
    fn execute(&mut self) -> Result<(), CommandError> {
        let src_component = self.source.get("component").unwrap();
        let src_terminal_block = self.source.get("terminal_block").unwrap();
        let src_terminal = self.source.get("terminal").unwrap();
        let dst_component = self.destination.get("component").unwrap();
        let dst_terminal_block = self.destination.get("terminal_block").unwrap();
        let dst_terminal = self.destination.get("terminal").unwrap();

        match self.connection_manager.add_connection(
            src_component.to_string(),
            src_terminal_block.to_string(),
            src_terminal.to_string(),
            dst_component.to_string(),
            dst_terminal_block.to_string(),
            dst_terminal.to_string(),
        ) {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => match e {
                ConnectionManagerError::DuplicateConnection => {
                    Err(CommandError::DuplicateConnection)
                }
                _ => Err(CommandError::Other(e.to_string())),
            },
        }
    }

    fn undo(&mut self) -> Result<(), CommandError> {
        if let Some(ref connection) = self.connection {
            match self.connection_manager.delete_connection(connection) {
                Ok(_) => {
                    // Notify the event system about the connection removed
                    Ok(())
                }
                Err(e) => Err(CommandError::Other(format!(
                    "Failed to delete connection: {}",
                    e
                ))),
            }
        } else {
            Err(CommandError::Other("No connection to undo".to_string()))
        }
    }

    fn redo(&mut self) -> Result<(), CommandError> {
        self.execute()
    }
}

struct DeleteConnectionCommand {
    connection_uuids: Vec<Uuid>,
    deleted_connections: Vec<Connection>,
    connection_manager: Arc<Mutex<ConnectionManager>>,
}

impl DeleteConnectionCommand {
    pub fn new(
        connection_uuids: Vec<Uuid>,
        connection_manager: Arc<Mutex<ConnectionManager>>,
    ) -> Self {
        Self {
            connection_uuids,
            deleted_connections: Vec::new(),
            connection_manager,
        }
    }
}

impl Command for DeleteConnectionCommand {
    fn execute(&mut self) -> Result<(), CommandError> {
        let mut mgr = self.connection_manager.lock().unwrap();
        for uuid in &self.connection_uuids {
            if let Some(conn) = mgr.connections.iter().find(|c| c.uuid == *uuid).cloned() {
                mgr.delete_connection(&conn);
                self.deleted_connections.push(conn);
            }
        }
        Ok(())
    }

    fn undo(&mut self) -> Result<(), CommandError> {
        let mut mgr = self.connection_manager.lock().unwrap();
        for conn in &self.deleted_connections {
            let _ = mgr.add_connection(
                conn.src_component.clone(),
                conn.src_terminal_block.clone(),
                conn.src_terminal.clone(),
                conn.dst_component.clone(),
                conn.dst_terminal_block.clone(),
                conn.dst_terminal.clone(),
            );
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<(), CommandError> {
        self.execute()
    }
}

struct EditConnectionCommand {
    connection_manager: Arc<Mutex<ConnectionManager>>,
    old_connection_uuid: Uuid,
    new_values: HashMap<String, String>,
    old_connection: Option<Connection>,
    new_connection_uuid: Option<Uuid>,
}

impl EditConnectionCommand {
    pub fn new(
        connection_manager: Arc<Mutex<ConnectionManager>>,
        old_connection_uuid: Uuid,
        new_values: HashMap<String, String>,
    ) -> Self {
        Self {
            connection_manager,
            old_connection_uuid,
            new_values,
            old_connection: None,
            new_connection_uuid: None,
        }
    }
}

impl Command for EditConnectionCommand {
    fn execute(&mut self) -> Result<(), CommandError> {
        let mut cm = self.connection_manager.lock().unwrap();

        // Find and store the old connection for undo
        if let Some(index) = cm
            .connections
            .iter()
            .position(|conn| conn.uuid == self.old_connection_uuid)
        {
            self.old_connection = Some(cm.connections[index].clone());

            // Remove the old connection
            cm.connections.remove(index);

            // Create and add the new connection
            let new_connection = Connection::new(
                self.new_values
                    .get("src_component")
                    .cloned()
                    .unwrap_or_default(),
                self.new_values
                    .get("src_terminal_block")
                    .cloned()
                    .unwrap_or_default(),
                self.new_values
                    .get("src_terminal")
                    .cloned()
                    .unwrap_or_default(),
                self.new_values
                    .get("dst_component")
                    .cloned()
                    .unwrap_or_default(),
                self.new_values
                    .get("dst_terminal_block")
                    .cloned()
                    .unwrap_or_default(),
                self.new_values
                    .get("dst_terminal")
                    .cloned()
                    .unwrap_or_default(),
            );

            // Track the UUID of the newly created connection
            self.new_connection_uuid = Some(new_connection.uuid);

            // Add the new connection
            cm.connections.push(new_connection);
        } else {
            return Err(CommandError::ConnectionNotFoundError);
        }

        Ok(())
    }

    fn undo(&mut self) -> Result<(), CommandError> {
        if let Some(new_connection_uuid) = self.new_connection_uuid {
            let mut cm = self.connection_manager.lock().unwrap();

            // Find and remove the newly added connection using its UUID
            if let Some(index) = cm
                .connections
                .iter()
                .position(|conn| conn.uuid == new_connection_uuid)
            {
                cm.connections.remove(index);
            }

            // Re-add the original (old) connection if it exists
            if let Some(old_connection) = &self.old_connection {
                cm.connections.push(old_connection.clone());
            }
        }
        Ok(())
    }

    fn redo(&mut self) -> Result<(), CommandError> {
        self.execute()
    }
}
