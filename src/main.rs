mod backend;
mod events;
mod render;
mod state;
mod translations;


use backend::headless_cli::Args;
use events::*;
use render::*;
use state::AppState;
use backend::headless_cli::run_headless;
use backend::logging::initialize_logging;

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use clap::Parser;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
i18n!("src/locales", fallback = "en");

fn main() -> Result<()> {
    initialize_logging().expect("Unexpected error while initializing logging");
    trace_dbg!("Starting the application");

    if std::env::args().len() > 1 {
        trace_dbg!("Arguments detected, running headless conversion");

        let cli_args = Args::parse();
        run_headless(cli_args)?;
    } else {
        trace_dbg!("No arguments detected, starting the TUI");

        // Initialize the alternate terminal screen, its input and the backend for it.
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        let mut state = AppState {
            // Default example values, remove if no longer needed
            input_path: "json/ebsi-elm/vcdm2.0-europass-edc-schema/examples/Bengales_highSchoolDiploma.json".to_string(),
            // mapping_path: "json/mapping/mapping_empty.json".to_string(),
            mapping_path: "json/mapping/custom_mapping.json".to_string(),
            output_path: "json/output_credential.json".to_string(),
            // custom_mapping_path: "json/mapping/custom_mapping.json".to_string(),
            selected_input_field: 1, // todo: what if none? Also after going back to tab 1 and changing file paths?
            selected_missing_field: 1, // todo: what if none?
            selected_optional_field: 1, // todo: what if none?
            select_mapping_option: true,
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
    }
    Ok(())
}
