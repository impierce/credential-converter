use ratatui::layout::Rect;
use strum::{Display, FromRepr};

use crate::backend::repository::Repository;

//////////      STRUCTS & ENUMS     //////////

#[derive(Debug, Default)]
pub struct AppState {
    // Fields for navigation and rendering
    pub tab: Tabs,
    pub p1_prompts: P1Prompts,
    pub p2_tabs: P2Tabs,
    pub popup_p2: bool,
    pub selected_transformations_tab: bool,
    pub select_multiplicity: bool,
    pub output_warning: bool,

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
    // Unused data path will automatically be input_path + "_unused_data.json", but user can change this.
    pub unused_data_path: String,

    // Fields extracted from the input json file.
    pub input_fields: Vec<(String, String)>,
    pub amount_input_fields: usize,
    pub selected_input_field: usize,
    pub selected_input_fields: Vec<String>,

    // Fields extracted from the output json format
    pub missing_data_field: Option<String>,
    pub missing_data_fields: Option<Vec<String>>,
    pub amount_missing_fields: usize,
    pub selected_missing_field: usize,
    pub selected_missing_fields: Vec<String>,

    pub candidate_data_value: Option<String>,

    pub repository: Repository,

    // Areas, for scrolling and clicking
    pub area: Rect,
    pub selector_area_p2: Rect,
    pub missing_fields_area_p2: Rect,
    pub popup_area_p2: Rect,
    pub popup_value_area_p2: Rect,
    pub popup_result_path_p2: Rect,
    pub popup_result_value_p2: Rect,
    pub back_area_p2: Rect,
    pub confirm_area_p2: Rect,
    pub finish_button: Rect,
    pub prev_page_button: Rect,

    // Scroll offsets/positions
    pub offset_value: u16,
    pub offset_result_path: u16,
    pub offset_result_value: u16,

    // test

    // testcase
    pub finish: bool, // change into button, thus a click event, not key press event
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum P2Tabs {
    #[default]
    InputFields = 0,
    MissingFields,
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
next_prev!(Multiplicity);
next_prev!(P2Tabs);
