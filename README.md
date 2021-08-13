### `Text To Speech module for Rust`

#### Really Simple Text to Speech module for rust

#### Can be used for bots, websites etc.

### Made with

![rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)

## Module available at -

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/0.1.0)

## Example

#### Cargo.toml

Add this to your cargo.toml file:

```toml
   tts_rust = "0.2.0"
```

#### main.rs

```rust
// use
use tts_rust::text_speech;
use tts_rust::text_speech_text;
use tts_rust::test_a_speech;

fn main() {
   // Anything inside the double quotes (" ") will be spoken out
    text_speech("Hello, World!");
    // Speak and display:
    text_speech_text("Hello, I am speaking and writing!");
   // Test the module by doing this:
   test_a_speech();
}


```

Thank you! :smile:

### This repo is licensed under:

- MIT
