use std::{io::{self, Write}, path::{Path, PathBuf}};
//
use std::fs::File;
use tokio::fs;
use csv_mapper::{mapping_data::MappingRule, parse_csv};
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("\n\n~~~~~   Welcome to the 'OpenBadges-ELM Mapper' command line interface!   ~~~~~\n\n");

    let path_input  = prompt_input_path("Please enter the path to the JSON file you would like to map,\nFor relative paths, the current directory is ");
    let path_target = prompt_output_path("Please enter the path to the output file you would like to use for mapping");
    // let path_format = prompt_option("Please enter the format of the input JSON file ", vec!["OBv3", "ELM"]);
    // Since we only have two options currently, we can assume target is the other format than the input.
    // This makes the next prompt unnecessary. Still debatable how to inform the user of this automatic deduction.
    // let target_format = prompt("Please enter the target format for the new JSON file [OBv3, ELM]");

    csv_mapper::generate_json_paths(&path_input).await;
    csv_mapper::add_schema_paths().await;
    println!("TEST");
    mapper(
        "../target/json_paths.csv",
        "test_master_schema.csv",
        &path_target
    ).await;

}

///////////     FUNCTIONS     ///////////

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

async fn get_mapping(src_path: &str, mapping_data: &csv_mapper::MappingData) -> Option<csv_mapper::mapping_data::MappingRule> {

    let src_path = src_path.split('.').last().unwrap();
    for rule in mapping_data.iter() {
        if rule.src_path == src_path {
            return Some(rule.clone());
        }
    }

    None
}

async fn mapper(input_path: &str, csv_path: &str, target_format: &str) {
    // Read .csv into MappingData struct
    let mapping_data = parse_csv(PathBuf::from(csv_path)).await.unwrap();
    // initializing the json object with the correct* type field
    // *still needs further tailoring to be a 100% correct depending on what `target_format` is passed
    let mut new_json_value = json!({
        "type": ["Example"],
    });

    let new_json_value = new_json_value.as_object_mut().unwrap();

    let input_string = fs::read_to_string(input_path).await.unwrap();
    for row in input_string.lines() {
        let kv: Vec<_> = row.split(',').map(|s| s.trim()).collect(); // Split by commas and trim whitespace
        if let Some(mapping_rule) = get_mapping(kv[0], &mapping_data).await {
            println!("Mapping rule: {:?}", mapping_rule);
            match mapping_rule.transformation.as_str() {
                "false" => {
                    new_json_value.insert(kv[0].to_string(), serde_json::Value::from(kv[1]));
                },
                _ => unimplemented!()
            };
        }
        // let value = serde_json::Value::from(kv[1]);
        // new_json_value.insert(kv[0].to_string(), value);
    }

     
    // // Use a vector to store key-value pairs in order
    // // changing the order in the vec to change the order it is inserted has no results sinds hashmaps don't keep order
    // let mut kv_pairs: Vec<(String, Value)> = new_json_value.as_object().unwrap()
    //     .into_iter()
    //     .map(|(k, v)| (k.to_string(), v.clone()))
    //     .collect();

    // let input_string = fs::read_to_string(input_path).await.unwrap();
    // for row in input_string.lines() {
    //     let kv: Vec<_> = row.split(',').map(|s| s.trim()).collect(); // Split by commas and trim whitespace
    //     let value = serde_json::Value::from(kv[1]);
    //     kv_pairs.push((kv[0].to_string(), value)); // Insert at the end
    //     // kv_pairs.insert(index, element)
    // }

    // // Convert vector to a serde_json::Map
    // let mut map = Map::new();
    // for (k, v) in kv_pairs {
    //     map.insert(k, v);
    // }

    // // Create a new JSON value from the map
    // let new_json_value = Value::Object(map);

    println!("\n\nNew JSON: {:?}", new_json_value);

    let new_path = format!("{}.json", target_format);
    let json_file = File::create(new_path).expect("Unable to create file");
    serde_json::to_writer_pretty(json_file, &new_json_value).expect("Unable to write to file");
}

//////////     HELPERS     //////////

fn prompt_input_path(question: &str) -> String {
    let valid_path = loop {
        print!("{}{}:\n", question, std::env::current_dir().unwrap().display());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();
        if Path::new(input_trimmed).is_file() {
            break input_trimmed.to_string();
        } 
        else {
            println!("Invalid file path.");
        }
    };

    valid_path
}

fn prompt_output_path(question: &str) -> String {
    print!("{}:\n", question);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input_trimmed = input.trim();
    if Path::new(input_trimmed).is_file() {
        println!("File {} exists.\n! Warning: the file will be wiped and written to !", input_trimmed);
    } else {
        println!("File {} will be created and written to.", input_trimmed);
    }

    input_trimmed.to_string()
}

fn prompt_option(question: &str, options: Vec<&str>) -> String {
    let valid_option = loop {
        print!("{}[{}]:\n", question, options.join(", "));
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_trimmed = input.trim();
        if options.contains(&input_trimmed) {
            println!("Option {} selected.", input_trimmed);
            break input_trimmed.to_string();
        } 
        else {
            println!("Invalid option.");
        }
    };
    
    valid_option
}
