use serde_json::json;
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};
use tokio::{fs, io};

mod traverse_tree;

/// Key: the JSON source path, Value: mapping rules
pub type MappingData = HashMap<String, Vec<MappingRule>>;

///////////     MAIN     ///////////
#[tokio::main]
async fn main() -> io::Result<()> {
    let res = mapper("examples/dummy.json", "example-mapper.csv", "TODO").await;

    res
}

#[derive(Debug)]
pub struct MappingRule {
    src_json_path: String,
    src_schema_object: String,
    src_schema_field: String,
    target_json_path: String,
    target_schema_object: String,
    target_schema_field: String,
}

///////////     FUNCTIONS     ///////////

async fn parse_csv(csv_path: PathBuf) -> io::Result<MappingData> {
    let csv_string = fs::read_to_string(csv_path).await?;

    let mut csv_data: MappingData = HashMap::new();

    for row in csv_string.lines().skip(1) {
        let columns: Vec<_> = row.split(',').map(|s| s.trim()).collect(); // Split by commas and trim whitespace

        assert!(
            columns.len() == 6,
            "Invalid CSV format (may only contain 6 items per row)"
        );

        let mapping_rule = MappingRule {
            src_json_path: columns[0].to_string(),
            src_schema_object: columns[1].to_string(),
            src_schema_field: columns[2].to_string(),
            target_json_path: columns[3].to_string(),
            target_schema_object: columns[4].to_string(),
            target_schema_field: columns[5].to_string(),
        };

        let json_path_mappings = csv_data.get_mut(&mapping_rule.src_json_path);

        if let Some(mappings) = json_path_mappings {
            mappings.push(mapping_rule);
        } else {
            csv_data.insert(mapping_rule.src_json_path.to_string(), vec![mapping_rule]);
        }
    }

    Ok(csv_data)
}

async fn json_example(path: PathBuf) -> serde_json::Result<serde_json::Value> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .await
        .expect("file does not exist");
    serde_json::from_reader(file.try_into_std().unwrap())
}

async fn mapper(json_path: &str, csv_path: &str, target_format: &str) -> io::Result<()> {
    // Read the CSV file into a matrix
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mapping_data = parse_csv(root.join(csv_path)).await?;
    let json_value = json_example(root.join(json_path)).await;

    // TODO handle errors gracefully.
    if let Err(err) = json_value {
        panic!("{}", err);
    }

    let json_value = json_value.unwrap();

    println!("CSV: {:?}", mapping_data);

    let mut target = serde_json::Map::new();
    if let serde_json::Value::Object(src) = json_value {
        println!("Source value: {:?}", src);

        traverse_tree::traverse_tree(src, &mut target, &mapping_data, "$");
    } else {
        // Throw some error
        panic!("Gaat mis");
    }

    println!("Target value: {:?}", target);

    // initializing the json object with the correct* type field
    // *still needs further tailoring to be a 100% correct depending on what `target_format` is passed
    //let mut new_json_value = json!({
    //"type": [target_format],
    //});
    //let new_json_value = new_json_value.as_object_mut().unwrap();

    //if let Some(obj) = json_value.as_object() {
    //if let Some(_type) = obj.get("type") {
    //// Unsure if schema title will always be findable like this, will be clarified after more introduction to the DESM tools.
    //// Assuming the schtitle represents the Json Format, this methodology works for OBv3.
    //println!("schTitle1: {:?}", _type);
    //if csv_checker(_type[1].as_str().unwrap(), target_format, &csv_matrix) {
    //println!("Mapping exists");
    //for field in obj {
    //if field.1.is_object() {
    //println!("Found object ----------------------- {:?}", field);
    //for sub_field in field.1.as_object().unwrap() {
    //for row in &csv_matrix {
    //if _type[1] == row[0]
    //&& row[3] == target_format
    //&& *sub_field.0 == row[1]
    //{
    //println!("Found mapping: {:?}\nFor: {:?}", row, sub_field);
    //new_json_value.insert(row[2].clone(), sub_field.1.clone());
    //}
    //}
    //println!("End of object -----------------------");
    //}
    //} else {
    //for row in &csv_matrix {
    //if _type[1] == row[0] && row[3] == target_format && *field.0 == row[1] {
    //println!("Found mapping: {:?}\nFor: {:?}", row, field);
    //new_json_value.insert(row[2].clone(), field.1.clone());
    //}
    //}
    //}
    //}
    //} else {
    //println!("Mapping does not exist");
    //}
    //} else {
    //println!("No type field found in JSON");
    //}
    //} else {
    //println!("JSON is not an object");
    //}

    //println!("\n\nNew JSON: {:?}", new_json_value);

    //let new_path = format!("target/{}.json", target_format);
    //let json_file = fs::File::create(new_path).await?.try_into_std().unwrap();
    //serde_json::to_writer_pretty(json_file, &new_json_value).expect("Unable to write to file");

    Ok(())
}
