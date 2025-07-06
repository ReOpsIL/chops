use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PersonaType {
    MadScientist,
    ZenMaster,
    PunkHacker,
    EmpatheticAI,
    ChaosEngineer,
    TimeTraveler,
    MindReader,
}

impl std::fmt::Display for PersonaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonaType::MadScientist => write!(f, "mad-scientist"),
            PersonaType::ZenMaster => write!(f, "zen-master"),
            PersonaType::PunkHacker => write!(f, "punk-hacker"),
            PersonaType::EmpatheticAI => write!(f, "empathetic-ai"),
            PersonaType::ChaosEngineer => write!(f, "chaos-engineer"),
            PersonaType::TimeTraveler => write!(f, "time-traveler"),
            PersonaType::MindReader => write!(f, "mind-reader"),
        }
    }
}

impl std::str::FromStr for PersonaType {
    type Err = crate::error::PersonaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mad-scientist" | "madscientist" => Ok(PersonaType::MadScientist),
            "zen-master" | "zenmaster" => Ok(PersonaType::ZenMaster),
            "punk-hacker" | "punkhacker" => Ok(PersonaType::PunkHacker),
            "empathetic-ai" | "empatheticai" => Ok(PersonaType::EmpatheticAI),
            "chaos-engineer" | "chaosengineer" => Ok(PersonaType::ChaosEngineer),
            "time-traveler" | "timetraveler" => Ok(PersonaType::TimeTraveler),
            "mind-reader" | "mindreader" => Ok(PersonaType::MindReader),
            _ => Err(crate::error::PersonaError::UnknownPersonaType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropySource {
    PseudoRandom,
    TrueRandom,
    QuantumRandom,
    ChaosEquation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandomDistribution {
    Uniform,
    Normal,
    Exponential,
    Chaotic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativityLevel {
    Low,
    Medium,
    High,
    Extreme,
    Transcendent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    Json,
    Yaml,
    PlainText,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeirднessLevel {
    Normal,
    Slightly,
    Medium,
    High,
    Extreme,
    Impossible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosParams {
    pub chaos_level: u8,
    pub entropy_source: EntropySource,
    pub persona_type: PersonaType,
    pub distribution: RandomDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityContext {
    pub persona_type: PersonaType,
    pub thinking_patterns: Vec<String>,
    pub vocabulary_enhancements: Vec<String>,
    pub creativity_bias: f64,
    pub ethics_filter: f64,
    pub risk_tolerance: f64,
    pub excitement_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedIdea {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub persona_used: PersonaType,
    pub chaos_level: f64,
    pub creativity_score: f64,
    pub feasibility_score: f64,
    pub novelty_score: f64,
    pub excitement_factor: f64,
    pub tags: Vec<String>,
    pub implementation_hints: Vec<String>,
    pub potential_risks: Vec<String>,
    pub experimental_variations: Vec<ExperimentalVariation>,
    pub analogies: Vec<Analogy>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalVariation {
    pub variation_type: VariationType,
    pub description: String,
    pub risk_level: RiskLevel,
    pub potential_breakthrough: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariationType {
    EthicsRelaxation,
    PowerAmplification,
    ChaosInjection,
    RealityBending,
    ParadoxCreation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analogy {
    pub source_domain: String,
    pub source_pattern: String,
    pub target_concept: String,
    pub structural_mapping: StructuralMapping,
    pub insight: String,
    pub confidence: f64,
    pub novelty_score: f64,
    pub practical_applicability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralMapping {
    pub source_domain: String,
    pub target_concept: String,
    pub source_elements: Vec<String>,
    pub target_elements: Vec<String>,
    pub relationship_mappings: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainPattern {
    pub name: String,
    pub description: String,
    pub structural_elements: Vec<String>,
    pub behavioral_dynamics: Vec<String>,
    pub success_metrics: Vec<String>,
}

impl DomainPattern {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            structural_elements: Vec::new(),
            behavioral_dynamics: Vec::new(),
            success_metrics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonParams {
    pub persona: PersonaType,
    pub domain: String,
    pub chaos_level: u8,
    pub timeline: Option<String>,
    pub constraints: Vec<String>,
    pub vibe: Option<String>,
    pub reality_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutateParams {
    pub input_file: String,
    pub direction: String,
    pub inject_personality: bool,
    pub add_easter_eggs: bool,
    pub make_weird: bool,
    pub keep_functional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProphecyParams {
    pub year: Option<u32>,
    pub domain: String,
    pub trend_analysis: bool,
    pub emerging_tech: bool,
    pub what_if: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborateParams {
    pub mode: CollaborationMode,
    pub topic: String,
    pub include_human: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationMode {
    ClaudeVsGpt,
    AiDebate,
    TrioMode,
    Consensus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CHOPSSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub persona_preferences: HashMap<String, PersonaType>,
    pub chaos_history: Vec<f64>,
    pub generated_ideas: Vec<Uuid>,
    pub learning_patterns: HashMap<String, f64>,
}

impl Default for PersonaType {
    fn default() -> Self {
        PersonaType::MadScientist
    }
}