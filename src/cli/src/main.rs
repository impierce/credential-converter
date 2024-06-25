pub mod credential_formats;
pub mod events;
pub mod render;
pub mod state;
pub mod backend;

use crate::credential_formats::*;
use crate::events::*;
use crate::render::*;

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use backend::logging::initialize_logging;
use ratatui::prelude::{CrosstermBackend, Terminal};
use state::Tabs;
use std::io::{stdout, Result};
use state::AppState;

fn main() -> Result<()> {
    initialize_logging().expect("Unexpected error while initializing logging");

    trace_dbg!("Starting the application");

    // Initialize the alternate terminal screen, its input and the backend for it.
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut state = AppState {
        // TODO: remove these hardcoded paths
        input_path: "res/source_credential_ELM.json".to_string(),
        mapping_path: "res/mapping_empty.json".to_string(),
        output_path: "res/output_credential.json".to_string(),

        // tab: Tabs::ManualMappingP2,
        missing_data_fields: vec![
            ("".to_string(), "".to_string()),
            ("field1".to_string(), "".to_string()),
            ("field2".to_string(), "".to_string()),
            ("field3".to_string(), "".to_string()),
        ], // todo: remove hard code testdata
        optional_fields: vec![
            ("".to_string(), "".to_string()),
            ("field4".to_string(), "".to_string()),
            ("field5".to_string(), "".to_string()),
            ("field6".to_string(), "".to_string()),
        ], // todo: remove hard code testdata

        selected_input_field: 1, // todo: what if none? Also after going back to tab 1 and changing file paths?
        selected_missing_field: 1, // todo: what if none?
        selected_optional_field: 1, // todo: what if none?
        select_multiplicity: true,
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
