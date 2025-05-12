use serde_json::Value;
use std::collections::HashMap;

use crate::{
    backend::jsonpointer::{JsonPath, JsonPointer},
    trace_dbg,
};

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

/// Needs the complete path in Pointer format, with '/'.
pub fn delete_leaf_node(repository: &mut Value, path: String) {
    // Convert the path to a JSON pointer
    if let Some((parent, child)) = path.rsplit_once('.') {
        let parent_pointer =
            JsonPointer::try_from(JsonPath(parent.to_string())).expect("JsonPath to JsonPointer conversion failed");

        if let Some(parent_value) = repository.pointer_mut(&parent_pointer.to_string()) {
            if let Some(parent_object) = parent_value.as_object_mut() {
                // Remove the child key from the parent object
                parent_object.remove(child);

                let dbg_message = format!("Removed leaf node {}", path);
                trace_dbg!(dbg_message);
            }
        }
    } else if let Some(parent_object) = repository.as_object_mut() {
        // Remove the child key from the parent object
        parent_object.remove(&path);

        let dbg_message = format!("Removed leaf node {}", path);
        trace_dbg!(dbg_message);
    }
}
