use crossterm::event::{self, KeyCode::*, KeyEventKind};
use crate::utils::{AppState, PathPrompts, Tabs};

pub fn events_handler(state: &mut AppState) -> Result<bool, std::io::Error> {
    let mut quit = false;

    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match state.tab {
                    Tabs::InputPromptsP1 => { quit = p1_handler(key, state)?; },
                    Tabs::ManualMappingP2 => { quit = p2_handler(key, state)?; },
                    Tabs::UnusedDataP3 => { quit = p3_handler(key, state)?; },

                }
            }
        }
    }

    Ok(quit)
}

fn p1_handler(key: event::KeyEvent, state: &mut AppState) -> Result<bool, std::io::Error> {
    match key.code {
        Esc => return Ok(true),
        Tab => { state.tab = state.tab.next(); },
        F(2) => { state.tab = state.tab.prev(); },
        Left => {},
        Right => {},
        Up => { state.path_prompts = state.path_prompts.prev(); },
        Down => { state.path_prompts = state.path_prompts.next(); },
        Enter => { 
            if state.path_prompts == PathPrompts::UnusedData {
                state.tab = state.tab.next();
            }
            state.path_prompts = state.path_prompts.next(); },
        Backspace => {
            match state.path_prompts {
                PathPrompts::Input => {
                    state.input_path.pop();
                }
                PathPrompts::Output => {
                    state.output_path.pop();
                }
                PathPrompts::UnusedData => {
                    state.unused_data_path.pop();
                }
            }
        },
        Char(value) => {
            match state.path_prompts {
                PathPrompts::Input => {
                    state.input_path.push(value);
                }
                PathPrompts::Output => {
                    state.output_path.push(value);
                }
                PathPrompts::UnusedData => {
                    state.unused_data_path.push(value);
                }
            }
        },
        _ => {}
    }

    Ok(false)
}

fn p2_handler(key: event::KeyEvent, state: &mut AppState) -> Result<bool, std::io::Error> {
    match key.code {
        Esc => return Ok(true),
        Tab => { state.tab = state.tab.next(); },
        F(2) => { state.tab = state.tab.prev(); },
        _ => {}
    }

    Ok(false)
}

fn p3_handler(key: event::KeyEvent, state: &mut AppState) -> Result<bool, std::io::Error> {
    match key.code {
        Esc => return Ok(true),
        Tab => { state.tab = state.tab.next(); },
        F(2) => { state.tab = state.tab.prev(); },
        _ => {}
    }

    Ok(false)
}
