use std::{io, thread, time::Duration};

mod libs;
use libs::ui::{create_terminal, restore_terminal, run_app, App};
use libs::utils::load_scripts;

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal().unwrap();

    let mut app = App::new();
    app.scripts = load_scripts("./static".to_string());
    let res = run_app(&mut terminal, app);

    Ok(restore_terminal(terminal).unwrap())
}
