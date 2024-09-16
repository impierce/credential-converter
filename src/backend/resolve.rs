use serde_json::{Map, Value};

use crate::{
    backend::preload_p2::{get_json, truncate_until_char},
    trace_dbg,
};

/// This function takes a ref and replaces the subschema/Value with the resolved ref, entirely.
/// All specs until the latest haven't allowed any additional fields inside the ref object.
/// So practically this function is not fully compliant with the latest specs.
/// However, additional fields inside a ref object in the latest draft are quite complex to utilize and still strongly discouraged.
pub fn resolve_ref(schema: &mut Value, root: Value) {
    if let Some(schema_obj) = schema.as_object() {
        if let Some(ref_) = schema_obj.get("$ref") {
            if let Some(ref_str) = ref_.as_str() {
                // at this point it's fair to assume the json schema ref is invalid if it contains something else than a string.
                if ref_str.starts_with("#/") {
                    let tmp = root.pointer(ref_str.trim_start_matches('#')).unwrap().clone();
                    *schema = tmp;
                } else {
                    let mut path = truncate_until_char(root["$id"].as_str().unwrap(), '/').to_owned()
                        + ref_str.trim_start_matches('.'); // relative paths are valid as "/xx/yy/zz" as well as "./xx/yy/zz" in both cases the root-id folder path is prepended.
                    path = path.trim_start_matches("file://").to_string(); // todo: this is hard-coded logic assuming we will ship the schemas as well, not allowing for http:// paths.
                    trace_dbg!(&path);
                    let tmp = get_json(path).unwrap();
                    *schema = tmp;
                }
            }
        }
    }
}

pub fn resolve_logic_construct(schema: &Value, map: &mut Map<String, Value>) {
    if let Some(all_of) = schema.get("allOf") {
        // Logical construct keys always consist of one array containing single objects
        if let Some(all_of_elmnts) = all_of.as_array() {
            for (i, e) in all_of_elmnts.iter().enumerate() {
                map.insert("allOf/".to_owned() + &i.to_string(), e.clone());
            }
        }
    }
    if let Some(any_of) = schema.get("anyOf") {
        if let Some(any_of_elmnts) = any_of.as_array() {
            for (i, e) in any_of_elmnts.iter().enumerate() {
                map.insert("anyOf/".to_owned() + &i.to_string(), e.clone());
            }
        }
    }
    if let Some(one_of) = schema.get("oneOf") {
        trace_dbg!(3);
        if let Some(one_of_elmnts) = one_of.as_array() {
            for (i, e) in one_of_elmnts.iter().enumerate() {
                map.insert("oneOf/".to_owned() + &i.to_string(), e.clone());
            }
        }
    }
    if let Some(not) = schema.get("not") {
        if let Some(not_elmnts) = not.as_array() {
            for (i, e) in not_elmnts.iter().enumerate() {
                map.insert("not/".to_owned() + &i.to_string(), e.clone());
            }
        }
    }
}

pub fn value_to_str(value: &Value) -> String {
    match value {
        Value::String(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(a) => {
            if a.is_empty() {
                "[]".to_string()
            } else {
                let mut str_ = "[".to_string();
                for e in a {
                    str_ = str_ + value_to_str(e).as_str() + ", ";
                }
                str_.truncate(str_.len() - 2);
                str_ += "]";
                str_.to_string()
            }
        }
        Value::Object(_) => "<Object>".to_string(),
        Value::Null => "".to_string(),
    }
}
