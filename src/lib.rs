//! Very Simple Module for text to speech
//! Example:
//! text_speech("Hello, World!");
//! See the Source code here :
//! https://github.com/dhairy-online/tts-rust-mod

use gtts::save_to_file;
use std::fs;
use std::io::BufReader;
use std::path::Path;
/// This play_mp3 function plays mp3 example: 
/// play_mp3("./path.mp3");
pub fn play_mp3(mp3: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(mp3).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}
/// This function takes the text and plays it by speaking the text
/// Example: text_speech("Hello, Rust!");
pub fn text_speech(input: &str) {
    save_to_file(input, "audio.mp3");
    play_mp3("audio.mp3");
    if Path::new("./audio.mp3").exists() {
        fs::remove_file("./audio.mp3").unwrap();
    }
}

/// if you want to test the module, use this! :smile:
pub fn test_a_speech(){
    save_to_file("G r r r r r r r r r r r r, haha i scared you!", "test.mp3");
    play_mp3("test.mp3");
    if Path::new("./test.mp3").exists() {
        fs::remove_file("./test.mp3").unwrap();
    }
}

/// Speak and display text at he same time: 
pub fn text_speech_text(input: &str){
    text_speech(&input);
    println!("{}", input);  
}


#[test]
fn test() {
    // TTS
     text_speech("Hi");
    // TTS Test Speech
     test_a_speech();
    // TTS with print macro
     text_speech_text("hello, world!");
}
