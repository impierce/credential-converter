use jsonpath_rust::path;
use serde_json::{Map, Value};

use crate::state::{self, Pages};

use super::preload_p2::{get_required_fields, resolve_logic_construct};

pub fn value_to_str(value: &Value) -> String {
    match value {
        Value::String(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => {
            let mut str = "[".to_string();
            for e in a {
                str = str + value_to_str(e).as_str() + ", "; //again we don't how many levels nested array/objects
            }
            str.truncate(str.len() - 2);
            str = str + "]";
            str.to_string()
        }
        Value::Object(_) => "<Object>".to_string(),
        Value::Null => "".to_string(),
    }
}

pub fn update_path(state: &mut state::AppState) {
    
}

pub fn update_display_section(state: &mut state::AppState) {
    let mut tmp_map = Map::new();
    let mut path = &state.missing_field_pointer;
    if state.page == Pages::ManualMappingP2 {
        
    }
    get_required_fields(&mut state.target_schema, path, &mut tmp_map);
    resolve_logic_construct(&mut state.target_schema, path, &mut tmp_map);

    state
        .resolved_subsets
        .insert(state.missing_field_pointer.clone(), Value::from(tmp_map));

}
