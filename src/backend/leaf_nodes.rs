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

    // Initialize the root of the JSON structure as null
    let mut current_value = Value::Null;


    // Iterate through the parts in reverse order to build the nested structure
    for part in parts.into_iter().rev() {
        // handle arrays differnt 
        if part.contains("[]") { // todo: this should include the number ofcourse
            let part_array: &str = &part[..part.len()-2];

            let v: Vec<Value> = Vec::new();

            // // ... fill in the vec with some Value::Object's as you like it ...
            // // ... in our case its one or more strings 
            
            let a = Value::Array(v);

            let mut new_object = Map::new();
            new_object.insert(part_array.to_string(),a);
            current_value = Value::Object(new_object);
        }
        else {
            let mut new_object = Map::new();
            new_object.insert(part.to_string(), current_value);
            current_value = Value::Object(new_object);
                
        }
    }

    // if the end of the path indicates an array than add an array to the leaf and return the array to be filled with a value

    current_value
}
