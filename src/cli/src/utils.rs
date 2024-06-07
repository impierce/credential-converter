use std::{collections::HashMap, default};

use crate::repository::Repository;
use ratatui::layout::Rect;
use serde_json::Value;
use strum::{Display, FromRepr};

//////////      STRUCTS & ENUMS     //////////

#[derive(Debug, Default)]
pub struct AppState {
    // Fields for navigation and rendering
    pub tab: Tabs,
    pub p1_prompts: P1Prompts,
    pub mapping: Mapping,
    pub popup_mapper_p2: bool,
    pub popup_selected_transformations: bool,
    pub selected_transformation: usize,
    pub transformations: Transformations,
    pub selected_transformations: Vec<Transformations>,
    // Paths
    pub input_path: String,
    pub mapping_path: String,
    pub output_path: String,
    pub custom_mapping_path: Option<String>,
    // Unused data path will automatically be input_path + "_unused_data.json", but user can change this.
    pub unused_data_path: String,
    // Fields extracted from the input json file.
    pub input_fields: Vec<(String, String)>,
    pub selected_input_field: usize,
    pub amount_input_fields: usize,

    pub missing_data_field: Option<String>,
    pub candidate_data_value: Option<String>,

    pub repository: Repository,

    // Scroll functionality fields
    // Hover Booleans
    pub hover_selector_p2: bool,
    pub hover_popup_p2: bool,
    pub hover_popup_value_p2: bool,
    pub hover_popup_result_path_p2: bool,
    pub hover_popup_result_value_p2: bool,

    // Areas
    // added Area to the appstate because it was problematic to pass it to the event_handler,
    // since the different closures in the main don't combine so easily
    pub area: Rect,
    pub popup_area_p2: Rect,
    pub selector_area_p2: Rect,
    pub popup_value_area_p2: Rect,
    pub popup_result_path_p2: Rect,
    pub popup_result_value_p2: Rect,

    // Scroll offsets/positions
    pub offset_value: u16,
    pub offset_result_path: u16,
    pub offset_result_value: u16,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum P1Prompts {
    #[default]
    Input = 0,
    Output,
    MappingFile,
    Mapping,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum Mapping {
    OBv3ToELM = 0,
    #[default]
    ELMToOBv3,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, Display)]
pub enum Transformations {
    #[default]
    Copy = 0,
    LowerCase,
    UpperCase,
    Split,
    Concat,
    OneToMany,
    ManyToOne,
    Regex,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum Tabs {
    #[default]
    InputPromptsP1 = 0,
    ManualMappingP2,
    UnusedDataP3,
}

#[macro_export]
macro_rules! next_prev {
    ($my_type:ty) => {
        impl $my_type {
            pub fn next(&mut self) {
                let current_index = *self as usize;
                let next_index = current_index.saturating_add(1);
                *self = Self::from_repr(next_index).unwrap_or(*self);
            }

            pub fn prev(&mut self) {
                let current_index = *self as usize;
                let prev_index = current_index.saturating_sub(1);
                *self = Self::from_repr(prev_index).unwrap_or(*self);
            }
        }
    };
}

next_prev!(Mapping);
next_prev!(Tabs);
next_prev!(P1Prompts);
next_prev!(Transformations);

//////////      HELPERS     //////////

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
