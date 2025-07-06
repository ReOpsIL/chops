use chops_core::{CHOPSResult, CHOPSError, PersonaType};
use chops_persona::{PersonaEngine, PersonaPrompt};
use chops_chaos::{ChaosEngine, ChaosInjectionResult};
use crate::models::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue};
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone)]
pub struct ClaudeClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
    config: ClaudeConfig,
    rate_limiter: RateLimiter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub max_tokens: u32,
    // pub temperature: f64,
    // pub top_p: f64,
    // pub top_k: Option<u32>,
    pub stop_sequences: Vec<String>,
    pub timeout_seconds: u64,
    pub retry_attempts: u8,
    pub retry_delay_ms: u64,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests_per_minute: u32,
    tokens_per_minute: u32,
    current_requests: u32,
    current_tokens: u32,
    last_reset: std::time::Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeRequest {
    pub model: String,
    pub messages: Vec<ClaudeMessage>,
    pub max_tokens: u32,
    // pub temperature: f64,
    // pub top_p: Option<f64>,
    // pub top_k: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    //pub system: Option<String>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResponse {
    pub id: String,
    pub model: String,
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl ClaudeClient {
    #[tracing::instrument(name = "claude_client_new", level = "info", skip(api_key))]
    pub fn new(api_key: String) -> CHOPSResult<Self> {
        tracing::info!("Creating new Claude client");
        
        if !api_key.starts_with("sk-ant-") {
            tracing::error!("Invalid Claude API key format - must start with 'sk-ant-'");
            return Err(CHOPSError::AuthenticationError(
                "Invalid Claude API key format. Must start with 'sk-ant-'".to_string()
            ));
        }
        
        tracing::debug!("API key format validated");

        let mut headers = HeaderMap::new();
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(CHOPSError::NetworkError)?;

        tracing::info!("Claude client created successfully with model: claude-3-5-sonnet-20241022");
        
        Ok(Self {
            client,
            api_key,
            base_url: "https://api.anthropic.com".to_string(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            config: ClaudeConfig::default(),
            rate_limiter: RateLimiter::new(),
        })
    }

    pub fn configure(&mut self, config: ClaudeConfig) {
        self.config = config;
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    #[tracing::instrument(name = "generate_idea_with_persona", level = "info", skip(self, persona_engine, chaos_engine))]
    pub async fn generate_idea_with_persona(
        &mut self,
        persona_engine: &PersonaEngine,
        chaos_engine: &mut ChaosEngine,
        prompt: &str,
        persona_type: PersonaType,
        domain: &str,
    ) -> CHOPSResult<GeneratedIdeaResponse> {
        tracing::info!("Generating idea with persona: {:?} for domain: '{}'", persona_type, domain);
        tracing::debug!("Input prompt length: {} characters", prompt.len());
        
        // Check rate limits
        tracing::debug!("Checking rate limits");
        self.rate_limiter.check_limits().await?;

        // Generate persona prompt
        tracing::debug!("Generating persona prompt");
        let persona_prompt = persona_engine.generate_persona_prompt(&persona_type, Some(domain))?;
        
        // Apply chaos injection to the base prompt
        tracing::debug!("Applying chaos injection");
        let chaos_result = chaos_engine.inject_creative_chaos(prompt).await?;
        tracing::debug!("Chaos injection complete - {} variations generated", chaos_result.variations_generated.len());
        
        // Construct enhanced prompt
        tracing::debug!("Constructing enhanced prompt");
        let enhanced_prompt = self.construct_enhanced_prompt(&persona_prompt, prompt, &chaos_result)?;
        
        tracing::debug!("Enhanced prompt length: {} characters", enhanced_prompt.len());

        // Make API request with retries
        tracing::debug!("Making API request to Claude");
        let response = self.make_request_with_retries(&enhanced_prompt).await?;
        
        // Parse and enhance response
        tracing::debug!("Parsing Claude response");
        let idea_response = self.parse_response(response, persona_type, chaos_result).await?;

        // Update rate limiter
        if let Some(usage) = &idea_response.usage {
            self.rate_limiter.record_usage(1, usage.input_tokens + usage.output_tokens);
            tracing::debug!("Rate limiter updated - tokens used: {}", usage.input_tokens + usage.output_tokens);
        }

        tracing::info!("Idea generation complete");
        Ok(idea_response)
    }

    #[tracing::instrument(name = "collaborate_ai_debate", level = "info", skip(self))]
    pub async fn collaborate_ai_debate(
        &mut self,
        topic: &str,
        positions: Vec<String>,
        rounds: u8,
    ) -> CHOPSResult<DebateResult> {
        tracing::info!("Starting AI collaboration debate on topic: '{}' with {} positions, {} rounds", 
            topic, positions.len(), rounds);
        
        let mut debate_rounds = Vec::new();
        let mut current_context = format!("Topic: {}", topic);

        for round in 1..=rounds {
            tracing::info!("Starting debate round {}/{}", round, rounds);
            
            let mut round_responses = Vec::new();
            
            for (i, position) in positions.iter().enumerate() {
                let debate_prompt = format!(
                    "You are participating in an AI collaboration debate. 
                    
                    Topic: {}
                    Your position: {}
                    Round: {}/{}
                    
                    Previous context: {}
                    
                    Provide a thoughtful, well-reasoned argument for your position. 
                    Build on previous arguments and address counterpoints.
                    Be creative but intellectually honest.",
                    topic, position, round, rounds, current_context
                );

                let response = self.make_request_with_retries(&debate_prompt).await?;
                let content = self.extract_text_content(&response)?;
                
                round_responses.push(DebateResponse {
                    position: position.clone(),
                    round,
                    argument: content.clone(),
                    timestamp: chrono::Utc::now(),
                });

                // Update context for next participant
                current_context = format!("{}\n\nPosition {}: {}", current_context, i + 1, content);
            }

            debate_rounds.push(DebateRound {
                round_number: round,
                responses: round_responses,
            });

            // Add delay between rounds to respect rate limits
            if round < rounds {
                tokio::time::sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
            }
        }

        // Generate synthesis
        let synthesis_prompt = format!(
            "Analyze this AI collaboration debate and provide a thoughtful synthesis:
            
            Topic: {}
            
            Full debate transcript:
            {}
            
            Provide:
            1. Key insights that emerged
            2. Areas of convergence and divergence
            3. Novel ideas that emerged from the collaboration
            4. Potential next steps or solutions",
            topic,
            self.format_debate_transcript(&debate_rounds)
        );

        let synthesis_response = self.make_request_with_retries(&synthesis_prompt).await?;
        let synthesis = self.extract_text_content(&synthesis_response)?;

        Ok(DebateResult {
            topic: topic.to_string(),
            rounds: debate_rounds,
            synthesis,
            total_rounds: rounds,
            participants: positions,
        })
    }

    pub async fn generate_future_prophecy(
        &mut self,
        domain: &str,
        year: Option<u32>,
        context: &str,
    ) -> CHOPSResult<ProphecyResponse> {
        let target_year = year.unwrap_or(2030);
        
        let prophecy_prompt = format!(
            "You are a time traveler from the year {} who has returned to share insights about the future of {}.
            
            Context: {}
            
            As someone who has witnessed the technological evolution, provide:
            1. Major breakthroughs that occurred between now and {}
            2. Unexpected developments that surprised even experts
            3. How current trends evolved in surprising ways
            4. Practical advice for developers/innovators working today
            5. Technologies or approaches that seemed promising but failed
            6. The most important paradigm shifts that occurred
            
            Write as if you're sharing memories of actual events you witnessed.
            Be specific and vivid, but maintain plausibility based on current trends.",
            target_year, domain, context, target_year
        );

        let response = self.make_request_with_retries(&prophecy_prompt).await?;
        let prophecy_content = self.extract_text_content(&response)?;

        let confidence_level = self.assess_prophecy_confidence(&prophecy_content);
        
        Ok(ProphecyResponse {
            domain: domain.to_string(),
            target_year,
            prophecy: prophecy_content,
            context: context.to_string(),
            confidence_level,
            generated_at: chrono::Utc::now(),
        })
    }

    fn construct_enhanced_prompt(
        &self,
        persona_prompt: &PersonaPrompt,
        base_prompt: &str,
        chaos_result: &ChaosInjectionResult,
    ) -> CHOPSResult<String> {
        let mut enhanced = String::new();

        // System prompt with persona
        enhanced.push_str(&persona_prompt.base_prompt);
        enhanced.push_str("\n\n");

        // Thinking patterns
        if !persona_prompt.thinking_patterns.is_empty() {
            enhanced.push_str("Your thinking patterns:\n");
            for pattern in &persona_prompt.thinking_patterns {
                enhanced.push_str(&format!("- {}\n", pattern));
            }
            enhanced.push_str("\n");
        }

        // Chaos injection context
        if chaos_result.chaos_applied > 0.1 {
            enhanced.push_str(&format!(
                "Chaos injection applied (level: {:.2}). Embrace these unexpected elements:\n",
                chaos_result.chaos_applied
            ));
            
            for element in &chaos_result.unexpected_elements {
                enhanced.push_str(&format!("- {}\n", element));
            }
            
            if !chaos_result.variations_generated.is_empty() {
                enhanced.push_str("\nChaos variations to consider:\n");
                for variation in &chaos_result.variations_generated {
                    enhanced.push_str(&format!("- {}\n", variation.description));
                }
            }
            enhanced.push_str("\n");
        }

        // Base prompt
        enhanced.push_str("Your task:\n");
        enhanced.push_str(base_prompt);
        
        // Response format guidance
        enhanced.push_str("\n\nProvide your response with creativity, insight, and the personality traits specified above.");

        Ok(enhanced)
    }

    async fn make_request_with_retries(&mut self, prompt: &str) -> CHOPSResult<ClaudeResponse> {
        let mut last_error = None;

        for attempt in 1..=self.config.retry_attempts {
            match self.make_request(prompt).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    last_error = Some(error);
                    
                    if attempt < self.config.retry_attempts {
                        warn!("Request attempt {} failed, retrying...", attempt);
                        tokio::time::sleep(Duration::from_millis(
                            self.config.retry_delay_ms * attempt as u64
                        )).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            CHOPSError::UnexpectedError("All retry attempts failed".to_string())
        }))
    }

    async fn make_request(&self, prompt: &str) -> CHOPSResult<ClaudeResponse> {
        let request = ClaudeRequest {
            model: self.model.clone(),
            messages: vec![ClaudeMessage {
                role: MessageRole::User,
                content: prompt.to_string(),
            }],
            max_tokens: self.config.max_tokens,
            // temperature: self.config.temperature,
            // top_p: Some(self.config.top_p),
            // top_k: self.config.top_k,
            stop_sequences: if self.config.stop_sequences.is_empty() {
                None
            } else {
                Some(self.config.stop_sequences.clone())
            },
            // system: None,
            stream: false,
        };

        let r = serde_json::to_string(&request).unwrap();

        //println!("------------\n{}------------\n",r);
        debug!("Making Claude API request to {}", self.base_url);

        let response = self.client
            .post(&format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .json(&request)
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(CHOPSError::NetworkError)?;

        let status = response.status();
        
        if status.is_success() {
            let claude_response: ClaudeResponse = response
                .json()
                .await
                .map_err(|e| CHOPSError::NetworkError(e))?;

            debug!("Successfully received Claude response");
            Ok(claude_response)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            let error = match status {
                reqwest::StatusCode::UNAUTHORIZED => {
                    CHOPSError::AuthenticationError("Invalid API key".to_string())
                },
                reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    CHOPSError::RateLimitError("Rate limit exceeded".to_string())
                },
                reqwest::StatusCode::BAD_REQUEST => {
                    CHOPSError::ApiError(format!("Bad request: {}", error_text))
                },
                _ => {
                    CHOPSError::ApiError(format!("HTTP {}: {}", status, error_text))
                }
            };

            error!("Claude API error: {:?}", error);
            Err(error)
        }
    }

    async fn parse_response(
        &self,
        response: ClaudeResponse,
        persona_type: PersonaType,
        chaos_result: ChaosInjectionResult,
    ) -> CHOPSResult<GeneratedIdeaResponse> {
        let content = self.extract_text_content(&response)?;
        
        // Analyze the generated content
        let creativity_score = self.assess_creativity_score(&content, &chaos_result);
        let feasibility_score = self.assess_feasibility_score(&content);
        let novelty_score = self.assess_novelty_score(&content);
        let excitement_factor = self.assess_excitement_factor(&content, &persona_type);

        Ok(GeneratedIdeaResponse {
            id: uuid::Uuid::new_v4(),
            content,
            persona_used: persona_type,
            chaos_level: chaos_result.chaos_applied,
            creativity_score,
            feasibility_score,
            novelty_score,
            excitement_factor,
            chaos_variations: chaos_result.variations_generated,
            unexpected_elements: chaos_result.unexpected_elements,
            coherence_score: chaos_result.coherence_score,
            raw_response: response.clone(),
            usage: response.usage,
            generated_at: chrono::Utc::now(),
        })
    }

    fn extract_text_content(&self, response: &ClaudeResponse) -> CHOPSResult<String> {
        if response.content.is_empty() {
            return Err(CHOPSError::ApiError("Empty response content".to_string()));
        }

        let mut content = String::new();
        for block in &response.content {
            if block.content_type == "text" {
                content.push_str(&block.text);
                content.push('\n');
            }
        }

        if content.trim().is_empty() {
            return Err(CHOPSError::ApiError("No text content in response".to_string()));
        }

        Ok(content.trim().to_string())
    }

    fn assess_creativity_score(&self, content: &str, chaos_result: &ChaosInjectionResult) -> f64 {
        let mut score = 0.5; // Base score

        // Length and complexity
        let word_count = content.split_whitespace().count();
        if word_count > 100 {
            score += 0.1;
        }

        // Chaos influence
        score += chaos_result.chaos_applied * 0.3;

        // Keyword analysis for creative indicators
        let creative_keywords = [
            "innovative", "revolutionary", "breakthrough", "novel", "unprecedented",
            "paradigm", "transform", "reimagine", "disrupt", "evolve"
        ];

        let creative_count = creative_keywords.iter()
            .map(|&keyword| {
                content.to_lowercase().matches(keyword).count()
            })
            .sum::<usize>();

        score += (creative_count as f64 * 0.05).min(0.2);

        // Unexpected elements influence
        score += chaos_result.unexpected_elements.len() as f64 * 0.02;

        score.min(1.0)
    }

    fn assess_feasibility_score(&self, content: &str) -> f64 {
        let mut score = 0.7; // Start optimistic

        // Check for impossible/fantasy elements
        let impossible_keywords = [
            "magic", "impossible", "violate physics", "time travel", "telepathy",
            "infinite", "zero cost", "perpetual motion"
        ];

        let impossible_count = impossible_keywords.iter()
            .map(|&keyword| {
                content.to_lowercase().matches(keyword).count()
            })
            .sum::<usize>();

        score -= impossible_count as f64 * 0.1;

        // Check for technical feasibility indicators
        let feasible_keywords = [
            "implementation", "algorithm", "database", "api", "framework",
            "library", "tool", "method", "process", "system"
        ];

        let feasible_count = feasible_keywords.iter()
            .map(|&keyword| {
                content.to_lowercase().matches(keyword).count()
            })
            .sum::<usize>();

        score += feasible_count as f64 * 0.02;

        score.max(0.0).min(1.0)
    }

    fn assess_novelty_score(&self, content: &str) -> f64 {
        let mut score = 0.5;

        // Check for novel combinations
        let combination_indicators = ["combine", "merge", "blend", "fusion", "hybrid"];
        let combination_count = combination_indicators.iter()
            .map(|&indicator| {
                content.to_lowercase().matches(indicator).count()
            })
            .sum::<usize>();

        score += combination_count as f64 * 0.05;

        // Check for unique perspective indicators
        let perspective_indicators = ["what if", "imagine", "consider", "alternatively"];
        let perspective_count = perspective_indicators.iter()
            .map(|&indicator| {
                content.to_lowercase().matches(indicator).count()
            })
            .sum::<usize>();

        score += perspective_count as f64 * 0.03;

        score.min(1.0)
    }

    fn assess_excitement_factor(&self, content: &str, persona_type: &PersonaType) -> f64 {
        let mut score = 0.5;

        // Exclamation marks and emotional language
        let exclamation_count = content.matches('!').count();
        score += (exclamation_count as f64 * 0.02).min(0.1);

        // Persona-specific excitement indicators
        match persona_type {
            PersonaType::MadScientist => {
                let mad_scientist_excitement = ["breakthrough", "impossible", "revolutionary"];
                let count = mad_scientist_excitement.iter()
                    .map(|&word| content.to_lowercase().matches(word).count())
                    .sum::<usize>();
                score += count as f64 * 0.05;
            },
            PersonaType::ChaosEngineer => {
                let chaos_excitement = ["chaos", "destruction", "antifragile", "emergence"];
                let count = chaos_excitement.iter()
                    .map(|&word| content.to_lowercase().matches(word).count())
                    .sum::<usize>();
                score += count as f64 * 0.05;
            },
            _ => {
                // Default excitement assessment
            }
        }

        score.min(1.0)
    }

    fn assess_prophecy_confidence(&self, prophecy: &str) -> f64 {
        let mut confidence = 0.5;

        // Specific years and dates increase confidence
        let year_regex = regex::Regex::new(r"\b20\d{2}\b").unwrap();
        let year_count = year_regex.find_iter(prophecy).count();
        confidence += (year_count as f64 * 0.05).min(0.2);

        // Specific technologies and companies
        let specific_indicators = ["by 2030", "within 5 years", "expected to", "likely to"];
        let specificity_count = specific_indicators.iter()
            .map(|&indicator| prophecy.to_lowercase().matches(indicator).count())
            .sum::<usize>();

        confidence += (specificity_count as f64 * 0.03).min(0.15);

        // Hedge words decrease confidence
        let hedge_words = ["might", "possibly", "potentially", "maybe", "could be"];
        let hedge_count = hedge_words.iter()
            .map(|&word| prophecy.to_lowercase().matches(word).count())
            .sum::<usize>();

        confidence -= (hedge_count as f64 * 0.02).min(0.2);

        confidence.max(0.1).min(0.9)
    }

    fn format_debate_transcript(&self, rounds: &[DebateRound]) -> String {
        let mut transcript = String::new();

        for round in rounds {
            transcript.push_str(&format!("=== Round {} ===\n", round.round_number));
            
            for response in &round.responses {
                transcript.push_str(&format!(
                    "\nPosition: {}\nArgument: {}\n",
                    response.position, response.argument
                ));
            }
            
            transcript.push_str("\n");
        }

        transcript
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests_per_minute: 50, // Conservative default
            tokens_per_minute: 40000,
            current_requests: 0,
            current_tokens: 0,
            last_reset: std::time::Instant::now(),
        }
    }

    pub async fn check_limits(&mut self) -> CHOPSResult<()> {
        self.reset_if_needed();

        if self.current_requests >= self.requests_per_minute {
            let wait_time = 60 - self.last_reset.elapsed().as_secs();
            if wait_time > 0 {
                warn!("Rate limit reached, waiting {} seconds", wait_time);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
                self.reset_counters();
            }
        }

        Ok(())
    }

    pub fn record_usage(&mut self, requests: u32, tokens: u32) {
        self.current_requests += requests;
        self.current_tokens += tokens;
    }

    fn reset_if_needed(&mut self) {
        if self.last_reset.elapsed().as_secs() >= 60 {
            self.reset_counters();
        }
    }

    fn reset_counters(&mut self) {
        self.current_requests = 0;
        self.current_tokens = 0;
        self.last_reset = std::time::Instant::now();
    }
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            // temperature: 0.7,
            // top_p: 0.9,
            // top_k: Some(50),
            stop_sequences: Vec::new(),
            timeout_seconds: 120,
            retry_attempts: 3,
            retry_delay_ms: 1000,
        }
    }
}