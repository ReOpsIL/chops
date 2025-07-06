use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{CHOPSError, CHOPSResult, GeneratedIdea, PersonaType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySystem {
    pub short_term: ShortTermMemory,
    pub working: WorkingMemory,
    pub long_term: LongTermMemory,
    pub episodic: EpisodicMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortTermMemory {
    pub recent_ideas: VecDeque<GeneratedIdea>,
    pub max_capacity: usize,
    pub retention_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    pub active_context: HashMap<String, String>,
    pub current_persona_state: Option<PersonaType>,
    pub chaos_momentum: f64,
    pub creativity_temperature: f64,
    pub cognitive_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMemory {
    pub successful_patterns: HashMap<String, PatternRecord>,
    pub persona_effectiveness: HashMap<PersonaType, EffectivenessMetrics>,
    pub domain_knowledge: HashMap<String, DomainKnowledge>,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub session_history: VecDeque<SessionEpisode>,
    pub breakthrough_moments: Vec<BreakthroughMoment>,
    pub failure_learnings: Vec<FailureLearning>,
    pub max_episodes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecord {
    pub pattern: String,
    pub success_rate: f64,
    pub usage_count: u32,
    pub last_used: DateTime<Utc>,
    pub context_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivenessMetrics {
    pub average_creativity_score: f64,
    pub average_feasibility_score: f64,
    pub user_satisfaction_rating: f64,
    pub usage_frequency: u32,
    pub domains_used_in: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainKnowledge {
    pub domain_name: String,
    pub expertise_level: f64,
    pub successful_approaches: Vec<String>,
    pub common_pitfalls: Vec<String>,
    pub key_concepts: HashMap<String, String>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_personas: Vec<PersonaType>,
    pub chaos_tolerance: f64,
    pub weirdness_acceptance: f64,
    pub favorite_domains: Vec<String>,
    pub output_style_preferences: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEpisode {
    pub session_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub commands_used: Vec<String>,
    pub personas_invoked: Vec<PersonaType>,
    pub ideas_generated: u32,
    pub overall_satisfaction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakthroughMoment {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub idea_id: Uuid,
    pub breakthrough_type: BreakthroughType,
    pub description: String,
    pub impact_score: f64,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakthroughType {
    CreativeLeap,
    TechnicalInsight,
    ParadigmShift,
    ProblemSolution,
    UnexpectedConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureLearning {
    pub timestamp: DateTime<Utc>,
    pub failure_context: String,
    pub attempted_approach: String,
    pub failure_reason: String,
    pub lessons_learned: Vec<String>,
    pub suggested_alternatives: Vec<String>,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            short_term: ShortTermMemory::new(),
            working: WorkingMemory::new(),
            long_term: LongTermMemory::new(),
            episodic: EpisodicMemory::new(),
        }
    }
    
    pub fn load_from_file(path: &std::path::Path) -> CHOPSResult<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(CHOPSError::FileSystemError)?;
        
        let memory: MemorySystem = serde_json::from_str(&content)
            .map_err(|e| CHOPSError::ConfigError(format!("Failed to load memory: {}", e)))?;
        
        Ok(memory)
    }
    
    pub fn save_to_file(&self, path: &std::path::Path) -> CHOPSResult<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(CHOPSError::FileSystemError)?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| CHOPSError::ConfigError(format!("Failed to serialize memory: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(CHOPSError::FileSystemError)?;
        
        Ok(())
    }
    
    pub fn add_idea(&mut self, idea: GeneratedIdea) {
        // Add to short-term memory
        self.short_term.add_idea(idea.clone());
        
        // Update working memory with current context
        self.working.update_from_idea(&idea);
        
        // Extract patterns for long-term memory
        self.long_term.extract_patterns_from_idea(&idea);
        
        // Update persona effectiveness metrics
        self.long_term.update_persona_effectiveness(&idea);
    }
    
    pub fn recall_similar_ideas(&self, query: &str, limit: usize) -> Vec<&GeneratedIdea> {
        self.short_term.recent_ideas
            .iter()
            .filter(|idea| {
                idea.description.to_lowercase().contains(&query.to_lowercase()) ||
                idea.title.to_lowercase().contains(&query.to_lowercase()) ||
                idea.tags.iter().any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .take(limit)
            .collect()
    }
    
    pub fn get_persona_recommendation(&self, domain: &str) -> Option<PersonaType> {
        self.long_term.persona_effectiveness
            .iter()
            .filter(|(_, metrics)| metrics.domains_used_in.contains(&domain.to_string()))
            .max_by(|(_, a), (_, b)| {
                let score_a = a.average_creativity_score * a.user_satisfaction_rating;
                let score_b = b.average_creativity_score * b.user_satisfaction_rating;
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(persona, _)| persona.clone())
    }
    
    pub fn optimize_chaos_level(&self, persona: &PersonaType) -> u8 {
        if let Some(metrics) = self.long_term.persona_effectiveness.get(persona) {
            // Use historical effectiveness to suggest optimal chaos level
            let base_level = 5;
            let effectiveness_modifier = (metrics.average_creativity_score - 0.5) * 4.0;
            ((base_level as f64 + effectiveness_modifier).max(1.0).min(11.0)) as u8
        } else {
            5 // Default chaos level
        }
    }
}

impl ShortTermMemory {
    pub fn new() -> Self {
        Self {
            recent_ideas: VecDeque::new(),
            max_capacity: 50,
            retention_minutes: 60,
        }
    }
    
    pub fn add_idea(&mut self, idea: GeneratedIdea) {
        // Remove expired ideas
        let cutoff_time = Utc::now() - chrono::Duration::minutes(self.retention_minutes as i64);
        self.recent_ideas.retain(|idea| idea.timestamp > cutoff_time);
        
        // Add new idea
        self.recent_ideas.push_back(idea);
        
        // Maintain capacity
        if self.recent_ideas.len() > self.max_capacity {
            self.recent_ideas.pop_front();
        }
    }
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self {
            active_context: HashMap::new(),
            current_persona_state: None,
            chaos_momentum: 0.5,
            creativity_temperature: 0.7,
            cognitive_load: 0.0,
        }
    }
    
    pub fn update_from_idea(&mut self, idea: &GeneratedIdea) {
        self.current_persona_state = Some(idea.persona_used.clone());
        self.chaos_momentum = (self.chaos_momentum * 0.8) + (idea.chaos_level * 0.2);
        self.creativity_temperature = (self.creativity_temperature * 0.8) + (idea.creativity_score * 0.2);
        
        // Update context with recent idea themes
        for tag in &idea.tags {
            self.active_context.insert(
                format!("recent_tag_{}", tag),
                idea.title.clone()
            );
        }
        
        // Limit context size
        if self.active_context.len() > 20 {
            let keys_to_remove: Vec<String> = self.active_context
                .keys()
                .take(self.active_context.len() - 20)
                .cloned()
                .collect();
            
            for key in keys_to_remove {
                self.active_context.remove(&key);
            }
        }
    }
}

impl LongTermMemory {
    pub fn new() -> Self {
        Self {
            successful_patterns: HashMap::new(),
            persona_effectiveness: HashMap::new(),
            domain_knowledge: HashMap::new(),
            user_preferences: UserPreferences::default(),
        }
    }
    
    pub fn extract_patterns_from_idea(&mut self, idea: &GeneratedIdea) {
        // Extract patterns from successful ideas (high scores)
        if idea.creativity_score > 0.7 && idea.feasibility_score > 0.6 {
            for tag in &idea.tags {
                let pattern_key = format!("tag_pattern_{}", tag);
                let pattern = self.successful_patterns
                    .entry(pattern_key)
                    .or_insert_with(|| PatternRecord {
                        pattern: tag.clone(),
                        success_rate: 0.0,
                        usage_count: 0,
                        last_used: Utc::now(),
                        context_tags: Vec::new(),
                    });
                
                pattern.usage_count += 1;
                pattern.last_used = Utc::now();
                pattern.success_rate = (pattern.success_rate * (pattern.usage_count - 1) as f64 + 
                                     (idea.creativity_score + idea.feasibility_score) / 2.0) / 
                                     pattern.usage_count as f64;
                
                // Add context tags
                for other_tag in &idea.tags {
                    if other_tag != tag && !pattern.context_tags.contains(other_tag) {
                        pattern.context_tags.push(other_tag.clone());
                    }
                }
            }
        }
    }
    
    pub fn update_persona_effectiveness(&mut self, idea: &GeneratedIdea) {
        let metrics = self.persona_effectiveness
            .entry(idea.persona_used.clone())
            .or_insert_with(|| EffectivenessMetrics {
                average_creativity_score: 0.0,
                average_feasibility_score: 0.0,
                user_satisfaction_rating: 0.0,
                usage_frequency: 0,
                domains_used_in: Vec::new(),
            });
        
        metrics.usage_frequency += 1;
        metrics.average_creativity_score = (metrics.average_creativity_score * 
                                          (metrics.usage_frequency - 1) as f64 + 
                                          idea.creativity_score) / 
                                          metrics.usage_frequency as f64;
        metrics.average_feasibility_score = (metrics.average_feasibility_score * 
                                           (metrics.usage_frequency - 1) as f64 + 
                                           idea.feasibility_score) / 
                                           metrics.usage_frequency as f64;
        
        // Infer domain from tags
        for tag in &idea.tags {
            if !metrics.domains_used_in.contains(tag) {
                metrics.domains_used_in.push(tag.clone());
            }
        }
    }
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            session_history: VecDeque::new(),
            breakthrough_moments: Vec::new(),
            failure_learnings: Vec::new(),
            max_episodes: 100,
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_personas: vec![PersonaType::MadScientist, PersonaType::ChaosEngineer],
            chaos_tolerance: 0.7,
            weirdness_acceptance: 0.6,
            favorite_domains: vec!["software".to_string(), "ai".to_string(), "creativity".to_string()],
            output_style_preferences: HashMap::new(),
        }
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}