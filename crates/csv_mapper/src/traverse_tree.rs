use serde_json::{Map, Value};

use crate::{MappingData, MappingRule};

/// Source object, target object
pub fn traverse_source(
    src_obj: &Map<String, Value>,
    tgt_obj: &mut Map<String, Value>,
    mapping_data: &MappingData,
    current_path: &str,
) {
    let rules = mapping_data.get(current_path).unwrap();

    for rule in rules.iter() {
        let field_value = src_obj.get(&rule.src_schema_field);
        let schema_value = &rule.src_schema_type;

        if let Some(value) = field_value {
            if value.is_string() || value.is_number() || value.is_boolean() {
                // Direct mapping
                insert_into_target(tgt_obj, value, rule, "$");
                continue;
            }

            match value {
                Value::Object(src_obj) => {
                    traverse_source(src_obj, tgt_obj, mapping_data, &rule.src_schema_field);
                }
                Value::Array(arr) => {
                    for item in arr.iter() {
                        if let Value::Object(src_obj) = item {
                            traverse_source(src_obj, tgt_obj, mapping_data, &rule.src_schema_field);
                        }
                    }
                }
                _ => {}
            }

            insert_target_type(tgt_obj, &rule.target_schema_object);
        } else {
            panic!("missing field error");
        }
    }
}

fn insert_target_type(tgt_obj: &mut Map<String, Value>, type_str: &str) {
    tgt_obj.insert(
        "type".to_string(),
        serde_json::to_value(type_str.to_string()).unwrap(),
    );
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
        // If object with type exist use it
        // otherwise create
    }
}
