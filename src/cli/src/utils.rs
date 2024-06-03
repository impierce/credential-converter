use strum::FromRepr;

#[derive(Debug, Default)]
pub struct AppState {
    pub input_path: String,
    pub output_path: String,
    pub unused_data_path: String,
    pub tab: Tabs,
    pub path_prompts: PathPrompts,
    pub selected_input_field: usize,
    pub amount_input_fields: usize
}

#[derive(Clone, Copy, FromRepr, Debug, Default, PartialEq)]
pub enum PathPrompts {
    #[default]
    Input = 0,
    Output,
    UnusedData,
}

#[derive(Clone, Copy, FromRepr, Debug, Default)]
pub enum Tabs {
    #[default]
    InputPromptsP1 = 0,
    ManualMappingP2,
    UnusedDataP3,
}

impl Tabs {

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn prev(self) -> Self {
        let current_index = self as usize;
        let prev_index = current_index.saturating_sub(1);
        Self::from_repr(prev_index).unwrap_or(self)
    }
}

impl PathPrompts {
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn prev(self) -> Self {
        let current_index = self as usize;
        let prev_index = current_index.saturating_sub(1);
        Self::from_repr(prev_index).unwrap_or(self)
    }
}
