use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::HashMap,
    io::{self, Stdout},
    sync::mpsc::Receiver,
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use uuid::Uuid;

use super::structs::{Script, ScriptState};

/// Create terminal with default config
pub fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore terminal after application finish
pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn exit(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    restore_terminal(terminal).unwrap();
    std::process::exit(0);
}

/// Handles app core loop
pub fn run_table_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: TableApp,
) -> io::Result<Vec<Script>> {
    loop {
        // Reads data from threads channel
        // and loads it into the states hashmap
        match app.receiver.try_recv() {
            Ok(state) => {
                if let std::collections::hash_map::Entry::Vacant(e) = app.states.entry(state.script.uuid) {
                    e.insert(state);
                } else {
                    let s = app.states.get_mut(&state.script.uuid).unwrap();
                    *s = state;
                }
            }
            _ => (),
        }

        terminal.draw(|f| table_ui(f, &mut app))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('c') => exit(terminal),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => exit(terminal),
                        KeyCode::Esc => exit(terminal),
                        // KeyCode::Down => app.next(),
                        // KeyCode::Up => app.previous(),
                        // KeyCode::Char(' ') => app.toggle(),
                        // KeyCode::Enter => return Ok(app.scripts),
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Controls ui for app
fn table_ui<B: Backend>(f: &mut Frame<B>, app: &mut TableApp) {
    let full_width = f.size().width - 2;

    // Padding used for scaling columns correctly
    let cell_1 = 25;
    let cell_2 = 10;
    let padding = 2;

    let rects = Layout::default()
        .constraints([Constraint::Length(full_width)].as_ref())
        .split(f.size());

    let normal_style = Style::default().bg(Color::White);

    let header_cells = ["Script", "Status", "Output"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    let header = Row::new(header_cells).style(normal_style).height(1);

    let rows = app.scripts.values().map(|script| {
        // Create status and output
        // Handles if no state has been ingested yet
        let status = match app.states.get(&script.uuid) {
            Some(script) => {
                format!("{:?}", script.status)
            }
            _ => "UNKNOWN".to_string(),
        };
        let output = match app.states.get(&script.uuid) {
            Some(script) => match script.output.clone() {
                Some(output) => output,
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let f_script = format(script.title.clone().unwrap(), cell_1 - padding);

        let dynamic_size: u16 = f.size().width - (cell_1 + cell_2 + padding);
        let f_output = format(output, dynamic_size - padding);

        let cells = vec![
            Cell::from(f_script.0),
            Cell::from(status),
            Cell::from(f_output.0),
        ];

        // Use the bigger cell height
        if f_script.1 > f_output.1 {
            Row::new(cells).height(f_script.1 + 1)
        } else {
            Row::new(cells).height(f_output.1 + 1)
        }
    });

    let constraints = [
        Constraint::Length(cell_1),
        Constraint::Length(cell_2),
        Constraint::Length(full_width),
    ];

    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(""))
        .widths(&constraints);

    f.render_stateful_widget(t, rects[0], &mut app.state);
}

/// Splits the content of a string based on the limit.
/// Returns a tuple of the split string and the number
/// of splits required.
fn format(content: String, limit: u16) -> (String, u16) {
    let mut formatted = "".to_string();
    let mut count = 0;
    let mut splits = 0;
    for c in content.chars() {
        formatted.push(c);

        count += 1;
        if count == limit {
            formatted.push('\n');
            count = 0;
            splits += 1;
        }
    }
    (formatted.to_string(), splits)
}

////////////////////////////////////////////
// Structs/Impls
////////////////////////////////////////////

pub struct TableApp {
    state: TableState,
    pub receiver: Receiver<ScriptState>,
    pub scripts: HashMap<Uuid, Script>,
    pub states: HashMap<Uuid, ScriptState>,
}

impl TableApp {
    pub fn new(rx: Receiver<ScriptState>, s: Vec<Script>) -> TableApp {
        TableApp {
            state: TableState::default(),
            receiver: rx,
            scripts: s
                .iter()
                .map(|script| (script.uuid, script.clone()))
                .collect(),
            states: HashMap::new(),
        }
    }
    // pub fn next(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i >= self.scripts.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    // pub fn previous(&mut self) {
    //     let i = match self.state.selected() {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.scripts.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.state.select(Some(i));
    // }

    // pub fn toggle(&mut self) {
    //     if let Some(i) = self.state.selected() {
    //         self.scripts[i].enabled = !self.scripts[i].enabled
    //     }
    // }
}
