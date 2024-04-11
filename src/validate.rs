use jsonschema::JSONSchema;
use std::{fs::{self, File}, io};

use crate::ValidateArgs;

pub struct Validate;

impl Validate {
    pub async fn exec(args: ValidateArgs) -> io::Result<()> {
        println!("args: {args:?}");

        let json_file = fs::File::open(args.file)?;

        match args.format {
            crate::Format::W3VC => validate_w3vc(json_file).await,
            crate::Format::OBV3 => validate_obv3(json_file).await,
            crate::Format::ELM => validate_elm(json_file).await,
        }
    }
}

async fn validate_obv3(json_file: fs::File) -> io::Result<()> {
    let schema_str = reqwest::get("https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.3.json")
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::NotConnected, err.to_string()))?;

    let schema = schema_str.json::<serde_json::Value>().await.unwrap();

    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let val: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let result = compiled.validate(&val);

    if let Err(errors) = result {
        for error in errors {
            eprintln!("Error: {}", error.to_string());
            eprintln!("Validation path: {:?}", error.instance_path.to_string());
        }
    }

    Ok(())
}

async fn validate_w3vc(json_file: fs::File) -> io::Result<()> {
    let schema_str = reqwest::get("https://www.w3.org/ns/credentials/v2")
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::NotConnected, err.to_string()))?;

    let schema = schema_str.json::<serde_json::Value>().await.unwrap();

    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let val: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let result = compiled.validate(&val);

    if let Err(errors) = result {
        for error in errors {
            eprintln!("Error: {}", error.to_string());
            eprintln!("Validation path: {:?}", error.instance_path.to_string());
        }
    }

    //let input = RemoteDocument::new(
    //Some(iri!("https://www.w3.org/ns/credentials/v2").to_owned()),
    //Some("application/ld+json".parse().unwrap()),
    //Value::parse_str(&json_str)
    //.map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?
    //.0,
    //);

    //let mut loader = json_ld::loader::ReqwestLoader::new();

    //let expand = input.expand(&mut loader).await.expect("Expand failed");

    //for obj in expand.objects() {
    //println!("itempje {:?}", obj);
    //}

    //println!("output: {}", expand.len());

    Ok(())
}

async fn validate_elm(json_file: fs::File) -> io::Result<()> {
    let root = env!("CARGO_MANIFEST_DIR");
    let file_path = format!("{}/elm/vocab.json", root);
    let file = File::open(file_path)?;

    println!("has voc");

    let schema: serde_json::Value = serde_json::from_reader(file).expect("No json");
    let compiled = JSONSchema::compile(&schema).expect("A valid schema");

    let val: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let result = compiled.validate(&val);

    if let Err(errors) = result {
        for error in errors {
            eprintln!("Error: {}", error.to_string());
            eprintln!("Validation path: {:?}", error.instance_path.to_string());
        }
    }

    Ok(())
}
