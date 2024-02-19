use iced::multi_window::Application;
use iced::Settings;
use std::path::PathBuf;

use label_wires::ui::views::main_view::Flags;
use label_wires::ui::views::main_view::MainApplication;

pub fn main() -> iced::Result {
    let flags = Flags {
        initial_file: Some(PathBuf::from("resources/data/connections.json")),
        settings_file: Some(PathBuf::from("resources/config/settings.json")),
        output_file: Some(PathBuf::from("resources/data/new_file.json")),
    };

    MainApplication::run(Settings::with_flags(flags))
}
