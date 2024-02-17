use crate::command::command::{Command, CommandError};

pub struct CommandManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandManager {
    pub fn execute_command(&mut self, mut command: Box<dyn Command>) {
        if command.execute().is_ok() {
            self.undo_stack.push(command);
            self.redo_stack.clear(); // Clear redo stack on new command execution
        }
    }

    pub fn undo(&mut self) -> Result<(), CommandError> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo()?;
            self.redo_stack.push(command);
            Ok(())
        } else {
            Err(CommandError::Other("No command to undo".into()))
        }
    }

    pub fn redo(&mut self) -> Result<(), CommandError> {
        if let Some(mut command) = self.redo_stack.pop() {
            command.redo()?;
            self.undo_stack.push(command);
            Ok(())
        } else {
            Err(CommandError::Other("No command to redo".into()))
        }
    }
}
