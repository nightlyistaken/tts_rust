use minreq::get;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::constants::GOOGLE_TTS_MAX_CHARS;
use crate::languages::Languages;
use crate::url::core::Core;

#[derive(Debug)]
pub struct GTTSClient {
  /// The volume of the audio. Must be between 0.0 and 1.0. Default is 1.0.
  ///
  /// recommended value is 1.0
  pub volume: f32,
  /// The language of the gTTS client (ISO code)
  ///
  /// example: Languages::English, Languages::Japanese
  pub language: Languages,
  /// top-level domain of the gTTS client
  ///
  /// example: "com"
  pub tld: &'static str,
}
pub enum Speed {
  Normal,
  Slow,
}
impl GTTSClient {
  /// Creates a new gTTS client default values.
  pub fn default() -> Self {
    GTTSClient {
      volume: 1.0,
      language: Languages::English,
      tld: "com",
    }
  }
  /// Creates a new gTTS client with the given volume and language.
  pub fn new(volume: f32, language: Languages, tld: &'static str) -> Self {
    GTTSClient {
      volume,
      language,
      tld,
    }
  }
  pub fn save_to_file(&self, text: &str, filename: &str) -> Result<(), String> {
    let len = text.len();
    if len > GOOGLE_TTS_MAX_CHARS {
      return Err(format!(
        "The text is too long. Max length is {}",
        GOOGLE_TTS_MAX_CHARS
      ));
    }
    let language = Languages::as_code(self.language.clone());
    let text = Core::fragmenter(text)?;
    let rep = get(format!("https://translate.google.{}/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", self.tld, text.encoded, language, len, language))
      .send()
      .map_err(|e| format!("{}", e))?;
    let mut file = File::create(filename).unwrap();
    let bytes = rep.as_bytes();
    if !bytes.is_empty() && file.write_all(bytes).is_ok() {
      return Ok(());
    }

    Err("Something went wrong".to_string())
  }
  fn play_mp3(&self, mp3: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.set_volume(self.volume);
    let file = std::fs::File::open(mp3).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
  }
  /// Speak the input according to the volume and language
  pub fn speak(&self, input: &str) -> Result<(), String> {
    self.save_to_file(input, "audio.mp3")?;
    self.play_mp3("audio.mp3");
    if Path::new("./audio.mp3").exists() {
      fs::remove_file("./audio.mp3").unwrap();
    }
    Ok(())
  }
  /// Speak and println! the input according to the volume and language
  pub fn display_and_speak(&self, input: &str) {
    self.speak(input).unwrap();
    println!("{}", input);
  }
  /// Fastest way to check if gTTS API works
  pub fn test(&self) {
    self.display_and_speak("Hello!");
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let narrator = GTTSClient::default();
    narrator.test();
  }
}
