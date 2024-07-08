use crate::state::{AppState, MappingOptions, P2P3Tabs, Transformations};

pub fn handle_esc(state: &mut AppState) -> bool {
    // Close popup warning P2
    if state.uncompleted_warning {
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

// todo: move to general file for both P2 and P3 and substitute as many events as possible with helper functions to make clear which are the same between P2 & P3
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
