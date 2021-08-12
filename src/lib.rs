use gtts::save_to_file;
use std::fs;
use std::io::BufReader;
use std::path::Path;
pub fn play_mp3(mp3: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(mp3).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

pub fn text_speech(input: &str) {
    save_to_file(input, "audio.mp3");
    play_mp3("audio.mp3");
    if Path::new("./audio.mp3").exists() {
        fs::remove_file("./audio.mp3").unwrap();
    }
}

#[test]
fn test1() {
    text_speech("Hi");
}
