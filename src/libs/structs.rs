use std::{thread::JoinHandle, sync::mpsc::Sender};

#[derive(Debug, Clone)]
pub struct Script {
    pub title: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub arguments: Vec<String>,
    pub script_location: Option<String>,
    pub enabled: bool,
}

impl Script {
    pub fn set_title(&mut self, content: String) {
        self.title = Some(content.trim_start_matches("Title: ").to_string());
    }
    pub fn is_title(content: &str) -> bool {
        content.contains("Title: ")
    }

    pub fn set_description(&mut self, content: String) {
        self.description = Some(content.trim_start_matches("Description: ").to_string());
    }
    pub fn is_description(content: &str) -> bool {
        content.contains("Description: ")
    }

    pub fn set_version(&mut self, content: String) {
        self.version = Some(content.trim_start_matches("Version: ").to_string());
    }
    pub fn is_version(content: &str) -> bool {
        content.contains("Version: ")
    }

    pub fn add_arguement(&mut self, content: String) {
        self.arguments.push(content);
    }
    pub fn is_argument(content: &str) -> bool {
        content.contains("Arguments:")
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    STARTING,
    RUNNING,
    FINISHED,
    FAILED
}

#[derive(Debug, Clone)]
pub struct ScriptState {
    pub script: Script,
    pub status: Status,
    pub output: String,
}

#[derive(Debug)]
pub struct ScriptRuntime {
    pub script: Script,
    pub handle: Option<JoinHandle<()>>,
    pub transmitter: Sender<ScriptState>
}
