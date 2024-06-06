use std::io::Write;

use crate::{
    jsonpointer::{JsonPath, JsonPointer},
    p1_handler::verify,
    repository::{self, construct_leaf_node, merge, Repository},
    trace_dbg,
    transformations::{DataLocation, OneToOne, Transformation},
    utils::{AppState, Mapping, Transformations},
};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};

fn selector(state: &mut AppState) {
    trace_dbg!(state.transformation);

    let (input_format, output_format) = match state.mapping {
        Mapping::OBv3ToELM => ("OBv3", "ELM"),
        Mapping::ELMToOBv3 => ("ELM", "OBv3"),
    };

    let (source_pointer, source_value) = state.input_fields[state.selected_input_field].clone();

    let pointer = state.missing_data_field.as_ref().unwrap().clone();
    let destination_path: JsonPath = JsonPointer(pointer.clone()).into();

    let mut temp_repository = Repository::from(state.repository.clone());

    let transformation = match state.transformation {
        Transformations::LowerCase => Transformation::OneToOne {
            type_: OneToOne::toLowerCase,
            source: DataLocation {
                format: "ELM".to_string(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: "OBv3".to_string(),
                path: destination_path.to_string(),
            },
        },
        Transformations::UpperCase => Transformation::OneToOne {
            type_: OneToOne::toUpperCase,
            source: DataLocation {
                format: "ELM".to_string(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: "OBv3".to_string(),
                path: destination_path.to_string(),
            },
        },
        Transformations::Copy | _ => Transformation::OneToOne {
            type_: OneToOne::copy,
            source: DataLocation {
                format: "ELM".to_string(),
                path: JsonPath::try_from(JsonPointer(source_pointer)).unwrap().to_string(),
            },
            destination: DataLocation {
                format: "OBv3".to_string(),
                path: destination_path.to_string(),
            },
        },
    };

    temp_repository.apply_transformation(transformation);

    let candidate_data_value = temp_repository.get(output_format).unwrap().pointer(&pointer).unwrap();

    state.candidate_data_value = Some(candidate_data_value.to_string());

    // let pointer = state.missing_data_field.as_ref().unwrap().clone();

    // let mut json_value = state.repository.get_mut(output_format).unwrap();

    // let mut leaf_node = construct_leaf_node(&pointer);

    // leaf_node
    //     .pointer_mut(&pointer)
    //     .map(|value| *value = serde_json::from_str(&source_value).unwrap());

    // merge(&mut json_value, leaf_node);

    // let temp = serde_json::to_string_pretty(&json_value).unwrap();

    // state.missing_data_field = verify(&mut json_value).err();
}

pub fn p2_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => return Ok(true),
                Tab => {
                    state.tab.next();
                }
                F(2) => {
                    state.tab.prev();
                }
                Left => {
                    state.transformation.prev();
                    selector(state);
                    trace_dbg!(&state.candidate_data_value);
                }
                Right => {
                    state.transformation.next();
                    selector(state);
                    trace_dbg!(&state.candidate_data_value);
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
                    } else {
                        // trace_dbg!(state.transformation);

                        let (input_format, output_format) = match state.mapping {
                            Mapping::OBv3ToELM => ("OBv3", "ELM"),
                            Mapping::ELMToOBv3 => ("ELM", "OBv3"),
                        };

                        // let (_, source_value) = state.input_fields[state.selected_input_field].clone();

                        let source_value = state.candidate_data_value.clone().unwrap();

                        let pointer = state.missing_data_field.as_ref().unwrap().clone();

                        let mut json_value = state.repository.get_mut(output_format).unwrap();

                        let mut leaf_node = construct_leaf_node(&pointer);

                        leaf_node
                            .pointer_mut(&pointer)
                            .map(|value| *value = serde_json::from_str(&source_value).unwrap());

                        merge(&mut json_value, leaf_node);

                        let temp = serde_json::to_string_pretty(&json_value).unwrap();

                        state.missing_data_field = verify(&mut json_value).err();

                        if state.missing_data_field.is_none() {
                            let mut file = std::fs::File::create(&state.output_path).unwrap();
                            file.write_all(temp.as_bytes()).unwrap();
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
            event::MouseEventKind::Moved => {
                state.hover_popup_p2 = is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_area_p2);
                state.hover_selector_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.selector_area_p2);
                state.hover_popup_value_p2 =
                    is_mouse_over_area(mouse_event.column, mouse_event.row, state.popup_value_p2);
            }
            event::MouseEventKind::ScrollDown => {
                if state.hover_popup_value_p2 {
                    if state.offset_value < state.amount_input_fields as u16 {
                        state.offset_value += 1;
                    }
                } else if state.hover_selector_p2 && !state.hover_popup_p2 {
                    if state.selected_input_field <= state.amount_input_fields {
                        state.selected_input_field += 1;
                    }
                }
            }
            event::MouseEventKind::ScrollUp => {
                if state.hover_popup_value_p2 {
                    if state.offset_value > 0 {
                        state.offset_value -= 1;
                    }
                } else if state.hover_selector_p2 && !state.hover_popup_p2 {
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
