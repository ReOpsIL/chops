use thiserror::Error;

#[derive(Error, Debug)]
pub enum CHOPSError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Persona error: {0}")]
    PersonaError(String),

    #[error("Chaos engine error: {0}")]
    ChaosError(String),

    #[error("Cognitive processing error: {0}")]
    CognitiveError(String),

    #[error("Memory management error: {0}")]
    MemoryError(String),

    #[error("Reality calibration error: {0}")]
    RealityError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[derive(Error, Debug)]
pub enum PersonaError {
    #[error("Unknown persona type: {0}")]
    UnknownPersonaType(String),

    #[error("Persona configuration invalid: {0}")]
    InvalidConfiguration(String),

    #[error("Persona processing failed: {0}")]
    ProcessingFailed(String),
}

#[derive(Error, Debug)]
pub enum ChaosError {
    #[error("Invalid chaos level: {0}")]
    InvalidChaosLevel(u8),

    #[error("Entropy source unavailable: {0}")]
    EntropySourceUnavailable(String),

    #[error("Chaos calculation failed: {0}")]
    CalculationFailed(String),
}

impl From<ChaosError> for CHOPSError {
    fn from(err: ChaosError) -> Self {
        CHOPSError::ChaosError(err.to_string())
    }
}

#[derive(Error, Debug)]
pub enum CognitiveError {
    #[error("Reasoning failed: {0}")]
    ReasoningFailed(String),

    #[error("Analogical mapping failed: {0}")]
    AnalogicalMappingFailed(String),

    #[error("Synthesis error: {0}")]
    SynthesisError(String),

    #[error("Perspective generation failed: {0}")]
    PerspectiveGenerationFailed(String),
}

pub type CHOPSResult<T> = Result<T, CHOPSError>;
pub type PersonaResult<T> = Result<T, PersonaError>;
pub type ChaosResult<T> = Result<T, ChaosError>;
pub type CognitiveResult<T> = Result<T, CognitiveError>;