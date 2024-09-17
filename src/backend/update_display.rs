use serde_json::Value;

use crate::{
    state::{AppState, P2P3Tabs, Pages},
    trace_dbg,
};

use super::{
    candidate_value::set_output_pointer,
    getters_resolvers::{get_optional_fields, get_required_fields, resolve_logic_construct, resolve_ref},
};

#[allow(clippy::collapsible_else_if)]
pub fn update_pointer(state: &mut AppState, forward_back: bool) {
    if forward_back {
        if state.page == Pages::RequiredDataP2
            && state.p2_p3_tabs == P2P3Tabs::OutputFields
            && state.output_display_subset[state.selected_output_field].1 == "<Object>"
        {
            state.required_field_pointer = state.required_field_pointer.to_owned()
                + "/"
                + state.output_display_subset[state.selected_output_field].0.as_str();
            state.selected_output_field = 1;
        } else if state.page == Pages::OptionalDataP3
            && state.p2_p3_tabs == P2P3Tabs::OutputFields
            && state.output_display_subset[state.selected_output_field].1 == "<Object>"
        {
            state.optional_field_pointer = state.optional_field_pointer.to_owned()
                + "/"
                + state.output_display_subset[state.selected_output_field].0.as_str();
            state.selected_output_field = 1;
        } else if state.p2_p3_tabs == P2P3Tabs::InputFields { // todo: implement input field similar structure // if state.input_display_section[state.selected_input_field].1 == "<Object>" {
             // state.input_field_pointer = state.input_field_pointer.to_owned()
             //     + "/"
             //     + state.input_display_section[state.selected_input_field].0.as_str();
             // state.selected_input_field = 1;
        }
    } else {
        if state.page == Pages::RequiredDataP2
            && state.p2_p3_tabs == P2P3Tabs::OutputFields
            && state.required_field_pointer != "/required"
        {
            state.required_field_pointer = truncate_until_char(&state.required_field_pointer, '/').to_string();
            if state.required_field_pointer.ends_with("allOf")
                || state.required_field_pointer.ends_with("anyOf")
                || state.required_field_pointer.ends_with("oneOf")
                || state.required_field_pointer.ends_with("not")
            {
                // todo could be done more elegantly
                state.required_field_pointer = truncate_until_char(&state.required_field_pointer, '/').to_string();
            }
            state.selected_output_field = 1;
        } else if state.page == Pages::OptionalDataP3
            && state.p2_p3_tabs == P2P3Tabs::OutputFields
            && state.optional_field_pointer != "/optional"
        {
            state.optional_field_pointer = truncate_until_char(&state.optional_field_pointer, '/').to_string();
            if state.optional_field_pointer.ends_with("allOf")
                || state.optional_field_pointer.ends_with("anyOf")
                || state.optional_field_pointer.ends_with("oneOf")
                || state.optional_field_pointer.ends_with("not")
            {
                // todo could be done more elegantly
                state.optional_field_pointer = truncate_until_char(&state.optional_field_pointer, '/').to_string();
            }
            state.selected_output_field = 1;
        } else if state.p2_p3_tabs == P2P3Tabs::InputFields { // todo:
             // truncate_until_char(&state.input_field_pointer, '/');
             // state.selected_input_field = 1;
        }
    }

    set_output_pointer(state);
}

pub fn update_display_section(state: &mut AppState, preload_p3: bool) {
    let mut tmp_map = serde_json::Map::new();
    let mut path = "";

    // Custom logic needed for preloading page 2 & 3
    if state.page == Pages::InputPromptsP1 {
        get_required_fields(&mut state.target_schema, &mut tmp_map);
        resolve_logic_construct(&state.target_schema, &mut tmp_map);
        path = "/required";
        state.required_field_pointer = path.to_string();
    } else if preload_p3 {
        get_optional_fields(&mut state.target_schema, &mut tmp_map);
        resolve_logic_construct(&state.target_schema, &mut tmp_map);
        path = "/optional";
        state.optional_field_pointer = path.to_string();
        state.output_pointer = "".to_string();
    } else if state.page == Pages::RequiredDataP2 {
        if state.resolved_subsets.contains_key(&state.required_field_pointer) {
            return;
        }

        path = &state.required_field_pointer;
        let mut subset_path = truncate_until_char(path, '/');
        if subset_path.ends_with("allOf")
            || subset_path.ends_with("anyOf")
            || subset_path.ends_with("oneOf")
            || subset_path.ends_with("not")
        {
            // todo could be done more elegantly
            subset_path = truncate_until_char(subset_path, '/');
        }

        let subset = state.resolved_subsets.get_mut(subset_path).unwrap(); // todo remove unwrap
        let key = path.trim_start_matches((subset_path.to_owned() + "/").as_str());

        trace_dbg!(&subset);
        resolve_ref(subset.get_mut(key).unwrap(), state.target_schema.clone()); // this should loop until there are no more refs
        trace_dbg!(&subset);
        resolve_logic_construct(subset.get_mut(key).unwrap(), &mut tmp_map);
        trace_dbg!(&subset);
        get_required_fields(subset.get_mut(key).unwrap(), &mut tmp_map); // todo remove unwrap
        trace_dbg!(&subset);

        // When a key-value contains no required field nor any logical construct, also not in a $ref or $def,
        // then we are left with 2 possibilities: either just the key is required but all fields within the key are optional, --> what to do?
        // or it's a leaf node. In the latter case we can resolve the leaf node with this function
        if tmp_map.is_empty() {
            // this should actually also check that type != object, if it is an object then it might be the case where an object (key) is required but all fields within the object are optional
            tmp_map.clone_from(subset.get_mut(key).unwrap().as_object().unwrap()); // todo remove unwrap
            if let Some(value) = state
                .repository
                .get(&state.mapping.output_format())
                .unwrap()
                .pointer(&state.output_pointer)
            {
                tmp_map.insert("Your input >>".to_string(), value.clone());
            } else {
                tmp_map.insert("Your input >>".to_string(), Value::Null);
            }
        }
    } else if state.page == Pages::OptionalDataP3 {
        if state.resolved_subsets.contains_key(&state.optional_field_pointer) {
            return;
        }

        path = &state.optional_field_pointer;
        let mut subset_path = truncate_until_char(path, '/');
        if subset_path.ends_with("allOf")
            || subset_path.ends_with("anyOf")
            || subset_path.ends_with("oneOf")
            || subset_path.ends_with("not")
        {
            // todo could be done more elegantly
            subset_path = truncate_until_char(subset_path, '/');
        }

        let subset = state.resolved_subsets.get_mut(subset_path).unwrap(); // todo remove unwrap
        let key = path.trim_start_matches((subset_path.to_owned() + "/").as_str());

        resolve_ref(subset.get_mut(key).unwrap(), state.target_schema.clone()); // this should loop until there are no more refs
        resolve_logic_construct(subset.get_mut(key).unwrap(), &mut tmp_map);
        get_optional_fields(subset.get_mut(key).unwrap(), &mut tmp_map); // todo remove unwrap
        resolve_ref(subset.get_mut(key).unwrap(), state.target_schema.clone()); // this should loop until there are no more refs

        if tmp_map.is_empty() {
            // this should actually also check that type != object, if it is an object then it might be the edge case where an object (key) is required but all fields within the object are optional
            tmp_map.clone_from(subset.get_mut(key).unwrap().as_object().unwrap()); // todo remove unwrap
            if let Some(value) = state
                .repository
                .get(&state.mapping.output_format())
                .unwrap()
                .pointer(&state.output_pointer)
            {
                tmp_map.insert("Your input >>".to_string(), value.clone());
            } else {
                tmp_map.insert("Your input >>".to_string(), Value::Null);
            }
        }
    }

    state.resolved_subsets.insert(path.to_string(), Value::from(tmp_map));
}

///// HELPERS /////

pub fn truncate_until_char(s: &str, ch: char) -> &str {
    match s.rfind(ch) {
        Some(pos) => &s[..pos],
        None => s,
    }
}
