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
//! narrator.speak("Hello!")?;
//! ```

pub mod languages;
pub mod tts;
pub mod url;
