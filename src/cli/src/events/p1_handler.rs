use crate::repository::{construct_leaf_node, merge, Repository};
use crate::trace_dbg;
use crate::transformations::Transformation;
use crate::utils::{get_leaf_nodes, AppState, Mapping, P1Prompts, Tabs};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};
use regex::Regex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

pub fn p1_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => return Ok(true),
                Tab => {
                    let path = Path::new(&state.input_path);
                    if state.tab == Tabs::InputPromptsP1 && path.exists() && path.is_file() {
                        state.tab.next();
                        preload_p2(state);
                    } else {
                        state.tab.next();
                    }
                }
                F(2) => {
                    state.tab.prev();
                }
                Left => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.prev();
                        let strr = format!("{:?}", state.mapping);
                        trace_dbg!(strr);
                    }
                }
                Right => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.next();
                        state.mapping = crate::utils::Mapping::ELMToOBv3;
                        let strr = format!("{:?}", state.mapping);
                        trace_dbg!(strr);
                    }
                }
                Up => {
                    state.p1_prompts.prev();
                }
                Down => {
                    state.p1_prompts.next();
                }
                Enter => {
                    let path = Path::new(&state.input_path);
                    if state.p1_prompts == P1Prompts::Mapping && path.exists() && path.is_file() {
                        state.tab.next();
                        preload_p2(state);
                    }
                    state.p1_prompts.next();
                }
                Backspace => match state.p1_prompts {
                    P1Prompts::Input => {
                        state.input_path.pop();
                    }
                    P1Prompts::Output => {
                        state.output_path.pop();
                    }
                    _ => {}
                },
                Char(value) => match state.p1_prompts {
                    P1Prompts::Input => {
                        state.input_path.push(value);
                    }
                    P1Prompts::Output => {
                        state.output_path.push(value);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(false)
}

fn preload_p2(state: &mut AppState) {
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

        state.repository = Repository::from(HashMap::from_iter(vec![(
            match state.mapping {
                Mapping::OBv3ToELM => "OBv3".to_string(),
                Mapping::ELMToOBv3 => "ELM".to_string(),
            },
            get_json(&state.input_path).expect("No source file found"),
        )]));

        trace_dbg!("Successfully loaded the input file");
    }

    // // Load the mapping file
    // {
    //     // TODO: remove this hardcoded code
    //     let mapping_path = include_str!("../../res/mapping.json");
    //     let rdr = std::fs::File::open(mapping_path).unwrap();
    //     let mut transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

    //     trace_dbg!("Successfully loaded the mapping file");

    //     // Load the custom mapping file
    //     {
    //         if let Some(custom_mapping_path) = &state.custom_mapping_path {
    //             // TODO: remove this hardcoded code
    //             let custom_mapping_path = include_str!("../../res/custom_mapping.json");
    //             let rdr = std::fs::File::open(custom_mapping_path).unwrap();
    //             let mut custom_transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();
    //             transformations.append(&mut custom_transformations);

    //             trace_dbg!("Successfully loaded the custom mapping file");
    //         }
    //     }

    //     state.repository.apply_transformations(transformations);
    // }
}

pub fn verify(json_value: &mut Value) -> Result<Value, String> {
    let mut json_as_string = json_value.to_string();
    loop {
        // TODO: make dynamic
        let mut de = serde_json::Deserializer::from_str(&json_as_string);
        match serde_path_to_error::deserialize::<_, OBv3>(&mut de) {
            Ok(obv3_credential) => return Ok(json!(obv3_credential)),
            Err(e) => {
                println!("Error: {:?}", e);
                let error_message = e.inner().to_string();
                println!("Error: {}", error_message);

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

                let pointer = if error_message.starts_with("invalid type") {
                    if path == "/" {
                        format!("{path}")
                    } else {
                        format!("/{path}/")
                    }
                } else {
                    panic!("Unknown error: {}", error_message);
                };

                let mut leaf_node = construct_leaf_node(&pointer);

                println!("Please type some data of the correct type");
                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string).unwrap();

                let (option, input_string) = input_string.trim_end().split_once(":").unwrap();

                merge(json_value, leaf_node);

                return Err("String".to_string());
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

fn temp(state: &mut AppState) {
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

        state.repository = Repository::from(HashMap::from_iter(vec![(
            match state.mapping {
                Mapping::OBv3ToELM => "OBv3".to_string(),
                Mapping::ELMToOBv3 => "ELM".to_string(),
            },
            get_json(&state.input_path).expect("No source file found"),
        )]));

        trace_dbg!("Successfully loaded the input file");
    }

    // Load the mapping file
    {
        // TODO: remove this hardcoded code
        let mapping_path = "../../res/mapping.json";
        let rdr = std::fs::File::open(mapping_path).unwrap();
        let mut transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

        trace_dbg!("Successfully loaded the mapping file");

        // Load the custom mapping file
        {
            if let Some(custom_mapping_path) = &state.custom_mapping_path {
                // TODO: remove this hardcoded code
                let custom_mapping_path = "../../res/custom_mapping.json";
                let rdr = std::fs::File::open(custom_mapping_path).unwrap();
                let mut custom_transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();
                transformations.append(&mut custom_transformations);

                trace_dbg!("Successfully loaded the custom mapping file");
            }
        }

        state.repository.apply_transformations(transformations);
    }
}

#[test]
fn temp_test() {
    let mut state = AppState {
        input_path: "../../res/source_credential.json".to_string(),
        ..Default::default()
    };

    temp(&mut state);
}

fn get_json<T>(path: impl AsRef<Path>) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OBv3 {
    person: Person,
    another_field: String,
    another_field2: Test,
}

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    test2: Test2,
}

#[derive(Serialize, Deserialize, Debug)]
struct Test2 {
    test3: String,
    test4: String,
    test5: Test5,
}

#[derive(Serialize, Deserialize, Debug)]
struct Test5 {
    test6: String,
}

fn extract_between_backticks(s: &str) -> Option<String> {
    // Define the regex pattern to match text between backticks
    let re = Regex::new(r"`([^`]*)`").unwrap();

    // Find the first match and extract the text between the backticks
    re.captures(s)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}
