use axum::{
    // routing::post,
    // Router,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
//use config::Value;
use serde_json::{json, Value};

use crate::backend::base64_encode::{decode_json, encode_json_file};
use crate::backend::headless_cli::load_files_apply_transformations;
use crate::state::{AppState, Mapping};
use std::{fs::File, io::Write, path::Path};
use tokio::fs;

pub async fn api(Json(input_json): Json<Value>) -> impl IntoResponse {
    //test the input types for this API
    // with JSON body: {
    //     "From": {"Name": "OB", "Version": "3.0"},
    //     "To": {"Name": "elm", "Version": "3.2"},
    //     "Parameters": { "PreferredLanguages": ["en", "sv"]},
    //     "Content": "Base 64 encoded content in From format"
    // }

    // Handle the file upload
    let input_file_path;
    let mapping_file_name;
    let mapping_type;

    // Create directories for uploads and outputs if they don't exist
    let upload_dir = "uploads";
    let output_dir = "outputs";
    let file_name = "export_file";
    let _ = fs::create_dir_all(upload_dir).await.map_err(|_| {
        let error_json = json!({
                "error": "Internal Server Error",
                "message" : "Failed to create upload directory"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
    });
    let _ = fs::create_dir_all(output_dir).await.map_err(|_| {
        let error_json = json!({
            "error": "Internal Server Error",
            "message" : "Failed to create output directory"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
    });

    match input_json
        .get("From")
        .and_then(|v| v.get("Name"))
        .and_then(|v| v.as_str())
    {
        Some("OB") => {
            mapping_file_name = "json/mapping/custom_mapping_OBv3_ELM_latest.json".to_string();
            mapping_type = Mapping::OBv3ToELM;
        }
        Some("ELM") => {
            mapping_file_name = "json/mapping/custom_mapping_ELM_OBv3_latest.json".to_string();
            mapping_type = Mapping::ELMToOBv3;
        }
        Some(value) => {
            let error_json = json!({
            "error": "Bad Request",
            "message" : format!("Invalid translation value: {}", value)});
            return (StatusCode::BAD_REQUEST, Json(error_json));
        }
        None => {
            let error_json = json!({
                "error": "Bad Request",
                "message" : "Invalid translation value: no key found"});
            return (StatusCode::BAD_REQUEST, Json(error_json));
        }
    }

    match input_json.get("Content").and_then(|v| v.as_str()) {
        Some(value) => {
            input_file_path = format!("{}/{}", upload_dir, file_name);
            let file = File::create(&input_file_path).map_err(|_| {
                let error_json = json!({
                        "error": "Internal Server Error",
                        "message" : "Failed to create file"});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
            });
            match decode_json(value) {
                Ok(data) => {
                    match file {
                        Ok(mut file_found) => {
                            let _ = file_found.write_all(&data);
                        }
                        Err(file_error) => {
                            return file_error;
                        } // Err(file_error) => {let error_json = json!({
                          //     "error": "Internal Server Error",
                          //     "message" : "Failed to write to file"});
                          // return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json));}
                    }
                }
                Err(_decode_err) => {
                    let error_json = json!({
                    "error": "Internal Server Error",
                    "message" : "Failed to read file data"});
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json));
                }
            }
        }

        None => {
            let error_json = json!({
                    "error": "Bad Request",
                    "message" : "Invalid data value: no key found"});
            return (StatusCode::BAD_REQUEST, Json(error_json));
        }
    }

    // Define the output file path
    let output_file_name = format!(
        "translated_{}",
        Path::new(&input_file_path).file_name().unwrap().to_str().unwrap()
    );
    let output_file_path = format!("{}/{}", output_dir, output_file_name);

    // start mapping based on the input form the API
    // 1 create a state needed for the mapping tool
    // 2 load all hte state elements needed for mapping

    let mut state = AppState {
        input_path: input_file_path,
        output_path: output_file_path.clone(),
        mapping_path: mapping_file_name,
        mapping: mapping_type,
        ..Default::default()
    };
    // let mut state = AppState::default();
    // state.input_path = _input_file_path;
    // state.output_path = output_file_path.clone();
    // state.mapping_path = _mapping_file_name;
    // state.mapping = _mapping_type;

    load_files_apply_transformations(&mut state);

    // Return the translated file as a response
    // 1 load file from fs into mem
    // 2 remove file from fs
    // 3 push mem to http output

    let output_file = fs::read(&output_file_path).await.map_err(|_| {
        let error_json = json!({
            "error": "Internal Server Error",
            "message" : "Failed to read output file"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
    });
    let _ = fs::remove_file(state.input_path).await.map_err(|_| {
        let error_json = json!({
            "error": "Internal Server Error",
            "message" : "Failed to remove input file"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
    });
    let _ = fs::remove_file(state.output_path).await.map_err(|_| {
        let error_json = json!({
            "error": "Internal Server Error",
            "message" : "Failed to remove output file"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
    });

    // Set the headers, including content disposition for download
    let mut headers = HeaderMap::new();
    // For better integration into EDCI change the output_file_name from *.json to *.jsonld
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/application/json"),
    );

    //println!("state_exitwarning: {:#?}", state.exit_warning);
    match state.exit_warning {
        true => {
            let error_json = json!({
                "error": "Internal Server Error",
                "message" : "Failed to encode the json file"});
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json));
        }
        false => {}
    }

    match output_file {
        Ok(content) => match encode_json_file(content) {
            Ok(encoded_json) => {
                let response_json = json!({"content": encoded_json});
                (StatusCode::OK, Json(response_json))
            }
            Err(_enc_error) => {
                let error_json = json!({
                    "error": "Internal Server Error",
                    "message" : "Failed to encode the json file"});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_json))
            }
        },
        Err(status) => status,
    }
    // let encoded_json = encode_json_file(output_file)?;

    // // Return the file content along with the appropriate headers
    // let response_json = json!({"content": encoded_json});
    // (StatusCode::OK, Json(response_json))
    // // Ok(output_file.into_response())
}
