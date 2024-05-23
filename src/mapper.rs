use tokio::io;
use csv_mapper::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    //let res = mapper("examples/dummy.json", "example-mapper.csv", "TODO").await;
    init_env_vars();

    //generate_json_paths("examples/request.json").await;
    add_schema_paths().await;

    Ok(())
}
