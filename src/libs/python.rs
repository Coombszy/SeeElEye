use std::{process::{Command, exit, Output}, collections::HashMap, io};

use super::structs::{Script};

/// Validate that python is installed
/// This is done using `python --version`
/// Does not return anything, causes app to exit(1)
pub fn validate_python() {
    let status =  Command::new("sh")
        .arg("-c")
        .arg("python3 --version")
        .output().expect("Failed to validate python install").status;
    if !status.success() {
        println!("command `python3` was not found, is it installed?");
        exit(1)
    }
}

/// Run python script using Script struct
/// Returns ????
pub fn run_script(script: &Script, arguments: &HashMap<String, String>) -> io::Result<Output> {

    // Build argument
    let mut arg = "python3".to_string();

    arg.push_str(&format!(" {}", script.script_location.clone().unwrap()));

    // Just push all arguments, script will ignore unused ones
    for kv in arguments.clone() {
        arg.push_str(&format!(" -{} {}", kv.0, kv.1 ));
    }

    Command::new("sh")
        .arg("-c")
        .arg(arg).output()
}
