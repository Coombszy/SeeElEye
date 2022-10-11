use std::io;

mod libs;
use libs::ui::{create_terminal, restore_terminal, run_app, App};
use libs::utils::load_scripts;

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal().unwrap();

    let mut app = App::new();
    app.scripts = load_scripts("./static".to_string());
    let _res = run_app(&mut terminal, app);

    restore_terminal(terminal).unwrap();
    Ok(())
}
