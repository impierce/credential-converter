use crate::backend::preload_p2::get_json;
use crate::backend::repository::Repository;
use crate::backend::transformations::Transformation;
use crate::p2_p3_common::create_output_files;
use crate::state::{AppState, Mapping};
use crate::trace_dbg;

use clap::Parser;
use serde_json::json;
use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Result;
use std::path::Path;

pub fn run_headless(cli_args: Args, state: &mut AppState) -> Result<()> {
    check_args(&cli_args)?;
    trace_dbg!(&cli_args);
    complete_appstate_headless(cli_args, state);

    state.repository = Repository::from(HashMap::from_iter(vec![
        (
            state.mapping.input_format(),
            get_json(&state.input_path).expect("No source file found"),
        ),
        (state.mapping.output_format(), json!({})),
    ]));

    trace_dbg!("Successfully loaded the input file");

    let rdr = std::fs::File::open(&state.mapping_path).unwrap();
    let transformations: Vec<Transformation> = serde_json::from_reader(rdr).unwrap();

    trace_dbg!("Successfully loaded the mapping file");

    state.repository.apply_transformations(transformations, state.mapping);
    
    create_output_files(state);

    Ok(())
}

pub fn check_args(cli_args: &Args) -> Result<()> {
    if let Some(input_f) = &cli_args.input_file {
        if !Path::new(&input_f).is_file() {
            panic!("The input file path does not exist: {}", &input_f);
        }
        else if !input_f.ends_with(".json") {
            panic!("The input file is not a json file: {}", &input_f);
        }
    }
    if let Some(input_dir) = &cli_args.input_directory {
        if !Path::new(&input_dir).is_dir() {
            panic!("The input directory path does not exist: {}", input_dir);
        }
        let mut json_count: usize = 0;
        json_count = check_input_dir(&input_dir, json_count);

        if json_count == 0 {
            panic!("The input directory does not contain any json files: {}", input_dir);
        }
    }
    if let Some(output_f) = &cli_args.output_file {
        if output_f.ends_with(".json") {
            panic!("The output file path doesn't end with \".json\": {}", output_f);
        }
    }
    if let Some(mapping_f) = &cli_args.mapping_file{
        if !Path::new(&mapping_f).is_file() {
            panic!("The mapping file path does not exist: {}", &mapping_f);
        }
        else if !mapping_f.ends_with(".json") {
            panic!("The mapping file is not a json file: {}", &mapping_f);
        }
    }

    Ok(())
}

pub fn check_input_dir(input_dir: &str, mut json_count: usize) -> usize {
    for entry in read_dir(input_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.ends_with(".json") {
            json_count += 1;
        } else if path.is_dir() {
            check_input_dir(path.to_str().unwrap(), json_count);
        }
    }
    json_count
}

pub fn complete_appstate_headless(args: Args, state: &mut AppState) {
    state.mapping = args.conversion.unwrap();
    state.mapping_path = args.mapping_file.unwrap();
    if let Some(input_f) = args.input_file {
        state.input_path = input_f;
        state.output_path = args.output_file.unwrap();
    }
    else if let Some(input_dir) = args.input_directory {
        state.input_path = input_dir;
        state.output_path = args.output_directory.unwrap();
    }
}


///// STRUCTS /////


#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = "This is the executable for the Credential Converter built by Impierce Technologies.\nWhen running without arguments it will start the Terminal User Interface.\nHere you can add, edit, save and tweak all the conversions manually\nFor headless conversion there are 2 options:\nConvert a file in .json format\nBatch conversion, convert all .json files in a given directory, also nested directories.\nFiles being output to an output directory will have the original name appended with _<conversion_destination_format>\nPaths to existing output files/directories will be overwritten.\nThe correct arguments need to be passed to the executable.\nRead more below:")]
pub struct Args {
    #[arg(short, long, requires_all = ["mapping_file", "output_file"], conflicts_with_all = ["input_directory", "output_directory"])]
    input_file: Option<String>, // if present cant have input_directory, and must have output_file

    #[arg(short = 'b', long, requires_all = ["mapping_file", "output_directory"], conflicts_with_all = ["input_file", "output_file"])]
    input_directory: Option<String>, // this demands an output_directory
    
    #[arg(short, long, requires_all = ["mapping_file", "input_file"], conflicts_with_all = ["input_directory", "output_directory"])]
    output_file: Option<String>,

    #[arg(short = 'd', long, requires_all = ["mapping_file", "input_directory"], conflicts_with_all = ["input_file", "output_file"])]
    output_directory: Option<String>,

    #[arg(short, long, required_if_eq_any = [("conversion", "Some"), ("input_file", "Some"), ("input_directory", "Some"), ("output_file", "Some"), ("output_directory", "Some")])]
    mapping_file: Option<String>,

    #[arg(short, long, value_enum , required_if_eq_any = [("mapping_file", "Some"), ("input_file", "Some"), ("input_directory", "Some"), ("output_file", "Some"), ("output_directory", "Some")])]
    conversion: Option<Mapping>,

    // #[arg(short, long)] // todo: nice feature for in the future
    // prefix_output: String,

    // #[arg(short, long)]
    // suffix_output: String,
}
