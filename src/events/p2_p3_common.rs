use crossterm::event::MouseEvent;
use serde_json::Value;
use std::io::Write;
use std::path::Path;
use std::{char, fs::File};

use super::{is_mouse_over_area, p2_handler::clear_progress};
use crate::backend::preload_p2::get_json;
use crate::{
    backend::{
        candidate_value::set_candidate_output_value,
        jsonpointer::{JsonPath, JsonPointer},
        preload_p2::{update_display_section, update_pointer},
        repository::update_repository,
        transformations::Transformation,
    },
    state::{AppState, MappingOptions, P2P3Tabs, Pages, Transformations},
    trace_dbg,
};

//////////     KEYBOARD EVENTS     //////////

pub fn handle_esc(state: &mut AppState) {
    if state.uncompleted_warning && state.page == Pages::RequiredDataP2 {
        state.uncompleted_warning = false;
    } else if state.popup_mapping_p2_p3 {
        clear_popup(state);
    } else if state.p2_p3_tabs == P2P3Tabs::MappingOptions && !state.select_mapping_option {
        clear_mapping_options(state);
    } else if state.return_to_p1_warning {
        state.return_to_p1_warning = false;
    } else if state.p2_p3_tabs == P2P3Tabs::OutputFields {
        update_pointer(state, false);
        update_display_section(state, false);
    } else {
        state.exit_warning = !state.exit_warning;
    }
}

pub fn handle_backspace(state: &mut AppState) {
    // Delete a selected transformation from the list of selected transformations
    if state.selected_transformations_tab && !state.selected_transformations.is_empty() {
        state.selected_transformations.remove(state.selected_transformation);
        if state.selected_transformation > 0 {
            state.selected_transformation -= 1;
        }
    }
    // Delete a character from the dividers
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions && state.mapping_option == MappingOptions::OneToMany {
        state.dividers.pop();
    }
}

pub fn handle_tab(state: &mut AppState) {
    // Check if inside Transformations bar in the transformations tab and switch to the selected transformations tab
    if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && !state.select_mapping_option
        && state.mapping_option == MappingOptions::Transformations
        && !state.selected_transformations_tab
    {
        state.selected_transformations_tab = true;
    }
    // Check if inside Transformation bar in the selected transformations tab and switch to button
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && !state.select_mapping_option
        && state.mapping_option == MappingOptions::Transformations
        && state.selected_transformations_tab
    {
        state.p2_p3_tabs.next();
    }
    // Check if right tab of Transformations bar is still selected and switch to the left when tabbing to the bar from output_fields
    else if state.p2_p3_tabs == P2P3Tabs::OutputFields && state.selected_transformations_tab {
        state.selected_transformations_tab = false;
        state.p2_p3_tabs.next();
    } else {
        state.p2_p3_tabs.next();
        //state.selected_transformations_tab = false;
    }
}

pub fn handle_f2(state: &mut AppState) {
    // Check if inside Transformations bar on the selected_transformations tab and switch to the transformations tab
    if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && !state.select_mapping_option
        && state.mapping_option == MappingOptions::Transformations
        && state.selected_transformations_tab
    {
        state.selected_transformations_tab = false;
    } else {
        state.p2_p3_tabs.prev();
    }
}

pub fn handle_up(state: &mut AppState) {
    match state.p2_p3_tabs {
        // Scroll up through input fields
        P2P3Tabs::InputFields => {
            if state.selected_input_field > 1 {
                state.selected_input_field -= 1;
            }
        }
        // Scroll up through output fields
        P2P3Tabs::OutputFields => {
            if state.selected_output_field > 1 {
                state.selected_output_field -= 1;
            }
        }
        // Move between tabs
        P2P3Tabs::MappingOptions => {
            state.p2_p3_tabs = P2P3Tabs::InputFields;
        }
        P2P3Tabs::Clear => {
            state.p2_p3_tabs = P2P3Tabs::OutputFields;
        }
        P2P3Tabs::View => {
            state.p2_p3_tabs = P2P3Tabs::OutputFields;
        }
    }
}

pub fn handle_down(state: &mut AppState) {
    match state.p2_p3_tabs {
        // Scroll down through input or output fields
        P2P3Tabs::InputFields => {
            if state.selected_input_field <= state.amount_input_fields {
                state.selected_input_field += 1;
            }
        }
        P2P3Tabs::OutputFields => {
            if state.selected_output_field <= state.amount_output_fields {
                state.selected_output_field += 1;
            }
        }
        _ => {}
    }
}
pub fn handle_left(state: &mut AppState) {
    // Move through mapping options bar, loops.
    if state.p2_p3_tabs == P2P3Tabs::MappingOptions && state.select_mapping_option {
        state.mapping_option.prev();
    }
    // Move through transformation bar, loops
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && !state.select_mapping_option
        && !state.selected_transformations_tab
    {
        state.transformations.prev();
    }
    // Move through selected transformations bar, loops
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions {
        if state.selected_transformation > 0 {
            state.selected_transformation -= 1;
        }
        // to stop it from crashing when the selected_transformation is empty.
        else if !state.selected_transformations.is_empty() {
            state.selected_transformation = state.selected_transformations.len() - 1;
        }
    }
    // move between input tab and output tab
    else if state.p2_p3_tabs == P2P3Tabs::OutputFields && !state.popup_mapping_p2_p3 {
        state.p2_p3_tabs = P2P3Tabs::InputFields;
    }
}

pub fn handle_right(state: &mut AppState) {
    // Move through mapping options bar, loops.
    if state.p2_p3_tabs == P2P3Tabs::MappingOptions && state.select_mapping_option {
        state.mapping_option.next();
    }
    // Move through transformation bar, loops
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && !state.select_mapping_option
        && !state.selected_transformations_tab
    {
        state.transformations.next();
    }
    // Move through selected transformations bar, loops
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && state.selected_transformations_tab
        && !state.selected_transformations.is_empty()
    {
        if state.selected_transformation < state.selected_transformations.len() - 1 {
            state.selected_transformation += 1;
        } else {
            state.selected_transformation = 0;
        }
    }
    // Move between input tab and output tab
    else if state.p2_p3_tabs == P2P3Tabs::InputFields && !state.popup_mapping_p2_p3 {
        state.p2_p3_tabs = P2P3Tabs::OutputFields;
    }
}

pub fn handle_char(state: &mut AppState, char: char) {
    if state.popup_mapping_p2_p3 && state.mapping_option == MappingOptions::OneToMany {
        state.dividers.push(char);
    }
}

pub fn handle_enter(state: &mut AppState) -> bool {
    // Close warning, reset values and move to next page
    if state.uncompleted_warning && state.page == Pages::RequiredDataP2 {
        state.uncompleted_warning = false;
        next_page(state);
        update_display_section(state, true); // feels a bit off being outside of the next_p fn, but also inside the next_p fn it feels off.
    } else if state.exit_warning {
        return true;
    } else if state.return_to_p1_warning {
        clear_mapping_options(state);
        clear_popup(state);
        clear_progress(state);
        state.return_to_p1_warning = false;

        state.page.prev();
    } else {
        match state.p2_p3_tabs {
            P2P3Tabs::MappingOptions => {
                if state.select_mapping_option {
                    // Fast-track mapping, Copy to output result value and reset values
                    if state.mapping_option == MappingOptions::DirectCopy {
                        // "Your input >>" needs to be selected to perform the mapping
                        if state.output_display_subset[1].0 == *"Your input >>" && state.selected_output_field == 1 {
                            confirm_mapping(state);
                        }
                    }
                    // Switch from mapping options tab to respective tab
                    else {
                        state.select_mapping_option = false;
                    }
                }
                // Select a transformation if it hasn't already been selected
                else if state.mapping_option == MappingOptions::Transformations
                    && !state.selected_transformations_tab
                    && !state.selected_transformations.contains(&state.transformations)
                {
                    state.selected_transformations.push(state.transformations);
                }
                // If transformation(s) selected open the view popup to show the result.
                else if state.selected_transformations_tab {
                    if !state.popup_mapping_p2_p3 {
                        set_candidate_output_value(state, false);
                        state.popup_mapping_p2_p3 = true;
                    } else if state.output_display_subset[1].0 == *"Your input >>" && state.selected_output_field == 1 {
                        confirm_mapping(state);
                    }
                }
            }
            P2P3Tabs::Clear => {
                clear_button(state);
            }
            P2P3Tabs::View => {
                if !state.popup_mapping_p2_p3 {
                    set_candidate_output_value(state, false);
                    state.popup_mapping_p2_p3 = true;
                } else if state.output_display_subset[1].0 == *"Your input >>" && state.selected_output_field == 1 {
                    confirm_mapping(state);
                }
            }
            _ => {
                // Complete a mapping from the view popup
                if state.popup_mapping_p2_p3 {
                    // "Your input >>" needs to be selected to perform the mapping
                    if state.output_display_subset[1].0 == *"Your input >>" && state.selected_output_field == 1 {
                        confirm_mapping(state);
                    }
                } else {
                    update_pointer(state, true);
                    update_display_section(state, false);
                    if state.p2_p3_tabs == P2P3Tabs::InputFields { // todo: tmp fix until input fields also implemented similar to output fields.
                        state.p2_p3_tabs.next();
                    } 
                }
            }
        }
    }
    false
}

//////////     MOUSE EVENTS     //////////

pub fn handle_scroll_down(state: &mut AppState, mouse_event: MouseEvent) {
    // Scroll through input/output fields
    if !state.popup_mapping_p2_p3 {
        if is_mouse_over_area(state.selector_area_p2_p3, mouse_event.column, mouse_event.row)
            && state.selected_input_field <= state.amount_input_fields
        {
            state.selected_input_field += 1;
        } else if is_mouse_over_area(state.output_fields_area_p2_p3, mouse_event.column, mouse_event.row)
            && state.selected_output_field <= state.amount_output_fields
        {
            state.selected_output_field += 1;
        }
    }
    // Scroll within tabs of the view popup
    else if is_mouse_over_area(state.popup_path_area, mouse_event.column, mouse_event.row)
        && state.popup_offset_path < state.popup_amount_lines_path as u16
    {
        state.popup_offset_path += 1;
    } else if is_mouse_over_area(state.popup_value_area, mouse_event.column, mouse_event.row)
        && state.popup_offset_value < state.popup_amount_lines_value as u16
    {
        state.popup_offset_value += 1;
    } else if is_mouse_over_area(state.popup_output_path, mouse_event.column, mouse_event.row)
        && state.popup_offset_output_path < state.popup_amount_lines_output_path as u16
    {
        state.popup_offset_output_path += 1;
    } else if is_mouse_over_area(state.popup_output_result, mouse_event.column, mouse_event.row)
        && state.popup_offset_result < state.popup_amount_lines_result as u16
    {
        state.popup_offset_result += 1;
    }
}

pub fn handle_scroll_up(state: &mut AppState, mouse_event: MouseEvent) {
    // Scroll through input/output fields
    if !state.popup_mapping_p2_p3 {
        if is_mouse_over_area(state.selector_area_p2_p3, mouse_event.column, mouse_event.row)
            && state.selected_input_field > 1
        {
            state.selected_input_field -= 1;
        } else if is_mouse_over_area(state.output_fields_area_p2_p3, mouse_event.column, mouse_event.row)
            && state.selected_output_field > 1
        {
            state.selected_output_field -= 1;
        }
    }
    // Scroll within tabs of the view popup
    else if is_mouse_over_area(state.popup_path_area, mouse_event.column, mouse_event.row)
        && state.popup_offset_path > 0
    {
        state.popup_offset_path -= 1;
    } else if is_mouse_over_area(state.popup_value_area, mouse_event.column, mouse_event.row)
        && state.popup_offset_value > 0
    {
        state.popup_offset_value -= 1;
    } else if is_mouse_over_area(state.popup_output_path, mouse_event.column, mouse_event.row)
        && state.popup_offset_output_path > 0
    {
        state.popup_offset_output_path -= 1;
    } else if is_mouse_over_area(state.popup_output_result, mouse_event.column, mouse_event.row)
        && state.popup_offset_result > 0
    {
        state.popup_offset_result -= 1;
    }
}

pub fn handle_mouse_up(state: &mut AppState, mouse_event: MouseEvent) {
    if is_mouse_over_area(state.complete_button, mouse_event.column, mouse_event.row) {
        if state.output_display_subset.len() - 1 == state.completed_required_fields.len() {
            next_page(state);
            update_display_section(state, true); // feels a bit off being outside of the next_p fn, but also inside the next_p fn it feels off.
        } else if state.page == Pages::OptionalDataP3 {
            next_page(state);
            create_output_files(state);
        } else {
            state.uncompleted_warning = true;
        }
    } else if is_mouse_over_area(state.view_button, mouse_event.column, mouse_event.row) {
        set_candidate_output_value(state, false);
        state.popup_mapping_p2_p3 = true;
    } else if is_mouse_over_area(state.clear_button, mouse_event.column, mouse_event.row) {
        clear_button(state);
    } else if is_mouse_over_area(state.confirm_button, mouse_event.column, mouse_event.row) {
        // "Your input >>" needs to be selected to perform the mapping
        if state.output_display_subset[1].0 == *"Your input >>" && state.selected_output_field == 1 {
            confirm_mapping(state);
        }
    } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
        if state.popup_mapping_p2_p3 {
            clear_popup(state);
        } else if state.uncompleted_warning {
            state.uncompleted_warning = false;
        } else if state.page == Pages::RequiredDataP2 {
            state.return_to_p1_warning = true;
        } else {
            clear_mapping_options(state);
            clear_popup(state);

            state.page.prev();
        }
    } else if !is_mouse_over_area(state.popup_path_area, mouse_event.column, mouse_event.row)
        && !is_mouse_over_area(state.popup_value_area, mouse_event.column, mouse_event.row)
        && !is_mouse_over_area(state.popup_output_path, mouse_event.column, mouse_event.row)
        && !is_mouse_over_area(state.popup_output_result, mouse_event.column, mouse_event.row)
    {
        clear_popup(state);
    } else if is_mouse_over_area(state.prev_page_button, mouse_event.column, mouse_event.row) {
        state.page.prev();
    }
}

//////////     HELPERS     //////////

pub fn next_page(state: &mut AppState) {
    clear_mapping_options(state);
    clear_popup(state);

    state.p2_p3_tabs = P2P3Tabs::InputFields;
    state.selected_input_field = 1;
    state.selected_output_field = 1;
    state.selected_output_field = 1;

    state.page.next();
}

/// It is recommended not to alter the custom mapping files manually, since this will likely result in an error.
pub fn create_output_files(state: &mut AppState) {
    let output_format = state.mapping.output_format();
    let json_value = state.repository.get_mut(&output_format).unwrap();

    // Create Output File
    trace_dbg!(&state.output_path);
    let mut file = std::fs::File::create(&state.output_path).unwrap();
    file.write_all(serde_json::to_string_pretty(&json_value).unwrap().as_bytes())
        .unwrap();

    // Create Mapping File if empty, otherwise append to it.
    if !state.custom_mapping_path.is_empty() {
        if Path::new(&state.custom_mapping_path).is_file() {
            let mut transformations: Vec<Transformation> = get_json(&state.custom_mapping_path).unwrap();
            transformations.extend(state.performed_mappings.iter().cloned());

            let mut file = File::create(&state.custom_mapping_path).unwrap();
            file.write_all(serde_json::to_string_pretty(&transformations).unwrap().as_bytes())
                .unwrap();
        } else {
            let mut file = File::create(&state.custom_mapping_path).unwrap();
            file.write_all(
                serde_json::to_string_pretty(&state.performed_mappings)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        }
    }
}

pub fn clear_mapping_options(state: &mut AppState) {
    state.select_mapping_option = true;
    state.selected_transformation = 0;
    state.selected_transformations.clear();
    state.selected_transformations_tab = false;
    state.mapping_option = MappingOptions::DirectCopy;
    state.transformations = Transformations::LowerCase;
}

pub fn clear_popup(state: &mut AppState) {
    state.popup_mapping_p2_p3 = false;
    state.popup_offset_path = 0;
    state.popup_offset_value = 0;
    state.popup_offset_output_path = 0;
    state.popup_offset_result = 0;
}

pub fn clear_button(state: &mut AppState) {
    // Close popup if open.
    if state.popup_mapping_p2_p3 {
        state.popup_mapping_p2_p3 = false;
    }
    // If mapping options have been chosen, clear mapping options.
    else if !state.select_mapping_option {
        clear_mapping_options(state);
    }
    // Clear selected missing/optional field
    else {
        if state.page == Pages::RequiredDataP2 {
            if state
                .resolved_subsets
                .get(&state.required_field_pointer)
                .unwrap()
                .get("Your input >>")
                .is_some()
            {
                *state
                    .resolved_subsets
                    .get_mut(&state.required_field_pointer)
                    .unwrap()
                    .get_mut("Your input >>")
                    .unwrap() = Value::Null;
            }
            state
                .completed_required_fields
                .retain(|(first, _)| first != &state.required_field_pointer);
        } else {
            if state
                .resolved_subsets
                .get(&state.optional_field_pointer)
                .unwrap()
                .get("Your input >>")
                .is_some()
            {
                *state
                    .resolved_subsets
                    .get_mut(&state.optional_field_pointer)
                    .unwrap()
                    .get_mut("Your input >>")
                    .unwrap() = Value::Null;
            }
            state
                .completed_optional_fields
                .retain(|(first, _)| first != &state.optional_field_pointer);
        }

        state
            .repository
            .clear_mapping(state.output_pointer.clone(), state.mapping);
        clear_performed_transformation(state);
        clear_mapping_options(state);
    }
}

pub fn confirm_mapping(state: &mut AppState) {
    set_candidate_output_value(state, true);
    update_repository(state);
    update_resolved_subset(state);
    save_completed_fields(state);
    move_active_fields(state);

    trace_dbg!(&state.completed_optional_fields);
    trace_dbg!(state.candidate_output_value.as_ref().unwrap());

    clear_popup(state);
    clear_mapping_options(state);
}

///// HELPERS /////

fn update_resolved_subset(state: &mut AppState) {
    if state.page == Pages::RequiredDataP2 {
        let output_map = state.resolved_subsets.get_mut(&state.required_field_pointer).unwrap();
        *output_map.get_mut("Your input >>").unwrap() = Value::from(state.candidate_output_value.clone().unwrap());
    } else if state.page == Pages::OptionalDataP3 {
        let output_map = state.resolved_subsets.get_mut(&state.optional_field_pointer).unwrap();
        *output_map.get_mut("Your input >>").unwrap() = Value::from(state.candidate_output_value.clone().unwrap());
    }
}

fn save_completed_fields(state: &mut AppState) {
    if state.page == Pages::RequiredDataP2 {
        if !state
            .completed_required_fields
            .iter()
            .any(|(first, _)| first == &state.required_field_pointer)
        {
            state.completed_required_fields.push((
                state.required_field_pointer.clone(),
                state.input_fields[state.selected_input_field].0.clone(),
            )); // todo: also here we still need to refactor input fields according to the outputfields
        } else {
            // Find the old mapping tuple and replace
            for tuple in &mut state.completed_required_fields {
                if tuple.0 == state.required_field_pointer {
                    *tuple = (
                        state.required_field_pointer.clone(),
                        state.input_fields[state.selected_input_field].0.clone(),
                    );
                    break;
                }
            }
        }
    } else if state.page == Pages::OptionalDataP3 {
        if !state
            .completed_optional_fields
            .iter()
            .any(|(first, _)| first == &state.optional_field_pointer)
        {
            state.completed_optional_fields.push((
                state.optional_field_pointer.clone(),
                state.input_fields[state.selected_input_field].0.clone(),
            ));
        } else {
            // Find the old mapping tuple and replace
            for tuple in &mut state.completed_optional_fields {
                if tuple.0 == state.optional_field_pointer {
                    *tuple = (
                        state.optional_field_pointer.clone(),
                        state.input_fields[state.selected_input_field].0.clone(),
                    );
                    break;
                }
            }
        }
    }
}

fn move_active_fields(state: &mut AppState) {
    if state.selected_output_field == state.output_display_subset.len() - 1 {
        state.selected_output_field = 1;
    } else {
        state.selected_output_field += 1;
    }

    if state.selected_input_field == state.input_fields.len() - 1 {
        state.selected_input_field = 1;
    } else {
        state.selected_input_field += 1;
    }

    state.p2_p3_tabs = P2P3Tabs::InputFields;
}

fn clear_performed_transformation(state: &mut AppState) {
    let output_path: JsonPath = JsonPointer(state.output_pointer.clone()).into();

    state.performed_mappings.retain(|transformation| match transformation {
        Transformation::OneToOne { destination, .. } => destination.path != output_path.to_string(),
        // other Transformations are not implemented yet
        _ => false,
    });
}
