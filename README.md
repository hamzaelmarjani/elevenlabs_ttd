# elevenlabs_ttd

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_ttd.svg)](https://crates.io/crates/elevenlabs_ttd)
[![Docs.rs](https://docs.rs/elevenlabs_ttd/badge.svg)](https://docs.rs/elevenlabs_ttd)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Text-to-Dialogue API](https://elevenlabs.io/docs/api-reference/text-to-dialogue/convert). Generate high-quality speech from text with a simple, ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring TTD requests
- **Predefined Voices**: Access to static voice definitions (`voices::all_voices::*`)
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Voice settings, Elevanlabs TTD APIs, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime

## Check-out Also:

**This project is part of a milestone to implement all ElevenLabs APIs in Rust.**

- **[Elevenlabs TTS](https://crates.io/crates/elevenlabs_tts)**: ElevenLabs Text-to-Speech API. ✅
- **[Elevenlabs SFX](https://crates.io/crates/elevenlabs_sfx)**: ElevenLabs Sound Effects API. ✅
- **[Elevenlabs STT](https://crates.io/crates/elevenlabs_stt)**: ElevenLabs Speech To Text API. ✅
- **[Elevenlabs TTD](https://crates.io/crates/elevenlabs_ttd)**: ElevenLabs Speech To Dialogue API. ✅
- **Elevenlabs TTV**: ElevenLabs Text To Voice API. ⏳
- **Elevenlabs VC**: ElevenLabs Voice Changer API. ⏳
- **Elevenlabs CM**: ElevenLabs Music Compose API. ⏳
- **Elevenlabs AUI**: ElevenLabs Audio Isolation API. ⏳
- **Elevenlabs DUB**: ElevenLabs Dubbing API. ⏳

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_ttd = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_ttd::ElevenLabsTTDClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsTTDClient::new("your-api-key");

    let inputs = vec![
        TTDInput {
            text: "I saw the sky this morning, it looked like fire.".to_string(),
            voice_id: voices::all_voices::ARNOLD.voice_id.to_string(),
        },
        TTDInput {
            text: "I noticed that too, the sunrise was unreal.".to_string(),
            voice_id: voices::all_voices::IVANA.voice_id.to_string(),
        },
    ];

    let audio = client.text_to_dialogue(inputs).execute().await?;
    std::fs::write("outputs.mp3", &audio)?;

    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_ttd::{ElevenLabsTTDClient, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("ELEVENLABS_API_KEY")
        .expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTDClient::new(api_key);

    let inputs = vec![
        TTDInput {
            text: "I saw the sky this morning, it looked like fire.".to_string(),
            voice_id: voices::all_voices::ARNOLD.voice_id.to_string(),
        },
        TTDInput {
            text: "I noticed that too, the sunrise was unreal.".to_string(),
            voice_id: voices::all_voices::IVANA.voice_id.to_string(),
        },
    ];

    let audio = client.text_to_dialogue(inputs).execute().await?;

    std::fs::create_dir_all("outputs")?;
    std::fs::write("outputs/output.mp3", audio)?;
    println!("Audio saved to outputs/output.mp3");
    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_ttd::{ElevenLabsTTDClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let api_key = env::var("ELEVENLABS_API_KEY")
        .expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTDClient::new(api_key);

    let inputs = vec![
        TTDInput {
            text: "I saw the sky this morning, it looked like fire.".to_string(),
            voice_id: voices::all_voices::ARNOLD.voice_id.to_string(),
        },
        TTDInput {
            text: "I noticed that too, the sunrise was unreal.".to_string(),
            voice_id: voices::all_voices::IVANA.voice_id.to_string(),
        },
    ];

    let settings = elevenlabs_ttd::TTDSettings::new()
        .stability(0.5)
        .speaker_boost(true);

    let audio = client
        .text_to_dialogue(inputs)
        .model(models::elevanlabs_models::ELEVEN_V3)
        .output_format("mp3_44100_128")
        .settings(settings)
        .seed(4000)
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    std::fs::write("outputs/output.mp3", audio)?;
    println!("Audio saved to outputs/output.mp3");
    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_ttd

# Run the advanced example
cargo run --example advanced_ttd
```

## API Overview

| Method                                                                   | Description                                                                                       |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| `ElevenLabsTTDClient::new(String)`                                       | Create client instance (required)\*                                                               |
| `.text_to_dialogue(String)`                                              | Build a TTD request (required)\*                                                                  |
| `.inputs(TTDInput)`                                                      | A list of dialogue inputs, each containing text & a voice_id (required)\*                         |
| `.output_format(String)`                                                 | Audio format (e.g. mp3_44100) (optional)                                                          |
| `.model_id(String)`                                                      | Only Eleven V3 Family Supported for now (optional)                                                |
| `.settings(TTDSettings)`                                                 | Settings controlling the dialogue generation. (optional)                                          |
| `.pronunciation_dictionary_locators(TTDPronunciationDictionaryLocators)` | A list of pronunciation dictionary locators (id, version_id) to be applied to the text (optional) |
| `.seed(u32)`                                                             | Deterministic sampling (optional)                                                                 |
| `.execute()`                                                             | Run request → audio (required)\*                                                                  |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.text_to_dialogue(inputs).execute().await {
    Ok(audio) => println!("Generated {} bytes of audio", audio.len()),
    Err(e) => eprintln!("TTD generation failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/text-to-dialogue/convert) for the most up-to-date API information.
