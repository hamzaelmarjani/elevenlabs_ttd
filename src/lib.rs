//! ElevenLabs Text-to-Dialogue API client
//!
//! A type-safe, async Rust client for the ElevenLabs TTD API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_ttd::{ElevenLabsTTDClient, TTDInput, voices};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsTTDClient::new("your-api-key");
//!
//!      let inputs = vec![
//!        TTDInput {
//!            text: "I saw the sky this morning, it looked like fire.".to_string(),
//!            voice_id: voices::all_voices::ARNOLD.voice_id.to_string(),
//!        },
//!        TTDInput {
//!            text: "I noticed that too, the sunrise was unreal.".to_string(),
//!            voice_id: voices::all_voices::IVANA.voice_id.to_string(),
//!        },
//!     ];
//!     
//!     let audio = client
//!         .text_to_dialogue(inputs)
//!         .execute()
//!         .await?;
//!     
//!     // audio is Vec<u8> - raw audio data
//!     std::fs::write("output.mp3", audio)?;
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;
pub mod voices;

pub use error::ElevenLabsTTDError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsTTDClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsTTDClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a Text-to-Dialogue request
    pub fn text_to_dialogue<I: Into<Vec<TTDInput>>>(&self, inputs: I) -> TextToDialogueBuilder {
        TextToDialogueBuilder::new(self.clone(), inputs.into())
    }

    /// Internal method to execute TTD request
    pub(crate) async fn execute_ttd(
        &self,
        request: TTDRequest,
    ) -> Result<Vec<u8>, ElevenLabsTTDError> {
        let mut url = format!("{}/text-to-dialogue", self.base_url);

        if request.output_format.is_some() {
            url = format!(
                "{}?output_format={}",
                url,
                request.output_format.clone().unwrap()
            );
        }

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsTTDError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

/// Builder for Text-to-Dialogue requests
pub struct TextToDialogueBuilder {
    client: ElevenLabsTTDClient,
    inputs: Vec<TTDInput>,
    output_format: Option<String>,
    model_id: Option<String>,
    settings: Option<TTDSettings>,
    pronunciation_dictionary_locators: Option<TTDPronunciationDictionaryLocators>,
    seed: Option<u32>,
}

impl TextToDialogueBuilder {
    fn new(client: ElevenLabsTTDClient, inputs: Vec<TTDInput>) -> Self {
        Self {
            client,
            inputs,
            output_format: None,
            model_id: None,
            settings: None,
            pronunciation_dictionary_locators: None,
            seed: None,
        }
    }

    /// Set the output format to use
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Set the model to use
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Set the settings to use
    pub fn settings(mut self, settings: TTDSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    /// Set the pronunciation dictionary locators to use
    pub fn pronunciation_dictionary_locators(
        mut self,
        pronunciation_dictionary_locators: TTDPronunciationDictionaryLocators,
    ) -> Self {
        self.pronunciation_dictionary_locators = Some(pronunciation_dictionary_locators);
        self
    }

    /// Set seeds to use
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Execute the Text-to-Dialogue request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsTTDError> {
        let output_format = self
            .output_format
            .unwrap_or_else(|| "mp3_44100_128".to_string()); // Default to: mp3_44100_128

        let request = TTDRequest {
            inputs: self.inputs,
            output_format: Some(output_format.clone()),
            seed: self.seed.or(None),
            model_id: self
                .model_id
                .unwrap_or_else(|| models::elevanlabs_models::ELEVEN_V3.to_string()), // Default to: eleven_v3
            settings: self.settings.or(None),
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators.or(None),
        };

        self.client.execute_ttd(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsTTDClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern() {
        let client = ElevenLabsTTDClient::new("test-key");
        let builder = client.text_to_dialogue([]).model("model-456");

        // Builder pattern works
        assert!(builder.inputs.is_empty());
        assert_eq!(builder.model_id, Some("model-456".to_string()));
    }
}
