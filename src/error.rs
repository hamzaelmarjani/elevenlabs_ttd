use std::fmt;

/// All possible errors that can occur when using the ElevenLabs API
#[derive(Debug)]
pub enum ElevenLabsTTDError {
    /// HTTP request failed (network issues, timeout, etc.)
    RequestError(reqwest::Error),

    /// API returned an error status code
    ApiError { status: u16, message: String },

    /// Failed to parse JSON response
    ParseError(serde_json::Error),

    /// Invalid API key or authentication failed
    AuthenticationError(String),

    /// Rate limit exceeded
    RateLimitError {
        retry_after: Option<u64>, // seconds
        message: String,
    },

    /// Quota exceeded (not enough credits)
    QuotaExceededError(String),

    /// Invalid input parameters
    ValidationError(String),
}

impl fmt::Display for ElevenLabsTTDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElevenLabsTTDError::RequestError(e) => write!(f, "Request failed: {}", e),
            ElevenLabsTTDError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ElevenLabsTTDError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            ElevenLabsTTDError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ElevenLabsTTDError::RateLimitError {
                retry_after,
                message,
            } => match retry_after {
                Some(seconds) => write!(
                    f,
                    "Rate limit exceeded (retry in {}s): {}",
                    seconds, message
                ),
                None => write!(f, "Rate limit exceeded: {}", message),
            },
            ElevenLabsTTDError::QuotaExceededError(msg) => write!(f, "Quota exceeded: {}", msg),
            ElevenLabsTTDError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ElevenLabsTTDError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElevenLabsTTDError::RequestError(e) => Some(e),
            ElevenLabsTTDError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ElevenLabsTTDError {
    fn from(error: reqwest::Error) -> Self {
        // Check if it's a specific HTTP status error
        if let Some(status) = error.status() {
            let status_code = status.as_u16();
            match status_code {
                401 => ElevenLabsTTDError::AuthenticationError("Invalid API key".to_string()),
                429 => {
                    // Try to extract retry-after header if available
                    ElevenLabsTTDError::RateLimitError {
                        retry_after: None, // Could be enhanced to parse Retry-After header
                        message: "Too many requests".to_string(),
                    }
                }
                402 => ElevenLabsTTDError::QuotaExceededError("Insufficient credits".to_string()),
                _ => ElevenLabsTTDError::ApiError {
                    status: status_code,
                    message: error.to_string(),
                },
            }
        } else {
            ElevenLabsTTDError::RequestError(error)
        }
    }
}

impl From<serde_json::Error> for ElevenLabsTTDError {
    fn from(error: serde_json::Error) -> Self {
        ElevenLabsTTDError::ParseError(error)
    }
}
