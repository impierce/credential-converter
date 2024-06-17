use std::io::Write;

use crate::{
    backend::{
        preload_p2::verify,
        repository::{construct_leaf_node, merge},
        selector::selector,
    },
    elm::ELM,
    obv3::OBv3,
    state::{AppState, Multiplicity, P2Tabs},
    trace_dbg,
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

use super::is_mouse_over_area;

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.popup_p2 {
                        state.popup_p2 = false;
                    } else {
                        return Ok(true);
                    }
                }
                Backspace => {
                    // Delete a selected transformation from the list of selected transformations
                    if state.selected_transformations_tab && !state.selected_transformations.is_empty() {
                        state.selected_transformations.remove(state.selected_transformation);
                        if state.selected_transformation > 0 {
                            state.selected_transformation -= 1;
                        }
                    } else if state.p2_tabs == P2Tabs::MappingOptions && state.multiplicity == Multiplicity::OneToMany {
                        state.dividers.pop();
                    }
                }
                Tab => {
                    if state.p2_tabs == P2Tabs::MappingOptions && !state.select_multiplicity {
                        state.selected_transformations_tab = !state.selected_transformations_tab;
                    } else {
                        state.p2_tabs.next();
                    }
                }
                F(2) => {
                    state.p2_tabs.prev();
                }
                Left => {
                    if state.p2_tabs == P2Tabs::MappingOptions && state.select_multiplicity {
                        state.multiplicity.prev();
                    } else if state.p2_tabs == P2Tabs::MappingOptions
                        && !state.select_multiplicity
                        && !state.selected_transformations_tab
                    {
                        state.transformations.prev();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    } else if state.p2_tabs == P2Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && state.selected_transformation > 0
                    {
                        state.selected_transformation -= 1;
                        trace_dbg!(state.selected_transformation);
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Right => {
                    if state.p2_tabs == P2Tabs::MappingOptions && state.select_multiplicity {
                        state.multiplicity.next();
                    } else if state.p2_tabs == P2Tabs::MappingOptions
                        && !state.select_multiplicity
                        && !state.selected_transformations_tab
                    {
                        state.transformations.next();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    } else if state.p2_tabs == P2Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && !state.selected_transformations.is_empty()
                        && state.selected_transformation < state.selected_transformations.len() - 1
                    {
                        state.selected_transformation += 1;
                        trace_dbg!(state.selected_transformation);
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Up => match state.p2_tabs {
                    P2Tabs::InputFields => {
                        if state.selected_input_field > 1 {
                            state.selected_input_field -= 1;
                            selector(state);
                            trace_dbg!(&state.candidate_data_value);
                        }
                    }
                    P2Tabs::MissingFields => {
                        if state.selected_missing_field > 1 {
                            state.selected_missing_field -= 1;
                        }
                    }
                    _ => {}
                },
                Down => match state.p2_tabs {
                    P2Tabs::InputFields => {
                        if state.selected_input_field <= state.amount_input_fields {
                            state.selected_input_field += 1;
                            selector(state);
                            trace_dbg!(&state.candidate_data_value);
                        }
                    }
                    P2Tabs::MissingFields => {
                        if state.selected_missing_field <= state.amount_missing_fields {
                            state.selected_missing_field += 1;
                        }
                    }
                    _ => {}
                },
                Enter => {
                    match state.p2_tabs {
                        P2Tabs::InputFields => {
                            state.popup_p2 = true;
                        }
                        P2Tabs::MissingFields => {
                            state.popup_p2 = true;
                        }
                        P2Tabs::MappingOptions => {
                            if state.select_multiplicity {
                                state.select_multiplicity = false;
                            } else if !state.selected_transformations_tab {
                                state.selected_transformations.push(state.transformations);
                            } else {
                                let output_format = state.mapping.output_format();

                                // let (_, source_value) = state.input_fields[state.selected_input_field].clone();

                                let source_value = state.candidate_data_value.clone().unwrap();

                                let pointer = state.missing_data_field.as_ref().unwrap().clone();

                                let mut json_value = state.repository.get_mut(&output_format).unwrap();

                                let mut leaf_node = construct_leaf_node(&pointer);

                                leaf_node
                                    .pointer_mut(&pointer)
                                    .map(|value| *value = serde_json::from_str(&source_value).unwrap());

                                merge(&mut json_value, leaf_node);

                                state.missing_data_field = match state.mapping.output_format().as_str() {
                                    "OBv3" => verify::<OBv3>(&mut json_value).err(),
                                    "ELM" => verify::<ELM>(&mut json_value).err(),
                                    _ => panic!(),
                                };

                                if state.missing_data_field.is_none() {
                                    let output_format = state.mapping.output_format();
                                    let json_value = state.repository.get_mut(&output_format).unwrap();
                                    let mut file = std::fs::File::create(&state.output_path).unwrap();
                                    file.write_all(serde_json::to_string_pretty(&json_value).unwrap().as_bytes())
                                        .unwrap();
                                    state.tab.next();
                                }

                                // if state.missing_data_field.is_none() {
                                //     let mut file = std::fs::File::create(&state.output_path).unwrap();
                                //     file.write_all(temp.as_bytes()).unwrap();
                                //     state.tab.next();
                                // }
                            }
                        }
                    }
                }
                Char(char) => {
                    if state.popup_p2 && state.multiplicity == Multiplicity::OneToMany {
                        state.dividers.push(char);
                    }
                }
                _ => {}
            }
        }
    }
    if let event::Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
            event::MouseEventKind::ScrollDown => {
                if !state.popup_p2 && is_mouse_over_area(state.selector_area_p2, mouse_event.column, mouse_event.row) {
                    if state.selected_input_field < state.amount_input_fields {
                        state.selected_input_field += 1;
                    }
                } else if !state.popup_p2
                    && is_mouse_over_area(state.missing_fields_area_p2, mouse_event.column, mouse_event.row)
                {
                    if state.selected_missing_field < state.amount_missing_fields {
                        state.selected_missing_field += 1;
                    }
                }
                // else if state.hover_popup_value_p2 {
                //     if state.offset_value < state.amount_input_fields as u16 {
                //         state.offset_value += 1;
                //     }
                // } else if state.hover_popup_result_path_p2 {
                //     if state.offset_result_path < state.amount_input_fields as u16 {
                //         state.offset_result_path += 1;
                //     }
                // } else if state.hover_popup_result_value_p2 {
                //     if state.offset_result_value < state.amount_input_fields as u16 {
                //         // todo
                //         state.offset_result_value += 1;
                //     }
                // }
            }
            event::MouseEventKind::ScrollUp => {
                if !state.popup_p2 && is_mouse_over_area(state.selector_area_p2, mouse_event.column, mouse_event.row) {
                    if state.selected_input_field > 1 {
                        state.selected_input_field -= 1;
                    }
                } else if !state.popup_p2
                    && is_mouse_over_area(state.missing_fields_area_p2, mouse_event.column, mouse_event.row)
                {
                    if state.selected_missing_field > 1 {
                        state.selected_missing_field -= 1;
                    }
                }
                // else if state.hover_popup_value_p2 {
                //     if state.offset_value > 0 {
                //         state.offset_value -= 1;
                //     }
                // } else if state.hover_popup_result_path_p2 {
                //     if state.offset_result_path > 0 {
                //         state.offset_result_path -= 1;
                //     }
                // } else if state.hover_popup_result_value_p2 {
                //     if state.offset_result_value > 0 {
                //         state.offset_result_value -= 1;
                //     }
                // }
            }
            event::MouseEventKind::Up(_) => {
                if is_mouse_over_area(state.complete_button, mouse_event.column, mouse_event.row) {
                    state.complete = true; // todo
                    state.popup_p2 = false;
                } else if is_mouse_over_area(state.confirm_button, mouse_event.column, mouse_event.row) {
                    // todo state.confirm = true;
                    state.popup_p2 = false;
                } else if is_mouse_over_area(state.back_button, mouse_event.column, mouse_event.row) {
                    state.select_multiplicity = true;
                    state.selected_transformations.clear();
                } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
                    state.tab.prev();
                }
            }
            _ => {}
        }
    }

    Ok(false)
}
