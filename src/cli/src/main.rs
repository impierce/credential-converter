pub mod events;
pub mod jsonpointer;
pub mod logging;
pub mod render;
pub mod repository;
pub mod transformations;
pub mod utils;

use crate::events::*;
use crate::render::*;

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use logging::initialize_logging;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use utils::AppState;

fn main() -> Result<()> {
    initialize_logging().expect("Unexpected error while initializing logging");

    trace_dbg!("Starting the application");

    // Initialize the alternate terminal screen, its input and the backend for it.
    //let mut stdout = stdout();
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut state = AppState {
        selected_input_field: 1,
        ..Default::default()
    };

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            state.area = area;
            render_page(frame, area, &mut state);
        })?;
        if events_handler(&mut state)? {
            break;
        };
    }

    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
