use serde::{Deserialize, Serialize};
use serde_json::{from_value, Result as JsonResult, Value};
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub src_component: String,
    pub src_terminal_block: String,
    pub src_terminal: String,
    pub dst_component: String,
    pub dst_terminal_block: String,
    pub dst_terminal: String,
    #[serde(default = "default_uuid")]
    pub uuid: Uuid,
}

impl Connection {
    pub fn new(
        src_component: String,
        src_terminal_block: String,
        src_terminal: String,
        dst_component: String,
        dst_terminal_block: String,
        dst_terminal: String,
    ) -> Self {
        let conn = Self {
            src_component,
            src_terminal_block,
            src_terminal,
            dst_component,
            dst_terminal_block,
            dst_terminal,
            uuid: Uuid::new_v4(),
        };
        debug!("New connection created: {:?}", conn.uuid);
        conn
    }

    pub fn is_empty(&self) -> bool {
        let attrs = [
            &self.src_component,
            &self.src_terminal_block,
            &self.src_terminal,
            &self.dst_component,
            &self.dst_terminal_block,
            &self.dst_terminal,
        ];
        attrs.iter().all(|attr| attr.is_empty())
    }

    pub fn to_tuple(&self) -> (String, String) {
        let tuple = (
            format!(
                "{}-{}-{}",
                self.src_component, self.src_terminal_block, self.src_terminal
            )
            .trim_end_matches('-')
            .to_string(),
            format!(
                "{}-{}-{}",
                self.dst_component, self.dst_terminal_block, self.dst_terminal
            )
            .trim_end_matches('-')
            .to_string(),
        );
        debug!("Connection {} transformed to tuple: {:?}", self.uuid, tuple);
        tuple
    }

    pub fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "src_component": self.src_component,
            "src_terminal_block": self.src_terminal_block,
            "src_terminal": self.src_terminal,
            "dst_component": self.dst_component,
            "dst_terminal_block": self.dst_terminal_block,
            "dst_terminal": self.dst_terminal,
        })
    }

    pub fn from_json_value(value: Value) -> JsonResult<Self> {
        from_value(value).map_err(|e| {
            error!("Failed to deserialize Connection from JSON: {:?}", e);
            e
        })
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        let is_normal_equal = self.src_component == other.src_component
            && self.src_terminal_block == other.src_terminal_block
            && self.src_terminal == other.src_terminal
            && self.dst_component == other.dst_component
            && self.dst_terminal_block == other.dst_terminal_block
            && self.dst_terminal == other.dst_terminal;

        let is_reverse_equal = self.src_component == other.dst_component
            && self.src_terminal_block == other.dst_terminal_block
            && self.src_terminal == other.dst_terminal
            && self.dst_component == other.src_component
            && self.dst_terminal_block == other.src_terminal_block
            && self.dst_terminal == other.src_terminal;

        is_normal_equal || is_reverse_equal
    }
}

fn default_uuid() -> Uuid {
    Uuid::new_v4()
}
