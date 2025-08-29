use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct TTDInput {
    // The text to be converted into speech.
    pub text: String,
    // The ID of the voice to be used for the generation.
    pub voice_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TTDSettings {
    // Determines how stable the voice is and the randomness between each generation.
    // Lower values introduce broader emotional range for the voice.
    // Higher values can result in a monotonous voice with limited emotion.
    // Must be one of: 0.0, 0.5, 1.0. Default to 0.5 (natural).
    pub stability: Option<f32>,
    // This setting boosts the similarity to the original speaker.
    // Using this setting requires a slightly higher computational load, which in turn increases latency.
    pub use_speaker_boost: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TTDPronunciationDictionaryLocators {
    // The ID of the pronunciation dictionary.
    pub pronunciation_dictionary_id: String,
    // The ID of the version of the pronunciation dictionary. If not provided, the latest version will be used.
    pub version_id: Option<String>,
}

/// Request body for Text-to-Dialogue API calls
#[derive(Debug, Clone, Serialize)]
pub struct TTDRequest {
    // Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    // MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    // Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    // Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    // Default to: mp3_44100_128
    // This goes in the URL path, not in the body.
    pub output_format: Option<String>,

    // A list of dialogue inputs, each containing text and a voice ID which will be converted into speech.
    pub inputs: Vec<TTDInput>,

    // Identifier of the model that will be used.
    // Only Eleven V3 Family Supported for now.
    pub model_id: String,

    // Settings controlling the dialogue generation.
    pub settings: Option<TTDSettings>,

    // A list of pronunciation dictionary locators (id, version_id) to be applied to the text. They will be applied in order.
    // You may have up to 3 locators per request
    pub pronunciation_dictionary_locators: Option<TTDPronunciationDictionaryLocators>,

    // If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result.
    // Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    pub seed: Option<u32>,
}

impl Default for TTDSettings {
    fn default() -> Self {
        Self {
            stability: Some(0.5),
            use_speaker_boost: Some(true),
        }
    }
}

impl TTDSettings {
    pub fn new() -> Self {
        Self {
            // Default stability is 0.5 (natural)
            stability: Some(0.5),
            // Default to true
            use_speaker_boost: Some(true),
        }
    }

    /// Set stability
    pub fn stability(mut self, stability: f32) -> Self {
        self.stability = Some(stability.clamp(0.0, 1.0));
        self
    }

    /// Enable speaker boost
    pub fn speaker_boost(mut self, enabled: bool) -> Self {
        self.use_speaker_boost = Some(enabled);
        self
    }
}

/// Represents a static voice
#[derive(Debug, Clone, Deserialize)]
pub struct StaticVoice {
    pub voice_id: &'static str,
    pub name: &'static str,
    pub gender: &'static str,
}

impl StaticVoice {
    pub const fn new(voice_id: &'static str, name: &'static str, gender: &'static str) -> Self {
        Self {
            voice_id: voice_id,
            name: name,
            gender: gender,
        }
    }

    /// Get the voice ID for API calls
    pub fn id(&self) -> &str {
        &self.voice_id
    }
}
