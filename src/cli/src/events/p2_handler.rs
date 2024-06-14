use std::io::Write;

use crate::{
    backend::{
        preload_p2::verify,
        repository::{construct_leaf_node, merge},
        selector::selector,
    },
    elm::ELM,
    obv3::OBv3,
    state::{AppState, Multiplicity},
    trace_dbg,
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

use super::is_mouse_over_area;

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.popup_mapper_p2 {
                        state.popup_mapper_p2 = false;
                        state.popup_selected_transformations = false;
                    } else {
                        return Ok(true);
                    }
                }
                Backspace => {
                    if state.popup_selected_transformations && !state.selected_transformations.is_empty() {
                        state.selected_transformations.remove(state.selected_transformation);
                        if state.selected_transformation > 0 {
                            state.selected_transformation -= 1;
                        }
                    } else if state.popup_mapper_p2 && state.multiplicity == Multiplicity::OneToMany {
                        state.dividers.pop();
                    }
                }
                Tab => {
                    if state.popup_mapper_p2 && !state.select_multiplicity {
                        state.popup_selected_transformations = !state.popup_selected_transformations;
                    } else if !state.popup_mapper_p2 {
                        state.tab.next();
                    }
                }
                F(2) => {
                    if state.popup_mapper_p2 {
                        state.select_multiplicity = true;
                    } else {
                        state.tab.prev();
                    }
                }
                Left => {
                    if state.popup_mapper_p2 && state.select_multiplicity {
                        state.multiplicity.prev();
                    } else if state.popup_mapper_p2
                        && !state.select_multiplicity
                        && !state.popup_selected_transformations
                    {
                        state.transformations.prev();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    } else if state.popup_mapper_p2
                        && state.popup_selected_transformations
                        && state.selected_transformation > 0
                    {
                        state.selected_transformation -= 1;
                        trace_dbg!(state.selected_transformation);
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Right => {
                    if state.popup_mapper_p2 && state.select_multiplicity {
                        state.multiplicity.next();
                    } else if state.popup_mapper_p2
                        && !state.select_multiplicity
                        && !state.popup_selected_transformations
                    {
                        state.transformations.next();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    } else if state.popup_mapper_p2
                        && state.popup_selected_transformations
                        && !state.selected_transformations.is_empty()
                        && state.selected_transformation < state.selected_transformations.len() - 1
                    {
                        state.selected_transformation += 1;
                        trace_dbg!(state.selected_transformation);
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Up => {
                    if state.selected_input_field > 1 {
                        state.selected_input_field -= 1;
                    }
                    selector(state);
                    trace_dbg!(&state.candidate_data_value);
                }
                Down => {
                    if state.selected_input_field <= state.amount_input_fields {
                        state.selected_input_field += 1;
                    }
                    selector(state);
                    trace_dbg!(&state.candidate_data_value);
                }
                Enter => {
                    if !state.popup_mapper_p2 {
                        state.popup_mapper_p2 = true;
                        state.select_multiplicity = true;
                    } else if state.popup_mapper_p2 && state.select_multiplicity {
                        state.select_multiplicity = false;
                    } else if !state.popup_selected_transformations {
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
                Char(char) => {
                    if state.popup_mapper_p2 && state.multiplicity == Multiplicity::OneToMany {
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
                state.hover_popup_p2 = is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_area_p2);
                state.hover_selector_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.selector_area_p2);
                state.hover_popup_value_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_value_area_p2);
                state.hover_popup_result_path_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_result_path_p2);
                state.hover_popup_result_value_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_result_value_p2);

                //trace_dbg!(state.hover_popup_result_p2);

                if state.hover_popup_value_p2 {
                    if state.offset_value < state.amount_input_fields as u16 {
                        state.offset_value += 1;
                    }
                } else if state.hover_popup_result_path_p2 {
                    if state.offset_result_path < state.amount_input_fields as u16 {
                        state.offset_result_path += 1;
                    }
                } else if state.hover_popup_result_value_p2 {
                    if state.offset_result_value < state.amount_input_fields as u16 {
                        // todo
                        state.offset_result_value += 1;
                    }
                } else if state.hover_selector_p2 && !state.popup_mapper_p2 {
                    if state.selected_input_field <= state.amount_input_fields {
                        state.selected_input_field += 1;
                    }
                }
            }
            event::MouseEventKind::ScrollUp => {
                // repeated because I don't what these calculations to take place at every mouse event in general.
                state.hover_popup_p2 = is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_area_p2);
                state.hover_selector_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.selector_area_p2);
                state.hover_popup_value_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_value_area_p2);
                state.hover_popup_result_path_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_result_path_p2);
                state.hover_popup_result_value_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_result_value_p2);

                if state.hover_popup_value_p2 {
                    if state.offset_value > 0 {
                        state.offset_value -= 1;
                    }
                } else if state.hover_popup_result_path_p2 {
                    if state.offset_result_path > 0 {
                        state.offset_result_path -= 1;
                    }
                } else if state.hover_popup_result_value_p2 {
                    if state.offset_result_value > 0 {
                        state.offset_result_value -= 1;
                    }
                } else if state.hover_selector_p2 && !state.popup_mapper_p2 {
                    if state.selected_input_field > 1 {
                        state.selected_input_field -= 1;
                    }
                }
            }
            event::MouseEventKind::Up(_) => {
                state.hover_finish_popup_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.finish_area_popup_p2);
                state.hover_prev_page = is_mouse_over_area(mouse_event.column, mouse_event.row, state.prev_page_button);
                if state.hover_finish_popup_p2 {
                    state.finish_mapping = true;
                    state.popup_mapper_p2 = false;
                } else if state.hover_prev_page {
                    state.tab.prev();
                }
            }
            _ => {}
        }
    }

    Ok(false)
}
