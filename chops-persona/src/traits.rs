use chops_core::CHOPSResult;
use crate::engine::{PersonaPrompt, PersonaFeedback, VocabularyStyle, ResponseFormat, StructureType, EmojiLevel, FormattingStyle};

pub trait PersonalityTrait: Send + Sync {
    fn generate_base_prompt(&self) -> PersonaPrompt;
    fn get_thinking_patterns(&self) -> Vec<String>;
    fn get_vocabulary_enhancements(&self) -> Vec<String>;
    fn get_creativity_bias(&self) -> f64;
    fn get_ethics_filter(&self) -> f64;
    fn get_risk_tolerance(&self) -> f64;
    fn get_excitement_level(&self) -> f64;
    fn adapt_to_context(&self, context: &str) -> String;
    fn apply_feedback(&mut self, feedback: PersonaFeedback) -> CHOPSResult<()>;
    
    // Optional method for specialized behavior
    fn get_specialized_commands(&self) -> Vec<String> {
        Vec::new()
    }
    
    fn generate_idea_enhancement(&self, base_idea: &str) -> String {
        format!("Enhanced idea: {}", base_idea)
    }
    
    fn get_conversation_style(&self) -> ConversationStyle {
        ConversationStyle::default()
    }
}

#[derive(Debug, Clone)]
pub struct ConversationStyle {
    pub formality_level: f64,      // 0.0 = very casual, 1.0 = very formal
    pub humor_frequency: f64,      // 0.0 = no humor, 1.0 = constant humor
    pub tangent_tendency: f64,     // 0.0 = stays on topic, 1.0 = goes off on tangents
    pub interruption_style: InterruptionStyle,
    pub question_asking_frequency: f64,
}

#[derive(Debug, Clone)]
pub enum InterruptionStyle {
    Never,
    Polite,
    Enthusiastic,
    Chaotic,
}

impl Default for ConversationStyle {
    fn default() -> Self {
        Self {
            formality_level: 0.5,
            humor_frequency: 0.3,
            tangent_tendency: 0.2,
            interruption_style: InterruptionStyle::Polite,
            question_asking_frequency: 0.4,
        }
    }
}

// Base trait implementation helpers
pub fn create_vocabulary_style(excitement: f64, technical: f64, metaphor: f64, unconventional: f64) -> VocabularyStyle {
    VocabularyStyle {
        excitement_level: excitement.max(0.0).min(1.0),
        technical_depth: technical.max(0.0).min(1.0),
        metaphor_usage: metaphor.max(0.0).min(1.0),
        unconventional_language: unconventional.max(0.0).min(1.0),
    }
}

pub fn create_response_format(structure: StructureType, emoji: EmojiLevel, formatting: FormattingStyle) -> ResponseFormat {
    ResponseFormat {
        structure_preference: structure,
        emoji_usage: emoji,
        formatting_style: formatting,
    }
}