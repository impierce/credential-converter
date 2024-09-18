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
use regex::Regex;
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

                let dest = destination_path.clone();
                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path)).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);
                if dest.contains("[]") {
                    // Deserialize the JSON string into a MyStruct
                    // remove the array element from the destantation and reuse this new pointer
                    // fill the array with the new value
                    let array_pointer = &pointer[..pointer.len() - 2];
                    if let Some(array) = leaf_node.pointer_mut(&array_pointer).and_then(|v| v.as_array_mut()) {
                        // Push a new string to the array
                        array.push(Value::String(source_value));
                    }
                } else if let Some(value) = leaf_node.pointer_mut(&pointer) {
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

                // run the source value through a markdown converter to fit the nested objects into a markdown string
                let json_source_value = json!(markdown_to_json(&source_value.to_string()));

                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    *value = transformation.apply(json_source_value);
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
    let indent = "    ".repeat(indent_level);

    match json {
        Value::Object(map) => {
            for (key, value) in map {
                markdown.push_str(&format!("{}**{}**:\n", indent, key));
                markdown.push_str(&json_to_markdown(value, indent_level + 1));
                markdown.push('\n');
            }
        }
        Value::Array(arr) => {
            for item in arr {
                markdown.push_str(&format!(
                    "{}- {}\n",
                    indent,
                    json_to_markdown(item, indent_level + 1).trim()
                ));
            }
        }
        Value::String(s) => {
            markdown.push_str(&format!("{}{}\n", indent, s));
        }
        Value::Number(n) => {
            markdown.push_str(&format!("{}{}\n", indent, n));
        }
        Value::Bool(b) => {
            markdown.push_str(&format!("{}{}\n", indent, b));
        }
        Value::Null => {
            markdown.push_str(&format!("{}null\n", indent));
        }
    }

    markdown
}

fn markdown_to_json(markdown: &str) -> Value {
    let mut lines = markdown.lines().peekable();
    let mut current_indent = 0;
    let mut stack: Vec<Value> = vec![Value::Object(Default::default())];
    let mut current_key: Option<String> = None;

    let heading_regex = Regex::new(r"^#+ (.+)").unwrap();
    let bold_regex = Regex::new(r"^\s*\*\*(.+?)\*\*\s*:$").unwrap();
    let list_item_regex = Regex::new(r"^\s*-\s*(.+)").unwrap();

    while let Some(line) = lines.next() {
        let line_indent = line.chars().take_while(|c| c.is_whitespace()).count();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Adjust stack based on indentation
        if line_indent > current_indent {
            stack.push(Value::Object(Default::default()));
        } else if line_indent < current_indent {
            let value = stack.pop().unwrap();
            let parent = stack.last_mut().unwrap();
            if let Some(key) = current_key.take() {
                if let Value::Object(ref mut obj) = parent {
                    obj.insert(key, value);
                }
            } else if let Value::Array(ref mut arr) = parent {
                arr.push(value);
            }
        }

        current_indent = line_indent;

        if let Some(caps) = heading_regex.captures(line) {
            let heading = caps.get(1).unwrap().as_str().trim().to_string();
            current_key = Some(heading);
        } else if let Some(caps) = bold_regex.captures(line) {
            let key = caps.get(1).unwrap().as_str().trim().to_string();
            let parent = stack.last_mut().unwrap();
            if let Value::Object(ref mut obj) = parent {
                obj.insert(key.clone(), Value::Null);
            }
            current_key = Some(key);
        } else if let Some(caps) = list_item_regex.captures(line) {
            let item = caps.get(1).unwrap().as_str().trim().to_string();
            let parent = stack.last_mut().unwrap();
            if let Value::Array(ref mut arr) = parent {
                arr.push(Value::String(item));
            } else {
                let arr = vec![Value::String(item)];
                stack.push(Value::Array(arr));
            }
        } else {
            let parent = stack.last_mut().unwrap();
            if let Some(key) = current_key.take() {
                if let Value::Object(ref mut obj) = parent {
                    obj.insert(key, Value::String(line.to_string()));
                }
            } else if let Value::Array(ref mut arr) = parent {
                arr.push(Value::String(line.to_string()));
            }
        }
    }

    // Handle remaining items in the stack
    while stack.len() > 1 {
        let value = stack.pop().unwrap();
        let parent = stack.last_mut().unwrap();
        if let Some(key) = current_key.take() {
            if let Value::Object(ref mut obj) = parent {
                obj.insert(key, value);
            }
        } else if let Value::Array(ref mut arr) = parent {
            arr.push(value);
        }
    }

    stack.pop().unwrap()
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
