use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::{CHOPSError, CHOPSResult, PersonaType, CreativityLevel, OutputFormat, WeirднessLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHOPSConfig {
    pub api_keys: ApiKeys,
    pub default_settings: DefaultSettings,
    pub persona_customizations: HashMap<String, PersonaCustomization>,
    pub output_preferences: OutputPreferences,
    pub behavior_settings: BehaviorSettings,
    pub template_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    pub claude_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub quantum_api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSettings {
    pub default_persona: PersonaType,
    pub default_chaos_level: u8,
    pub default_creativity: CreativityLevel,
    pub default_format: OutputFormat,
    pub default_weirdness_tolerance: WeirднessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaCustomization {
    pub custom_prompt_additions: Vec<String>,
    pub personality_amplifiers: HashMap<String, f64>,
    pub thinking_pattern_overrides: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPreferences {
    pub default_directory: PathBuf,
    pub auto_backup: bool,
    pub include_metadata: bool,
    pub timestamp_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    pub reality_distortion_enabled: bool,
    pub safe_mode: bool,
    pub auto_save_ideas: bool,
    pub enable_learning: bool,
}

impl CHOPSConfig {
    #[tracing::instrument(name = "config_load", level = "info")]
    pub fn load_from_file(path: &std::path::Path) -> CHOPSResult<Self> {
        tracing::info!("Loading configuration from path: {}", path.display());
        
        if !path.exists() {
            tracing::info!("Config file not found at {}, creating default configuration", path.display());
            let default_config = Self::default();
            tracing::debug!("Created default config with persona: {:?}, chaos_level: {}", 
                default_config.default_settings.default_persona,
                default_config.default_settings.default_chaos_level);
            return Ok(default_config);
        }
        
        tracing::debug!("Reading config file content");
        let content = std::fs::read_to_string(path)
            .map_err(|e| {
                tracing::error!("Failed to read config file {}: {}", path.display(), e);
                CHOPSError::FileSystemError(e)
            })?;
        
        tracing::debug!("Parsing TOML content, {} bytes", content.len());
        let config: CHOPSConfig = toml::from_str(&content)
            .map_err(|e| {
                tracing::error!("Invalid TOML in config file {}: {}", path.display(), e);
                CHOPSError::ConfigError(format!("Invalid TOML: {}", e))
            })?;
        
        tracing::debug!("Validating loaded configuration");
        config.validate()?;
        
        tracing::info!("Successfully loaded configuration from {}", path.display());
        Ok(config)
    }
    
    #[tracing::instrument(name = "config_save", level = "info")]
    pub fn save_to_file(&self, path: &std::path::Path) -> CHOPSResult<()> {
        tracing::info!("Saving configuration to path: {}", path.display());
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            tracing::debug!("Creating parent directories: {}", parent.display());
            std::fs::create_dir_all(parent)
                .map_err(|e| {
                    tracing::error!("Failed to create parent directories {}: {}", parent.display(), e);
                    CHOPSError::FileSystemError(e)
                })?;
        }
        
        tracing::debug!("Serializing configuration to TOML");
        let content = toml::to_string_pretty(self)
            .map_err(|e| {
                tracing::error!("Failed to serialize config to TOML: {}", e);
                CHOPSError::ConfigError(format!("Failed to serialize config: {}", e))
            })?;
        
        tracing::debug!("Writing {} bytes to config file", content.len());
        std::fs::write(path, content)
            .map_err(|e| {
                tracing::error!("Failed to write config file {}: {}", path.display(), e);
                CHOPSError::FileSystemError(e)
            })?;
        
        tracing::info!("Configuration saved successfully to {}", path.display());
        Ok(())
    }
    
    #[tracing::instrument(name = "get_config_path", level = "debug")]
    pub fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        let chops_config_path = config_dir.join("chops").join("config.toml");
        
        tracing::debug!("Determined config path: {}", chops_config_path.display());
        chops_config_path
    }
    
    #[tracing::instrument(name = "config_validate", level = "debug")]
    fn validate(&self) -> CHOPSResult<()> {
        tracing::debug!("Starting configuration validation");
        
        // Validate API keys format
        if let Some(ref key) = self.api_keys.claude_api_key {
            tracing::debug!("Validating Claude API key format");
            if !key.starts_with("sk-ant-") {
                tracing::error!("Invalid Claude API key format - must start with 'sk-ant-'");
                return Err(CHOPSError::ConfigError(
                    "Invalid Claude API key format. Must start with 'sk-ant-'".to_string()
                ));
            }
            tracing::debug!("Claude API key format is valid");
        } else {
            tracing::debug!("No Claude API key configured");
        }
        
        if let Some(ref key) = self.api_keys.openai_api_key {
            tracing::debug!("Validating OpenAI API key format");
            if !key.starts_with("sk-") {
                tracing::error!("Invalid OpenAI API key format - must start with 'sk-'");
                return Err(CHOPSError::ConfigError(
                    "Invalid OpenAI API key format. Must start with 'sk-'".to_string()
                ));
            }
            tracing::debug!("OpenAI API key format is valid");
        } else {
            tracing::debug!("No OpenAI API key configured");
        }
        
        // Validate chaos level bounds
        tracing::debug!("Validating chaos level: {}", self.default_settings.default_chaos_level);
        if self.default_settings.default_chaos_level > 11 {
            tracing::error!("Chaos level {} exceeds maximum of 11", self.default_settings.default_chaos_level);
            return Err(CHOPSError::ConfigError(
                "Chaos level must be between 1 and 11".to_string()
            ));
        }
        
        // Validate output directory exists or can be created
        tracing::debug!("Validating output directory: {}", self.output_preferences.default_directory.display());
        if !self.output_preferences.default_directory.exists() {
            tracing::info!("Creating output directory: {}", self.output_preferences.default_directory.display());
            std::fs::create_dir_all(&self.output_preferences.default_directory)
                .map_err(|e| {
                    tracing::error!("Failed to create output directory {}: {}", 
                        self.output_preferences.default_directory.display(), e);
                    CHOPSError::ConfigError(
                        format!("Cannot create output directory: {}", e)
                    )
                })?;
        }
        
        // Validate persona customizations
        tracing::debug!("Validating {} persona customizations", self.persona_customizations.len());
        for (persona_name, customization) in &self.persona_customizations {
            if persona_name.parse::<PersonaType>().is_err() {
                tracing::warn!("Unknown persona type in customizations: {}", persona_name);
            }
            
            // Validate amplifier values are reasonable
            tracing::debug!("Validating {} amplifiers for persona {}", 
                customization.personality_amplifiers.len(), persona_name);
            for (amplifier, value) in &customization.personality_amplifiers {
                if *value < 0.0 || *value > 2.0 {
                    tracing::error!("Personality amplifier '{}' value {} out of range [0.0, 2.0]", 
                        amplifier, value);
                    return Err(CHOPSError::ConfigError(
                        format!("Personality amplifier '{}' value {} out of range [0.0, 2.0]", 
                               amplifier, value)
                    ));
                }
            }
        }
        
        tracing::info!("Configuration validation completed successfully");
        Ok(())
    }
    
    #[tracing::instrument(name = "get_claude_api_key", level = "debug")]
    pub fn get_claude_api_key(&self) -> CHOPSResult<&str> {
        tracing::debug!("Retrieving Claude API key from configuration");
        
        match self.api_keys.claude_api_key.as_ref() {
            Some(key) => {
                tracing::debug!("Claude API key found in configuration");
                Ok(key.as_str())
            },
            None => {
                tracing::error!("Claude API key not configured");
                Err(CHOPSError::ConfigError(
                    "Claude API key not configured. Set CLAUDE_API_KEY environment variable or add to config file.".to_string()
                ))
            }
        }
    }
    
    #[tracing::instrument(name = "merge_with_env", level = "debug")]
    pub fn merge_with_env(&mut self) {
        tracing::debug!("Merging configuration with environment variables");
        
        // Override with environment variables if present
        if let Ok(key) = std::env::var("CLAUDE_API_KEY") {
            tracing::info!("Overriding Claude API key from environment variable");
            self.api_keys.claude_api_key = Some(key);
        }
        
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            tracing::info!("Overriding OpenAI API key from environment variable");
            self.api_keys.openai_api_key = Some(key);
        }
        
        if let Ok(key) = std::env::var("QUANTUM_API_KEY") {
            tracing::info!("Overriding Quantum API key from environment variable");
            self.api_keys.quantum_api_key = Some(key);
        }
        
        if let Ok(chaos_level) = std::env::var("CHOPS_DEFAULT_CHAOS") {
            tracing::debug!("Found CHOPS_DEFAULT_CHAOS environment variable: {}", chaos_level);
            if let Ok(level) = chaos_level.parse::<u8>() {
                if level <= 11 {
                    tracing::info!("Overriding default chaos level from environment: {}", level);
                    self.default_settings.default_chaos_level = level;
                } else {
                    tracing::warn!("Invalid chaos level in environment variable: {} (max 11)", level);
                }
            } else {
                tracing::warn!("Invalid chaos level format in environment variable: {}", chaos_level);
            }
        }
        
        if let Ok(safe_mode) = std::env::var("CHOPS_SAFE_MODE") {
            tracing::debug!("Found CHOPS_SAFE_MODE environment variable: {}", safe_mode);
            if let Ok(enabled) = safe_mode.parse::<bool>() {
                tracing::info!("Overriding safe mode from environment: {}", enabled);
                self.behavior_settings.safe_mode = enabled;
            } else {
                tracing::warn!("Invalid safe mode value in environment variable: {}", safe_mode);
            }
        }
        
        tracing::debug!("Configuration merge with environment completed");
    }
}

impl Default for CHOPSConfig {
    fn default() -> Self {
        Self {
            api_keys: ApiKeys {
                claude_api_key: None,
                openai_api_key: None,
                quantum_api_key: None,
            },
            default_settings: DefaultSettings {
                default_persona: PersonaType::MadScientist,
                default_chaos_level: 5,
                default_creativity: CreativityLevel::High,
                default_format: OutputFormat::Markdown,
                default_weirdness_tolerance: WeirднessLevel::Medium,
            },
            persona_customizations: HashMap::new(),
            output_preferences: OutputPreferences {
                default_directory: dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("chops-output"),
                auto_backup: true,
                include_metadata: true,
                timestamp_files: true,
            },
            behavior_settings: BehaviorSettings {
                reality_distortion_enabled: true,
                safe_mode: false,
                auto_save_ideas: true,
                enable_learning: true,
            },
            template_directories: vec![
                PathBuf::from("/usr/local/share/chops/templates"),
                dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("chops")
                    .join("templates"),
            ],
        }
    }
}