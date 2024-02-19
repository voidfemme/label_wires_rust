use iced::widget::{button, pick_list, text_input, Button, Column, Container, PickList, Text};
use iced::{Command, Element, Length};
use std::path::PathBuf;

use crate::LOCALIZER;

pub struct SettingsState {
    language_list_state: pick_list::State<()>,
    default_save_location_state: text_input::State<()>,
    csv_save_location_state: text_input::State<()>,

    save_button_state: button::State,
    cancel_button_state: button::State,

    selected_language: Option<String>,
    default_save_location: Option<PathBuf>,
    csv_save_location: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum SettingsWindowMessage {
    // Settings menu messages
    Open,
    LanguageSelected(String),
    DefaultSaveLocationChanged(PathBuf),
    BrowseDefaultSaveLocation,
    CsvSaveLocationChanged(PathBuf),
    BrowseCsvSaveLocation,
    SaveSettings,
    CancelChanges,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            language_list_state: pick_list::State::default(),
            default_save_location_state: text_input::State::new(),
            csv_save_location_state: text_input::State::new(),
            save_button_state: button::State::new(),
            cancel_button_state: button::State::new(),
            selected_language: Some("en".to_string()),
            default_save_location: Some(PathBuf::new()),
            csv_save_location: Some(PathBuf::new()),
        }
    }

    pub fn update(&mut self, message: SettingsWindowMessage) -> Command<SettingsWindowMessage> {
        match message {
            SettingsWindowMessage::LanguageSelected(lang) => {}
            SettingsWindowMessage::DefaultSaveLocationChanged(file_path) => {}
            SettingsWindowMessage::BrowseDefaultSaveLocation => {}
            SettingsWindowMessage::CsvSaveLocationChanged(file_path) => {}
            SettingsWindowMessage::BrowseCsvSaveLocation => {}
            SettingsWindowMessage::SaveSettings => {}
            SettingsWindowMessage::CancelChanges => {}
            _ => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<'_, SettingsWindowMessage> {
        let localizer = LOCALIZER.as_ref();

        let language_list_state = &self.language_list_state;
        let languages = vec!["en", "ru", "fr", "es", "shakespeare"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let language_pick_list =
            PickList::new(languages, self.selected_language.clone(), |language| {
                SettingsWindowMessage::LanguageSelected(language)
            });

        // Construct other UI elements here, for example:
        let save_button = Button::new(Text::new(localizer.get("save")))
            .on_press(SettingsWindowMessage::SaveSettings);
        let cancel_button = Button::new(Text::new(localizer.get("cancel")))
            .on_press(SettingsWindowMessage::CancelChanges);

        // Arrange UI elements in a column
        let content = Column::new()
            .push(Text::new(localizer.get("settings")))
            .push(language_pick_list)
            .push(save_button)
            .push(cancel_button);

        // Wrap content in a container
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
