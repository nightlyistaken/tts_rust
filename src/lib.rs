//! # Very Simple Module for gTTS
//! Example:
//! ```rust
//! use tts_rust::languages::Languages;
//! use tts_rust::GTTSClient;
//! 
//! let narrator = GTTSClient {
//!     volume: 1.0,
//!     language: Languages::English,
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

pub mod languages;
use languages::Languages;

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

const ENCODE_FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

impl GTTSClient {
    fn save_to_file(&self, text: &str, filename: &str) -> bool {
        let len = text.len();
        let text = utf8_percent_encode(text, ENCODE_FRAGMENT).to_string();
        let ln = match self.language {
            Languages::Afrikaans => "af",
            Languages::Albanian => "sq",
            Languages::Arabic => "ar",
            Languages::Armenian => "hy",
            Languages::Bengali => "bn",
            Languages::Bosnian => "bs",
            Languages::Bulgarian => "bg",
            Languages::Catalan => "ca",
            Languages::Chinese => "zh-CN",
            Languages::Croatian => "hr",
            Languages::Czech => "cs",
            Languages::Danish => "da",
            Languages::Dutch => "nl",
            Languages::English => "en",
            Languages::Esperanto => "eo",
            Languages::Estonian => "et",
            Languages::Filipino => "tl",
            Languages::Finnish => "fi",
            Languages::French => "fr",
            Languages::German => "de",
            Languages::Greek => "el",
            Languages::Gujarati => "gu",
            Languages::Hindi => "hi",
            Languages::Hungarian => "hu",
            Languages::Icelandic => "is",
            Languages::Indonesian => "id",
            Languages::Italian => "it",
            Languages::Japanese => "ja",
            Languages::Javanese => "jw",
            Languages::Kannada => "kn",
            Languages::Khmer => "km",
            Languages::Korean => "ko",
            Languages::Latin => "la",
            Languages::Latvian => "lv",
            Languages::Macedonian => "mk",
            Languages::Marathi => "mr",
            Languages::Nepali => "ne",
            Languages::Norwegian => "no",
            Languages::Polish => "pl",
            Languages::Portuguese => "pt",
            Languages::Romanian => "ro",
            Languages::Russian => "ru",
            Languages::Serbian => "sr",
            Languages::Sinhala => "si",
            Languages::Slovak => "sk",
            Languages::Spanish => "es",
            Languages::Swahili => "sw",
            Languages::Swedish => "sv",
            Languages::Tamil => "ta",
            Languages::Telugu => "te",
            Languages::Thai => "th",
            Languages::Turkish => "tr",
            Languages::Ukrainian => "uk",
            Languages::Urdu => "ur",
            Languages::Vietnamese => "vi",
            Languages::Welsh => "cy",
            Languages::MyanmarAKABurmese => "my",
            Languages::Malayalam => "ml",
            Languages::Sundanese => "su",
        };
        if let Ok(rep) = get(format!("https://translate.google.fr/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", text, ln, len, ln)).send() {
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
    /// Speak and println! the input according to the volume and language
    pub fn display_and_speak(&self, input: &str) {
        self.speak(input);
        println!("{}", input);
    }
    /// Fastest way to check if gTTS works
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
        language: Languages::Afrikaans,
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
        language: Languages::English,
    };
    tester.test();
    tester.display_and_speak("displaying and speaking boi")
}
