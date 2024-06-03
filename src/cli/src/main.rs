pub mod utils;
pub mod render;
pub mod events;

use crate::render::*;
use crate::events::*;

use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use utils::AppState;
use std::io::{stdout, Result};

fn main() -> Result<()> {

    // Initialize the alternate terminal screen, its input and the backend for it.
    stdout().execute(EnterAlternateScreen)?;
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
            render_page(frame, area, &mut state);
        })?;
        if events_handler(&mut state)? {
            break;
        };
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
