use clap::Parser;
use config::Config;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
// This contains all the flags we need for the tool
#[derive(Serialize, Parser, Debug)]
struct Args {
    #[arg(short = 'H', long)]
    hostname: String,
    #[arg(short, long)]
    description: String,
    #[arg(short, long)]
    servicestateid: u8,
    #[arg(short, long)]
    lastproblemid: String,
    #[arg(short, long)]
    problemid: String,
    #[arg(long)]
    lastservicestateid: u8,
    #[arg(long)]
    notification: String,
    #[arg(long)]
    servicestate: String,
    #[arg(long = "attempt_number")]
    attempt_number: u8,
    #[arg(long = "max_attempts")]
    max_attempts: u8,
    #[arg(long)]
    test_api: bool,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    sync: bool,
    #[arg(long)]
    validate: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let data = json!(args);

    // This is reading our configuration file
    // And then we can override with NGLUE_ variables, like NGLUE_DEBUG=1
    let settings = Config::builder()
        //.add_source(config::File::with_name("nglue"))
        .add_source(config::Environment::with_prefix("NGLUE"))
        .build()
        .unwrap();
    let conf = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    // Now let us send in the data
    let _response = ureq::post(conf.get("server").expect("NGLUE_SERVER not set")) // The URL to POST to
        .send_json(&data)?; // The data we are sending
    Ok(())
}
