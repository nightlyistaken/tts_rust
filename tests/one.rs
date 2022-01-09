use tts_rust::tts::GTTSClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let narrator = GTTSClient {
    volume: 1.0,
    language: tts_rust::languages::Languages::English,
    tld: "com",
  };
  let result = narrator.speak("Hello!").unwrap();

  println!("{:?}", result);
  Ok(())
}
