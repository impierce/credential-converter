use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        transformations::{DataLocation, StringValue, Transformation},
    },
    state::{AppState, Mapping},
    trace_dbg,
};
use jsonpath_rust::JsonPathFinder;
use serde_json::{json, Map, Value};
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
                source:
                    StringValue {
                        value: source_value,
                    },
                destination:
                    DataLocation {
                        format: destination_format,
                        path: destination_path,
                    },
            } => {
                if destination_format != mapping.output_format() {
                    return None;
                }

                let dest  = destination_path.clone(); 
                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path)).unwrap();

                let mut leaf_node = construct_leaf_node(&pointer);
                if dest.contains("[]") {
                    // Deserialize the JSON string into a MyStruct
                    // remove the array element from the destantation and reuse this new pointer
                    // fill the array with the new value
                    let array_pointer = &pointer[..pointer.len()-2];
                    if let Some(array) = leaf_node.pointer_mut(&array_pointer) .and_then(|v| v.as_array_mut())  {
                        // Push a new string to the array
                            array.push(Value::String(source_value));
                    }
                }
                else if let Some(value) = leaf_node.pointer_mut(&pointer) {
                   *value = transformation.apply(source_value);
                }

                merge(destination_credential, leaf_node);
                None
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

pub fn construct_leaf_node(path: &str) -> Value {
    // Split the input string by '/' and filter out any empty parts
    let parts: Vec<&str> = path.split('/').filter(|&s| !s.is_empty()).collect();

    // Initialize the root of the JSON structure as null // todo: isn't this actually the value of the leaf node, not the root?
    let mut current_value = Value::Null;


    // Iterate through the parts in reverse order to build the nested structure
    for part in parts.into_iter().rev() {
        // handle arrays differnt 
        if part.contains("[]") {
            let part_array: &str = &part[..part.len()-2];

            let v: Vec<Value> = Vec::new();

            // // ... fill in the vec with some Value::Object's as you like it ...
            // // ... in our case its one or more strings 
            
            let a = Value::Array(v);
            // let mut map: serde_json::Map<String,Value> = serde_json::Map::new();
            // map.insert("person",a);
            // let o = Value::Object(map);

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

pub fn merge(a: &mut Value, b: Value) {
    // todo: here anything non object is actually simply overwritten. So here we need to introduce type checking of the Value b.
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v); //
            }
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
