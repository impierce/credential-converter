use clap::ValueEnum;
use ratatui::layout::Rect;
use serde_json::Value;
use std::{borrow::Cow, collections::HashMap};
use strum::{AsRefStr, Display, FromRepr};

use crate::backend::{repository::Repository, transformations::Transformation};

//////////      STRUCTS & ENUMS     //////////

#[derive(Debug, Default, Clone)]
pub struct AppState {
    // Main pages of the program
    pub page: Pages,

    // User input prompts on P1
    pub p1_prompts: P1Prompts,
    pub language: Languages,
    pub mapping: Mapping,

    // Paths
    pub input_path: String,
    pub mapping_path: String,
    pub output_path: String,
    pub custom_mapping_path: String,

    // Tabs for P2 and P3
    pub p2_p3_tabs: P2P3Tabs,
    pub selected_transformations_tab: bool,
    pub select_mapping_option: bool,

    // Popups
    pub overwrite_warning: bool,
    pub uncompleted_warning: bool,
    pub popup_mapping_p2_p3: bool,
    pub exit_warning: bool,
    pub return_to_p1_warning: bool,

    // Mapping options
    pub mapping_option: MappingOptions,
    pub transformations: Transformations,
    pub selected_transformation: usize,
    pub selected_transformations: Vec<Transformations>,

    // Transformation data, this can't be added to the Transformations enum since this will break it's loopability in the UI.
    pub dividers: String,
    pub transformation_index: Option<usize>,

    // Fields extracted from the input json file.
    pub input_fields: Vec<(String, String)>,
    pub amount_input_fields: usize,
    pub selected_input_field: usize,
    pub input_field_pointer: String,                  // todo: not in use yet
    pub input_display_section: Vec<(String, String)>, // todo: not in use yet

    // Fields extracted from the output json schema.
    pub output_display_subset: Vec<(String, String)>,
    pub selected_output_field: usize,
    pub amount_output_fields: usize,

    pub required_field_pointer: String,
    pub optional_field_pointer: String,
    pub output_pointer: String,

    pub completed_fields: Vec<(String, String)>, // (output_field_path, input_field_path)

    // Backend
    pub target_schema: Value,
    pub resolved_subsets: HashMap<String, Value>, // <JsonPointer, Value> JsonPointers is unnecessary complexity for now.
    pub candidate_output_value: String,
    pub repository: Repository,
    pub performed_mappings: Vec<Transformation>,

    // Total Area
    pub area: Rect,

    // Areas for scrolling
    pub selector_area_p2_p3: Rect,
    pub output_fields_area_p2_p3: Rect,
    pub popup_path_area: Rect,
    pub popup_value_area: Rect,
    pub popup_output_path: Rect,
    pub popup_output_result: Rect,

    // Areas for clicking
    pub clear_button: Rect,
    pub view_button: Rect,
    pub complete_button: Rect,
    pub prev_page_button: Rect,
    pub confirm_button: Rect,

    // Scroll offsets and line amounts popup
    pub popup_offset_value: u16,
    pub popup_offset_path: u16,
    pub popup_offset_result: u16,
    pub popup_offset_output_path: u16,
    pub popup_amount_lines_value: usize,
    pub popup_amount_lines_path: usize,
    pub popup_amount_lines_output_path: usize,
    pub popup_amount_lines_result: usize,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, AsRefStr)]
pub enum Languages {
    #[default]
    EN = 0,
    NL,
    BG,
    CS,
    DA,
    DE,
    EL,
    ES,
    ET,
    FI,
    FR,
    HR,
    HU,
    IS,
    IT,
    LT,
    LV,
    PL,
    PT,
    RO,
    RU,
    SK,
    SL,
    SV,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum P2P3Tabs {
    #[default]
    InputFields = 0,
    OutputFields,
    MappingOptions,
    Clear,
    View,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum P1Prompts {
    #[default]
    Language = 0,
    Input,
    Output,
    MappingFile,
    Mapping,
    CustomMapping,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, ValueEnum, Display)]
pub enum Mapping {
    #[clap(name = "OBv3toELM")]
    OBv3ToELM = 0,
    #[default]
    #[clap(name = "ELMtoOBv3")]
    ELMToOBv3,
}

impl Mapping {
    pub fn input_format(&self) -> String {
        match self {
            Mapping::OBv3ToELM => "OBv3".to_string(),
            Mapping::ELMToOBv3 => "ELM".to_string(),
        }
    }

    pub fn output_format(&self) -> String {
        match self {
            Mapping::OBv3ToELM => "ELM".to_string(),
            Mapping::ELMToOBv3 => "OBv3".to_string(),
        }
    }
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, Display)]
pub enum MappingOptions {
    #[default]
    DirectCopy,
    Transformations,
    OneToMany,
    ManyToOne,
}
#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, Display)]
pub enum Transformations {
    DirectCopy,
    #[default]
    LowerCase,
    UpperCase,
    TakeIndex, // should be included in Slice
    Slice,     // Slice should accept indexing singular points and ranges in the future, e.g. (9) or (1..9)
    Regex,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum Pages {
    #[default]
    InputPromptsP1 = 0,
    RequiredDataP2,
    OptionalDataP3,
    EndP4,
}

//////////      HELPERS     //////////

pub fn translate(tag: &str) -> Cow<str> {
    rust_i18n::t!(tag, pwd = std::env::current_dir().unwrap().display())
}

#[macro_export]
macro_rules! next_prev {
    ($my_type:ty, $min:expr, $max:expr) => {
        impl $my_type {
            pub fn next(&mut self) {
                let current_index = *self as usize;
                let max_index = $max as usize;
                let next_index = if current_index == max_index {
                    $min as usize
                } else {
                    current_index + 1
                };
                *self = Self::from_repr(next_index).unwrap_or(*self);
            }

            pub fn prev(&mut self) {
                let current_index = *self as usize;
                let min_index = $min as usize;
                let max_index = $max as usize;
                let prev_index = if current_index == min_index {
                    max_index
                } else {
                    current_index.saturating_sub(1)
                };
                *self = Self::from_repr(prev_index).unwrap_or(*self);
            }
        }
    };
}

// todo: choose to use next_prev with or without loop or both

// #[macro_export]
// macro_rules! next_prev {
//     ($my_type:ty) => {
//         impl $my_type {
//             pub fn next(&mut self) {
//                 let current_index = *self as usize;
//                 let next_index = current_index.saturating_add(1);
//                 *self = Self::from_repr(next_index).unwrap_or(*self);
//             }

//             pub fn prev(&mut self) {
//                 let current_index = *self as usize;
//                 let prev_index = current_index.saturating_sub(1);
//                 *self = Self::from_repr(prev_index).unwrap_or(*self);
//             }
//         }
//     };
// }

next_prev!(Languages, Languages::EN, Languages::SV);
next_prev!(Mapping, Mapping::OBv3ToELM, Mapping::ELMToOBv3);
next_prev!(Pages, Pages::InputPromptsP1, Pages::EndP4);
next_prev!(P1Prompts, P1Prompts::Language, P1Prompts::CustomMapping);
next_prev!(Transformations, Transformations::LowerCase, Transformations::Regex);
next_prev!(MappingOptions, MappingOptions::DirectCopy, MappingOptions::ManyToOne);
next_prev!(P2P3Tabs, P2P3Tabs::InputFields, P2P3Tabs::View);
