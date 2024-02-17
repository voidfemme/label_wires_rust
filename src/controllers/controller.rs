// use crate::config::settings::Settings;
// use crate::events::event_system::EventSystem;
// use crate::models::command::{AddConnectionCommand, DeleteConnectionCommand};
// use crate::models::connection_manager::{self, ConnectionManager, ConnectionManagerError};
// use crate::ui::views::main_view::MainView;
// use crate::ui::views::new_project_dialog::NewProjectDialog;
// use crate::utils::csv_exporting_strategy::{
//     ExportCableToCSVStrategy, ExportFormat, ExportWireToCSVStrategy,
// };
// use crate::utils::{command_manager, file_handler};
// use crate::utils::{
//     command_manager::CommandManager,
//     file_handler::FileHandler,
//     localizer::{self, Localizer},
// };
// use std::path::PathBuf;
//
// struct Controller {
//     pub settings: Arc<Mutex<Settings>,
//     pub localizer: Localizer,
//     pub command_manager: CommandManager,
//     pub event_system: EventSystem,
//     pub connection_manager: ConnectionManager,
//     pub view: MainView,
//     full_file_path: Option<PathBuf>,
//     file_handler: FileHandler,
// }
//
// impl Controller {
//     pub fn new(
//         settings_file_path: PathBuf,
//         wire_label_path_name: PathBuf,
//         output_file_name: PathBuf,
//     ) -> Result<Self, Box<dyn std::error::Error>> {
//         // Initialize settings
//         let settings = Settings::new(settings_file_path.clone())?;
//
//         // Initialize Localizer with the language from settings
//         let localizer = Localizer::new(settings.get("language").unwrap_or(&"en".to_string()));
//
//         // Initialize CommandManager
//         let command_manager = CommandManager::new();
//
//         // Initialize EventSystem
//         let event_system = EventSystem::new();
//
//         // Initialize ConnectionManager with necessary paths
//         let connection_manager = ConnectionManager::new(
//             wire_label_path_name,
//             output_file_name,
//             settings_file_path.clone(),
//         )?;
//
//         // Initialize MainView (assuming it's simple and requires no parameters for now)
//         let view = MainView::new();
//
//         // Initialize FileHandler
//         let file_handler = FileHandler::new(Some(settings_file_path));
//
//         Ok(Controller {
//             settings,
//             localizer,
//             command_manager,
//             event_system,
//             connection_manager,
//             view,
//             full_file_path: None,
//             file_handler,
//         })
//     }
//
//     fn open_new_project_dialog(&self) {
//         todo!();
//         // let dialog = NewProjectDialog::new(self.settings, self.localizer);
//         // let result = dialog.show();
//
//         // match result {
//         //     Some(file_path) => {
//         //         self.full_file_path = Some
//         //     }
//         // }
//         // dialog.wait_window()
//     }
//
//     // Retrieves the path of the currently loaded file.
//     fn get_file_path(&self) {}
//
//     // Set or update the path of the loaded file
//     fn populate_connections(&self) {
//         todo!();
//     }
//
//     fn load_connections(&self) {
//         self.load_from_json_file()
//         // If the connections cannot be loaded, display a message
//         // self.view.footer.display_status("Example Error message");
//     }
//
//     fn update_connection_list(&self) {
//         // update the list of connections in the UI
//     }
//
//     fn save_edited_connection_command(&self) {
//         let p1_values = [
//             /* get the values from the entry frame */
//         ];
//         let p2_values = [
//             /* get the values from the entry frame */
//         ];
//         let new_values = [
//             /* get the values from the entry frame */
//         ];
//
//         // Add the edited connection
//         // self.add_connection_command(p1_values, p2_values)
//     }
//
//     fn add_connection_command(&self, source: String, destination: String) {}
//
//     fn load_from_json_file(&self) {
//         // let connection_dicts = self.file_handler.load_wires();
//         // self.connection_manager
//         //     .populate_connections(connection_dicts);
//     }
//
//     fn display_status(&self, message: String) {
//         // self.view.footer.display_status(message);
//     }
// }
