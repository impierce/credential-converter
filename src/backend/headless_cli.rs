use crate::trace_dbg;

use clap::Parser;
use std::collections::HashMap;
use std::io::Result;

pub fn run_headless(cli_args: Args) -> Result<()> {
    let args: HashMap<String, String> = parse_cli_args(cli_args);
    check_arg_paths(&args)?;
    trace_dbg!(&args);
    

    Ok(())
}

pub fn parse_cli_args(cli_args: Args) -> HashMap<String, String> {
    let mut args = HashMap::new();
    if let Some(e) = cli_args.input_file {
        args.insert("input_file".to_string(), e);
    }
    if let Some(e) = cli_args.input_directory {
        args.insert("input_directory".to_string(), e);
    }
    if let Some(e) = cli_args.output_file {
        args.insert("output_file".to_string(), e);
    }
    if let Some(e) = cli_args.output_directory {
        args.insert("output_directory".to_string(), e);
    }
    if let Some(e) = cli_args.mapping_file {
        args.insert("mapping_file".to_string(), e);
    }

    args
}

pub fn check_arg_paths(args: &HashMap<String, String>) -> Result<()> {
    if args.contains_key("input_file") {
        if !std::path::Path::new(&args["input_file"]).exists() {
            panic!("The input file path does not exist: {}", &args["input_file"]);
        }
    }
    if args.contains_key("input_directory") {
        if !std::path::Path::new(&args["input_directory"]).exists() {
            panic!("The input directory path does not exist: {}", &args["input_directory"]);
        }
    }

    Ok(())
}

///// STRUCTS /////


#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = "This is the executable for the Credential Converter built by Impierce Technologies.\nWhen running without arguments it will start the Terminal User Interface.\nHere you can add, edit, save and tweak all the conversions manually\nFor headless conversion(s), also batch, the correct arguments need to be passed to the executable.\nRead more below:")]
pub struct Args {
    #[arg(short, long, requires_all = ["mapping_file", "output_file"], conflicts_with_all = ["input_directory", "output_directory"])]
    input_file: Option<String>, // if present cant have input_directory, and must have output_file

    #[arg(short = 'b', long, requires_all = ["mapping_file", "output_directory"], conflicts_with_all = ["input_file", "output_file"])]
    input_directory: Option<String>, // this demands an output_directory
    
    #[arg(short, long, requires_all = ["mapping_file", "input_file"], conflicts_with_all = ["input_directory", "output_directory"])]
    output_file: Option<String>,

    #[arg(short = 'd', long, requires_all = ["mapping_file", "input_directory"], conflicts_with_all = ["input_file", "output_file"])]
    output_directory: Option<String>,

    #[arg(short, long, required_if_eq_any = [("input_file", "Some"), ("input_directory", "Some"), ("output_file", "Some"), ("output_directory", "Some")])]
    mapping_file: Option<String>,

    // #[arg(short, long)] // todo: nice feature for in the future
    // prefix_output: String,

    // #[arg(short, long)]
    // suffix_output: String,
}
