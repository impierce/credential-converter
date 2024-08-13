use core::panic;
use digital_credential_data_models::{elmv3::EuropassEdcCredential, obv3::AchievementCredential};
use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use super::repository::{construct_leaf_node, merge};
use crate::{
    backend::{
        leaf_nodes::get_leaf_nodes, repository::Repository, transformations::Transformation,
    },
    state::{AppState, Mapping, P2P3Tabs, Pages},
    trace_dbg,
};

// todo: when going back to p1 and loading again, everything in backend is wiped because of this preload fn.
// this is fine but then also state info must be wiped
pub fn preload_p2(state: &mut AppState) {
    get_missing_data_fieldss(state); // testing

    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

    // Load the input file
    {
        let rdr = std::fs::File::open(&state.input_path).unwrap();
        let input_value: Value = serde_json::from_reader(rdr).unwrap();
        let leaf_nodes: HashMap<String, Value> = get_leaf_nodes(input_value);
        let mut input_fields = vec![(String::new(), String::new())];

        for (key, value) in leaf_nodes {
            input_fields.push((key, value.to_string()));
        }

        input_fields.sort();
        state.amount_input_fields = input_fields.len() - 2;
        state.input_fields = input_fields;

        state.repository = Repository::from(HashMap::from_iter(vec![
            (
                input_format.to_string(),
                get_json(&state.input_path).expect("No source file found"),
            ),
            (output_format.to_string(), json!({})),
        ]));

        trace_dbg!("Successfully loaded the input file");
    }

    // Load the mapping file
    {
        let rdr = std::fs::File::open(&state.mapping_path).unwrap();
        let transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

        trace_dbg!("Successfully loaded the mapping file");

        // Load the custom mapping file
        // { // todo: I added the custom_mapping_path as the destination to save the manual mappings to. not to load mappings from, thats the mapping_path
        //     if !state.custom_mapping_path.is_empty() && Path::new(&state.custom_mapping_path).is_file() && state.custom_mapping_path.ends_with(".json"){
        //         let rdr = std::fs::File::open(&state.custom_mapping_path).unwrap();
        //         let mut custom_transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();
        //         transformations.append(&mut custom_transformations);

        //         trace_dbg!("Successfully loaded the custom mapping file");
        //     }
        // }

        state.repository.apply_transformations(transformations);
    }

    trace_dbg!(&output_format);
    // trace_dbg!(&state.repository);
    let json_value = state.repository.get(&output_format).unwrap().clone();

    state.missing_data_fields = [
        vec![("".to_string(), "".to_string())],
        match state.mapping.output_format().as_str() {
            "OBv3" => get_missing_data_fields::<AchievementCredential>(json_value.clone()),
            "ELM" => get_missing_data_fields::<EuropassEdcCredential>(json_value.clone()),
            _ => panic!(),
        }
        .into_iter()
        .map(|pointer| (pointer, "".to_string()))
        .collect(),
    ]
    .concat();

    //selector(state);
}

pub fn verify<T>(json_value: &mut Value) -> Result<Value, String>
where
    T: DeserializeOwned + Serialize,
{
    let mut json_as_string = json_value.to_string();
    loop {
        // TODO: make dynamic
        let mut de = serde_json::Deserializer::from_str(&json_as_string);
        match serde_path_to_error::deserialize::<_, T>(&mut de) {
            Ok(obv3_credential) => {
                return Ok(json!(obv3_credential));
            }
            Err(e) => {
                let error_message = e.inner().to_string();

                let path = e.path().to_string().replace('.', "/");

                if error_message.starts_with("missing field") {
                    let missing_field = extract_between_backticks(&e.to_string()).unwrap();
                    let pointer = if path == "/" {
                        format!("{path}{missing_field}")
                    } else {
                        format!("/{path}/{missing_field}")
                    };

                    let mut leaf_node = construct_leaf_node(&pointer);

                    leaf_node.pointer_mut(&pointer).map(|value| *value = json!({})).unwrap(); // could be a problem when we add field constraints

                    merge(json_value, leaf_node);

                    json_as_string = json_value.to_string();

                    continue;
                }

                if error_message.starts_with("data did not match any variant of untagged enum") {
                    let pointer = if path == "/" { path } else { format!("/{path}") };

                    let mut leaf_node = construct_leaf_node(&pointer);

                    leaf_node.pointer_mut(&pointer).map(|value| *value = json!("")).unwrap(); // doesnt work

                    merge(json_value, leaf_node);

                    json_as_string = json_value.to_string();

                    continue;
                }

                if error_message.starts_with("input contains invalid characters") {
                    let pointer = if path == "/" { path } else { format!("/{path}") };

                    let mut leaf_node = construct_leaf_node(&pointer);

                    leaf_node
                        .pointer_mut(&pointer)
                        .map(|value| *value = json!("2010-01-01T00:00:00Z"))
                        .unwrap();

                    merge(json_value, leaf_node);

                    json_as_string = json_value.to_string();

                    continue;
                }

                if error_message.starts_with("invalid value") {
                    let pointer = if path == "/" { path } else { format!("/{path}") };

                    let mut leaf_node = construct_leaf_node(&pointer);

                    let expected_value = extract_string_value(&error_message).unwrap();

                    if expected_value != "https://www.w3.org/ns/credentials/v2" {
                        panic!("Expected value: {}", expected_value);
                    }

                    leaf_node
                        .pointer_mut(&pointer)
                        .map(|value| *value = json!([expected_value]))
                        .unwrap();

                    merge(json_value, leaf_node);

                    json_as_string = json_value.to_string();

                    continue;
                }

                if error_message.starts_with("invalid type") {
                    let pointer = if error_message.contains("invalid type: map")
                        && error_message.contains("expected a sequence")
                    {
                        let pointer = if path == "/" { path } else { format!("/{path}") };

                        let mut leaf_node = construct_leaf_node(&pointer);

                        leaf_node
                            .pointer_mut(&pointer)
                            .map(|value| *value = json!(["TEMP"]))
                            .unwrap();

                        merge(json_value, leaf_node);

                        // json_as_string = json_value.to_string();

                        return Err(format!("{pointer}/0"));
                    } else if path == "/" {
                        path
                    } else {
                        format!("/{path}")
                    };

                    return Err(pointer);
                } else {
                    panic!("Unknown error: {}", error_message);
                };
            }
        }
    }
}

pub fn get_missing_data_fields<T>(mut temp_credential: Value) -> Vec<String>
where
    T: DeserializeOwned + Serialize,
{
    let mut missing_data_fields = vec![];
    while let Err(pointer) = verify::<T>(&mut temp_credential) {
        trace_dbg!(&temp_credential);
        trace_dbg!(&pointer);

        match temp_credential
            .pointer_mut(&pointer)
            .map(|value| *value = json!("TEMP")) // could be a problem when we add field constraints
        {
            Some(_) => {}
            None => return vec![],
        }
        missing_data_fields.push(pointer);
    }

    missing_data_fields
}

fn extract_string_value(input: &str) -> Option<&str> {
    let re = Regex::new(r"expected (.*?) at line").unwrap();
    re.captures(input).and_then(|cap| cap.get(1).map(|m| m.as_str()))
}

fn get_json<T>(path: impl AsRef<Path>) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
}

fn extract_between_backticks(s: &str) -> Option<String> {
    // Define the regex pattern to match text between backticks
    let re = Regex::new(r"`([^`]*)`").unwrap();

    // Find the first match and extract the text between the backticks
    re.captures(s)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

// testing
pub fn get_missing_data_fieldss(state: &mut AppState) {
    init_schema(state);

    update_display_section(state, false);
}

pub fn init_schema(state: &mut AppState) {
    // Init target_schema
    match state.mapping {
        Mapping::OBv3ToELM => {
            state.target_schema = get_json("json/ebsi-elm/vcdm2.0-europass-edc-schema/schema.json").expect("error: couldn't retrieve Europass EDC ELM schema");
        }
        Mapping::ELMToOBv3 => {
            state.target_schema = get_json("json/obv3/obv3_schema.json").expect("error: couldn't retrieve OpenBadges version 3 schema");
        }
    }
}

// All of the below is common to P2 & P3

pub fn get_required_fields(schema: &mut Value, tmp_map: &mut Map<String, Value>) {
    if let Some(properties) = schema.get("properties") {
        if let Some(required) = schema.get("required") {
            if let Some(required) = required.as_array() {
                for key in required {
                    if let Some(key) = key.as_str() {
                        tmp_map.insert(key.to_string(), properties[key].clone());
                    }
                }
            }
        }
    }
}

pub fn get_optional_fields(schema: &mut Value, tmp_map: &mut Map<String, Value>) {
    if let Some(properties) = schema.get("properties") {
        for key in properties.as_object().expect("error: invalid property key-value pair in schema") {
            tmp_map.insert(key.0.clone(), key.1.clone());
        }
    }
}


pub fn resolve_logic_construct(schema: &Value, path: &str, map: &mut Map<String, Value>) {
    if let Some(schema) = schema.pointer(path) {
        if let Some(all_of) = schema.get("allOf") {
            if let Some(all_of_elmnts) = all_of.as_array() {
                for i in 0..all_of_elmnts.len() {
                    map.insert(path.to_owned() + "/allOf/" + i.to_string().as_str(), Value::Null);
                }
            }
        }
        if let Some(any_of) = schema.get("anyOf") {
            if let Some(any_of_elmnts) = any_of.as_array() {
                for i in 0..any_of_elmnts.len() {
                    map.insert(path.to_owned() + "/anyOf/" + i.to_string().as_str(), Value::Null);
                }
            }
        }
        if let Some(one_of) = schema.get("oneOf") {
            if let Some(one_of_elmnts) = one_of.as_array() {
                for i in 0..one_of_elmnts.len() {
                    map.insert(path.to_owned() + "/oneOf/" + i.to_string().as_str(), Value::Null);
                }
            }
        }
        if let Some(not) = schema.get("not") {
            if let Some(not_elmnts) = not.as_array() {
                for i in 0..not_elmnts.len() {
                    map.insert(path.to_owned() + "/not/" + i.to_string().as_str(), Value::Null);
                }
            }
        }
    }
}

pub fn update_path(state: &mut AppState, forward_back: bool) {
    if forward_back {
        if state.page == Pages::ManualMappingP2 && state.p2_p3_tabs == P2P3Tabs::OutputFields && state.missing_display_subset[state.selected_missing_field].1 == "<Object>" {
            state.missing_field_pointer = state.missing_field_pointer.to_owned()
                + "/"
                + state.missing_display_subset[state.selected_missing_field].0.as_str();
            state.selected_missing_field = 1;
        } else if state.page == Pages::UnusedDataP3 && state.p2_p3_tabs == P2P3Tabs::OutputFields && state.optional_display_subset[state.selected_optional_field].1 == "<Object>" {
            state.optional_field_pointer = state.optional_field_pointer.to_owned()
                + "/"
                + state.optional_display_subset[state.selected_optional_field].0.as_str();
            state.selected_optional_field = 1;
        } else if state.p2_p3_tabs == P2P3Tabs::InputFields { // todo: implement input field similar structure // if state.input_display_section[state.selected_input_field].1 == "<Object>" {
            // state.input_field_pointer = state.input_field_pointer.to_owned()
            //     + "/"
            //     + state.input_display_section[state.selected_input_field].0.as_str();
            // state.selected_input_field = 1;
        }
    } else {
        if state.page == Pages::ManualMappingP2 && state.p2_p3_tabs == P2P3Tabs::OutputFields && state.missing_field_pointer != "/required" {
            state.missing_field_pointer = truncate_until_char(&state.missing_field_pointer, '/').to_string();
            state.selected_missing_field = 1;
        } else if state.page == Pages::UnusedDataP3 && state.p2_p3_tabs == P2P3Tabs::OutputFields && state.optional_field_pointer != "/optional" {
            truncate_until_char(&state.optional_field_pointer, '/');
            state.selected_optional_field = 1;
        } else if state.p2_p3_tabs == P2P3Tabs::InputFields { // todo: 
            // truncate_until_char(&state.input_field_pointer, '/');
            // state.selected_input_field = 1;
        }
    }
}

pub fn update_display_section(state: &mut AppState, preload_p3: bool) {
    let mut tmp_map = Map::new();
    let mut path = "";

    // Custom logic needed for preloading page 2 & 3
    if state.page == Pages::InputPromptsP1 {
        get_required_fields(&mut state.target_schema, &mut tmp_map);
        resolve_logic_construct(&mut state.target_schema, path, &mut tmp_map);
        path = "/required";
        state.missing_field_pointer = path.to_string();
    } else if preload_p3 {
        get_optional_fields(&mut state.target_schema, &mut tmp_map);
        resolve_logic_construct(&mut state.target_schema, path, &mut tmp_map);
        path = "/optional";
        state.optional_field_pointer = path.to_string();
    } else if state.page == Pages::ManualMappingP2 {
        if state.resolved_subsets.contains_key(&state.missing_field_pointer) {
            return ;
        }

        path = &state.missing_field_pointer;
        let subset_path = truncate_until_char(path, '/');
        let subset = state.resolved_subsets.get_mut(subset_path).unwrap(); // todo remove unwrap
        let key = path.trim_start_matches((subset_path.to_owned() + "/").as_str());

        resolve_ref(subset.get_mut(key).unwrap(), state.target_schema.clone()); // this should loop until there are no more refs
        get_required_fields(subset.get_mut(key).unwrap(), &mut tmp_map); // todo remove unwrap
        resolve_logic_construct(subset.get_mut(key).unwrap(), key, &mut tmp_map);

        // When a key-value contains no required field nor any logical construct, also not in a $ref or $def,
        // then we are left with 2 possibilities: either just the key is required but all fields within the key are optional, --> what to do?
        // or it's a leaf node. In the latter case we can resolve the leaf node with this function
        if tmp_map.is_empty() { // this should actually also check that type != object, if it is an object then it might be the frequent case where an object (key) is required but all fields within the object are optional
            tmp_map = subset.get_mut(key).unwrap().as_object().unwrap().clone(); // todo remove unwrap
            tmp_map.insert("Your input >>".to_string(), Value::Null);
            // resolve_leaf_node(subset.get_mut(key).unwrap(), key, &mut tmp_map);
        }
    } else if state.page == Pages::UnusedDataP3 {
        if state.resolved_subsets.contains_key(&state.optional_field_pointer) {
            return ;
        }
        
        path = &state.optional_field_pointer;
        let subset_path = truncate_until_char(path, '/');
        let subset = state.resolved_subsets.get_mut(subset_path).unwrap(); // todo remove unwrap
        let key = path.trim_start_matches((subset_path.to_owned() + "/").as_str());

        resolve_ref(subset.get_mut(key).unwrap(), state.target_schema.clone()); // this should loop until there are no more refs
        get_required_fields(subset.get_mut(key).unwrap(), &mut tmp_map); // todo remove unwrap
        resolve_logic_construct(subset.get_mut(key).unwrap(), key, &mut tmp_map);

        if tmp_map.is_empty() { // this should actually also check that type != object, if it is an object then it might be the edge case where an object (key) is required but all fields within the object are optional
            tmp_map = subset.get_mut(key).unwrap().as_object().unwrap().clone(); // todo remove unwrap
            tmp_map.insert("Your input >>".to_string(), Value::Null);
        }
    }

    state.resolved_subsets.insert(path.to_string(), Value::from(tmp_map));
    // state.missing_display_subset = value_to_str(state.resolved_subsets.get(path).unwrap());
    // trace_dbg!(&state.resolved_subsets);
}


/// This function takes a ref and replaces the subschema/Value with the resolved ref, entirely.
/// All specs until the latest haven't allowed any additional fields inside the ref object.
/// So practically this function is not fully compliant with the latest specs.
/// However, additional fields inside a ref object in the latest draft are quite complex to utilize and still strongly discouraged.
pub fn resolve_ref(schema: &mut Value, root: Value) {
    if let Some(schema_obj) = schema.as_object() {
        if let Some(ref_) = schema_obj.get("$ref") {
            if let Some(ref_str) = ref_.as_str() {
                // at this point it's fair to assume the json schema ref is invalid if it contains something else than a string.
                if ref_str.starts_with("#/") {
                    let tmp = root.pointer(ref_str.trim_start_matches("#")).unwrap().clone();
                    *schema = tmp;
                } else {
                    let path = truncate_until_char(root["$id"].as_str().unwrap(), '/').to_owned()
                        + ref_str.trim_start_matches('.'); // relative paths are valid as "/xx/yy/zz" as well as "./xx/yy/zz" in both cases the root-id folder path is prepended.
                    let tmp = get_json(path).unwrap();
                    *schema = tmp;
                }
            }
        }
    }
}

fn truncate_until_char(s: &str, ch: char) -> &str {
    match s.rfind(ch) {
        Some(pos) => &s[..pos],
        None => s,
    }
}
