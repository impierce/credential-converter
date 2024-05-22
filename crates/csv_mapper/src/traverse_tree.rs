#![allow(unused)]
use std::str::FromStr;

use crate::{MappingData, MappingRule};
use jsonpath_rust::{JsonPathFinder, JsonPathInst, JsonPathQuery, JsonPathValue};
use serde_json::{Map, Value};

/// Source object, target object

pub type JsonPathKV = Vec<(String, String)>;

pub fn store_all_json_paths(
    src_obj: &Map<String, Value>,
    json_path: String,
    out: &mut JsonPathKV,
) {
    fn traverse_array(arr: &Vec<Value>, json_path: String, out: &mut JsonPathKV) {
        for (i, value) in arr.iter().enumerate() {
            let new_path = format!("{json_path}[:{i}]");

            match value {
                Value::Bool(b) => out.push((new_path, b.to_string())),
                Value::Number(n) => out.push((new_path, n.to_string())),
                Value::String(s) => out.push((new_path, s.to_string())),
                Value::Object(obj) => store_all_json_paths(obj, new_path, out),
                Value::Array(arr) => traverse_array(arr, new_path, out),
                Value::Null => {
                    // todo
                }
            }
        }
    }

    for item in src_obj.iter() {
        if item.0 == "@context" {
            continue;
        }

        let json_path = format!("{json_path}.{}", item.0);

        match item.1 {
            Value::Bool(b) => out.push((json_path, b.to_string())),
            Value::Number(n) => out.push((json_path, n.to_string())),
            Value::String(s) => out.push((json_path, s.to_string())),
            Value::Object(obj) => store_all_json_paths(obj, json_path, out),
            Value::Array(arr) => traverse_array(arr, json_path, out),
            Value::Null => {
                //todo
            }
        }
    }
}

pub fn traverse_source(
    src_obj: Value,
    tgt_obj: &mut Map<String, Value>,
    mapping_data: &MappingData,
) {
    let box_val = Box::new(src_obj);

    for rule in mapping_data.iter() {
        //let p = JsonPathInst::from_str(&rule.src_path).unwrap();
        //let pf = JsonPathFinder::new(&box_val, Box::new(p));

        ////.find()
        //if let Ok(src_val) = pf.find() {
            //insert_into_target(tgt_obj, src_val, rule, 0);
        //}
    }
}

fn insert_type(obj: &mut Map<String, Value>, type_str: &str) {
    obj.insert(
        "type".to_string(),
        serde_json::to_value(type_str.to_string()).unwrap(),
    );
}

fn insert_into_target(
    tgt_obj: &mut Map<String, Value>,
    value: Value,
    rule: &MappingRule,
    current_segment: usize,
) {
    //
}
