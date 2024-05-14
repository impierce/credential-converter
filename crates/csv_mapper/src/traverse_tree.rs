use serde_json::{Map, Value};

use crate::{MappingData, MappingRule};

/// Source object, target object
pub fn traverse_tree(
    src_obj: Map<String, Value>,
    tgt_obj: &mut Map<String, Value>,
    mapping_data: &MappingData,
    current_path: &str,
) {
    let rules = mapping_data.get(current_path).unwrap();

    for rule in rules.iter() {
        let src_value = src_obj.get(&rule.src_schema_field);

        if let Some(value) = src_value {
            if value.is_string() || value.is_number() || value.is_boolean() {
                // Direct mapping
                insert_into_target(tgt_obj, value, rule, "$");
            } else {
                // Indirect mapping
                //insert_into_target(target_obj, value, rule, current_path.to_string());
            }
            // insert type
            tgt_obj.insert(
                "type".to_string(),
                serde_json::to_value(rule.target_schema_object.to_string()).unwrap(),
            );
        } else {
            panic!("missing field error");
        }
    }

    //let src_type = src_obj.get("type");

    //rules[0].target_schema_field
}

fn insert_into_target(
    target_obj: &mut Map<String, Value>,
    value: &Value,
    rule: &MappingRule,
    current_path: &str,
) {
    if rule.target_json_path == current_path {
        // Apply directly
        target_obj.insert(rule.target_schema_field.to_string(), value.clone());
    } else {
        // recursive action
    }
}
