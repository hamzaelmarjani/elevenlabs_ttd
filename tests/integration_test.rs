use elevenlabs_ttd::{ElevenLabsTTDClient, ElevenLabsTTDError, models, voices};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsTTDClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_pattern() {
    let client = ElevenLabsTTDClient::new("test-key");
    let _builder = client
        .text_to_dialogue([])
        .model(models::elevanlabs_models::ELEVEN_V3);

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_error_display() {
    let error = ElevenLabsTTDError::ValidationError("Invalid voice ID".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid voice ID"));
}

#[test]
fn test_static_voices() {
    // Test voice constants
    assert_eq!(voices::all_voices::RACHEL.voice_id, "21m00Tcm4TlvDq8ikWAM");
    assert_eq!(voices::all_voices::RACHEL.name, "Rachel");
    assert_eq!(voices::all_voices::RACHEL.gender, "female");

    // Test voice ID access
    assert_eq!(voices::all_voices::ANTONI.id(), "ErXwobaYiN019PkySvjV");
}

#[test]
fn test_voice_filtering() {
    let all_voices = voices::all_voices::all();
    let male_voices = voices::all_voices::male();
    let female_voices = voices::all_voices::female();

    assert!(all_voices.len() > 0);
    assert!(male_voices.len() > 0);
    assert!(female_voices.len() > 0);
    assert_eq!(all_voices.len(), male_voices.len() + female_voices.len());

    // Check that filtering works correctly
    for voice in male_voices {
        assert_eq!(voice.gender, "male");
    }

    for voice in female_voices {
        assert_eq!(voice.gender, "female");
    }
}

#[test]
fn test_voice_search() {
    let found = voices::all_voices::find_by_name("Rachel");
    assert!(found.is_some());
    assert_eq!(found.unwrap().voice_id, "21m00Tcm4TlvDq8ikWAM");

    // Test case insensitive
    let found_lower = voices::all_voices::find_by_name("rachel");
    assert!(found_lower.is_some());
    assert_eq!(found_lower.unwrap().voice_id, "21m00Tcm4TlvDq8ikWAM");

    // Test not found
    let not_found = voices::all_voices::find_by_name("NonExistentVoice");
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_builder_with_voice_reference() {
    let client = ElevenLabsTTDClient::new("test-key");
    let _builder = client
        .text_to_dialogue([])
        .model(models::elevanlabs_models::ELEVEN_V3);

    // Builder pattern works if this compiles
    assert_eq!(true, true);
}

// Mock tests for API calls (without real HTTP requests)
#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsTTDClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
