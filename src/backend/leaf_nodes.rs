use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn extract_leaf_nodes(json_object: &Value, path: String, result: &mut HashMap<String, Value>) {
    match json_object {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}/{}", path, key)
                };
                extract_leaf_nodes(value, new_path, result);
            }
        }
        _ => {
            result.insert(path, json_object.clone());
        }
    }
}

pub fn get_leaf_nodes(json_object: Value) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    extract_leaf_nodes(&json_object, String::new(), &mut result);
    result
        .into_iter()
        .map(|(key, value)| (format!("/{}", key), value))
        .collect()
}

pub fn construct_leaf_node(path: &str) -> Value {
    // Split the input string by '/' and filter out any empty parts
    let parts: Vec<&str> = path.split('/').filter(|&s| !s.is_empty()).collect();

    // Initialize the leaf_node value to Null, the actual value will be inserted later by the apply_transformation function
    let mut current_value = Value::Null;


    // Iterate through the parts in reverse order to build the nested structure
    for part in parts.into_iter().rev() {
        // Check if the part is an array index
        if part.chars().all(|c| c.is_numeric()) {
            if let Ok(part_array_index) = part.parse::<usize>() {
                let mut new_array = Vec::new();
                for i in 0..part_array_index {
                    new_array.insert(i, Value::Null);
                }

                new_array.insert(part_array_index, current_value);
                current_value = Value::Array(new_array);
            }
        }
        else {
            let mut new_object = Map::new();
            new_object.insert(part.to_string(), current_value);
            current_value = Value::Object(new_object);
                
        }
    }

    current_value
}
