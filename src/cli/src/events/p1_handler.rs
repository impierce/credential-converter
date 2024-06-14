use crate::backend::preload_p2::preload_p2;
use crate::trace_dbg;
use crate::state::{AppState, P1Prompts, Tabs};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};
use std::path::Path;

pub fn p1_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.output_warning {
                        state.output_warning = false;
                    }
                    else {
                        return Ok(true)
                    }
                }
                Tab => {
                    let path = Path::new(&state.input_path);
                    if state.tab == Tabs::InputPromptsP1 && path.exists() && path.is_file() {
                        state.tab.next();
                        preload_p2(state);
                    } else {
                        state.tab.next();
                    }
                }
                F(2) => {
                    state.tab.prev();
                }
                Left => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.prev();
                    }
                }
                Right => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.next();
                    }
                }
                Up => {
                    state.p1_prompts.prev();
                }
                Down => {
                    state.p1_prompts.next();
                }
                Enter => {

                    let input_path = Path::new(&state.input_path);
                    let output_path = Path::new(&state.output_path);
                    let mapping_path = Path::new(&state.mapping_path);
                    if state.p1_prompts == P1Prompts::Output && output_path.is_file() && !state.output_warning {
                        state.output_warning = true;
                    }
                    else if state.p1_prompts == P1Prompts::Mapping && input_path.is_file() && mapping_path.is_file() {
                        state.tab.next();
                        preload_p2(state);
                    }
                    else if state.output_warning {
                        state.output_warning = false;
                        state.p1_prompts.next();
                    }
                    else {
                        state.p1_prompts.next();
                    }
                }
                Backspace => match state.p1_prompts {
                    P1Prompts::Input => {
                        state.input_path.pop();
                    }
                    P1Prompts::Output => {
                        state.output_path.pop();
                    }
                    P1Prompts::MappingFile => {
                        state.mapping_path.pop();
                    }
                    _ => {}
                },
                Char(value) => {
                    if !state.output_warning {
                        match state.p1_prompts {
                            P1Prompts::Input => {
                                state.input_path.push(value);
                            }
                            P1Prompts::Output => {
                                state.output_path.push(value);
                            }
                            P1Prompts::MappingFile => {
                                state.mapping_path.push(value);
                            }
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
    }
    Ok(false)
}
