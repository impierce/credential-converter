use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        leaf_nodes::construct_leaf_node,
        transformations::{DataLocation, StringValue, Transformation},
    },
    state::{AppState, Mapping},
    trace_dbg,
};
use jsonpath_rust::JsonPathFinder;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default, Clone)]
pub struct Repository(HashMap<String, Value>);

impl DerefMut for Repository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Repository {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<String, Value>> for Repository {
    fn from(map: HashMap<String, Value>) -> Self {
        Self(map)
    }
}

impl Repository {
    pub fn apply_transformation(
        &mut self,
        transformation: Transformation,
        mapping: Mapping,
    ) -> Option<(String, String)> {
        match transformation {
            Transformation::OneToOne {
                type_: transformation,
                source:
                    DataLocation {
                        format: source_format,
                        path: mut source_path,
                    },
                destination:
                    DataLocation {
                        format: destination_format,
                        path: destination_path,
                    },
            } => {
                if source_format != mapping.input_format() || destination_format != mapping.output_format() {
                    return None;
                }

                let source_credential = self.get(&source_format).unwrap();

                // custom code to handle the special character '@' in the source_path
                if source_path == "$.@context" {
                    source_path = r#"$["@context"]"#.to_string();
                };

                let finder = JsonPathFinder::from_str(&source_credential.to_string(), &source_path).unwrap();

                let source_value = match finder.find().as_array() {
                    // todo: still need to investigate other find() return types
                    Some(array) => array.first().unwrap().clone(),
                    None => {
                        return None;
                    }
                };

                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.

                let pointer = JsonPointer::try_from(JsonPath(destination_path.clone())).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);

                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    *value = transformation.apply(source_value);
                }

                merge(destination_credential, leaf_node);

                trace_dbg!("Successfully completed transformation");
                Some((destination_path, source_path))
            }
            Transformation::ManyToOne {
                type_: transformation,
                sources,
                destination,
            } => {
                if sources.iter().any(|source| source.format != mapping.input_format())
                    || destination.format != mapping.output_format()
                {
                    return None;
                }

                let source_values = sources
                    .iter()
                    .map(|source| {
                        let source_credential = self.get(&source.format).unwrap();

                        let finder = JsonPathFinder::from_str(&source_credential.to_string(), &source.path).unwrap();
                        finder.find().as_array().unwrap().first().unwrap().clone()
                    })
                    .collect::<Vec<_>>();

                let destination_credential = self.entry(destination.format).or_insert(json!({}));
                let pointer = JsonPointer::try_from(JsonPath(destination.path.clone())).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);

                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    *value = transformation.apply(source_values);
                }

                merge(destination_credential, leaf_node);

                trace_dbg!("Successfully completed transformation");
                None // Todo: this is not implemented yet, so returns None for now
            }

            Transformation::StringToOne {
                type_: transformation,
                source: StringValue { value: source_value },
                destination:
                    DataLocation {
                        format: destination_format,
                        path: destination_path,
                    },
            } => {
                if destination_format != mapping.output_format() {
                    return None;
                }

                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path)).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);
                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    *value = transformation.apply(source_value);
                }

                merge(destination_credential, leaf_node);
                None
            }

            Transformation::JsonToMarkdown {
                type_: transformation,
                source:
                    DataLocation {
                        format: source_format,
                        path: source_path,
                    },
                destination:
                    DataLocation {
                        format: destination_format,
                        path: destination_path,
                    },
            } => {
                if source_format != mapping.input_format() || destination_format != mapping.output_format() {
                    return None;
                }

                let source_credential = self.get(&source_format).unwrap();

                let finder = JsonPathFinder::from_str(&source_credential.to_string(), &source_path).unwrap();

                let source_value = match finder.find().as_array() {
                    // todo: still need to investigate other find() return types
                    Some(array) => array.first().unwrap().clone(),
                    None => {
                        return None;
                    }
                };

                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path.clone())).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);

                // run the source value through a markdown converter to fit the nested objects into a markdown string
                let markdown_source_value = json!(json_to_markdown(&source_value, 0));

                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    *value = transformation.apply(markdown_source_value);
                }

                merge(destination_credential, leaf_node);

                trace_dbg!("Successfully completed transformation");
                Some((destination_path, source_path))
            }

            Transformation::MarkdownToJson {
                type_: transformation,
                source:
                    DataLocation {
                        format: source_format,
                        path: source_path,
                    },
                destination:
                    DataLocation {
                        format: destination_format,
                        path: destination_path,
                    },
            } => {
                if source_format != mapping.input_format() || destination_format != mapping.output_format() {
                    return None;
                }

                let source_credential = self.get(&source_format).unwrap();

                let finder = JsonPathFinder::from_str(&source_credential.to_string(), &source_path).unwrap();

                let source_value = match finder.find().as_array() {
                    // todo: still need to investigate other find() return types
                    Some(array) => array.first().unwrap().clone(),
                    None => {
                        return None;
                    }
                };

                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path.clone())).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);


                if let Some(inner_string) = &source_value.as_str() {
                    let mut lines: Vec<&str> = inner_string.lines().collect();

                    lines.insert(0, "");

                    // Split the string by newlines and collect into Vec<&str>
                    let markdown_function_result = markdown_to_json(&lines);
                    
                    if let Some(value) = leaf_node.pointer_mut(&pointer) {
                        *value = transformation.apply(markdown_function_result);
                    }
                }

  
                


                merge(destination_credential, leaf_node);

                trace_dbg!("Successfully completed transformation");
                Some((destination_path, source_path))
            }

            _ => todo!(),
        }
    }

    pub fn apply_transformations(
        &mut self,
        transformations: Vec<Transformation>,
        mapping: Mapping,
    ) -> Vec<(String, String)> {
        let mut completed_fields: Vec<(String, String)> = Vec::new();

        for transformation in transformations {
            if let Some(completed_field) = self.apply_transformation(transformation, mapping) {
                trace_dbg!(&completed_field);
                completed_fields.push(completed_field);
            }
        }

        completed_fields
    }

    pub fn clear_mapping(&mut self, mut output_pointer: String, mapping: Mapping) {
        let output_json = self.get_mut(&mapping.output_format()).unwrap();

        output_pointer = output_pointer.trim_start_matches("/").to_string();
        let keys: Vec<String> = output_pointer.split('/').map(|s| s.to_string()).collect();

        remove_key_recursive(output_json, &keys);
    }
}

pub fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a @ &mut Value::Array(_), Value::Array(b_arr)) => {
            let a_arr = a.as_array_mut().unwrap();
            let a_len = a_arr.len();
            let b_iter = b_arr.into_iter();

            for (i, b_val) in b_iter.enumerate() {
                if i < a_len {
                    merge(&mut a_arr[i], b_val);
                } else {
                    a_arr.push(b_val);
                }
            }
        }
        (_, Value::Null) => {
            // If the incoming merge Json Value is `Value::Null`, do nothing to the existing Json Value,the current repository, `a`
        }
        (a, b) => *a = b,
    }
}

pub fn update_repository(state: &mut AppState) {
    let output_pointer = state.output_pointer.clone();
    let output_format = state.mapping.output_format();

    let source_value = state.candidate_output_value.clone();
    let output_json = state.repository.get_mut(&output_format).unwrap();

    let mut leaf_node = construct_leaf_node(&output_pointer);

    if let Some(value) = leaf_node.pointer_mut(&output_pointer) {
        *value = serde_json::from_str(&source_value).unwrap();
    }

    merge(output_json, leaf_node);
}

fn remove_key_recursive(current_json: &mut Value, keys: &[String]) -> bool {
    if let Some((first, rest)) = keys.split_first() {
        if let Some(obj) = current_json.as_object_mut() {
            if rest.is_empty() {
                obj.remove(first);
            } else if let Some(child) = obj.get_mut(first) {
                // Recursively traverse deeper layers
                if remove_key_recursive(child, rest) {
                    // If the child is now empty, remove it
                    obj.remove(first);
                }
            }

            // Return true if the current object is empty
            return obj.is_empty();
        }
    }

    false
}

fn json_to_markdown(json: &Value, indent_level: usize) -> String {
    let mut markdown = String::new();
    let indent = "  ".repeat(indent_level);

    match json {
        // Handle JSON objects (key-value pairs)
        Value::Object(map) => {
            for (key, value) in map {
                // Add the key as a bold label
                markdown.push_str(&format!("{}**{}**:\n", indent, key));
                // Recursively handle the value
                markdown.push_str(&json_to_markdown(value, indent_level + 1));
            }
        }

        // Handle JSON arrays
        Value::Array(array) => {
            for item in array {
                // Add each array item as a list item
                markdown.push_str(&format!("{}- ", indent));
                markdown.push_str(&json_to_markdown(item, indent_level + 1));
            }
        }

        // Handle primitive types: strings, numbers, booleans, and null
        Value::String(s) => markdown.push_str(&format!("{}{}\n", indent, s)),
        Value::Number(n) => markdown.push_str(&format!("{}{}\n", indent, n)),
        Value::Bool(b) => markdown.push_str(&format!("{}{}\n", indent, b)),
        Value::Null => markdown.push_str(&format!("{}null\n", indent)),
    }

    markdown
}




/// Recursively converts indented lines of Markdown into a JSON structure.
fn markdown_to_json(lines: &[&str]) -> Value {
    let mut i = 0;
    let mut position: Vec<String> = Vec::new();

    //lets create a string to which we will concatenate new lines based on the markdown lines
    position.insert(0, "".to_string());
    let mut json_string = String::from("");

    while i < lines.len() {
        let line = lines[i];

        // Handle key-value pairs (e.g., **key**: value)
        let (obj_type, depth) = evaluate_line(line);
        if obj_type == "E" && i == 0 {
            // open a json object
            json_string.push_str("{\n");
        }

        if obj_type == "O" {
            while depth < position.len() - 1 {
                // we need to close the positions
                if let Some(last_value) = position.last() {
                    if last_value == "O" {
                        // The last value is O we can now close this object
                        if let Some(_last_value) = position.pop() {
                            json_string.pop();
                            json_string = json_string.trim_end_matches(',').to_string();
                            json_string.push_str("},\n");
                        }
                    } else if last_value == "A" {
                        // close value A
                        if let Some(_last_value) = position.pop() {
                            json_string.push_str("]\n");
                        }
                    } else if last_value == "OA" && depth < position.len() - 1 {
                        //The last value is OA
                        if let Some(_last_value) = position.pop() {
                            // first close the array
                            json_string.pop();
                            json_string = json_string.trim_end_matches(',').to_string();
                            json_string.push_str("],\n");
                            if depth > 0 {
                                if let Some(_last_value) = position.pop() {
                                    // then close the object
                                    json_string.push_str("},\n");
                                }
                            } else {
                                //"The vector was empty, nothing to remove.");
                            }
                        } else {
                            // ("The vector was empty, nothing to remove.");
                        }
                    } else {
                        // we have a different last value we will remove it from tha vector
                        if let Some(_last_value) = position.pop() {
                            // json_string.push_str("]\n");
                        }
                    }
                } else {
                    // println!("The vector is empty.");
                }
            }

            // setup the vector array for the right value and position
            if depth >= position.len() - 1 && i >0 {
                //test previous line to see is we might have a nested object
                let (last_obj_type, _new_depth) = evaluate_line(lines[i - 1]);
                if last_obj_type == "O" {
                    json_string.push('{');
                    json_string.push_str(&cleanup_string(line));
                    json_string.push(':');
                    position.insert(depth, "O".to_string());
                } else if let Some(last_value) = position.last() {
                    if last_value == "OA" && depth == position.len() - 1 {
                        json_string.push_str(&cleanup_string(line));
                        json_string.push(':');
                    } else if depth > position.len() - 1 {
                        json_string.push('{');
                        json_string.push_str(&cleanup_string(line));
                        json_string.push(':');
                        position.insert(depth, "O".to_string());
                    } else {
                        json_string.push_str(&cleanup_string(line));
                        json_string.push(':');
                        position[depth] = "O".to_string();
                    }
                }
            }
        } else if obj_type == "A" {
            while depth < position.len() - 1 {
                // we need to close the positions
                if let Some(last_value) = position.last() {
                    if last_value == "O" {
                        if let Some(_last_value) = position.pop() {
                            json_string.push_str("},\n");
                        }
                    } else if last_value == "A" {
                        if let Some(_last_value) = position.pop() {
                            json_string.push_str("]\n");
                        }
                    } else {
                        // The last value is something leave it
                    }
                } else {
                    // The vector is empty.
                }
            }

            // setup the vector array for the right value and position
            if depth >= position.len() - 1 {
                if let Some(last_value) = position.last() {
                    if last_value == "OA" {
                        json_string.push('[');
                    } else if last_value == "A" && depth == position.len() - 1 {
                        position.insert(depth, "A".to_string());
                    } else {
                        position.insert(depth, "A".to_string());
                        json_string.push('[');
                    }
                }
                json_string.push_str(&cleanup_string(line));
            }
        } else if obj_type == "OA" {
            while depth < position.len() - 1 {
                // we need to close the positions
                if let Some(last_value) = position.last() {
                    if last_value == "O" {
                        if let Some(_last_value) = position.pop() {
                            json_string.pop();
                            json_string.pop();
                            json_string.push_str("},\n");
                        }
                    } else if last_value == "A" {
                        if let Some(_last_value) = position.pop() {
                            json_string.push_str("]\n");
                        } else {
                            // The vector was empty, nothing to remove.
                        }
                    } else {
                        // The last value is something else leave it
                    }
                } else {
                    // The vector is empty
                }
            }

            // we are creating a new array that will contain objects of the same type
            json_string.push_str("[ \n {");
            json_string.push_str(&cleanup_string(line));
            json_string.push(':');
            // test if extra handling is needed for closing
            position.insert(depth - 1, "OA".to_string());
            position.insert(depth, "O".to_string());
        } else if obj_type == "V" {
            json_string.push_str(&cleanup_string(line));
            json_string.push_str(",\n");
        }

        i += 1;
    }

    // Finalize the string to which we will concatenate new lines based on the markdown lines
    json_string.pop();
    json_string = json_string.trim_end_matches(',').to_string();
    json_string.push_str("\n}");
    let parsed_json: Value = serde_json::from_str(&json_string).unwrap();
    parsed_json
}

fn evaluate_line(line_to_test: &str) -> (String, usize) {
    //test depth
    let mut depth = line_to_test
        .chars()
        .take_while(|c| c.is_whitespace())
        .count()
        / 2;
    //test type
    let line_type;
    let trimmed = line_to_test.trim();
    if line_to_test.is_empty() {
        // Handle list as object items of previous depth
        line_type = "E";
    } else if trimmed.starts_with("-") && trimmed.ends_with("**:") {
        // Handle list as object items of previous depth
        line_type = "OA";
        depth += 1;
    } else if trimmed.starts_with("**") {
        // Handle list as object items of previous depth
        line_type = "O";
    } else if trimmed.starts_with("-") {
        // Handle as array items of previous depth
        line_type = "A";
    } else {
        // Handle value of previous depth
        line_type = "V";
    }

    (line_type.to_string(), depth)
}

fn cleanup_string(string_to_clean: &str) -> String {
    //trim the string
    let string_to_clean1 = string_to_clean.trim();
    let string_to_clean2 = string_to_clean1.replace("-", "");
    let string_to_clean3 = string_to_clean2.trim();
    let string_to_clean4 = string_to_clean3.replace("**:", "");
    let cleaned_string = string_to_clean4.trim().trim_matches('*').to_string();
    // Add quotes around the cleaned string
    format!("\"{}\"", cleaned_string)
}


// 1.	Parsing Markdown:
// •	Headings (#): These are treated as keys in the resulting JSON object.
// •	Bold Text (**): This is also treated as a key in the JSON object.
// •	List Items (-): These are treated as elements in a JSON array.
// •	Plain Text: If it’s not part of a list or a key, it’s treated as a value associated with the last key in the current JSON object.
// 2.	Indentation Handling:
// •	The code tracks the current indentation level of the Markdown. If the indentation increases, it means a new nested structure (object or array) is starting. If it decreases, the last completed structure is attached to the parent object or array.
// 3.	Stack Management:
// •	A stack is used to manage the nested structure. Each time a new nested object or array is detected, it’s pushed onto the stack. Once the nesting ends (indentation decreases), the structure is popped from the stack and integrated into the parent structure.
// 4.	Regex Patterns:
// •	heading_regex: Matches Markdown headings (e.g., # Title).
// •	bold_regex: Matches bolded keys (e.g., **Key**:).
// •	list_item_regex: Matches list items (e.g., - item).
