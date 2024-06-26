use std::io::Write;

use crate::{
    backend::{
        preload_p2::verify,
        repository::{construct_leaf_node, merge},
        selector::selector,
    },
    elm::ELM,
    obv3::OBv3,
    state::{AppState, Multiplicity, P2P3Tabs, Transformations},
    trace_dbg,
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

use super::is_mouse_over_area;

pub fn p3_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.popup_unused_data {
                        state.popup_unused_data = false;
                    } else if state.popup_mapping_p2_p3 {
                        state.popup_mapping_p2_p3 = false;
                        state.popup_offset_path = 0;
                        state.popup_offset_value = 0;
                    } else {
                        return Ok(true);
                    }
                }
                Backspace => {
                    if state.popup_unused_data {
                        state.unused_data_path.pop();
                    } else if state.selected_transformations_tab && !state.selected_transformations.is_empty() {
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
                        // selector(state);
                        // trace_dbg!(&state.candidate_data_value);
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && state.selected_transformation > 0
                    {
                        state.selected_transformation -= 1;
                        // trace_dbg!(state.selected_transformation);
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
                        // selector(state);
                        // trace_dbg!(&state.candidate_data_value);
                    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
                        && state.selected_transformations_tab
                        && !state.selected_transformations.is_empty()
                        && state.selected_transformation < state.selected_transformations.len() - 1
                    {
                        state.selected_transformation += 1;
                        // trace_dbg!(state.selected_transformation);
                    } else if state.p2_p3_tabs == P2P3Tabs::InputFields && !state.popup_mapping_p2_p3 {
                        state.p2_p3_tabs = P2P3Tabs::OutputFields;
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Up => match state.p2_p3_tabs {
                    P2P3Tabs::InputFields => {
                        if state.selected_input_field > 1 {
                            state.selected_input_field -= 1;
                            // selector(state);
                            // trace_dbg!(&state.candidate_data_value);
                        }
                    }
                    P2P3Tabs::OutputFields => {
                        if state.selected_optional_field > 1 {
                            state.selected_optional_field -= 1;
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
                            // selector(state);
                            // trace_dbg!(&state.candidate_data_value);
                        }
                    }
                    P2P3Tabs::OutputFields => {
                        if state.selected_optional_field <= state.amount_optional_fields {
                            state.selected_optional_field += 1;
                        }
                    }
                    _ => {}
                },
                Enter => {
                    if state.popup_unused_data {
                        state.page.next(); // make a "finished" tab, signal finish to backend
                        trace_dbg!(state.page);
                        trace_dbg!("hiero");
                    }
                    match state.p2_p3_tabs {
                        P2P3Tabs::MappingOptions => {
                            if state.select_multiplicity {
                                state.select_multiplicity = false;
                                if state.multiplicity == Multiplicity::OneToMany {
                                    state.popup_mapping_p2_p3 = true;
                                }
                            } else if !state.selected_transformations_tab {
                                state.selected_transformations.push(state.transformations);
                            } else if state.selected_transformations_tab {
                                state.popup_mapping_p2_p3 = true;
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
                                    //state.tab.next();
                                }

                                // if state.missing_data_field.is_none() {
                                //     let mut file = std::fs::File::create(&state.output_path).unwrap();
                                //     file.write_all(temp.as_bytes()).unwrap();
                                //     state.tab.next();
                                // }
                            }
                        }
                        _ => {
                            if !state.popup_mapping_p2_p3 {
                                // todo
                                state.popup_mapping_p2_p3 = true;
                            } else {
                                state.popup_mapping_p2_p3 = false;
                                state.p2_p3_tabs.next();
                            }
                        }
                    }
                }
                Char(char) => {
                    if state.popup_unused_data {
                        state.unused_data_path.push(char);
                    } else if state.popup_mapping_p2_p3 && state.multiplicity == Multiplicity::OneToMany {
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
                        if state.selected_optional_field <= state.amount_optional_fields {
                            state.selected_optional_field += 1;
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
                        if state.selected_optional_field > 1 {
                            state.selected_optional_field -= 1;
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
                    if state.input_fields.len() == state.completed_input_fields.len() {
                        state.popup_mapping_p2_p3 = false;
                        state.page.next();
                        trace_dbg!(state.page);
                    } else {
                        state.popup_unused_data = true;
                    }
                } else if is_mouse_over_area(state.review_button, mouse_event.column, mouse_event.row) {
                    state.review = true;
                    state.popup_mapping_p2_p3 = true;
                } else if is_mouse_over_area(state.abort_button, mouse_event.column, mouse_event.row) {
                    state.transformations = Transformations::Copy;
                    state.selected_transformations_tab = false;
                    state.select_multiplicity = true;
                    state.selected_transformation = 0;
                    state.selected_transformations.clear();
                } else if is_mouse_over_area(state.confirm_button, mouse_event.column, mouse_event.row) {
                    state.popup_mapping_p2_p3 = false;
                    state.select_multiplicity = true;
                    state.selected_transformations.clear();
                    state.popup_offset_path = 0;
                    state.popup_offset_value = 0;
                    state.p2_p3_tabs = P2P3Tabs::InputFields;

                    state.completed_input_fields.push(state.selected_input_field);
                    state.completed_optional_fields.push(state.selected_optional_field);
                    state.optional_fields[state.selected_optional_field].1 =
                        state.candidate_data_value.clone().unwrap();
                    trace_dbg!(state.candidate_data_value.as_ref().unwrap());
                    trace_dbg!(state.optional_fields.clone()[state.selected_optional_field].to_owned());
                } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
                    if state.popup_mapping_p2_p3 {
                        state.popup_mapping_p2_p3 = false;
                    } else if state.popup_unused_data {
                        state.popup_unused_data = false;
                    } else {
                        state.select_multiplicity = true;
                        state.selected_transformation = 0;
                        state.selected_transformations.clear();
                        state.transformations = Transformations::Copy;

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
