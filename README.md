### `tts_rust`

Really Simple Text to Speech module for rust

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/)

Uses `cargo fmt` as formatter

### Example...

```rust
use tts_rust::{ tts::GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("Hello, World!");
}
```

### ...Or a more advanced one

```rust
use tts_rust::{ tts::GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
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
```

### License

#### MIT
