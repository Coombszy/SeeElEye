use std::collections::HashMap;

use std::io::{self, stdin};
use std::process::exit;
use std::str::from_utf8;
use std::sync::mpsc::Receiver;
use std::thread;

mod libs;
use libs::structs::Status;
use libs::utils::load_scripts;
use libs::{execution_ui, select_ui};

use crate::libs::python::{run_script, validate_python};
use crate::libs::structs::{ScriptRuntime, ScriptState};
use crate::libs::utils::create_runtimes;

fn main() -> Result<(), io::Error> {
    validate_python();

    let mut terminal = select_ui::create_terminal().unwrap();
    // Use chooses what scripts to be ran
    let mut app = select_ui::TableApp::new();
    app.scripts = load_scripts("./static".to_string());
    let mut scripts =
        select_ui::run_table_app(&mut terminal, app).expect("Failed to return scripts from ui");
    select_ui::restore_terminal(&mut terminal).unwrap();

    // DEBUGGING ONLY! DELETE ME! ------------------------------------------------------------------------------
    // let mut scripts = load_scripts("./static".to_string());
    // scripts[0].enabled = true; // Set first one to be enabled, no idea which one :)

    clearscreen::clear().expect("Failed to clear terminal");

    // Filter scripts to only enabled and get all arguments
    scripts.retain(|script| script.enabled);
    let mut arguments: HashMap<String, String> = HashMap::new();
    for script in &scripts {
        for arg in &script.arguments {
            if !arguments.contains_key(arg) {
                arguments.insert(arg.clone(), "NULL".to_string());
            }
        }
    }

    // Any scripts selected?
    if scripts.is_empty() {
        println!("No scripts were selected");
        exit(1);
    }

    // Write arguments to user
    println!("With the current selected scripts, the follow arguments are required:");
    let mut arg_list: Vec<String> = arguments
        .clone()
        .into_keys()
        .map(|arg| format!(" - {}", arg))
        .collect();
    arg_list.sort();
    println!("{}\n", arg_list.join("\n"));
    // Get arguments from user
    // This is a horrible mess, but i cannot think of a nice way to do it while sorting
    // alphabetically.
    let mut ordered_arg_list: Vec<String> = arguments.clone().into_keys().collect();
    ordered_arg_list.sort();
    for arg in ordered_arg_list {
        println!("{}:", arg);
        let arg_val = arguments
            .get_mut(&arg)
            .expect("Could not get arg value from dict");
        let mut new_val = String::new();
        stdin().read_line(&mut new_val).unwrap();
        *arg_val = new_val.trim().to_string();
    }

    // Get all script runtimes and receiver
    let (runtimes, rx): (Vec<ScriptRuntime>, Receiver<ScriptState>) =
        create_runtimes(scripts.clone(), arguments);

    for mut r in runtimes {
        let handle = thread::spawn(move || {
            let mut state = ScriptState {
                script: r.script.clone(),
                status: Status::RUNNING,
                output: None,
            };
            r.transmitter
                .send(state.clone())
                .expect("Failed to transmit script state");

            let output =
                run_script(&r.script, &r.arguments).expect("Failed to get script execution result");
            if output.status.success() {
                let data = from_utf8(&output.stdout).unwrap().trim().to_string();
                state.status = Status::SUCCESS;
                state.output = Some(data);
            } else {
                let data = from_utf8(&output.stderr).unwrap().trim().to_string();
                state.status = Status::FAILED;
                state.output = Some(data);
            }
            r.transmitter
                .send(state)
                .expect("Failed to transmit script state");
        });

        r.handle = Some(handle);
    }

    let mut terminal = execution_ui::create_terminal().unwrap();
    // Use chooses what scripts to be ran
    let app = execution_ui::TableApp::new(rx, scripts);
    execution_ui::run_table_app(&mut terminal, app).expect("Failed to return scripts from ui");
    execution_ui::restore_terminal(&mut terminal).unwrap();

    Ok(())
}
