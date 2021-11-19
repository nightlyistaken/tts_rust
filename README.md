### `tts_rust`

Really Simple Text to Speech module for rust

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/)

## Example

#### Cargo.toml

Add this to your cargo.toml file:

```toml
   tts_rust = "0.2.0"
```

#### main.rs

```rust
fn main() {
    let english_speaker = tts_rust::TTS {
        volume: 1.5,
        language: "mr".to_string(),
    };
    english_speaker.speak("Hello There!");
}
```

### License

- MIT
