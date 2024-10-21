mod backend;
mod events;
mod render;
mod state;

use backend::headless_cli::run_headless;
use backend::headless_cli::Args;
use backend::logging::initialize_logging;
use events::*;
use render::*;
use state::AppState;

use clap::Parser;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::net::SocketAddr;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
i18n!("src/locales", fallback = "en");

fn main() -> Result<()> {
    initialize_logging().expect("Unexpected error while initializing logging");
    trace_dbg!("Starting the application");

    let mut state = AppState::default();

    // if arguments are passed we are working headless
    // two options:
    // 1. headless CLI (with arguments -o -i -m)
    // 2. headless webservice (with argument -w)
    if std::env::args().len() > 1 {
        trace_dbg!("Arguments detected, running headless conversion");
        let mut cli_args = Args::parse();
        println!("Arguments detected, running headless conversion");
        let args: Vec<String> = std::env::args().collect();
        if args.contains(&"-w".to_string()) {
            println!("Let's run the webservice!!!");
            if args[args.len() - 1].parse::<SocketAddr>().is_ok() {
                println!("Let's use {}", args[2].clone());
                crate::backend::web::api_service(Some(args[args.len() - 1].clone()));
            } else {
                println!("The webservice runs default 127.0.0.1:3000");
                crate::backend::web::api_service(None);
            }
        } else {
            run_headless(&mut cli_args, &mut state)?;
        }
    } else {
        trace_dbg!("No arguments detected, starting the TUI");

        // Default example values, remove if no longer needed
        state.input_path =
            "json/ebsi-elm/vcdm2.0-europass-edc-schema/examples/Bengales_highSchoolDiploma.json".to_string();
        // state.mapping_path = "json/mapping/mapping_empty.json".to_string();
        state.mapping_path = "DESM".to_string();
        state.output_path = "json/output_credential.json".to_string();
        state.custom_mapping_path = "json/mapping/custom_mapping.json".to_string();

        // Currently it's set to 1 due to the empty line added above the field lists for spacing
        state.selected_input_field = 1;
        state.selected_output_field = 1;
        state.selected_output_field = 1;
        state.select_mapping_option = true;

        // Initialize the alternate terminal screen, its input and the backend for it.
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

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
