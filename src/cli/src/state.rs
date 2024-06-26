use ratatui::layout::Rect;
use strum::{Display, FromRepr};

use crate::backend::repository::Repository;

//////////      STRUCTS & ENUMS     //////////

#[derive(Debug, Default)]
pub struct AppState {
    // Fields for navigation and rendering
    pub page: Pages,
    pub p1_prompts: P1Prompts,
    pub p2_p3_tabs: P2P3Tabs,
    pub popup_mapping_p2_p3: bool,
    pub selected_transformations_tab: bool,
    pub select_multiplicity: bool,
    pub output_warning: bool,
    pub review: bool,
    pub popup_uncompleted_warning: bool,
    pub popup_unused_data: bool,

    // Mapping options
    pub mapping: Mapping,
    pub multiplicity: Multiplicity,
    pub transformations: Transformations,
    pub selected_transformation: usize,
    pub selected_transformations: Vec<Transformations>,
    pub dividers: String,

    // Paths
    pub input_path: String,
    pub mapping_path: String,
    pub output_path: String,
    pub custom_mapping_path: Option<String>,
    pub unused_data_path: String,

    // Fields extracted from the input json file.
    pub input_fields: Vec<(String, String)>,
    pub amount_input_fields: usize,
    pub selected_input_field: usize,
    pub selected_input_fields: Vec<String>, //string or usize?

    // Mandatory fields extracted from the output json format
    pub missing_data_field: Option<String>,
    pub missing_data_fields: Vec<(String, String)>,
    pub amount_missing_fields: usize,
    pub selected_missing_field: usize,
    pub selected_missing_fields: Vec<String>, //string or usize?

    // Optional fields extracted from the output json format
    pub optional_fields: Vec<(String, String)>,
    pub amount_optional_fields: usize,
    pub selected_optional_field: usize,
    pub selected_optional_fields: Vec<String>, //string or usize?

    pub candidate_data_value: Option<String>,
    pub completed_input_fields: Vec<usize>,
    pub completed_missing_fields: Vec<usize>,
    pub completed_optional_fields: Vec<usize>,

    pub repository: Repository,

    // Areas, for scrolling and clicking
    pub area: Rect,
    pub selector_area_p2_p3: Rect,
    pub output_fields_area_p2_p3: Rect,
    pub popup_path_area_p2: Rect,
    pub popup_value_area_p2: Rect,
    pub abort_button: Rect,
    pub review_button: Rect,
    pub complete_button: Rect,
    pub prev_page_button: Rect,
    pub popup_input_path_p2: Rect,
    pub popup_output_path_p2: Rect,
    pub popup_input_value_p2: Rect,
    pub popup_output_result_p2: Rect,
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

    // testcase
    pub complete: bool, // change into button, thus a click event, not key press event
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum P2P3Tabs {
    #[default]
    InputFields = 0,
    OutputFields,
    MappingOptions,
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
pub enum Multiplicity {
    #[default]
    OneToOne,
    OneToMany,
    ManyToOne,
}
#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq, Display)]
pub enum Transformations {
    #[default]
    Copy = 0,
    LowerCase,
    UpperCase,
    Slice,
    Regex,
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum Pages {
    #[default]
    InputPromptsP1 = 0,
    ManualMappingP2,
    UnusedDataP3,
    EndP4,
}

// test
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
//

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

next_prev!(Mapping, Mapping::OBv3ToELM, Mapping::ELMToOBv3);
next_prev!(Pages, Pages::InputPromptsP1, Pages::EndP4);
next_prev!(P1Prompts, P1Prompts::Input, P1Prompts::Mapping);
next_prev!(Transformations, Transformations::Copy, Transformations::Regex);
next_prev!(Multiplicity, Multiplicity::OneToOne, Multiplicity::ManyToOne);
next_prev!(P2P3Tabs, P2P3Tabs::InputFields, P2P3Tabs::MappingOptions);
