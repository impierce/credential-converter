use crate::{
    backend::{
        jsonpointer::{JsonPath, JsonPointer},
        transformations::{DataLocation, Transformation},
    },
    state::{AppState, Pages},
};
use jsonpath_rust::JsonPathFinder;
use regex::Regex;
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
    // fn new() -> Self {
    //     Self(HashMap::new())
    // }

    pub fn apply_transformation(&mut self, transformation: Transformation) {
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
                let source_credential = self.get(&source_format).unwrap();

                if source_path == "$.@context" {
                    source_path = r#"$["@context"]"#.to_string();
                };
                let finder = JsonPathFinder::from_str(&source_credential.to_string(), &source_path).unwrap();
                let source_value = finder.find().as_array().unwrap().first().unwrap().clone();

                // trace_dbg!(&destination_path);
                let destination_credential = self.entry(destination_format).or_insert(json!({})); // or_insert should never happen, since repository is initialized with all formats, incl empty json value when not present.
                let pointer = JsonPointer::try_from(JsonPath(destination_path)).unwrap();
                // trace_dbg!(&pointer);

                let mut leaf_node = construct_leaf_node(&pointer);

                // trace_dbg!(&leaf_node);
                if let Some(value) = leaf_node.pointer_mut(&pointer) {
                    // trace_dbg!(&value);
                    *value = transformation.apply(source_value);
                    // trace_dbg!(&value);
                }
                // trace_dbg!(&leaf_node);
                merge(destination_credential, leaf_node);
            }
            Transformation::ManyToOne {
                type_: transformation,
                sources,
                destination,
            } => {
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
            }

            _ => todo!(),
        }
    }

    pub fn apply_transformations(&mut self, transformations: Vec<Transformation>) {
        for transformation in transformations {
            self.apply_transformation(transformation)
        }
    }
}

pub fn construct_leaf_node(path: &str) -> Value {
    // Split the input string by '/' and filter out any empty parts
    let parts: Vec<&str> = path.split('/').filter(|&s| !s.is_empty()).collect();

    // Initialize the root of the JSON structure as null // todo: isn't this actually the value of the leaf node, not the root?
    let mut current_value = Value::Null;

    // Iterate through the parts in reverse order to build the nested structure
    for part in parts.into_iter().rev() {
        let mut new_object = Map::new();
        new_object.insert(part.to_string(), current_value);
        current_value = Value::Object(new_object);
    }

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
    let output_format = state.mapping.output_format();
    let mut output_pointer = state.missing_field_pointer.trim_start_matches("/required").to_string(); // todo: remove subset paths segment is double code, also in selector
    if state.page == Pages::UnusedDataP3 {
        output_pointer = state.optional_field_pointer.trim_start_matches("/optional").to_string();
    }
    if output_pointer.contains("/allOf") || output_pointer.contains("/anyOf") || output_pointer.contains("/oneOf") || output_pointer.contains("/not"){
        let re_allof = Regex::new(r"allOf/.*/").unwrap();
        let re_anyof = Regex::new(r"anyOf/.*/").unwrap();
        let re_oneof = Regex::new(r"oneOf/.*/").unwrap();
        let re_not = Regex::new(r"not/.*/").unwrap();
    
        output_pointer = re_allof.replace_all(&output_pointer, "").to_string();
        output_pointer = re_anyof.replace_all(&output_pointer, "").to_string();
        output_pointer = re_oneof.replace_all(&output_pointer, "").to_string();
        output_pointer = re_not.replace_all(&output_pointer, "").to_string();
    }

    let source_value = state.candidate_data_value.clone().unwrap();
    let output_json = state.repository.get_mut(&output_format).unwrap();

    let mut leaf_node = construct_leaf_node(&output_pointer);

    if let Some(value) = leaf_node.pointer_mut(&output_pointer) {
        *value = serde_json::from_str(&source_value).unwrap();
    }

    // trace_dbg!(&leaf_node);

    merge(output_json, leaf_node);
    // trace_dbg!(output_json);
}
