use std::io;

mod libs;
use libs::ui::{create_terminal, restore_terminal, run_table_app, TableApp};
use libs::utils::load_scripts;

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal().unwrap();

    let mut app = TableApp::new();
    app.scripts = load_scripts("./static".to_string());
    let scripts = run_table_app(&mut terminal, app);

    restore_terminal(&mut terminal).unwrap();

    print!("DEBUG: {:?}", scripts);

    Ok(())
}
