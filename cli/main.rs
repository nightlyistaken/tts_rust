use quicli::prelude::*;
use structopt::StructOpt;
use tts_rust::languages::Languages;
use tts_rust::tts::GTTSClient;

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
    tld: "com",
  };
  if let Err(msg) = client.speak(&args.text) {
    println!("An error occurred: {}", msg);
  }
  Ok(())
}
// test cli
#[cfg(test)]
mod cli_tests {
  use super::*;

  #[test]
  fn test_cli_default() {
    let args = GTTSCli::from_iter(&["tts_rust", "--lang", "en", "Hello!"]);
  }
  #[test]
  fn test_cli_en() {
    let args = GTTSCli::from_iter(&["tts_rust", "--lang", "en", "Hello!"]);
    let client = GTTSClient {
      volume: 1.0,
      language: Languages::English,
      tld: "com",
    };
    let _result = client.speak(&args.text).unwrap();
  }
  #[test]
  fn test_cli_ja() {
    let args = GTTSCli::from_iter(&["tts_rust", "--lang", "ja", "Hello!"]);
    let client = GTTSClient {
      volume: 1.0,
      language: Languages::Japanese,
      tld: "com",
    };
    let _result = client.speak(&args.text).unwrap();
  }
}
