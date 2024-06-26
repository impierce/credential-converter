use std::io::Write;

use crate::{
    backend::{
        preload_p2::get_missing_data_fields,
        repository::{construct_leaf_node, merge},
    },
    state::{AppState, Multiplicity, P2P3Tabs, Pages, Transformations},
    trace_dbg,
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};
use digital_credential_data_models::{elmv3::EuropassEdcCredential, obv3::AchievementCredential};

use super::is_mouse_over_area;

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.popup_uncompleted_warning {
                        state.popup_uncompleted_warning = false;
                    } else if state.popup_mapping_p2_p3 {
                        state.popup_mapping_p2_p3 = false;
                        state.popup_offset_path = 0;
                        state.popup_offset_value = 0;
                    } else {
                        return Ok(true);
                    }
                }
                Backspace => {
                    // Delete a selected transformation from the list of selected transformations
                    if state.popup_custom_mapping {
                        state.custom_mapping_path.pop();
                    }
                    else if state.selected_transformations_tab && !state.selected_transformations.is_empty() {
                        state.selected_transformations.remove(state.selected_transformation);
                        if state.selected_transformation > 0 {
                            state.selected_transformation -= 1;
                        }
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && state.multiplicity == Multiplicity::OneToMany
                    {
                        state.dividers.pop();
                    }
                }
                Tab => {
                    if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && !state.select_multiplicity
                        && state.multiplicity == Multiplicity::OneToOne
                    {
                        state.selected_transformations_tab = !state.selected_transformations_tab;
                    } else {
                        state.p2_p3_tabs.next();
                        state.selected_transformations_tab = false;
                    }
                }
                F(2) => {
                    state.p2_p3_tabs.prev();
                }
                Left => {
                    if state.p2_p3_tabs == P2P3Tabs::MappingOptions && state.select_multiplicity {
                        state.multiplicity.prev();
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && !state.select_multiplicity
                        && !state.selected_transformations_tab
                    {
                        state.transformations.prev();
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && state.selected_transformation > 0
                    {
                        state.selected_transformation -= 1;
                    } else if state.p2_p3_tabs == P2P3Tabs::OutputFields && !state.popup_mapping_p2_p3 {
                        state.p2_p3_tabs = P2P3Tabs::InputFields;
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Right => {
                    if state.p2_p3_tabs == P2P3Tabs::MappingOptions && state.select_multiplicity {
                        state.multiplicity.next();
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && !state.select_multiplicity
                        && !state.selected_transformations_tab
                    {
                        state.transformations.next();
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && !state.selected_transformations.is_empty()
                        && state.selected_transformation < state.selected_transformations.len() - 1
                    {
                        state.selected_transformation += 1;
                    } else if state.p2_p3_tabs == P2P3Tabs::InputFields && !state.popup_mapping_p2_p3 {
                        state.p2_p3_tabs = P2P3Tabs::OutputFields;
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Up => match state.p2_p3_tabs {
                    P2P3Tabs::InputFields => {
                        if state.selected_input_field > 1 {
                            state.selected_input_field -= 1;
                        }
                    }
                    P2P3Tabs::OutputFields => {
                        if state.selected_missing_field > 1 {
                            state.selected_missing_field -= 1;
                        }
                    }
                    P2P3Tabs::MappingOptions => {
                        state.p2_p3_tabs.next();
                    }
                },
                Down => match state.p2_p3_tabs {
                    P2P3Tabs::InputFields => {
                        if state.selected_input_field <= state.amount_input_fields {
                            state.selected_input_field += 1;
                        }
                    }
                    P2P3Tabs::OutputFields => {
                        if state.selected_missing_field <= state.amount_missing_fields {
                            state.selected_missing_field += 1;
                        }
                    }
                    _ => {}
                },
                Enter => {
                    if state.popup_custom_mapping {
                        state.popup_custom_mapping = false;
                    }
                    else if state.popup_uncompleted_warning {
                        state.popup_uncompleted_warning = false;
                        state.popup_mapping_p2_p3 = false;
                        state.selected_input_field = 1;
                        state.selected_transformation = 0;
                        state.selected_transformations_tab = false;
                        state.selected_transformations.clear();
                        state.select_multiplicity = true;
                        state.p2_p3_tabs = P2P3Tabs::InputFields;
                        state.page.next();
                    } else {
                        match state.p2_p3_tabs {
                            P2P3Tabs::MappingOptions => {
                                if state.select_multiplicity {
                                    state.select_multiplicity = false;
                                    if state.multiplicity == Multiplicity::OneToMany {
                                        state.popup_mapping_p2_p3 = true;
                                    }
                                } else if !state.selected_transformations_tab {
                                    state.selected_transformations.push(state.transformations);
                                } else if state.popup_mapping_p2_p3 {
                                    state.popup_mapping_p2_p3 = false;
                                    state.selected_transformations_tab = false;
                                    state.select_multiplicity = true;
                                    state.selected_transformations.clear();
                                    state.popup_offset_path = 0;
                                    state.popup_offset_value = 0;
                                    state.p2_p3_tabs = P2P3Tabs::InputFields;

                                    state.completed_input_fields.push(state.selected_input_field);
                                    state.completed_missing_fields.push(state.selected_missing_field);
                                    state.missing_data_fields[state.selected_missing_field].1 =
                                        state.candidate_data_value.clone().unwrap();
                                    trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                                    trace_dbg!(&state.missing_data_fields[state.selected_missing_field]);

                                    update_repository(state);
                                } else if state.selected_transformations_tab {
                                    state.popup_mapping_p2_p3 = true;
                                } else {
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
                Char(char) => {
                    if state.popup_custom_mapping {
                            state.custom_mapping_path.push(char);
                    }
                    else if state.popup_mapping_p2_p3 && state.multiplicity == Multiplicity::OneToMany {
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
                if !state.popup_mapping_p2_p3 {
                    if is_mouse_over_area(state.selector_area_p2_p3, mouse_event.column, mouse_event.row) {
                        if state.selected_input_field <= state.amount_input_fields {
                            state.selected_input_field += 1;
                        }
                    } else if is_mouse_over_area(state.output_fields_area_p2_p3, mouse_event.column, mouse_event.row) {
                        if state.selected_missing_field <= state.amount_missing_fields {
                            state.selected_missing_field += 1;
                        }
                    }
                } else {
                    if !state.select_multiplicity {
                        if is_mouse_over_area(state.popup_input_path_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_path < state.popup_amount_lines_path as u16 {
                                state.popup_offset_path += 1;
                            }
                        } else if is_mouse_over_area(state.popup_input_value_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_value < state.popup_amount_lines_value as u16 {
                                state.popup_offset_value += 1;
                            }
                        } else if is_mouse_over_area(state.popup_output_path_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_output_path < state.popup_amount_lines_output_path as u16 {
                                state.popup_offset_output_path += 1;
                            }
                        } else if is_mouse_over_area(state.popup_output_result_p2, mouse_event.column, mouse_event.row)
                        {
                            if state.popup_offset_result < state.popup_amount_lines_result as u16 {
                                state.popup_offset_result += 1;
                            }
                        }
                    } else if is_mouse_over_area(state.popup_path_area_p2, mouse_event.column, mouse_event.row) {
                        if state.popup_offset_path < state.popup_amount_lines_path as u16 {
                            state.popup_offset_path += 1;
                        }
                    } else if is_mouse_over_area(state.popup_value_area_p2, mouse_event.column, mouse_event.row) {
                        if state.popup_offset_value < state.popup_amount_lines_value as u16 {
                            state.popup_offset_value += 1;
                        }
                    }
                }
            }
            event::MouseEventKind::ScrollUp => {
                if !state.popup_mapping_p2_p3 {
                    if is_mouse_over_area(state.selector_area_p2_p3, mouse_event.column, mouse_event.row) {
                        if state.selected_input_field > 1 {
                            state.selected_input_field -= 1;
                        }
                    } else if is_mouse_over_area(state.output_fields_area_p2_p3, mouse_event.column, mouse_event.row) {
                        if state.selected_missing_field > 1 {
                            state.selected_missing_field -= 1;
                        }
                    }
                } else {
                    if !state.select_multiplicity {
                        if is_mouse_over_area(state.popup_input_path_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_path > 0 {
                                state.popup_offset_path -= 1;
                            }
                        } else if is_mouse_over_area(state.popup_input_value_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_value > 0 {
                                state.popup_offset_value -= 1;
                            }
                        } else if is_mouse_over_area(state.popup_output_path_p2, mouse_event.column, mouse_event.row) {
                            if state.popup_offset_output_path > 0 {
                                state.popup_offset_output_path -= 1;
                            }
                        } else if is_mouse_over_area(state.popup_output_result_p2, mouse_event.column, mouse_event.row)
                        {
                            if state.popup_offset_result > 0 {
                                state.popup_offset_result -= 1;
                            }
                        }
                    } else if is_mouse_over_area(state.popup_path_area_p2, mouse_event.column, mouse_event.row) {
                        if state.popup_offset_path > 0 {
                            state.popup_offset_path -= 1;
                        }
                    } else if is_mouse_over_area(state.popup_value_area_p2, mouse_event.column, mouse_event.row) {
                        if state.popup_offset_value > 0 {
                            state.popup_offset_value -= 1;
                        }
                    }
                }
            }
            event::MouseEventKind::Up(_) => {
                if is_mouse_over_area(state.complete_button, mouse_event.column, mouse_event.row) {
                    if state.missing_data_fields.len() - 1 == state.completed_missing_fields.len() {
                        state.popup_mapping_p2_p3 = false;
                        state.transformations = Transformations::LowerCase;
                        state.selected_transformations_tab = false;
                        state.select_multiplicity = true;
                        state.selected_transformation = 0;
                        state.selected_transformations.clear();
                        state.page.next();
                    } else {
                        state.popup_uncompleted_warning = true;
                    }
                } else if is_mouse_over_area(state.review_button, mouse_event.column, mouse_event.row) {
                    state.review = true;
                    state.popup_mapping_p2_p3 = true;
                } else if is_mouse_over_area(state.abort_button, mouse_event.column, mouse_event.row) {
                    state.transformations = Transformations::LowerCase;
                    state.selected_transformations_tab = false;
                    state.select_multiplicity = true;
                    state.selected_transformation = 0;
                    state.selected_transformations.clear();
                } else if is_mouse_over_area(state.confirm_button, mouse_event.column, mouse_event.row) {
                    state.popup_mapping_p2_p3 = false;
                    state.selected_transformations_tab = false;
                    state.select_multiplicity = true;
                    state.selected_transformations.clear();
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                    state.p2_p3_tabs = P2P3Tabs::InputFields;

                    state.completed_input_fields.push(state.selected_input_field);
                    state.completed_missing_fields.push(state.selected_missing_field);
                    state.missing_data_fields[state.selected_missing_field].1 =
                        state.candidate_data_value.clone().unwrap();
                    trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                    trace_dbg!(state.missing_data_fields.clone()[state.selected_missing_field].to_owned());
                } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
                    if state.popup_mapping_p2_p3 {
                        state.popup_mapping_p2_p3 = false;
                    } else if state.popup_uncompleted_warning {
                        state.popup_uncompleted_warning = false;
                    } else {
                        state.select_multiplicity = true;
                        state.selected_transformation = 0;
                        state.selected_transformations.clear();
                        state.transformations = Transformations::LowerCase;

                        state.page.prev();
                    }
                } else if !is_mouse_over_area(state.popup_path_area_p2, mouse_event.column, mouse_event.row)
                    && !is_mouse_over_area(state.popup_value_area_p2, mouse_event.column, mouse_event.row)
                {
                    state.popup_mapping_p2_p3 = false;
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
                    state.page.prev();
                } else if !is_mouse_over_area(state.popup_path_area_p2, mouse_event.column, mouse_event.row)
                    && !is_mouse_over_area(state.popup_value_area_p2, mouse_event.column, mouse_event.row)
                {
                    state.popup_mapping_p2_p3 = false;
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                }
            }
            _ => {}
        }
    }

    Ok(false)
}

pub fn update_repository(state: &mut AppState) {
    let output_format = state.mapping.output_format();

    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();

    let source_value = state.candidate_data_value.clone().unwrap();

    trace_dbg!(state.selected_missing_field);
    if state.selected_missing_field == 0 {
        return;
    }

    let pointer = state.missing_data_fields[state.selected_missing_field].0.clone();
    trace_dbg!(&pointer);

    let mut json_value = state.repository.get_mut(&output_format).unwrap();

    let mut leaf_node = construct_leaf_node(&pointer);

    leaf_node
        .pointer_mut(&pointer)
        .map(|value| *value = serde_json::from_str(&source_value).unwrap());
    trace_dbg!(&leaf_node);

    merge(&mut json_value, leaf_node);
    trace_dbg!(json_value);
}
