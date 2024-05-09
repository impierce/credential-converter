use std::{any::type_name, fs::File, io::Read, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, to_string, Value};

///////////     FUNCTIONS     ///////////

fn parse_csv_to_matrix(csv_path: &str) -> Vec<Vec<String>> {
    let mut csv_file = File::open(csv_path).expect("Unable to open file");
    let mut csv_buff = String::new();
    csv_file
        .read_to_string(&mut csv_buff)
        .expect("Error reading file");

    let rows = csv_buff.lines();
    let mut matrix = Vec::new();

    for row in rows {
        let columns: Vec<String> = row.split(',').map(|s| s.trim().to_string()).collect(); // Split by commas and trim whitespace
        matrix.push(columns);
    }

    matrix
}

/// Check if mapping exists in csv file.
fn csv_checker(object_a_name: &str, object_b_name: &str, matrix: &Vec<Vec<String>>) -> bool {
    for row in matrix {
        // This checks the entire row, if we want to be 100% sure that the mapping is correct, we should check only column 1 and 4 in the example csv (the "schTitle" columns).
        // Since I'm not yet sure if i can just assume all the csv files that will be provided are the same structure, I'm checking the entire row.
        if row.contains(&object_a_name.to_string()) && row.contains(&object_b_name.to_string()) {
            return true;
        }
    }

    false
}

pub fn json_example(path: &str) -> serde_json::Value {
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader(file).expect("could not parse json")
}

fn mapper(json_path: &str, csv_path: &str, target_format: &str) 
{
    // Read the CSV file into a matrix
    let csv_matrix = parse_csv_to_matrix(csv_path);
    let json_value = json_example(json_path);
    // initializing the json object with the correct* type field
    // *still needs further tailoring to be a 100% correct depending on what `target_format` is passed
    let mut new_json_value = json!({
        "type": [target_format],
    });
    let new_json_value = new_json_value.as_object_mut().unwrap();
    
    if let Some(obj) = json_value.as_object() {
        if let Some(_type) = obj.get("type") {
            // Unsure if schema title will always be findable like this, will be clarified after more introduction to the DESM tools.
            // Assuming the schtitle represents the Json Format, this methodology works for OBv3.
            println!("schTitle1: {:?}", _type);
            for row in &csv_matrix {
                if _type[1] == row[0] && row[3] == target_format {
                    println!("Found mapping: {:?}", row);
                    new_json_value.insert(row[2].clone(), obj.get(&row[1]).unwrap().clone());
                }
            }
        }
        else {
            println!("No type field found in JSON");
        }
    }
    else {
        println!("JSON is not an object");
    }

    println!("New JSON: {:?}", new_json_value);

    let new_path = format!("target/{}.json", target_format);
    let json_file = File::create(new_path).expect("Unable to create file");
    serde_json::to_writer_pretty(json_file, &new_json_value).expect("Unable to write to file");
}

///////////     MAIN     ///////////

fn main() {
    mapper("test.json", "test_equivProps.csv", "Credential Transparency Description Language (CTDL)")
}
