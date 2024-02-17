use iced::application::Application;
use iced::theme::Button as ThemeButton;
use iced::widget::{
    button, scrollable, text_input, Button, Checkbox, Column, Container, Row, Scrollable, Text,
    TextInput,
};
use iced::{Command, Element, Length, Renderer};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::warn;
use uuid::Uuid;

use crate::models::connection::Connection;
use crate::models::connection_manager::ConnectionManager;
use crate::ui::theme::ConnectionStyle;
use crate::utils::localizer::Localizer;

#[derive(Clone)]
pub struct Flags {
    pub initial_file: Option<PathBuf>,
    pub settings_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}

pub struct MainApplication {
    // Text input states for source
    src_component_input_state: text_input::State,
    src_terminal_block_input_state: text_input::State,
    src_terminal_input_state: text_input::State,

    // Text input states for destination
    dst_component_input_state: text_input::State,
    dst_terminal_block_input_state: text_input::State,
    dst_terminal_input_state: text_input::State,

    // Current values for text input widgets for source
    src_component_value: String,
    src_terminal_block_value: String,
    src_terminal_value: String,

    // Current values for text input widgets for destination
    dst_component_value: String,
    dst_terminal_block_value: String,
    dst_terminal_value: String,

    // State for buttons
    edit_button_state: button::State,
    delete_button_state: button::State,
    settings_button_state: button::State,
    export_wires_button_state: button::State,
    export_cables_button_state: button::State,
    quit_button_state: button::State,

    // Checkbox states and values
    lock_destination_checked: bool,
    increment_field1_checked: bool,
    increment_field2_checked: bool,

    // State for the list view
    connections_scroll: scrollable::State,
    connections_buttons: Vec<button::State>,
    connections: Vec<Connection>,
    selected_connections: HashMap<Uuid, bool>,

    // Commands and connection manager
    command_stack: Vec<Box<dyn crate::command::command::Command>>,
    undo_stack: Vec<Box<dyn crate::command::command::Command>>,
    connection_manager: ConnectionManager,
    localizer: Localizer,
}

impl MainApplication {
    // TODO: Remove this dummy code
    fn generate_dummy_connections() -> Vec<Connection> {
        warn!("Remove this function");
        (1..51)
            .map(|i| {
                Connection::new(
                    format!("SrcComp{}", i),
                    format!("SrcBlock{}", i),
                    format!("SrcTerm{}", i),
                    format!("DstComp{}", i),
                    format!("DstBlock{}", i),
                    format!("DstTerm{}", i),
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    AddConnectionPressed,
    SourceChanged(String),
    DestinationChanged(String),
    ConnectionSelected(usize), // Index of the selected connection
    SaveFilePressed,
    LockDestinationChanged(bool),
    IncrementChanged(bool),
    ToggleConnectionSelected(Uuid),
    DeleteSelectedConnections,
    EditConnection(Uuid),
    OpenSettingsWindow,
    ExportWiresPressed,
    ExportCablesPressed,
    QuitPressed,
    IncrementField1Changed(bool),
    IncrementField2Changed(bool),
    EditPressed,
    DeletePressed,
}

impl Application for MainApplication {
    type Executor = iced::executor::Default;
    // The flags required to initialize the application
    type Flags = Flags;
    // Define the type of message that will be used for communication within the app
    type Message = MainMessage;
    // The default theme for the application's appearance
    type Theme = iced::Theme;

    // Implement the new function to initialize the application state
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Determine file paths from flags, providing defaults as necessary
        let wire_label_path = flags
            .initial_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));
        let output_file_path = flags
            .output_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));
        let settings_file_path = flags
            .settings_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));

        // Initialize the ConnectionManager
        let connection_manager =
            ConnectionManager::new(Some(wire_label_path), output_file_path, settings_file_path)
                .expect("Failed to initialize ConnectionManager");

        // Initialize the localizer
        let localizer = Localizer::new("en");

        // Return the initial application state and any initial commands
        (
            Self {
                // Text input states for source
                src_component_input_state: text_input::State::new(),
                src_terminal_block_input_state: text_input::State::new(),
                src_terminal_input_state: text_input::State::new(),

                // Text input states for destination
                dst_component_input_state: text_input::State::new(),
                dst_terminal_block_input_state: text_input::State::new(),
                dst_terminal_input_state: text_input::State::new(),

                // Current values for text input widgets for source
                src_component_value: String::new(),
                src_terminal_block_value: String::new(),
                src_terminal_value: String::new(),

                // Current values for text input widgets for destination
                dst_component_value: String::new(),
                dst_terminal_block_value: String::new(),
                dst_terminal_value: String::new(),

                // State for buttons
                edit_button_state: button::State::new(),
                delete_button_state: button::State::new(),
                settings_button_state: button::State::new(),
                export_wires_button_state: button::State::new(),
                export_cables_button_state: button::State::new(),
                quit_button_state: button::State::new(),

                // Checkbox states and values
                lock_destination_checked: false,
                increment_field1_checked: false,
                increment_field2_checked: false,

                // State for the list view
                connections_scroll: scrollable::State::new(),
                connections_buttons: Vec::new(),
                connections: Self::generate_dummy_connections(),
                selected_connections: HashMap::new(),

                // Commands and connection manager
                command_stack: Vec::new(),
                undo_stack: Vec::new(),
                connection_manager,
                localizer,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let title = self.localizer.get("application_title");
        String::from(title)
    }

    // Define how to handle each type of message
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            MainMessage::ConnectionSelected(_) => {
                // Handle what to do when selecting a connection
            }
            MainMessage::AddConnectionPressed => {
                // Handle the Add Connection button being pressed
            }
            MainMessage::SourceChanged(source) => {
                // Handle the source text input changing
            }
            MainMessage::DestinationChanged(destination) => {
                // Handle the destination text input changing
            }
            MainMessage::SaveFilePressed => {
                // Handle the save file button press
            }
            MainMessage::IncrementChanged(_) => {
                // handle the increment checkbox
            }
            MainMessage::LockDestinationChanged(new_value) => {
                // handle the lock destination checkbox
                self.lock_destination_checked = new_value;
            }
            MainMessage::ToggleConnectionSelected(uuid) => {
                if let Some(is_selected) = self.selected_connections.get_mut(&uuid) {
                    *is_selected = !*is_selected;
                } else {
                    self.selected_connections.insert(uuid, true);
                }
            }
            MainMessage::DeleteSelectedConnections => {
                // Here you would create a DeleteConnectionCommand and execute it. This would
                // involve removing connections from `self.connections` based on the UUIDs in
                // `self.selected_connections`
            }
            MainMessage::EditConnection(uuid) => {
                // Here you would create an EditConnectionCommand and execute it. You need to
                // determine how you will handle editing within your UI
            }
            MainMessage::QuitPressed => {}
            MainMessage::EditPressed => {}
            MainMessage::DeletePressed => {}
            MainMessage::ExportWiresPressed => {}
            MainMessage::ExportCablesPressed => {}
            MainMessage::IncrementField1Changed(new_value) => {
                self.increment_field1_checked = new_value;
            }
            MainMessage::IncrementField2Changed(new_value) => {
                self.increment_field2_checked = new_value;
            }
            _ => {}
        }
        Command::none()
    }

    // Define the layout of the application
    fn view(&self) -> Element<'_, Self::Message> {
        // Source input field with label
        let component_string = self.localizer.get("source");
        let terminal_block_string = self.localizer.get("terminal_block");
        let terminal_string = self.localizer.get("terminal");
        let src_component_input: TextInput<MainMessage> =
            TextInput::new(&component_string, &self.src_component_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);
        let src_terminal_block_input: TextInput<MainMessage> =
            TextInput::new(&terminal_block_string, &self.src_terminal_block_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);
        let src_terminal_input: TextInput<MainMessage> =
            TextInput::new(&terminal_string, &self.src_terminal_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);

        // Destination input field with label
        let dst_component_input: TextInput<MainMessage> =
            TextInput::new(&component_string, &self.dst_component_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);
        let dst_terminal_block_input: TextInput<MainMessage> =
            TextInput::new(&terminal_block_string, &self.dst_terminal_block_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);
        let dst_terminal_input: TextInput<MainMessage> =
            TextInput::new(&terminal_string, &self.dst_terminal_value)
                .size(10)
                .on_input(MainMessage::SourceChanged)
                .padding(2);

        // Add connection button
        let add_connection_string = self.localizer.get("add_connection");
        let add_connection_button: Button<MainMessage> =
            Button::new(Text::new(add_connection_string).size(12))
                .on_press(MainMessage::AddConnectionPressed)
                .padding(2);

        // Save file button
        let save_file_string = self.localizer.get("save_file");
        let save_file_button: Button<MainMessage> =
            Button::new(Text::new(save_file_string).size(12))
                .on_press(MainMessage::SaveFilePressed)
                .padding(2);

        // Additional buttons and checkbox states not yet included in the view
        let export_wires_string = self.localizer.get("export_wires");
        let export_wires_button: Button<MainMessage> =
            Button::new(Text::new(export_wires_string).size(12))
                .on_press(MainMessage::ExportWiresPressed)
                .padding(2);

        let export_cables_string = self.localizer.get("export_labels");
        let export_cables_button: Button<MainMessage> =
            Button::new(Text::new(export_cables_string).size(12))
                .on_press(MainMessage::ExportCablesPressed)
                .padding(2);

        let quit_string = self.localizer.get("quit");
        let quit_button: Button<MainMessage> = Button::new(Text::new(quit_string).size(12))
            .on_press(MainMessage::QuitPressed)
            .padding(2);

        let lock_destination_string = self.localizer.get("lock_destination");
        let lock_destination_checkbox: Checkbox<MainMessage> = Checkbox::new(
            lock_destination_string,
            self.lock_destination_checked,
            move |new_value| MainMessage::LockDestinationChanged(new_value),
        )
        .size(10)
        .text_size(10);
        let increment_checkbox_string = self.localizer.get("increment");
        let increment_field1_checkbox: Checkbox<MainMessage> = Checkbox::new(
            increment_checkbox_string.clone(),
            self.increment_field1_checked,
            move |new_value| MainMessage::IncrementField1Changed(new_value),
        )
        .size(10)
        .text_size(10);
        let increment_field2_checkbox: Checkbox<MainMessage> = Checkbox::new(
            increment_checkbox_string.clone(),
            self.increment_field2_checked,
            move |new_value| MainMessage::IncrementField2Changed(new_value),
        )
        .size(10)
        .text_size(10);

        let edit_string = self.localizer.get("edit");
        let edit_button: Button<MainMessage> = Button::new(Text::new(edit_string).size(12))
            .on_press(MainMessage::EditPressed)
            .padding(2);
        let delete_string = self.localizer.get("delete");
        let delete_button: Button<MainMessage> = Button::new(Text::new(delete_string).size(12))
            .on_press(MainMessage::DeletePressed)
            .padding(2);

        let connections_list = self.connections.iter().enumerate().fold(
            Column::new().spacing(2),
            |column: Column<'_, MainMessage, Renderer>, (i, connection)| {
                // Format the source and destination strings
                let source_str = format!(
                    "{}-{}-{}",
                    connection.src_component,
                    connection.src_terminal_block,
                    connection.src_terminal,
                );
                let destination_str = format!(
                    "{}-{}-{}",
                    connection.dst_component,
                    connection.dst_terminal_block,
                    connection.dst_terminal,
                );

                // Check if the current connection is selected
                let is_selected = self
                    .selected_connections
                    .get(&connection.uuid)
                    .unwrap_or(&false);

                let button_style = if *is_selected {
                    ConnectionStyle::Selected
                } else {
                    ConnectionStyle::Unselected
                };

                // Create a button for the connection
                let button: Button<MainMessage, Renderer> = Button::new(
                    Text::new(format!("{} - {}", source_str, destination_str))
                        .size(10)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .on_press(MainMessage::ToggleConnectionSelected(connection.uuid))
                .width(Length::FillPortion(1))
                .padding(2)
                .style(ThemeButton::Custom(Box::new(button_style)))
                .into();

                column.push(button)
            },
        );

        let connections_scrollable = Scrollable::new(connections_list)
            .width(Length::Fill)
            .height(Length::Fill);

        // Construct rows for the source and destination input fields
        let source_inputs_row = Row::new()
            .spacing(20)
            .push(src_component_input)
            .push(src_terminal_block_input)
            .push(src_terminal_input)
            .push(increment_field1_checkbox)
            .padding(2);
        let destination_inputs_row = Row::new()
            .spacing(10)
            .push(dst_component_input)
            .push(dst_terminal_block_input)
            .push(dst_terminal_input)
            .push(increment_field2_checkbox)
            .padding(2);

        let lock_and_add_row = Row::new()
            .spacing(20)
            .push(add_connection_button)
            .push(lock_destination_checkbox)
            .padding(2);

        // Combine edit and delete buttons into a row
        let edit_and_delete_row = Row::new()
            .spacing(20)
            .push(edit_button)
            .push(delete_button)
            .padding(2);

        let connections_column = Column::new()
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .push(connections_scrollable)
            .push(edit_and_delete_row);

        // Combine additional buttons into a row
        let bottom_button_row = Row::new()
            .spacing(10)
            .push(save_file_button)
            .push(export_wires_button)
            .push(export_cables_button)
            .push(quit_button);

        let inputs_and_actions_column = Column::new()
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .push(source_inputs_row)
            .push(destination_inputs_row)
            .push(lock_and_add_row)
            .push(bottom_button_row);

        let main_row = Row::new()
            .spacing(20)
            .push(connections_column)
            .push(inputs_and_actions_column);

        // Construct the main container with the Scrollable
        let title = self.localizer.get("application_title");
        let content = Column::new()
            .spacing(20)
            .push(Text::new(title).size(15))
            .push(main_row);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
