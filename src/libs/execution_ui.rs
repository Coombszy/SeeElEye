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
    vec,
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
) -> io::Result<()> {
    loop {
        // Reads data from threads channel
        // and loads it into the states hashmap
        match app.receiver.try_recv() {
            Ok(state) => {
                if let std::collections::hash_map::Entry::Vacant(e) =
                    app.states.entry(state.script.uuid)
                {
                    e.insert(state);
                } else {
                    let s = app.states.get_mut(&state.script.uuid).unwrap();
                    *s = state;
                }
            }
            _ => (),
        }

        terminal.draw(|f| table_ui(f, &mut app))?;

        if poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('c') => exit(terminal),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Esc => break,
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
    Ok(())
}

/// Controls ui for app
fn table_ui<B: Backend>(f: &mut Frame<B>, app: &mut TableApp) {
    // Padding used for scaling columns correctly
    let cell_1 = 25;
    let cell_2 = 10;
    let padding = 2;
    let full_width = f.size().width - padding;
    let dynamic_size: u16 = full_width - (cell_1 + cell_2);
    let max_lines = 8;

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
            _ => "STARTING".to_string(),
        };
        let output = match app.states.get(&script.uuid) {
            Some(script) => match script.output.clone() {
                Some(output) => output,
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let f_script = format(script.title.clone().unwrap(), cell_1 - padding);
        let f_output = multiline_format(output, dynamic_size - padding, max_lines);

        let cells = vec![
            Cell::from(f_script.0),
            Cell::from(status),
            Cell::from(f_output.0),
        ];

        // Use the bigger cell height
        // But cap it
        if f_script.1 > max_lines || f_output.1 > max_lines {
            Row::new(cells).height(max_lines)
        } else if f_script.1 > f_output.1 {
            Row::new(cells).height(f_script.1 + 1)
        } else {
            Row::new(cells).height(f_output.1 + 1)
        }
    });

    let constraints = [
        Constraint::Length(cell_1),
        Constraint::Length(cell_2),
        Constraint::Length(dynamic_size),
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

/// Splits the content of a multiline string based on the limit and /n.
/// Returns a tuple of the splis string and the number of splits rquired
fn multiline_format(content: String, limit: u16, lines_limit: u16) -> (String, u16) {
    let mut split_vec: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
    let mut total_splits: u16 = split_vec.len() as u16 - 1; // offset by 1

    for split in split_vec.iter_mut() {
        let (c, x) = format(split.clone().to_string(), limit);
        *split = c;
        total_splits += x;
    }

    if split_vec.len() as u16 > lines_limit {
        (
            split_vec[split_vec.len() - (lines_limit as usize)..].join("\n"),
            lines_limit,
        )
    } else {
        (split_vec.join("\n"), total_splits)
    }
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
}
