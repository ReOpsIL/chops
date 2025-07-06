use chops_core::PersonaType;
use chops_chaos::ChaosVariation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike};
use crate::client::{ClaudeResponse, Usage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedIdeaResponse {
    pub id: Uuid,
    pub content: String,
    pub persona_used: PersonaType,
    pub chaos_level: f64,
    pub creativity_score: f64,
    pub feasibility_score: f64,
    pub novelty_score: f64,
    pub excitement_factor: f64,
    pub chaos_variations: Vec<ChaosVariation>,
    pub unexpected_elements: Vec<String>,
    pub coherence_score: f64,
    pub raw_response: ClaudeResponse,
    pub usage: Option<Usage>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateResult {
    pub topic: String,
    pub rounds: Vec<DebateRound>,
    pub synthesis: String,
    pub total_rounds: u8,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateRound {
    pub round_number: u8,
    pub responses: Vec<DebateResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateResponse {
    pub position: String,
    pub round: u8,
    pub argument: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProphecyResponse {
    pub domain: String,
    pub target_year: u32,
    pub prophecy: String,
    pub context: String,
    pub confidence_level: f64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationResult {
    pub original_content: String,
    pub mutated_content: String,
    pub mutations_applied: Vec<MutationDescription>,
    pub personality_injections: Vec<String>,
    pub easter_eggs: Vec<String>,
    pub weirdness_level: f64,
    pub functionality_preserved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationDescription {
    pub mutation_type: MutationType,
    pub description: String,
    pub location: String,
    pub impact_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationType {
    PersonalityInjection,
    EasterEggAddition,
    WeirdnessEnhancement,
    StructuralModification,
    ConceptualShift,
    ChaosElement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: Uuid,
    pub topic: String,
    pub mode: CollaborationMode,
    pub participants: Vec<CollaborationParticipant>,
    pub rounds: Vec<CollaborationRound>,
    pub synthesis: Option<String>,
    pub insights: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationMode {
    Debate,
    Brainstorm,
    Consensus,
    DevilsAdvocate,
    Synthesis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationParticipant {
    pub name: String,
    pub role: String,
    pub perspective: String,
    pub persona_type: Option<PersonaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationRound {
    pub round_number: u8,
    pub contributions: Vec<CollaborationContribution>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationContribution {
    pub participant: String,
    pub content: String,
    pub contribution_type: ContributionType,
    pub timestamp: DateTime<Utc>,
    pub creativity_score: f64,
    pub insight_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    Argument,
    Question,
    CounterPoint,
    BuildingOn,
    Synthesis,
    Insight,
    Challenge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogicalInsight {
    pub source_domain: String,
    pub target_domain: String,
    pub analogy_description: String,
    pub structural_mappings: Vec<StructuralMapping>,
    pub novel_insights: Vec<String>,
    pub practical_applications: Vec<String>,
    pub confidence_score: f64,
    pub surprise_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralMapping {
    pub source_element: String,
    pub target_element: String,
    pub relationship: String,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnalysis {
    pub current_state: String,
    pub historical_patterns: Vec<HistoricalPattern>,
    pub future_projections: Vec<FutureProjection>,
    pub trend_analysis: TrendAnalysis,
    pub timeline_scenarios: Vec<TimelineScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalPattern {
    pub pattern_name: String,
    pub description: String,
    pub time_period: String,
    pub relevance_to_current: f64,
    pub cyclical_nature: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProjection {
    pub scenario_name: String,
    pub description: String,
    pub probability: f64,
    pub time_horizon: String,
    pub key_indicators: Vec<String>,
    pub potential_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub emerging_trends: Vec<String>,
    pub declining_trends: Vec<String>,
    pub stable_patterns: Vec<String>,
    pub disruptive_potentials: Vec<String>,
    pub convergence_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineScenario {
    pub scenario_id: String,
    pub name: String,
    pub description: String,
    pub key_events: Vec<TimelineEvent>,
    pub probability: f64,
    pub desirability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub year: u32,
    pub event_description: String,
    pub impact_level: f64,
    pub uncertainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalProfile {
    pub unspoken_desires: Vec<String>,
    pub hidden_fears: Vec<String>,
    pub unconscious_patterns: Vec<String>,
    pub motivation_drivers: Vec<String>,
    pub decision_biases: Vec<String>,
    pub emotional_triggers: Vec<String>,
    pub subconscious_needs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityDistortionField {
    pub distortion_level: f64,
    pub impossible_elements: Vec<String>,
    pub paradox_injections: Vec<String>,
    pub reality_bends: Vec<RealityBend>,
    pub coherence_maintenance: f64,
    pub feasibility_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityBend {
    pub bend_type: RealityBendType,
    pub description: String,
    pub intensity: f64,
    pub scope: String,
    pub potential_breakthrough: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealityBendType {
    PhysicsViolation,
    LogicParadox,
    TimeDistortion,
    CausalityLoop,
    DimensionalShift,
    ConsciousnessHack,
    InformationTranscendence,
}

impl GeneratedIdeaResponse {
    pub fn calculate_overall_score(&self) -> f64 {
        let weights = IdeaScoreWeights::default();
        
        self.creativity_score * weights.creativity +
        self.feasibility_score * weights.feasibility +
        self.novelty_score * weights.novelty +
        self.excitement_factor * weights.excitement +
        self.coherence_score * weights.coherence
    }
    
    pub fn get_quality_tier(&self) -> QualityTier {
        let overall_score = self.calculate_overall_score();
        
        match overall_score {
            s if s >= 0.9 => QualityTier::Transcendent,
            s if s >= 0.8 => QualityTier::Brilliant,
            s if s >= 0.7 => QualityTier::Excellent,
            s if s >= 0.6 => QualityTier::Good,
            s if s >= 0.5 => QualityTier::Decent,
            _ => QualityTier::NeedsWork,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaScoreWeights {
    pub creativity: f64,
    pub feasibility: f64,
    pub novelty: f64,
    pub excitement: f64,
    pub coherence: f64,
}

impl Default for IdeaScoreWeights {
    fn default() -> Self {
        Self {
            creativity: 0.3,
            feasibility: 0.25,
            novelty: 0.2,
            excitement: 0.15,
            coherence: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityTier {
    Transcendent,
    Brilliant,
    Excellent,
    Good,
    Decent,
    #[serde(rename = "needs_work")]
    NeedsWork,
}

impl std::fmt::Display for QualityTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityTier::Transcendent => write!(f, "🌟 Transcendent"),
            QualityTier::Brilliant => write!(f, "💎 Brilliant"),
            QualityTier::Excellent => write!(f, "🔥 Excellent"),
            QualityTier::Good => write!(f, "✨ Good"),
            QualityTier::Decent => write!(f, "👍 Decent"),
            QualityTier::NeedsWork => write!(f, "🔧 Needs Work"),
        }
    }
}

impl DebateResult {
    pub fn get_winner(&self) -> Option<String> {
        // Simple implementation - could be more sophisticated
        if let Some(last_round) = self.rounds.last() {
            if let Some(response) = last_round.responses.first() {
                return Some(response.position.clone());
            }
        }
        None
    }
    
    pub fn extract_key_insights(&self) -> Vec<String> {
        // Extract insights from synthesis
        self.synthesis
            .lines()
            .filter(|line| line.trim().starts_with("- ") || line.trim().starts_with("* "))
            .map(|line| line.trim().trim_start_matches("- ").trim_start_matches("* ").to_string())
            .collect()
    }
}

impl ProphecyResponse {
    pub fn get_confidence_tier(&self) -> ConfidenceTier {
        match self.confidence_level {
            c if c >= 0.8 => ConfidenceTier::HighlyConfident,
            c if c >= 0.6 => ConfidenceTier::Confident,
            c if c >= 0.4 => ConfidenceTier::Moderate,
            c if c >= 0.2 => ConfidenceTier::Uncertain,
            _ => ConfidenceTier::Speculative,
        }
    }
    
    pub fn years_ahead(&self) -> u32 {
        let current_year = chrono::Utc::now().year() as u32;
        if self.target_year > current_year {
            self.target_year - current_year
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceTier {
    HighlyConfident,
    Confident,
    Moderate,
    Uncertain,
    Speculative,
}

impl std::fmt::Display for ConfidenceTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfidenceTier::HighlyConfident => write!(f, "🎯 Highly Confident"),
            ConfidenceTier::Confident => write!(f, "✅ Confident"),
            ConfidenceTier::Moderate => write!(f, "🤔 Moderate"),
            ConfidenceTier::Uncertain => write!(f, "❓ Uncertain"),
            ConfidenceTier::Speculative => write!(f, "🔮 Speculative"),
        }
    }
}