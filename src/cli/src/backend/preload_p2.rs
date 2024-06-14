use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use regex::Regex;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::{backend::{leaf_nodes::get_leaf_nodes, repository::Repository, selector::selector, transformations::Transformation}, elm::ELM, obv3::OBv3, state::AppState, trace_dbg};
use super::repository::{construct_leaf_node, merge};

pub fn preload_p2(state: &mut AppState) {
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
        let mut transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

        trace_dbg!("Successfully loaded the mapping file");

        // Load the custom mapping file
        {
            if let Some(custom_mapping_path) = &state.custom_mapping_path {
                let rdr = std::fs::File::open(custom_mapping_path).unwrap();
                let mut custom_transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();
                transformations.append(&mut custom_transformations);

                trace_dbg!("Successfully loaded the custom mapping file");
            }
        }

        state.repository.apply_transformations(transformations);
    }

    trace_dbg!(&output_format);
    trace_dbg!(&state.repository);
    let mut json_value = state.repository.get(&output_format).unwrap().clone();

    state.missing_data_field = match state.mapping.output_format().as_str() {
        "OBv3" => verify::<OBv3>(&mut json_value).err(),
        "ELM" => verify::<ELM>(&mut json_value).err(),
        _ => panic!(),
    };

    selector(state);
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
            Ok(obv3_credential) => return Ok(json!(obv3_credential)),
            Err(e) => {
                // println!("Error: {:?}", e);
                let error_message = e.inner().to_string();
                // println!("Error: {}", error_message);

                let path = e.path().to_string().replace('.', "/");

                if error_message.starts_with("missing field") {
                    let missing_field = extract_between_backticks(&e.to_string()).unwrap();
                    let pointer = if path == "/" {
                        format!("{path}{missing_field}")
                    } else {
                        format!("/{path}/{missing_field}")
                    };

                    let mut leaf_node = construct_leaf_node(&pointer);

                    leaf_node.pointer_mut(&pointer).map(|value| *value = json!({})).unwrap();

                    merge(json_value, leaf_node);

                    json_as_string = json_value.to_string();

                    continue;
                }

                if error_message.starts_with("invalid type") {
                    let pointer = if path == "/" {
                        format!("{path}")
                    } else {
                        format!("/{path}")
                    };

                    return Err(pointer);
                } else {
                    panic!("Unknown error: {}", error_message);
                };

                // match option {
                //     "1" => {
                //         println!("pointer: {:?}", pointer);

                //         leaf_node
                //             .pointer_mut(&pointer)
                //             .map(|value| *value = serde_json::from_str(input_string.trim_end()).unwrap());

                //         println!("{}", leaf_node);

                //         merge(&mut json_value, leaf_node);

                //         temp = json_value.to_string();
                //     }
                //     "2" => {
                //         let source_credential = repository.get(&settings.source.format).unwrap();

                //         let finder = JsonPathFinder::from_str(&source_credential.to_string(), &input_string).unwrap();
                //         // TODO: use loop in case the value can not be found.
                //         let source_value = finder.find().as_array().unwrap().first().unwrap().clone();

                //         println!("pointer: {:?}", pointer);
                //         let destination_path: JsonPath = JsonPointer(pointer.clone()).into();

                //         let transformation = Transformation::OneToOne {
                //             type_: OneToOne::copy,
                //             source: DataLocation {
                //                 format: "ELM".to_string(),
                //                 path: input_string.to_string(),
                //             },
                //             destination: DataLocation {
                //                 format: "OBv3".to_string(),
                //                 path: destination_path.to_string(),
                //             },
                //         };

                //         settings.custom_mapping_file.as_ref().map(|custom_mapping_file| {
                //             let mut custom_transformations = custom_mapping_file
                //                 .exists
                //                 .then(|| {
                //                     get_json::<Vec<Transformation>>(&custom_mapping_file.path)
                //                         .expect("No custom mapping file found")
                //                 })
                //                 .unwrap_or_default();

                //             println!("before: custom_transformations: {:?}", custom_transformations.len());
                //             custom_transformations.push(transformation);
                //             println!("after: custom_transformations: {:?}", custom_transformations.len());

                //             // For pretty printing
                //             let pretty_file = std::fs::OpenOptions::new()
                //                 .create(true)
                //                 .write(true)
                //                 .truncate(true)
                //                 .open(&custom_mapping_file.path)
                //                 .unwrap();

                //             // Serialize the struct to pretty JSON and write it to the file
                //             serde_json::to_writer_pretty(&pretty_file, &custom_transformations)
                //                 .expect("Failed to serialize");
                //         });

                //         println!("destination_path: {:?}", destination_path);

                //         leaf_node.pointer_mut(&pointer).map(|value| *value = source_value);

                //         println!("{}", leaf_node);

                //         merge(&mut json_value, leaf_node);

                //         temp = json_value.to_string();
                //     }
                //     _ => {
                //         panic!("Invalid option");
                //     }
                // }
            }
        }
    }
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
