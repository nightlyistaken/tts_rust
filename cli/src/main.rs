use quicli::prelude::*;
use structopt::StructOpt;
use tts_rust::{languages::Languages, GTTSClient};

#[derive(Debug, StructOpt)]
struct GTTSCli {
    text: String,

    #[structopt(short = "l", long = "lang", default_value = "en")]
    language: Languages,
}

fn main() -> CliResult {
    let args = GTTSCli::from_args();
    let client = GTTSClient {
        volume: 1.0,
        language: args.language,
    };
    client.speak(&args.text).unwrap();
    Ok(())
}