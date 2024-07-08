use std::char;

use crate::state::{AppState, MappingOptions, P2P3Tabs, Pages, Transformations};

pub fn handle_esc(state: &mut AppState) -> bool {
    // Close popup warning P2
    if state.uncompleted_warning && state.page == Pages::ManualMappingP2 {
        state.uncompleted_warning = false;
        false
    }
    // Close popup mapping and reset scroll offsets
    else if state.popup_mapping_p2_p3 {
        state.popup_mapping_p2_p3 = false;
        state.popup_offset_path = 0;
        state.popup_offset_value = 0;
        state.popup_offset_output_path = 0;
        state.popup_offset_result = 0;
        false
    }
    // clear mapping
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions && !state.select_mapping_option {
        state.select_mapping_option = true;
        state.selected_transformations_tab = false;
        state.selected_transformations.clear();
        state.transformations = Transformations::LowerCase;
        state.mapping_option = MappingOptions::DirectCopy;
        false
    }
    // Close program
    else {
        true
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
    else if state.p2_p3_tabs == P2P3Tabs::MappingOptions
        && state.mapping_option == MappingOptions::OneToMany
    {
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
            if state.selected_missing_field > 1 {
                state.selected_missing_field -= 1;
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
            if state.selected_missing_field <= state.amount_missing_fields {
                state.selected_missing_field += 1;
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
        } else if state.selected_transformations.is_empty() {
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
