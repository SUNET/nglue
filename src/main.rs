use clap::Parser;
use config::Config;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Flow {
    TestApi,
    Sync,
    Update,
}

// This contains all the flags we need for the tool
#[derive(Serialize, Parser, Debug)]
struct Cli {
    #[arg(short = 'H', long)]
    hostname: Option<String>,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    servicestateid: Option<u8>,
    #[arg(short, long)]
    lastproblemid: Option<String>,
    #[arg(short, long)]
    problemid: Option<String>,
    #[arg(long)]
    lastservicestateid: Option<u8>,
    #[arg(long)]
    notification: Option<String>,
    #[arg(long)]
    servicestate: Option<String>,
    #[arg(long = "attempt_number")]
    attempt_number: Option<u8>,
    #[arg(long = "max_attempts")]
    max_attempts: Option<u8>,
    #[arg(long, conflicts_with_all = ["test_api", "hostname"])]
    sync: bool,
    #[arg(long, conflicts_with_all = ["sync", "hostname"])]
    test_api: bool,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    validate: bool,
}

fn validate_flow(cli: &Cli) -> Result<Flow, String> {
    // Exactly one flow active
    let flow_count = (cli.test_api as usize) + (cli.sync as usize) + (cli.hostname.is_some() as usize);
    if flow_count != 1 {
        return Err("Exactly one flow: --test-api OR --sync OR hostname (-H)".into());
    }

    // UPDATE flow validation - ALL mandatory
    if cli.hostname.is_some() {
        let missing = vec![
            ("description", cli.description.is_none()),
            ("servicestateid", cli.servicestateid.is_none()),
            ("servicestate", cli.servicestate.is_none()),
            ("lastservicestateid", cli.lastservicestateid.is_none()),
            ("lastproblemid", cli.lastproblemid.is_none()),
            ("problemid", cli.problemid.is_none()),
            ("notification", cli.notification.is_none()),
            ("attempt_number", cli.attempt_number.is_none()),
            ("max_attempts", cli.max_attempts.is_none()),
        ]
        .into_iter()
        .filter_map(|(name, is_missing)| if is_missing { Some(name) } else { None })
        .collect::<Vec<_>>();

        if !missing.is_empty() {
            return Err(format!("UPDATE requires: {}", missing.join(", ")));
        }
    }

    Ok(if cli.test_api { Flow::TestApi }
       else if cli.sync { Flow::Sync }
       else { Flow::Update })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    if cli.debug {
        println!("{:#?}", cli);
    }

    let flow = validate_flow(&cli)?;

    if cli.validate {
        println!("✓ Validation passed: {:?}", flow);
    }

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

    let server = conf.get("server")
        .ok_or("NGLUE_SERVER required")?
        .to_string();
    
    if cli.debug {
        println!("Server: {}", server);
    }

    // Flow execution
    let response = match flow {
        Flow::TestApi => {
            ureq::post(&server)
                .send_json(json!({"test_api": true, "debug": cli.debug }))?
        }
        Flow::Sync => {
            ureq::post(&server)
                .send_json(json!({"sync": true, "debug": cli.debug }))?
        }
        Flow::Update => {
            // Safe unwraps - validation passed
            ureq::post(&server)
                .send_json(json!({
                    "hostname": cli.hostname.as_ref().unwrap(),
                    "description": cli.description.as_ref().unwrap(),
                    "servicestateid": cli.servicestateid.unwrap(),
                    "servicestate": cli.servicestate.unwrap(),
                    "lastservicestateid": cli.lastservicestateid.unwrap(),
                    "lastproblemid": cli.lastproblemid.as_ref().unwrap_or(&"0".to_string()),
                    "problemid": cli.problemid.as_ref().unwrap_or(&"0".to_string()),
                    "notification": cli.notification.as_ref().unwrap(),
                    "attempt_number": cli.attempt_number.unwrap(),
                    "max_attempts": cli.max_attempts.unwrap(),
                    "debug": cli.debug,
                    "validate" : cli.validate
                }))?
        }
    };

    if cli.debug {
        println!("Status: {}", response.status());
        println!("Response: {}", response.into_string()?);
    }

    println!("NGLUE {:?} complete", flow);
    Ok(())
}
