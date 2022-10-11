use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use super::structs::Script;

/// Creates a script object from given filepath
/// Does not validate.
fn load_script(file_path: &String) -> Script {
    let file = File::open(&file_path).expect("Should be able to open file path");
    let reader = BufReader::new(file);

    let mut script = Script {
        title: None,
        description: None,
        version: None,
        arguments: vec![],
        script_location: None,
        enabled: false,
    };

    let mut arg_processing = false;

    for line in reader.lines() {
        let content = line.unwrap();
        let trimmed = content.trim_start_matches("# ");
        // If end of meta data section, escape
        if trimmed.contains("##########") {
            break;
        }
        if arg_processing {
            let arg = trimmed.trim_start_matches("  - ");
            script.add_arguement(arg.to_string());
            continue;
        }
        // Ingest script metadata
        if Script::is_title(trimmed) {
            script.set_title(trimmed.to_string());
        } else if Script::is_description(trimmed) {
            script.set_description(trimmed.to_string())
        } else if Script::is_version(trimmed) {
            script.set_version(trimmed.to_string())
        } else if Script::is_argument(trimmed) {
            arg_processing = true;
        }
    }

    script.script_location = Some(file_path.clone());

    script
}

/// Creates vec of all scripts in directory.
/// Does not validate.
pub fn load_scripts(file_dir: String) -> Vec<Script> {
    // Load all fils in dir and filter to only .py
    let mut files = fs::read_dir(file_dir)
        .unwrap()
        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    files.retain(|file| file.contains(".py"));

    // Return file paths mapped to vec<Script>
    files.iter().map(|file| load_script(file)).collect()
}
