//! # Very Simple Module for gTTS
//! Example:
//! ```rust
//! let narrator = GTTSClient {
//!      volume: 1.0,
//!     language: "en",
//! };
//! narrator.speak("Hello!");
//! ```

use minreq::get;
use percent_encoding::utf8_percent_encode;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct GTTSClient<'a> {
    /// The volume of the gTTS client
    ///
    /// recommended value is 1.0
    pub volume: f32,
    /// Use Language Codes according to ISO
    ///
    /// example: en(english), ja(japanese), hi(hindi)
    pub language: &'a str,
}

const ENCODE_FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

impl<'a> GTTSClient<'a> {
    fn save_to_file(&self, text: &str, filename: &str) -> bool {
        let len = text.len();
        let text = utf8_percent_encode(text, ENCODE_FRAGMENT).to_string();

        if let Ok(rep) = get(format!("https://translate.google.fr/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", text, self.language, len, self.language)).send() {
        if let Ok(mut file) = File::create(filename) {
            let bytes = rep.as_bytes();
            if bytes.len() > 0 {
                if file.write_all(bytes).is_ok() {
                    return true;
                }
            }
        }
    }
        false
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
    pub fn speak(&self, input: &str) {
        self.save_to_file(input, "audio.mp3");
        self.play_mp3("audio.mp3");
        if Path::new("./audio.mp3").exists() {
            fs::remove_file("./audio.mp3").unwrap();
        }
    }
    pub fn display_and_speak(&self, input: &str) {
        self.speak(input);
        println!("{}", input);
    }
    /// Fastest way to check if TTS works
    pub fn test(&self) {
        self.save_to_file("Hello!", "audio.mp3");
        self.play_mp3("audio.mp3");
        if Path::new("./audio.mp3").exists() {
            fs::remove_file("./audio.mp3").unwrap();
        }
    }
}

#[test]
fn check_function_1() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: "en",
    };
    narrator.speak("Starting test?");
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..9 {
        narrator.volume += 1.0;
        let to_speak: String = String::from("Loop ") + &narrator.volume.to_string();
        narrator.speak(&to_speak);
        std::thread::sleep(ms);
    }
}

#[test]
fn check_function_2() {
    let tester: GTTSClient = GTTSClient {
        volume: 1.5,
        language: "ja",
    };
    tester.test();
    tester.display_and_speak("displaying and speaking boi")
}
