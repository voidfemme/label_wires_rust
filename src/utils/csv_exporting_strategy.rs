use crate::models::connection::Connection;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub enum ExportFormat {
    Wire,
    Cable,
}

trait ExportToCsvStrategy {
    fn export_to_csv(
        &self,
        file_path: PathBuf,
        connection_list: &[Connection],
    ) -> Result<(), Box<dyn Error>>;
    fn generate_csv_string(&self, connection_list: &[Connection]) -> String;
}

pub struct ExportCableToCSVStrategy;
pub struct ExportWireToCSVStrategy;

impl ExportToCsvStrategy for ExportWireToCSVStrategy {
    fn export_to_csv(
        &self,
        mut file_path: PathBuf,
        connection_list: &[Connection],
    ) -> Result<(), Box<dyn Error>> {
        if file_path.extension().unwrap_or_default() != "csv" {
            file_path.set_extension("csv");
        }
        let file = File::create(file_path)?;
        let mut wtr = csv::WriterBuilder::new().delimiter(b'|').from_writer(file);

        for conn in connection_list {
            let source = format!(
                "{}-{}-{}",
                conn.src_component, conn.src_terminal_block, conn.src_terminal
            );
            let destination = format!(
                "{}-{}-{}",
                conn.dst_component, conn.dst_terminal_block, conn.dst_terminal
            );
            wtr.write_record(&[&source, &destination])?;
        }
        wtr.flush()?;
        println!("Successfully exported wires");
        Ok(())
    }

    fn generate_csv_string(&self, connection_list: &[Connection]) -> String {
        connection_list
            .iter()
            .map(|conn| {
                let source = format!(
                    "{}-{}-{}",
                    conn.src_component, conn.src_terminal_block, conn.src_terminal
                );
                let destination = format!(
                    "{}-{}-{}",
                    conn.dst_component, conn.dst_terminal_block, conn.dst_terminal
                );
                format!("{}|{}", source, destination)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl ExportToCsvStrategy for ExportCableToCSVStrategy {
    fn export_to_csv(
        &self,
        mut file_path: PathBuf,
        connection_list: &[Connection],
    ) -> Result<(), Box<dyn Error>> {
        if file_path.extension().unwrap_or_default() != "csv" {
            file_path.set_extension("csv");
        }
        let file = File::create(file_path)?;
        let mut wtr = csv::WriterBuilder::new().delimiter(b'|').from_writer(file);

        for conn in connection_list {
            let source = format!(
                "{}-{} [{}]",
                conn.src_component, conn.src_terminal_block, conn.src_terminal
            );
            let destination = format!(
                "{}-{} [{}]",
                conn.dst_component, conn.dst_terminal_block, conn.dst_terminal
            );
            wtr.write_record(&[&source, &destination])?;
        }
        wtr.flush()?;
        println!("Successfully exported wires");
        Ok(())
    }

    fn generate_csv_string(&self, connection_list: &[Connection]) -> String {
        connection_list
            .iter()
            .map(|conn| {
                let source = format!(
                    "{}-{} [{}]",
                    conn.src_component, conn.src_terminal_block, conn.src_terminal
                );
                let destination = format!(
                    "{}-{} [{}]",
                    conn.dst_component, conn.dst_terminal_block, conn.dst_terminal
                );
                format!("{}|{}", source, destination)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
