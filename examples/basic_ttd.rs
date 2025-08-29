use elevenlabs_ttd::{ElevenLabsTTDClient, TTDInput, voices};
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
            text: "I saw the sky this morning, it looked like fire.".to_string(),
            voice_id: voices::all_voices::ARNOLD.voice_id.to_string(),
        },
        TTDInput {
            text: "I noticed that too, the sunrise was unreal.".to_string(),
            voice_id: voices::all_voices::IVANA.voice_id.to_string(),
        },
    ];

    // Convert text to dialogue audio
    let audio = client.text_to_dialogue(inputs).execute().await?;

    // Save to file to outputs directory
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
