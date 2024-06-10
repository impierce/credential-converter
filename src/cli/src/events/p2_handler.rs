use std::io::Write;

use crate::{
    elm::ELM,
    jsonpointer::{JsonPath, JsonPointer},
    obv3::OBv3,
    p1_handler::verify,
    repository::{self, construct_leaf_node, merge, Repository},
    trace_dbg,
    transformations::{DataLocation, OneToOne, Transformation},
    utils::{AppState, Mapping, Transformations},
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

pub fn selector(state: &mut AppState) {
    let transformation = Transformations::from_repr(state.selected_transformation).unwrap();
    trace_dbg!(transformation);

    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

    let (source_pointer, source_value) = state.input_fields[state.selected_input_field].clone();

    let pointer = state.missing_data_field.as_ref().unwrap().clone();
    let destination_path: JsonPath = JsonPointer(pointer.clone()).into();

    let mut temp_repository = Repository::from(state.repository.clone());

    let transformation = match transformation {
        Transformations::LowerCase => Transformation::OneToOne {
            type_: OneToOne::toLowerCase,
            source: DataLocation {
                format: input_format.clone(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: output_format.clone(),
                path: destination_path.to_string(),
            },
        },
        Transformations::UpperCase => Transformation::OneToOne {
            type_: OneToOne::toUpperCase,
            source: DataLocation {
                format: input_format.clone(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: output_format.clone(),
                path: destination_path.to_string(),
            },
        },
        Transformations::Copy | _ => Transformation::OneToOne {
            type_: OneToOne::copy,
            source: DataLocation {
                format: input_format.clone(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: output_format.clone(),
                path: destination_path.to_string(),
            },
        },
    };

    temp_repository.apply_transformation(transformation);

    let candidate_data_value = temp_repository.get(&output_format).unwrap().pointer(&pointer).unwrap();

    state.candidate_data_value = Some(candidate_data_value.to_string());
}

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.popup_mapper_p2 {
                        state.popup_mapper_p2 = false;
                        state.popup_selected_transformations = false;
                    } 
                    else {
                        return Ok(true);
                    }
                }
                Backspace => {
                    if state.popup_selected_transformations {
                        state.selected_transformations.remove(state.selected_transformation);
                        // state.selected_transformations.pop(); // todo
                    } 
                }
                Tab => {
                    if state.popup_mapper_p2 {
                        state.popup_selected_transformations = !state.popup_selected_transformations;
                    } else {
                        state.tab.next();
                    }
                }
                F(2) => {
                    state.tab.prev();
                }
                Left => {
                    if state.popup_mapper_p2 && !state.popup_selected_transformations {
                        state.transformations.prev();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    }
                    else if state.popup_mapper_p2 && state.popup_selected_transformations && state.selected_transformation > 0 {
                        trace_dbg!(state.selected_transformation);
                        state.selected_transformation -= 1;
                    }
                    // let (_, source_value) = state.input_fields[state.selected_input_field].clone();
                }
                Right => {
                    if state.popup_mapper_p2 && !state.popup_selected_transformations {
                        state.transformations.next();
                        selector(state);
                        trace_dbg!(&state.candidate_data_value);
                    }
                    else if state.popup_mapper_p2 && state.popup_selected_transformations && state.selected_transformation < state.selected_transformations.len() -1 {
                        trace_dbg!(state.selected_transformation);
                        state.selected_transformation += 1;
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
                    if !state.map_input_field {
                        state.map_input_field = true;
                    } 
                    else if !state.popup_selected_transformations {
                        state.selected_transformations.push(state.transformations);
                    }
                    else {
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
            _ => {}
        }
    }

    Ok(false)
}

//////////     HELPERS     //////////

fn is_mouse_over_area(mouse_x: u16, mouse_y: u16, area: ratatui::layout::Rect) -> bool {
    mouse_x >= area.x && mouse_x < area.x + area.width && mouse_y >= area.y && mouse_y < area.y + area.height
}
