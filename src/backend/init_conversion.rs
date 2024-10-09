use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use crate::{
    backend::{leaf_nodes::get_leaf_nodes, repository::Repository, transformations::Transformation},
    state::{AppState, Mapping},
    trace_dbg,
};

use super::{desm_mapping::apply_desm_mapping, update_display::update_display_section};

pub fn init_conversion(state: &mut AppState) {
    init_schema(state);
    load_input_file(state, false);
    load_mapping_file(state);
    enter_fixed_context_values(state);
    enter_fixed_schema_values(state);
    update_display_section(state, false);
}

/// Initialize output schema
fn init_schema(state: &mut AppState) {
    match state.mapping {
        Mapping::OBv3ToELM => {
            state.target_schema = get_json("json/ebsi-elm/vcdm2.0-europass-edc-schema/schema.json")
                .expect("error: couldn't retrieve Europass EDC ELM schema");
        }
        Mapping::ELMToOBv3 => {
            state.target_schema =
                get_json("json/obv3/obv3_schema.json").expect("error: couldn't retrieve OpenBadges version 3 schema");
        }
    }
}

pub fn load_input_file(state: &mut AppState, headless: bool) {
    let (input_format, output_format) = (state.mapping.input_format(), state.mapping.output_format());

    if let Ok(rdr) = std::fs::File::open(&state.input_path) {
        if let Ok(input_value) = serde_json::from_reader(rdr) {
            let leaf_nodes: HashMap<String, Value> = get_leaf_nodes(input_value);
            if !leaf_nodes.is_empty() {
                if !headless {
                    let mut input_fields = vec![(String::new(), String::new())];

                    for (key, value) in leaf_nodes {
                        input_fields.push((key, value.to_string()));
                    }

                    input_fields.sort();
                    state.amount_input_fields = input_fields.len() - 2;
                    state.input_fields = input_fields;
                }

                state.repository = Repository::from(HashMap::from_iter(vec![
                    (
                        input_format.to_string(),
                        get_json(&state.input_path).expect("No source file found"),
                    ),
                    (output_format.to_string(), json!({})),
                ]));

                trace_dbg!("Successfully loaded the input file");
            } else {
                trace_dbg!("Empty input file");
            }
        } else {
            trace_dbg!("Input file is not a .json file or invalid");
        }
    } else {
        trace_dbg!("File in input path doesnt exist or could not be read");
    }
}

pub fn load_mapping_file(state: &mut AppState) {
    if state.mapping_path == "DESM" {
        apply_desm_mapping(state);
    } else {
        let rdr = std::fs::File::open(&state.mapping_path).unwrap();
        let transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

        trace_dbg!("Successfully loaded the mapping file");

        state.repository.apply_transformations(transformations, state.mapping);
        // todo: add applied transformation to completed fields
    }
}

/// Enter fixed values into '@context' field, as demanded by the respective json-schema
fn enter_fixed_context_values(state: &mut AppState) {
    if state.mapping.output_format() == "ELM" {
        let output_elm = state.repository.get_mut("ELM").unwrap().as_object_mut().unwrap();
        output_elm.insert(
            "@context".to_string(),
            Value::Array(vec![
                json!("https://www.w3.org/ns/credentials/v2"),
                json!("http://data.europa.eu/snb/model/context/edc-ap"),
                ]),
        );
    } else if state.mapping.output_format() == "OBv3" {
        let output_obv3 = state.repository.get_mut("OBv3").unwrap().as_object_mut().unwrap();
        output_obv3.insert(
            "@context".to_string(),
            Value::Array(vec![
                json!("https://www.w3.org/ns/credentials/v2"),
                json!("https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.3.json"),
            ]),
        );
    }

}


/// Enter fixed values into '@context' field, as demanded by the respective json-schema
fn enter_fixed_schema_values(state: &mut AppState) {
    if state.mapping.output_format() == "ELM" {
        let output_elm = state.repository.get_mut("ELM").unwrap().as_object_mut().unwrap();
        output_elm.insert(
            "credentialSchema".to_string(),
            Value::Array(vec![
                json!({"id": "http://data.europa.eu/snb/model/ap/edc-generic-full","type": "ShaclValidator2017"}), 
                json!({"id": "https://api-pilot.ebsi.eu/trusted-schemas-registry/v3/schemas/0x7ff3bc76bd5e37b3d29721b8698646a722a24a4f4ab0a0ba63d4bbbe0ef9758d",
      "type": "JsonSchema"})]),
        );
    } else if state.mapping.output_format() == "OBv3" {
        let output_obv3 = state.repository.get_mut("OBv3").unwrap().as_object_mut().unwrap();
        output_obv3.insert(
            "credentialSchema".to_string(),
            Value::Array(vec![
                json!({"id": "https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_endorsementcredential_schema.json", "type": "1EdTechJsonSchemaValidator2019"}),
                json!({"id": "https://accrediter.edu/schema/endorsementcredential.json","type": "1EdTechJsonSchemaValidator2019"})]),
        );
    }
}


////////     HELPERS     ////////

pub fn get_json<T>(path: impl AsRef<Path>) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
}

// Regex

// use regex::Regex;

// fn extract_string_value(input: &str) -> Option<&str> {
//     let re = Regex::new(r"expected (.*?) at line").unwrap();
//     re.captures(input).and_then(|cap| cap.get(1).map(|m| m.as_str()))
// }

// fn extract_between_backticks(s: &str) -> Option<String> {
//     // Define the regex pattern to match text between backticks
//     let re = Regex::new(r"`([^`]*)`").unwrap();

//     // Find the first match and extract the text between the backticks
//     re.captures(s)
//         .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
// }
