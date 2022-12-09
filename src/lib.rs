//! # Very Simple Module for gTTS
//! Example:
//!
mod constants;
/// ```
/// use tts_rust::languages::Languages;
/// use tts_rust::tts::GTTSClient;
///
/// let narrator = GTTSClient::new(1.0, Languages::English, "com");
/// narrator.speak("Hello!").unwrap();
/// ```
pub mod languages;
pub mod tts;
pub mod url;
