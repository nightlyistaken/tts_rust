## `Text To Speech module for Rust`

Simple Text to Speech module for rust

Can be used for bots, websites etc.

### Made with ![rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)

## Module available at -

- [crates.io](https://crates.io/crates/tts_rust)
- [docs.rs](https://docs.rs/tts_rust/0.1.0)

## Example

#### Cargo.toml

Add this to your cargo.toml file:

```toml
 # [dependencies]
   tts_rust = "LATEST_VERSION"
```
Replace LATEST_VERSION with the newest version of [tts_rust](https://crates.io/crates/tts_rust)
#### main.rs

```rust
use tts_rust::text_speech;

fn main() {
   // Anything inside the double quotes (" ") will be spoken out
    text_speech("Hello, World!");
}


```

Thanks! :smile:

This repo is licensed under MIT license.
