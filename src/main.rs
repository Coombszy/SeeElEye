use std::{fs::File, io::{BufReader, BufRead}};

use libs::structs::Script;
mod libs;

fn main() {

    let file_path = "./static/favicon.py";

    // Testing
    let file = File::open(file_path).expect("Should be able to open file");
    let reader = BufReader::new(file);

    let mut script = Script{ title: None, description: None, version: None, arguments: vec![], script_location: None };

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
        }
        else if Script::is_description(trimmed) {
            script.set_description(trimmed.to_string())
        }
        else if Script::is_version(trimmed) {
            script.set_version(trimmed.to_string())
        }
        else if Script::is_argument(trimmed) {
            arg_processing = true;
        }
    }

    println!("{:?}", script);

}
