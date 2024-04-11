use clap::{Parser, ValueEnum};
use std::path::PathBuf;

pub mod validate;

#[derive(Parser)] // requires `derive` feature
#[command(name = "imp-mapper")]
#[command(bin_name = "imp-mapper")]
enum CliAction {
    Validate(ValidateArgs),
    Convert(ConvertArgs),
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Format {
    W3VC,
    OBV3,
    ELM,
}

/// Validate a JSON-ld file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ValidateArgs {
    pub file: PathBuf,
    #[clap(short = 'f', long)]
    pub format: Format,
}

/// Convert a JSON-ld file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConvertArgs {
    pub todo: String,
}

#[tokio::main]
async fn main() {
    let result = CliAction::parse();

    let result = match result {
        CliAction::Convert(_) => {
            println!("todo");
            Ok(())
        }
        CliAction::Validate(args) => validate::Validate::exec(args).await,
    };

    if let Err(err) = result {
        eprintln!("{}", err.to_string());
    }
}
