use chops_core::{PersonaType, PersonalityContext, CHOPSResult, CHOPSError};
use crate::{personalities::*, PersonalityTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

pub struct PersonaEngine {
    personas: HashMap<PersonaType, Box<dyn PersonalityTrait>>,
    context_history: Vec<PersonalityContext>,
    adaptation_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaPrompt {
    pub base_prompt: String,
    pub thinking_patterns: Vec<String>,
    pub personality_modifiers: Vec<String>,
    pub vocabulary_style: VocabularyStyle,
    pub response_format: ResponseFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyStyle {
    pub excitement_level: f64,
    pub technical_depth: f64,
    pub metaphor_usage: f64,
    pub unconventional_language: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub structure_preference: StructureType,
    pub emoji_usage: EmojiLevel,
    pub formatting_style: FormattingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    Linear,
    Hierarchical,
    Creative,
    Chaotic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmojiLevel {
    None,
    Minimal,
    Moderate,
    Enthusiastic,
    Extreme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormattingStyle {
    Clean,
    Artistic,
    Technical,
    Experimental,
}

impl PersonaEngine {
    #[tracing::instrument(name = "persona_engine_new", level = "info")]
    pub fn new() -> Self {
        tracing::info!("Initializing PersonaEngine with all personality types");
        
        let mut personas: HashMap<PersonaType, Box<dyn PersonalityTrait>> = HashMap::new();
        
        tracing::debug!("Registering MadScientist personality");
        personas.insert(PersonaType::MadScientist, Box::new(MadScientistPersonality::new()));
        
        tracing::debug!("Registering ZenMaster personality");
        personas.insert(PersonaType::ZenMaster, Box::new(ZenMasterPersonality::new()));
        
        tracing::debug!("Registering PunkHacker personality");
        personas.insert(PersonaType::PunkHacker, Box::new(PunkHackerPersonality::new()));
        
        tracing::debug!("Registering EmpatheticAI personality");
        personas.insert(PersonaType::EmpatheticAI, Box::new(EmpatheticAIPersonality::new()));
        
        tracing::debug!("Registering ChaosEngineer personality");
        personas.insert(PersonaType::ChaosEngineer, Box::new(ChaosEngineerPersonality::new()));
        
        tracing::debug!("Registering TimeTraveler personality");
        personas.insert(PersonaType::TimeTraveler, Box::new(TimeTravelerPersonality::new()));
        
        tracing::debug!("Registering MindReader personality");
        personas.insert(PersonaType::MindReader, Box::new(MindReaderPersonality::new()));
        
        tracing::info!("PersonaEngine initialized with {} personalities", personas.len());
        
        Self {
            personas,
            context_history: Vec::new(),
            adaptation_enabled: true,
        }
    }
    
    #[tracing::instrument(name = "generate_persona_prompt", level = "info", skip(self))]
    pub fn generate_persona_prompt(&self, persona_type: &PersonaType, context: Option<&str>) -> CHOPSResult<PersonaPrompt> {
        tracing::info!("Generating persona prompt for: {:?}", persona_type);
        
        let personality = self.personas.get(persona_type)
            .ok_or_else(|| {
                tracing::error!("Unknown persona type requested: {:?}", persona_type);
                CHOPSError::PersonaError(format!("Unknown persona type: {:?}", persona_type))
            })?;
        
        tracing::debug!("Generating base prompt for persona");
        let mut prompt = personality.generate_base_prompt();
        
        // Add contextual adaptations if context is provided
        if let Some(ctx) = context {
            tracing::debug!("Adding contextual adaptations for context length: {}", ctx.len());
            let contextual_additions = personality.adapt_to_context(ctx);
            prompt.base_prompt.push_str(&format!("\n\nContext Adaptation: {}", contextual_additions));
        }
        
        // Add thinking patterns
        tracing::debug!("Adding thinking patterns");
        prompt.thinking_patterns = personality.get_thinking_patterns();
        
        // Add personality modifiers based on recent usage
        if self.adaptation_enabled {
            tracing::debug!("Adding adaptive modifiers");
            prompt.personality_modifiers.extend(
                self.get_adaptive_modifiers(persona_type)
            );
        }
        
        tracing::info!("Persona prompt generated - base length: {}, {} thinking patterns, {} modifiers", 
            prompt.base_prompt.len(), prompt.thinking_patterns.len(), prompt.personality_modifiers.len());
        
        Ok(prompt)
    }
    
    #[tracing::instrument(name = "activate_persona", level = "info", skip(self))]
    pub fn activate_persona(&mut self, persona_type: PersonaType, domain: &str) -> CHOPSResult<PersonalityContext> {
        tracing::info!("Activating persona: {:?} for domain: '{}'", persona_type, domain);
        
        let personality = self.personas.get(&persona_type)
            .ok_or_else(|| {
                tracing::error!("Unknown persona type for activation: {:?}", persona_type);
                CHOPSError::PersonaError(format!("Unknown persona type: {:?}", persona_type))
            })?;
        
        tracing::debug!("Creating base personality context");
        let mut context = PersonalityContext {
            persona_type: persona_type.clone(),
            thinking_patterns: personality.get_thinking_patterns(),
            vocabulary_enhancements: personality.get_vocabulary_enhancements(),
            creativity_bias: personality.get_creativity_bias(),
            ethics_filter: personality.get_ethics_filter(),
            risk_tolerance: personality.get_risk_tolerance(),
            excitement_level: personality.get_excitement_level(),
        };
        
        tracing::debug!("Context created with {} thinking patterns, creativity: {:.2}, risk tolerance: {:.2}", 
            context.thinking_patterns.len(), context.creativity_bias, context.risk_tolerance);
        
        // Apply domain-specific modifications
        tracing::debug!("Applying domain modifications for: '{}'", domain);
        context = self.apply_domain_modifications(context, domain)?;
        
        // Apply adaptive learning if enabled
        if self.adaptation_enabled {
            tracing::debug!("Applying adaptive learning");
            context = self.apply_adaptive_learning(context)?;
        }
        
        // Store context for future adaptation
        self.context_history.push(context.clone());
        tracing::debug!("Added context to history, total contexts: {}", self.context_history.len());
        
        // Limit history size
        if self.context_history.len() > 100 {
            self.context_history.remove(0);
            tracing::debug!("Trimmed context history to stay within limit");
        }
        
        tracing::info!("Persona activation complete for {:?}", persona_type);
        Ok(context)
    }
    
    #[tracing::instrument(name = "blend_personas", level = "info", skip(self))]
    pub fn blend_personas(&self, primary: PersonaType, secondary: PersonaType, blend_ratio: f64) -> CHOPSResult<PersonalityContext> {
        tracing::info!("Blending personas: {:?} ({:.2}) + {:?} ({:.2})", 
            primary, blend_ratio, secondary, 1.0 - blend_ratio);
        
        if !(0.0..=1.0).contains(&blend_ratio) {
            tracing::error!("Invalid blend ratio: {:.2}", blend_ratio);
            return Err(CHOPSError::InvalidParameter("Blend ratio must be between 0.0 and 1.0".to_string()));
        }
        
        let primary_personality = self.personas.get(&primary)
            .ok_or_else(|| {
                tracing::error!("Unknown primary persona type: {:?}", primary);
                CHOPSError::PersonaError(format!("Unknown persona type: {:?}", primary))
            })?;
        
        let secondary_personality = self.personas.get(&secondary)
            .ok_or_else(|| {
                tracing::error!("Unknown secondary persona type: {:?}", secondary);
                CHOPSError::PersonaError(format!("Unknown persona type: {:?}", secondary))
            })?;
        
        tracing::debug!("Creating blended personality context");
        
        // Create blended context
        let mut blended_context = PersonalityContext {
            persona_type: primary.clone(),
            thinking_patterns: Vec::new(),
            vocabulary_enhancements: Vec::new(),
            creativity_bias: primary_personality.get_creativity_bias() * blend_ratio + 
                           secondary_personality.get_creativity_bias() * (1.0 - blend_ratio),
            ethics_filter: primary_personality.get_ethics_filter() * blend_ratio + 
                          secondary_personality.get_ethics_filter() * (1.0 - blend_ratio),
            risk_tolerance: primary_personality.get_risk_tolerance() * blend_ratio + 
                           secondary_personality.get_risk_tolerance() * (1.0 - blend_ratio),
            excitement_level: primary_personality.get_excitement_level() * blend_ratio + 
                             secondary_personality.get_excitement_level() * (1.0 - blend_ratio),
        };
        
        tracing::debug!("Blended attributes - creativity: {:.2}, ethics: {:.2}, risk: {:.2}, excitement: {:.2}", 
            blended_context.creativity_bias, blended_context.ethics_filter, 
            blended_context.risk_tolerance, blended_context.excitement_level);
        
        // Blend thinking patterns
        let primary_patterns = primary_personality.get_thinking_patterns();
        let secondary_patterns = secondary_personality.get_thinking_patterns();
        
        let primary_count = (primary_patterns.len() as f64 * blend_ratio).ceil() as usize;
        let secondary_count = (secondary_patterns.len() as f64 * (1.0 - blend_ratio)).ceil() as usize;
        
        blended_context.thinking_patterns.extend(primary_patterns.into_iter().take(primary_count));
        blended_context.thinking_patterns.extend(secondary_patterns.into_iter().take(secondary_count));
        
        // Blend vocabulary enhancements
        let primary_vocab = primary_personality.get_vocabulary_enhancements();
        let secondary_vocab = secondary_personality.get_vocabulary_enhancements();
        
        blended_context.vocabulary_enhancements.extend(primary_vocab);
        blended_context.vocabulary_enhancements.extend(secondary_vocab);
        
        // Remove duplicates
        blended_context.vocabulary_enhancements.sort();
        blended_context.vocabulary_enhancements.dedup();
        
        Ok(blended_context)
    }
    
    pub fn evolve_persona(&mut self, persona_type: &PersonaType, feedback: PersonaFeedback) -> CHOPSResult<()> {
        if !self.adaptation_enabled {
            return Ok(());
        }
        
        // Apply evolutionary pressure based on feedback
        if let Some(personality) = self.personas.get_mut(persona_type) {
            personality.apply_feedback(feedback)?;
        }
        
        Ok(())
    }
    
    fn apply_domain_modifications(&self, mut context: PersonalityContext, domain: &str) -> CHOPSResult<PersonalityContext> {
        match domain.to_lowercase().as_str() {
            "debugging" => {
                context.thinking_patterns.push("What assumptions might be wrong here?".to_string());
                context.thinking_patterns.push("What edge cases haven't been considered?".to_string());
                context.risk_tolerance *= 0.8; // More careful with debugging
            },
            "architecture" => {
                context.thinking_patterns.push("How will this scale in the future?".to_string());
                context.thinking_patterns.push("What are the long-term implications?".to_string());
                context.creativity_bias *= 1.2; // More creative for architecture
            },
            "performance" => {
                context.thinking_patterns.push("Where are the bottlenecks?".to_string());
                context.thinking_patterns.push("What can be optimized?".to_string());
                context.ethics_filter *= 0.9; // Willing to bend rules for performance
            },
            "security" => {
                context.thinking_patterns.push("What could go wrong?".to_string());
                context.thinking_patterns.push("How might this be exploited?".to_string());
                context.risk_tolerance *= 0.6; // Very cautious with security
            },
            "ui" | "ux" => {
                context.thinking_patterns.push("How will users feel about this?".to_string());
                context.thinking_patterns.push("What's the emotional impact?".to_string());
                context.excitement_level *= 1.3; // More enthusiasm for user-facing work
            },
            _ => {
                // Generic domain - no specific modifications
            }
        }
        
        Ok(context)
    }
    
    fn apply_adaptive_learning(&self, mut context: PersonalityContext) -> CHOPSResult<PersonalityContext> {
        // Analyze recent context history to adapt behavior
        if self.context_history.len() < 5 {
            return Ok(context); // Not enough history for adaptation
        }
        
        let recent_contexts: Vec<&PersonalityContext> = self.context_history
            .iter()
            .rev()
            .take(10)
            .collect();
        
        // Calculate average values from recent usage
        let avg_creativity = recent_contexts.iter()
            .map(|c| c.creativity_bias)
            .sum::<f64>() / recent_contexts.len() as f64;
        
        let avg_risk_tolerance = recent_contexts.iter()
            .map(|c| c.risk_tolerance)
            .sum::<f64>() / recent_contexts.len() as f64;
        
        let avg_excitement = recent_contexts.iter()
            .map(|c| c.excitement_level)
            .sum::<f64>() / recent_contexts.len() as f64;
        
        // Apply momentum-based adaptation (gradual drift toward recent patterns)
        let adaptation_factor = 0.1; // How much to adapt
        
        context.creativity_bias = context.creativity_bias * (1.0 - adaptation_factor) + 
                                 avg_creativity * adaptation_factor;
        context.risk_tolerance = context.risk_tolerance * (1.0 - adaptation_factor) + 
                                avg_risk_tolerance * adaptation_factor;
        context.excitement_level = context.excitement_level * (1.0 - adaptation_factor) + 
                                  avg_excitement * adaptation_factor;
        
        Ok(context)
    }
    
    fn get_adaptive_modifiers(&self, persona_type: &PersonaType) -> Vec<String> {
        let mut modifiers = Vec::new();
        
        // Count recent usage of this persona
        let recent_usage = self.context_history
            .iter()
            .rev()
            .take(20)
            .filter(|c| &c.persona_type == persona_type)
            .count();
        
        if recent_usage > 5 {
            modifiers.push("You've been active recently, so channel your accumulated wisdom and experience".to_string());
        } else if recent_usage == 0 {
            modifiers.push("You're fresh and energized, ready to bring your unique perspective".to_string());
        }
        
        // Add random variation to prevent stagnation
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.3 {
            modifiers.push("Add a subtle twist or unexpected angle to your usual approach".to_string());
        }
        
        modifiers
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFeedback {
    pub effectiveness_rating: f64, // 0.0 to 1.0
    pub creativity_rating: f64,
    pub user_satisfaction: f64,
    pub specific_feedback: Option<String>,
}

impl Default for PersonaEngine {
    fn default() -> Self {
        Self::new()
    }
}