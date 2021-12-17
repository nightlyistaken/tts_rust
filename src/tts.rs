use minreq::get;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use crate::languages::Languages;
use crate::url::core::Core;

#[derive(Debug)]
pub struct GTTSClient {
  /// The volume of the gTTS client
  ///
  /// recommended value is 1.0
  pub volume: f32,
  /// Use Language Codes according to Languages (enum)
  ///
  /// example: Languages::English, Languages::Japanese
  pub language: Languages,
}

impl GTTSClient {
  pub fn save_to_file(&self, text: &str, filename: &str) -> Result<(), String> {
    let len = text.len();
    let language = Languages::as_code(self.language.clone());
    let text = Core::fragmenter(text)?;
    let rep = get(format!("https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", text.encoded, language, len, language)).send().unwrap();
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
    let narrator = GTTSClient {
      volume: 1.0,
      language: Languages::English,
    };
    narrator.test();
  }
  #[test]
  fn test_2() -> Result<(), String> {
    let mut narrator: GTTSClient = GTTSClient {
      volume: 1.0,
      language: Languages::Telugu,
    };
    narrator.speak("Starting test?")?;
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..9 {
      narrator.volume += 1.0;
      let to_speak: String =
        String::from("Loop ") + &narrator.volume.to_string();
      narrator.speak(&to_speak)?;
      std::thread::sleep(ms);
    }
    Ok(())
  }
  #[test]
  fn test_3() {
    let tester: GTTSClient = GTTSClient {
      volume: 1.5,
      language: Languages::English,
    };
    tester.test();
    tester.display_and_speak("displaying and speaking boi")
  }
}
