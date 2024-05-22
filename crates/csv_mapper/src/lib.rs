use crate::mapping_data::MappingRule;
use digital_credential_data_models::{common::AddSchemaTypes, elmv3::EuropassEdcCredential};
use demo_schema::DigitalCredential;
use env_logger::Env;
use mapping_data::MappingData;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::{
    fs,
    io::{self},
};

mod mapping_data;
mod traverse_tree;
mod demo_schema;

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
    let mut schema_types = Vec::new();
    //DigitalCredential::add_schema_types(&mut schema_types, "ROOT", "$", false);
    EuropassEdcCredential::add_schema_types(&mut schema_types, "UNUSED", "UNUSED", true);

    let mut lines = vec![];

    for row in schema_types.iter() {
        lines.push(row.to_string());
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

    Ok(())
}
