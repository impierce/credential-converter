use axum::extract::multipart::Field;
use axum::{
    extract::Multipart,
    // routing::post,
    // Router,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use crate::backend::headless_cli::load_files_apply_transformations;
use crate::state::AppState;
use std::{fs::File, io::Write, path::Path};
use tokio::fs;

pub async fn translate_file(mut multipart: Multipart) -> Result<Response, (StatusCode, String)> {
    // Create directories for uploads and outputs if they don't exist
    let upload_dir = "uploads";
    let output_dir = "outputs";
    fs::create_dir_all(upload_dir).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create upload directory".to_string(),
        )
    })?;
    fs::create_dir_all(output_dir).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create output directory".to_string(),
        )
    })?;

    // Handle the file upload
    let mut input_file_path = String::new();
    let mut mapping_file_name = String::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to process uploaded file".to_string()))?
    {
        let name = field.name().unwrap().to_string();
        match name.as_str() {
            "input_file" => match process_file_field(&field) {
                Ok(file_name) => {
                    input_file_path = format!("{}/{}", upload_dir, file_name);
                    let mut file = File::create(&input_file_path)
                        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file".to_string()))?;
                    let data = field.bytes().await.map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to read file data".to_string(),
                        )
                    })?;
                    file.write_all(&data)
                        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file".to_string()))?;
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            "translation" => {
                // Mapping::OBv3ToELM => "OBv3".to_string(),
                // Mapping::ELMToOBv3 => "ELM".to_string(),
                if let Ok(translation_value) = field.text().await {
                    // Match on the value of the text field and call different functions
                    match translation_value.as_str() {
                        "OBv3ToELM" => {
                            mapping_file_name = format!("json/mapping/custom_mapping_OBv3_ELM_latest.json");
                        }
                        "ELMToOBv3" => {
                            mapping_file_name = format!("json/mapping/custom_mapping_ELM_OBv3_latest.json");
                        }
                        _ => {
                            return Err((
                                StatusCode::BAD_REQUEST,
                                format!("Invalid translation value: {}", translation_value),
                            ))
                        }
                    };
                } else {
                    // Handle the case where reading the field text fails
                    return Err((StatusCode::BAD_REQUEST, "Failed to read translation value".to_string()));
                }
            }
            &_ => return Err((StatusCode::BAD_REQUEST, "Received unwanted values".to_string())),
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

    let mut state = AppState::default();
    state.input_path = input_file_path;
    state.output_path = output_file_path.clone();
    state.mapping_path = mapping_file_name;

    load_files_apply_transformations(&mut state);

    // Return the translated file as a response
    // 1 load file from fs into mem
    // 2 remove file from fs
    // 3 push mem to http output

    let output_file = fs::read(&output_file_path).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read output file".to_string(),
        )
    })?;
    fs::remove_file(state.input_path).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to remove output file".to_string(),
        )
    })?;
    fs::remove_file(state.output_path).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to remove output file".to_string(),
        )
    })?;

    // Set the headers, including content disposition for download
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", output_file_name)).unwrap(),
    );

    // Return the file content along with the appropriate headers
    Ok((headers, output_file).into_response())

    // Ok(output_file.into_response())
}

fn process_file_field(field: &Field) -> Result<String, String> {
    match field.file_name() {
        Some(file_name) => Ok(file_name.to_string()),
        None => Err("No file name found in the field.".to_string()),
    }
}
