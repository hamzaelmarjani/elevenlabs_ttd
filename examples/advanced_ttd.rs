use elevenlabs_ttd::{ElevenLabsTTDClient, TTDInput, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    // Create ElevenLabs client
    let client = ElevenLabsTTDClient::new(api_key);

    // Example dialogue inputs (2 speakers)
    let inputs = vec![
        TTDInput {
            text: "I started reading a book last night, couldn’t stop.".to_string(),
            voice_id: voices::all_voices::ALICE.voice_id.to_string(),
        },
        TTDInput {
            text: "That’s the best feeling, when pages pull you in.".to_string(),
            voice_id: voices::all_voices::ANTONI.voice_id.to_string(),
        },
        TTDInput {
            text: " It felt like living another life for a while.".to_string(),
            voice_id: voices::all_voices::ALICE.voice_id.to_string(),
        },
        TTDInput {
            text: "Books do that better than anything else.".to_string(),
            voice_id: voices::all_voices::ANTONI.voice_id.to_string(),
        },
    ];

    // Custom speech settings
    let settings = elevenlabs_ttd::TTDSettings::new()
        .stability(0.5)
        .speaker_boost(true);

    // Convert text to dialogue audio
    let audio = client
        .text_to_dialogue(inputs)
        .model(models::elevanlabs_models::ELEVEN_V3)
        .output_format("mp3_44100_128")
        .settings(settings)
        .seed(4000)
        .execute()
        .await?;

    // Save to file to outputs directory
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
