use iced::multi_window;
use iced::window::Id as WindowId;
use iced::{self, Command, Element};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::ui::views::states::{
    file_operation_state::{FileOperationMessage, FileOperationState},
    main_window_state::{MainWindowMessage, MainWindowState},
    settings_state::{SettingsState, SettingsWindowMessage},
};
use crate::{LOCALIZER, SETTINGS};

#[derive(Clone)]
pub struct Flags {
    pub initial_file: Option<PathBuf>,
    pub settings_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}

struct AppState {
    main_window_state: MainWindowState,
    settings_state: SettingsState,
    file_operation_state: FileOperationState,
    active_window: ActiveWindow,
}

impl AppState {
    pub fn new(
        wire_label_path: PathBuf,
        output_file_path: PathBuf,
        settings_file_path: PathBuf,
    ) -> Self {
        Self {
            main_window_state: MainWindowState::new(
                wire_label_path,
                output_file_path,
                settings_file_path,
            ),
            settings_state: SettingsState::new(),
            file_operation_state: FileOperationState::new(),
            active_window: ActiveWindow::Main,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Main(MainWindowMessage),
    Settings(SettingsWindowMessage),
    FileOperation(FileOperationMessage),
}

pub struct MainApplication {
    app_state: AppState,
    windows: HashMap<WindowId, Window>,
}

struct Window {
    title: String,
    visible: bool,
}

pub enum ActiveWindow {
    Main,
    Settings,
    FileOperation,
}

impl multi_window::Application for MainApplication {
    type Executor = iced::executor::Default;
    type Flags = Flags;
    type Message = AppMessage;
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let wire_label_path = flags
            .initial_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));
        let output_file_path = flags
            .output_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));
        let settings_file_path = flags
            .settings_file
            .unwrap_or_else(|| PathBuf::from("default_initial.json"));

        let app = Self {
            app_state: AppState::new(wire_label_path, output_file_path, settings_file_path),
            windows: HashMap::from([(
                WindowId::MAIN,
                Window {
                    title: "Main Window".to_string(),
                    visible: true,
                },
            )]),
        };

        (app, Command::none())
    }

    fn title(&self, window: WindowId) -> String {
        let title: String;
        match window {
            WindowId::MAIN => {
                let localizer = LOCALIZER.as_ref();
                title = localizer.get("application_title");
            }
            _ => title = "Unknown Window".to_string(),
        }
        String::from(title)
    }
    // Define how to handle each type of message
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::Main(MainWindowMessage::Open) => {
                let position = calculate_next_window_postion();
                let (window_id, window_settings) = window::spawn(/* window settings */);
                self.windows.insert(
                    window_id,
                    Window {
                        title: "Main Window".to_string(),
                        visible: true,
                    },
                );
                Command::none()
            }
            AppMessage::Settings(SettingsWindowMessage::Open) => {
                let (window_id, window_settings) = window::spawn(/* window settings */);
                self.windows.insert(
                    window_id,
                    Window {
                        title: "Settings".to_string(),
                        visible: true,
                    },
                );
                Command::none()
            }
            AppMessage::FileOperation(FileOperationMessage::Open) => {
                let (window_id, window_settings) = window::spawn(/* window settings */);
                self.windows.insert(
                    window_id,
                    Window {
                        title: "File".to_string(),
                        visible: true,
                    },
                );
                Command::none()
            }
        }
    }

    fn view(&self, window: WindowId) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match self.app_state.active_window {
            ActiveWindow::Main if Some(window) == self.main_window_id => self
                .app_state
                .main_window_state
                .view()
                .map(AppMessage::Main),
            ActiveWindow::Settings if Some(window) == self.settings_window_id => self
                .app_state
                .settings_state
                .view()
                .map(AppMessage::Settings),
            ActiveWindow::FileOperation if Some(window) == self.file_operation_window_id => self
                .app_state
                .file_operation_state
                .view()
                .map(AppMessage::FileOperation),
            _ => iced::widget::Text::new("Unknown Window").into(),
        }
    }
}
