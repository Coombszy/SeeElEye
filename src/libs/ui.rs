use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame, Terminal,
};

use super::structs::Script;

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
pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Handles app core loop
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| table_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers == KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('c') => return Ok(()),
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char(' ') => app.toggle(),
                    _ => {}
                }
            }
        }
    }
}

/// Controls ui for app
fn table_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let full_width = f.size().width - 2;

    let cell_1 = 5;
    let cell_2 = 25;

    let padding = 2;

    let rects = Layout::default()
        .constraints([Constraint::Length(full_width.clone())].as_ref())
        .split(f.size());

    // Create table
    let selected_style = Style::default().bg(Color::DarkGray);
    let normal_style = Style::default().bg(Color::White);
    let header_cells = ["Run?", "Script", "Description"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
    let header = Row::new(header_cells).style(normal_style).height(1);
    let rows = app.scripts.iter().map(|item| {
        let mut enabled_state = "";
        if item.enabled {
            enabled_state = " X";
        }

        let f_title = format(item.title.clone().unwrap(), cell_2 - padding);

        let dynamic_size: u16 = f.size().width - (cell_1 + cell_2 + padding);
        let f_description = format(
            item.description.clone().unwrap(),
            (dynamic_size - padding).try_into().unwrap(),
        );

        let cells = vec![
            Cell::from(enabled_state),
            Cell::from(f_title.0),
            Cell::from(f_description.0),
        ];
        // Use the bigger cell height
        if f_title.1 > f_description.1 {
            Row::new(cells).height(f_title.1 + 1)
        } else {
            Row::new(cells).height(f_description.1 + 1)
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
        .highlight_style(selected_style)
        .highlight_symbol(">")
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
            formatted.push_str("\n");
            count = 0;
            splits += 1;
        }
    }
    return (formatted.to_string(), splits);
}

////////////////////////////////////////////
// Structs/Impls
////////////////////////////////////////////

pub struct App {
    state: TableState,
    pub scripts: Vec<Script>,
}

impl<'a> App {
    pub fn new() -> App {
        App {
            state: TableState::default(),
            scripts: vec![],
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.scripts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.scripts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn toggle(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.scripts[i].enabled = !self.scripts[i].enabled;
            }
            _ => (),
        }
    }
}
