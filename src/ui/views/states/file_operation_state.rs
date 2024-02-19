use iced::widget::{Column, Container, TextInput};
use iced::{Command, Element};
use iced_core::Length;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileOperationMessage {
    Open,
    Close,
    OpenFile,
    SaveFile(PathBuf),
    NewFile,
    FileSelected(PathBuf),
    CancelOperation,
    FileNameChanged(String),
    DirectoryChanged(String),
}

pub struct FileOperationState {
    file_name: String,
    directory: String,
    is_visible: bool,
}

impl FileOperationState {
    pub fn new() -> Self {
        Self {
            file_name: String::new(),
            directory: String::new(),
            is_visible: false,
        }
    }
    pub fn update(&mut self, message: FileOperationMessage) -> Command<FileOperationMessage> {
        match message {
            FileOperationMessage::Open => {
                self.is_visible = true;
            }
            FileOperationMessage::Close => {
                self.is_visible = false;
            }
            FileOperationMessage::OpenFile => {}
            FileOperationMessage::SaveFile(_file_path) => {}
            FileOperationMessage::NewFile => {}
            FileOperationMessage::FileSelected(_file_path) => {}
            FileOperationMessage::CancelOperation => {}
            FileOperationMessage::FileNameChanged(_) => {}
            FileOperationMessage::DirectoryChanged(_) => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, FileOperationMessage> {
        let file_name_input = TextInput::new("File Name", &self.file_name)
            .on_input(|value| FileOperationMessage::FileNameChanged(value))
            .padding(10)
            .size(16);

        let directory_input = TextInput::new("Directory", &self.directory)
            .on_input(|value| FileOperationMessage::DirectoryChanged(value))
            .padding(10)
            .size(16);

        // Layout UI elements here
        let content = Column::new().push(file_name_input).push(directory_input);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
