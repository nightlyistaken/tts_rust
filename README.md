### `tts_rust`

Really Simple Text to Speech module for rust

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/)

#### Cargo.toml

Add this to your `Cargo.toml` file:

```toml
tts_rust = "0.3.4"
```

Uses `cargo fmt` as formatter

use `cargo update` to update to version `0.3.3`

### Example...

```rust
use tts_rust::{ GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
    };
    narrator.speak("Hello, World!");
}
```

### ...Or a more advanced one

```rust
use tts_rust::{ GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
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
