use crate::ValidateArgs;
use std::io;
use tokio::fs;

pub struct Validate;

impl Validate {
    pub async fn exec(args: ValidateArgs) -> io::Result<()> {
        println!("args: {args:?}");

        let json_file = fs::File::open(args.file).await?;

        match args.format {
            crate::Format::W3VC => validate_w3vc(json_file).await,
            crate::Format::OBV3 => validate_obv3(json_file).await,
            crate::Format::ELM => validate_elm(json_file).await,
        }
    }
}

async fn validate_obv3(_json_file: fs::File) -> io::Result<()> {
    // TODO
    Ok(())
}

async fn validate_w3vc(_json_file: fs::File) -> io::Result<()> {
    // TODO

    Ok(())
}

async fn validate_elm(_json_file: fs::File) -> io::Result<()> {
    // TODO

    Ok(())
}
