use crate::{
    backend::{
        repository::update_repository, selector::selector
    }, p2_p3_common::{handle_backspace, handle_char, handle_down, handle_esc, handle_f2, handle_left, handle_mouse_up, handle_right, handle_scroll_down, handle_scroll_up, handle_tab, handle_up}, state::{AppState, MappingOptions, P2P3Tabs, Pages, Transformations}, trace_dbg
};

use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if handle_esc(state) {
                        return Ok(true);
                    }
                }
                Backspace => {
                    handle_backspace(state);
                }
                Tab => {
                    handle_tab(state);
                }
                F(2) => {
                    handle_f2(state);
                }
                Left => {
                    handle_left(state);
                }
                Right => {
                    handle_right(state);
                }
                Up => {
                    handle_up(state);
                }
                Down => {
                    handle_down(state);
                }
                Enter => {
                    handle_enter(state);
                }
                Char(char) => {
                    handle_char(state, char);
                }
                _ => {}
            }
        }
    }
    if let event::Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
            event::MouseEventKind::ScrollDown => {
                handle_scroll_down(state, mouse_event);
            }
            event::MouseEventKind::ScrollUp => {
                handle_scroll_up(state, mouse_event);
            }
            event::MouseEventKind::Up(_) => {
                handle_mouse_up(state, mouse_event);
            }
            _ => {}
        }
    }

    Ok(false)
}

/////     HELPERS     /////

fn handle_enter(state: &mut AppState) {
    // Close warning, reset values and move to next page
    if state.uncompleted_warning {
        state.uncompleted_warning = false;
        state.popup_mapping_p2_p3 = false;
        state.selected_input_field = 1;
        state.selected_transformation = 0;
        state.selected_transformations_tab = false;
        state.selected_transformations.clear();
        state.select_mapping_option = true;
        state.p2_p3_tabs = P2P3Tabs::InputFields;
        state.page.next();
    } else {
        match state.p2_p3_tabs {
            P2P3Tabs::MappingOptions => {
                // Switch from mapping options tab to respective tab
                if state.select_mapping_option {
                    // Fast-track mapping, Copy to output result value and reset values
                    if state.mapping_option == MappingOptions::DirectCopy {
                        selector(state);
                        state.selected_transformations_tab = false;
                        state.popup_mapping_p2_p3 = false;
                        state.select_mapping_option = true;
                        state.selected_transformations.clear();
                        state.popup_offset_path = 0;
                        state.popup_offset_value = 0;
                        state.p2_p3_tabs = P2P3Tabs::InputFields;

                        // Only push input field and missing field if they are not already in the completed fields
                        if !state.completed_input_fields.contains(&state.selected_input_field) {
                            state.completed_input_fields.push(state.selected_input_field);
                        }
                        if !state.completed_missing_fields.contains(&state.selected_missing_field) {
                            state.completed_missing_fields.push(state.selected_missing_field);
                        }

                        state.missing_data_fields[state.selected_missing_field].1 =
                            state.candidate_data_value.clone().unwrap();

                        // Move active fields to next field
                        if state.selected_input_field == state.input_fields.len() - 1 {
                            state.selected_input_field = 1;
                        } else {
                            state.selected_input_field += 1;
                        }

                        if state.selected_missing_field == state.missing_data_fields.len() - 1 {
                            state.selected_missing_field = 1;
                        } else {
                            state.selected_missing_field += 1;
                        }

                        update_repository(state);
                    } else {
                        state.select_mapping_option = false;
                    }
                }
                // Select a transformation
                else if state.mapping_option == MappingOptions::Transformations
                    && !state.selected_transformations_tab
                    && !state.selected_transformations.contains(&state.transformations)
                {
                    state.selected_transformations.push(state.transformations);
                }
                // Complete a mapping from the view popup
                else if state.popup_mapping_p2_p3 {
                    state.popup_mapping_p2_p3 = false;
                    state.selected_transformations_tab = false;
                    state.select_mapping_option = true;
                    state.selected_transformations.clear();
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                    state.p2_p3_tabs = P2P3Tabs::InputFields;

                    // Only push input field and missing field if they are not already in the completed fields
                    if !state.completed_input_fields.contains(&state.selected_input_field) {
                        state.completed_input_fields.push(state.selected_input_field);
                    }
                    if !state.completed_missing_fields.contains(&state.selected_missing_field) {
                        state.completed_missing_fields.push(state.selected_missing_field);
                    }

                    selector(state);
                    state.missing_data_fields[state.selected_missing_field].1 =
                        state.candidate_data_value.clone().unwrap();

                    // Move active fields to next field
                    if state.selected_input_field == state.input_fields.len() - 1 {
                        state.selected_input_field = 1;
                    } else {
                        state.selected_input_field += 1;
                    }

                    if state.selected_missing_field == state.missing_data_fields.len() - 1 {
                        state.selected_missing_field = 1;
                    } else {
                        state.selected_missing_field += 1;
                    }

                    update_repository(state);
                }
                // If transformation(s) selected open the view popup to show the result.
                else if state.selected_transformations_tab {
                    state.popup_mapping_p2_p3 = true;
                }
            }
            P2P3Tabs::Clear => {
                // Close popup if open.
                if state.popup_mapping_p2_p3 {
                    state.popup_mapping_p2_p3 = false;
                }
                // If mapping options have been chosen, clear mapping options.
                else if !state.select_mapping_option {
                    state.transformations = Transformations::LowerCase;
                    state.selected_transformations.clear();
                    state.mapping_option = MappingOptions::DirectCopy;
                    state.selected_transformations_tab = false;
                    state.select_mapping_option = true;
                }
                // Clear selected missing field
                else {
                    state.missing_data_fields[state.selected_missing_field].1.clear();

                    state.transformations = Transformations::LowerCase;
                    state.selected_transformations.clear();
                    state.mapping_option = MappingOptions::DirectCopy;
                    state.selected_transformations_tab = false;
                    state.select_mapping_option = true;
                }
            }
            P2P3Tabs::View => {
                // Open Popup
                if !state.popup_mapping_p2_p3 {
                    state.popup_mapping_p2_p3 = true;
                } else {
                    state.popup_mapping_p2_p3 = false;
                    state.selected_transformations_tab = false;
                    state.select_mapping_option = true;
                    state.selected_transformations.clear();
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                    state.p2_p3_tabs = P2P3Tabs::InputFields;

                    if !state.completed_input_fields.contains(&state.selected_input_field) {
                        state.completed_input_fields.push(state.selected_input_field);
                    }
                    if !state.completed_missing_fields.contains(&state.selected_missing_field) {
                        state.completed_missing_fields.push(state.selected_missing_field);
                    }
                    state.missing_data_fields[state.selected_missing_field].1 =
                        state.candidate_data_value.clone().unwrap();
                    trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                    trace_dbg!(state.missing_data_fields.clone()[state.selected_missing_field].to_owned());

                    // Move active fields to next field
                    if state.selected_input_field == state.input_fields.len() - 1 {
                        state.selected_input_field = 1;
                    } else {
                        state.selected_input_field += 1;
                    }

                    if state.selected_missing_field == state.missing_data_fields.len() - 1 {
                        state.selected_missing_field = 1;
                    } else {
                        state.selected_missing_field += 1;
                    }
                }
            }
            _ => {
                if !state.popup_mapping_p2_p3 && state.page != Pages::UnusedDataP3 {
                    state.p2_p3_tabs.next();
                } else {
                    state.popup_mapping_p2_p3 = false;
                    state.p2_p3_tabs.next();
                }
            }
        }
    }
}
