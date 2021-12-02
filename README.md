### `tts_rust`

Really Simple Text to Speech module for rust

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/)

#### Cargo.toml

Add this to your `Cargo.toml` file:

```toml
   tts_rust = "0.3.2"
```

### Example...

```rust
use tts_rust::{ GTTSClient, Languages }
fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: tts_rust::Languages::English, // use the Languages enum
    };
    narrator.speak("Hello, World!");
}
```
### ...Or a more advanced one

```rust
fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: "en",
    };
    narrator.speak("Starting test?");
    let ms = std::time::Duration::from_millis(1000);
    for _x in 1..10 {
        narrator.volume += 1.0;
        let to_speak: String = String::from("Loop ") + &narrator.volume.to_string();
        narrator.speak(&to_speak);
        std::thread::sleep(ms);
    }
}
```

### License

- MIT
