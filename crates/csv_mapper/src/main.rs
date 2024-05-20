use crate::mapping_data::MappingRule;
use digital_credential_data_models::common::{traits::*, GenPaths, OneOrMany};
use env_logger::Env;
use mapping_data::MappingData;
use serde_json::Value;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::{
    fs,
    io::{self},
};

mod mapping_data;
mod traverse_tree;

#[tokio::main]
async fn main() -> io::Result<()> {
    //let res = mapper("examples/dummy.json", "example-mapper.csv", "TODO").await;
    init_env_vars();

    generate_json_paths().await;
    add_schema_paths().await;

    Ok(())
}

#[derive(GenPaths, serde::Deserialize)]
pub struct LatLng {
    pub lat: f32,
    pub lng: f32,
}

#[derive(GenPaths, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub number: usize,
    pub province: Province,
    pub lat_lng: LatLng,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Province {
    pub name: String,
    pub code: String,
}

#[derive(GenPaths, Debug, serde::Deserialize)]
pub struct School {
    pub name: String,
    pub level: String,
}

#[derive(GenPaths, serde::Deserialize)]
pub struct Person {
    pub name: String,

    #[serde(rename = "surName", default)]
    pub sur_name: Option<String>,
    pub age: usize,
    pub address: Address,
    pub synonyms: Vec<String>,
    pub schools: Option<OneOrMany<School>>,
}

async fn generate_json_paths() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let elm_cred = root.join("examples/francisco-cruz.json");

    let value = read_json(elm_cred).await.unwrap();

    if let Value::Object(obj) = value {
        let mut out = Vec::new();
        traverse_tree::store_all_json_paths(&obj, "$".to_string(), &mut out);
        let res: Vec<_> = out.into_iter().map(|(k, v)| format!("{k}, {v}")).collect();

        let _ = fs::write(root.join("target/json_paths.csv"), res.join("\n")).await;
    }
}

fn init_env_vars() {
    // load environment variables from .env file
    //dotenv().expect(".env file not found");

    // The `Env` lets us tweak what the environment
    // variables to read are and what the default value is if they're missing
    let env = Env::default()
        .filter_or(env_key("LOG_LEVEL"), "debug")
        .write_style_or(env_key("LOG_STYLE"), "always");

    env_logger::init_from_env(env);
}

fn env_key(key: &str) -> String {
    format!("IMP_MAPPER_{}", key)
}

async fn add_schema_paths() {
    let mut schema_types = HashMap::new();
    Person::add_schema_types(&mut schema_types);

    let mut lines = vec![];

    for (_, data) in schema_types.iter() {
        for row in data.iter() {
            lines.push(row.to_string());
        }
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _ = fs::write(root.join("target/schema.csv"), lines.join("\n")).await;
}

async fn parse_csv(csv_path: PathBuf) -> io::Result<MappingData> {
    let csv_string = fs::read_to_string(csv_path).await?;

    let mut rules = Vec::new();

    for row in csv_string.lines().skip(1) {
        let columns: Vec<_> = row.split(',').map(|s| s.trim()).collect(); // Split by commas and trim whitespace

        assert!(
            columns.len() == 3,
            "Invalid CSV format (may only contain 3 items per row)"
        );

        let mapping_rule = MappingRule {
            src_path: columns[0].to_string(),
            target_path: columns[1].to_string(),
            transformation: columns[2].to_string(),
        };

        rules.push(mapping_rule);
    }

    Ok(MappingData::new(rules))
}

async fn read_json(path: PathBuf) -> serde_json::Result<serde_json::Value> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .await
        .expect("file does not exist");
    serde_json::from_reader(file.try_into_std().unwrap())
}

#[allow(unused)]
async fn mapper(json_path: &str, csv_path: &str, _target_format: &str) -> io::Result<()> {
    // Read the CSV file into a matrix
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mapping_data = parse_csv(root.join(csv_path)).await?;
    let json_value = read_json(root.join(json_path)).await;

    // TODO handle errors gracefully.
    if let Err(err) = json_value {
        panic!("{}", err);
    }

    let json_value = json_value.unwrap();

    println!("CSV: {:?}", mapping_data);

    if let serde_json::Value::Object(src) = json_value {
        println!("Source value: {:?}", src);

        let mut target = serde_json::Map::new();

        //traverse_tree::traverse_source(&src, &mut target, &mapping_data);

        println!("Target value: {:?}", target);
    } else {
        // Throw some error
        panic!("Gaat mis");
    }

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
