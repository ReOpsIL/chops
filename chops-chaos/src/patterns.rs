use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosPattern {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub chaos_signature: ChaosSignature,
    pub effectiveness_score: f64,
    pub usage_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub success_rate: f64,
    pub context_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosSignature {
    pub entropy_pattern: Vec<f64>,
    pub frequency_distribution: HashMap<String, f64>,
    pub mathematical_fingerprint: MathematicalFingerprint,
    pub emergence_indicators: EmergenceIndicators,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathematicalFingerprint {
    pub fractal_dimension: f64,
    pub lyapunov_exponent: f64,
    pub correlation_sum: f64,
    pub entropy_rate: f64,
    pub complexity_measure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceIndicators {
    pub novelty_score: f64,
    pub coherence_level: f64,
    pub surprise_factor: f64,
    pub creative_potential: f64,
    pub implementation_feasibility: f64,
}

#[derive(Debug, Clone)]
pub struct PatternRecognizer {
    known_patterns: HashMap<Uuid, ChaosPattern>,
    similarity_threshold: f64,
    pattern_evolution_tracking: Vec<PatternEvolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternEvolution {
    pub pattern_id: Uuid,
    pub generation: u32,
    pub mutation_applied: String,
    pub fitness_change: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_id: Uuid,
    pub similarity_score: f64,
    pub context_relevance: f64,
    pub suggested_adaptations: Vec<String>,
}

impl ChaosPattern {
    #[tracing::instrument(name = "chaos_pattern_new", level = "debug")]
    pub fn new(name: String, description: String) -> Self {
        tracing::debug!("Creating new ChaosPattern: '{}'", name);
        
        let pattern = Self {
            id: Uuid::new_v4(),
            name: name.clone(),
            description: description.clone(),
            chaos_signature: ChaosSignature::default(),
            effectiveness_score: 0.0,
            usage_count: 0,
            created_at: Utc::now(),
            last_used: Utc::now(),
            success_rate: 0.0,
            context_tags: Vec::new(),
        };
        
        tracing::debug!("Created ChaosPattern with ID: {}", pattern.id);
        pattern
    }
    
    #[tracing::instrument(name = "update_pattern_effectiveness", level = "debug", skip(self))]
    pub fn update_effectiveness(&mut self, new_score: f64) {
        tracing::debug!("Updating effectiveness for pattern '{}' with score: {:.3}", self.name, new_score);
        
        let old_effectiveness = self.effectiveness_score;
        let old_success_rate = self.success_rate;
        
        self.usage_count += 1;
        self.last_used = Utc::now();
        
        // Running average of effectiveness
        self.effectiveness_score = (self.effectiveness_score * (self.usage_count - 1) as f64 + new_score) / self.usage_count as f64;
        
        // Update success rate (scores above 0.6 considered successful)
        let successes = if new_score > 0.6 { 1.0 } else { 0.0 };
        self.success_rate = (self.success_rate * (self.usage_count - 1) as f64 + successes) / self.usage_count as f64;
        
        tracing::debug!("Pattern effectiveness updated - usage: {}, effectiveness: {:.3} -> {:.3}, success rate: {:.3} -> {:.3}", 
            self.usage_count, old_effectiveness, self.effectiveness_score, old_success_rate, self.success_rate);
    }
    
    pub fn add_context_tag(&mut self, tag: String) {
        if !self.context_tags.contains(&tag) {
            self.context_tags.push(tag);
        }
    }
    
    #[tracing::instrument(name = "calculate_pattern_similarity", level = "debug", skip(self, other))]
    pub fn calculate_similarity(&self, other: &ChaosPattern) -> f64 {
        tracing::debug!("Calculating similarity between patterns '{}' and '{}'", self.name, other.name);
        
        let signature_similarity = self.chaos_signature.calculate_similarity(&other.chaos_signature);
        let context_similarity = self.calculate_context_similarity(&other.context_tags);
        
        let final_similarity = (signature_similarity + context_similarity) / 2.0;
        
        tracing::debug!("Pattern similarity: signature={:.3}, context={:.3}, final={:.3}", 
            signature_similarity, context_similarity, final_similarity);
        
        final_similarity
    }
    
    fn calculate_context_similarity(&self, other_tags: &[String]) -> f64 {
        if self.context_tags.is_empty() && other_tags.is_empty() {
            return 1.0;
        }
        
        if self.context_tags.is_empty() || other_tags.is_empty() {
            return 0.0;
        }
        
        let common_tags = self.context_tags.iter()
            .filter(|tag| other_tags.contains(tag))
            .count();
        
        let total_unique_tags = self.context_tags.len() + other_tags.len() - common_tags;
        
        if total_unique_tags == 0 {
            1.0
        } else {
            common_tags as f64 / total_unique_tags as f64
        }
    }
}

impl ChaosSignature {
    #[tracing::instrument(name = "chaos_signature_from_entropy", level = "debug")]
    pub fn from_entropy_sequence(sequence: &[f64]) -> Self {
        tracing::debug!("Creating ChaosSignature from entropy sequence of length: {}", sequence.len());
        
        let signature = Self {
            entropy_pattern: sequence.to_vec(),
            frequency_distribution: Self::calculate_frequency_distribution(sequence),
            mathematical_fingerprint: MathematicalFingerprint::from_sequence(sequence),
            emergence_indicators: EmergenceIndicators::analyze_sequence(sequence),
        };
        
        tracing::debug!("ChaosSignature created with {} frequency bands", signature.frequency_distribution.len());
        signature
    }
    
    fn calculate_frequency_distribution(sequence: &[f64]) -> HashMap<String, f64> {
        let mut distribution = HashMap::new();
        
        if sequence.is_empty() {
            return distribution;
        }
        
        // Analyze frequency bands
        let bands = [
            ("low", 0.0, 0.2),
            ("low_mid", 0.2, 0.4),
            ("mid", 0.4, 0.6),
            ("high_mid", 0.6, 0.8),
            ("high", 0.8, 1.0),
        ];
        
        for (band_name, min_val, max_val) in &bands {
            let count = sequence.iter()
                .filter(|&&x| x >= *min_val && x < *max_val)
                .count();
            
            distribution.insert(band_name.to_string(), count as f64 / sequence.len() as f64);
        }
        
        // Analyze transitions
        let mut transitions = 0;
        for i in 1..sequence.len() {
            if (sequence[i] - sequence[i-1]).abs() > 0.1 {
                transitions += 1;
            }
        }
        
        distribution.insert("transition_rate".to_string(), transitions as f64 / (sequence.len() - 1) as f64);
        
        distribution
    }
    
    #[tracing::instrument(name = "chaos_signature_similarity", level = "debug", skip(self, other))]
    pub fn calculate_similarity(&self, other: &ChaosSignature) -> f64 {
        tracing::debug!("Calculating ChaosSignature similarity");
        
        let pattern_similarity = self.calculate_pattern_similarity(&other.entropy_pattern);
        let frequency_similarity = self.calculate_frequency_similarity(&other.frequency_distribution);
        let fingerprint_similarity = self.mathematical_fingerprint.calculate_similarity(&other.mathematical_fingerprint);
        let emergence_similarity = self.emergence_indicators.calculate_similarity(&other.emergence_indicators);
        
        let final_similarity = (pattern_similarity + frequency_similarity + fingerprint_similarity + emergence_similarity) / 4.0;
        
        tracing::debug!("Signature similarity components - pattern: {:.3}, frequency: {:.3}, fingerprint: {:.3}, emergence: {:.3}, final: {:.3}", 
            pattern_similarity, frequency_similarity, fingerprint_similarity, emergence_similarity, final_similarity);
        
        final_similarity
    }
    
    fn calculate_pattern_similarity(&self, other_pattern: &[f64]) -> f64 {
        if self.entropy_pattern.is_empty() || other_pattern.is_empty() {
            return 0.0;
        }
        
        let min_len = self.entropy_pattern.len().min(other_pattern.len());
        let mut correlation_sum = 0.0;
        
        for i in 0..min_len {
            let diff = (self.entropy_pattern[i] - other_pattern[i]).abs();
            correlation_sum += 1.0 - diff; // Inverse of difference
        }
        
        correlation_sum / min_len as f64
    }
    
    fn calculate_frequency_similarity(&self, other_frequencies: &HashMap<String, f64>) -> f64 {
        let mut total_similarity = 0.0;
        let mut count = 0;
        
        for (key, &value) in &self.frequency_distribution {
            if let Some(&other_value) = other_frequencies.get(key) {
                let similarity = 1.0 - (value - other_value).abs();
                total_similarity += similarity;
                count += 1;
            }
        }
        
        if count > 0 {
            total_similarity / count as f64
        } else {
            0.0
        }
    }
}

impl MathematicalFingerprint {
    #[tracing::instrument(name = "mathematical_fingerprint_from_sequence", level = "debug")]
    pub fn from_sequence(sequence: &[f64]) -> Self {
        tracing::debug!("Creating MathematicalFingerprint from sequence of length: {}", sequence.len());
        
        let fingerprint = Self {
            fractal_dimension: Self::calculate_fractal_dimension(sequence),
            lyapunov_exponent: Self::calculate_lyapunov_exponent(sequence),
            correlation_sum: Self::calculate_correlation_sum(sequence),
            entropy_rate: Self::calculate_entropy_rate(sequence),
            complexity_measure: Self::calculate_complexity_measure(sequence),
        };
        
        tracing::debug!("MathematicalFingerprint created - fractal_dim: {:.3}, lyapunov: {:.3}, entropy: {:.3}", 
            fingerprint.fractal_dimension, fingerprint.lyapunov_exponent, fingerprint.entropy_rate);
        
        fingerprint
    }
    
    fn calculate_fractal_dimension(sequence: &[f64]) -> f64 {
        if sequence.len() < 10 {
            return 1.0;
        }
        
        // Box-counting method approximation
        let mut box_counts = Vec::new();
        let scales = [0.1, 0.05, 0.02, 0.01];
        
        for &scale in &scales {
            let mut boxes = std::collections::HashSet::new();
            
            for &value in sequence {
                let box_id = (value / scale).floor() as i32;
                boxes.insert(box_id);
            }
            
            box_counts.push(boxes.len() as f64);
        }
        
        // Linear regression to find dimension
        let mut sum_log_scale = 0.0;
        let mut sum_log_count = 0.0;
        let mut sum_log_scale_sq = 0.0;
        let mut sum_log_scale_count = 0.0;
        
        for (i, &count) in box_counts.iter().enumerate() {
            let log_scale = scales[i].ln();
            let log_count = count.ln();
            
            sum_log_scale += log_scale;
            sum_log_count += log_count;
            sum_log_scale_sq += log_scale * log_scale;
            sum_log_scale_count += log_scale * log_count;
        }
        
        let n = scales.len() as f64;
        let slope = (n * sum_log_scale_count - sum_log_scale * sum_log_count) / 
                   (n * sum_log_scale_sq - sum_log_scale * sum_log_scale);
        
        -slope // Fractal dimension is negative slope
    }
    
    fn calculate_lyapunov_exponent(sequence: &[f64]) -> f64 {
        if sequence.len() < 5 {
            return 0.0;
        }
        
        let mut sum = 0.0;
        let mut count = 0;
        
        for i in 1..sequence.len() {
            let delta = (sequence[i] - sequence[i-1]).abs();
            if delta > 0.0 && delta < 1.0 {
                sum += delta.ln();
                count += 1;
            }
        }
        
        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }
    
    fn calculate_correlation_sum(sequence: &[f64]) -> f64 {
        if sequence.len() < 10 {
            return 0.0;
        }
        
        let epsilon = 0.1;
        let mut correlations = 0;
        let mut total_pairs = 0;
        
        for i in 0..sequence.len()-1 {
            for j in i+1..sequence.len() {
                let distance = (sequence[i] - sequence[j]).abs();
                if distance < epsilon {
                    correlations += 1;
                }
                total_pairs += 1;
            }
        }
        
        correlations as f64 / total_pairs as f64
    }
    
    fn calculate_entropy_rate(sequence: &[f64]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        // Discretize and calculate Shannon entropy
        let bins = 10;
        let mut histogram = vec![0; bins];
        
        for &value in sequence {
            let bin = (value * bins as f64).floor() as usize;
            let bin = bin.min(bins - 1);
            histogram[bin] += 1;
        }
        
        let total = sequence.len() as f64;
        let mut entropy = 0.0;
        
        for count in histogram {
            if count > 0 {
                let probability = count as f64 / total;
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    fn calculate_complexity_measure(sequence: &[f64]) -> f64 {
        if sequence.len() < 3 {
            return 0.0;
        }
        
        // Lempel-Ziv complexity approximation
        let mut dictionary = std::collections::HashSet::new();
        let mut current_word = String::new();
        let mut complexity = 0;
        
        // Convert to binary representation
        for &value in sequence {
            let bit = if value > 0.5 { '1' } else { '0' };
            current_word.push(bit);
            
            if !dictionary.contains(&current_word) {
                dictionary.insert(current_word.clone());
                complexity += 1;
                current_word.clear();
            }
        }
        
        complexity as f64 / sequence.len() as f64
    }
    
    #[tracing::instrument(name = "fingerprint_similarity", level = "debug", skip(self, other))]
    pub fn calculate_similarity(&self, other: &MathematicalFingerprint) -> f64 {
        tracing::debug!("Calculating MathematicalFingerprint similarity");
        
        let fractal_sim = 1.0 - (self.fractal_dimension - other.fractal_dimension).abs() / 3.0_f64.max(self.fractal_dimension.max(other.fractal_dimension));
        let lyapunov_sim = 1.0 - (self.lyapunov_exponent - other.lyapunov_exponent).abs() / 2.0_f64.max(self.lyapunov_exponent.abs().max(other.lyapunov_exponent.abs()));
        let correlation_sim = 1.0 - (self.correlation_sum - other.correlation_sum).abs();
        let entropy_sim = 1.0 - (self.entropy_rate - other.entropy_rate).abs() / 4.0_f64.max(self.entropy_rate.max(other.entropy_rate));
        let complexity_sim = 1.0 - (self.complexity_measure - other.complexity_measure).abs();
        
        let final_similarity = (fractal_sim + lyapunov_sim + correlation_sim + entropy_sim + complexity_sim) / 5.0;
        
        tracing::debug!("Fingerprint similarity components - fractal: {:.3}, lyapunov: {:.3}, correlation: {:.3}, entropy: {:.3}, complexity: {:.3}, final: {:.3}", 
            fractal_sim, lyapunov_sim, correlation_sim, entropy_sim, complexity_sim, final_similarity);
        
        final_similarity
    }
}

impl EmergenceIndicators {
    pub fn analyze_sequence(sequence: &[f64]) -> Self {
        Self {
            novelty_score: Self::calculate_novelty_score(sequence),
            coherence_level: Self::calculate_coherence_level(sequence),
            surprise_factor: Self::calculate_surprise_factor(sequence),
            creative_potential: Self::calculate_creative_potential(sequence),
            implementation_feasibility: Self::calculate_implementation_feasibility(sequence),
        }
    }
    
    fn calculate_novelty_score(sequence: &[f64]) -> f64 {
        if sequence.len() < 3 {
            return 0.5;
        }
        
        // Measure how much the sequence deviates from predictable patterns
        let mut predictability = 0.0;
        
        for i in 2..sequence.len() {
            let predicted = 2.0 * sequence[i-1] - sequence[i-2];
            let actual = sequence[i];
            let error = (predicted - actual).abs();
            predictability += 1.0 - error.min(1.0);
        }
        
        predictability /= (sequence.len() - 2) as f64;
        
        // Novelty is inverse of predictability
        1.0 - predictability
    }
    
    fn calculate_coherence_level(sequence: &[f64]) -> f64 {
        if sequence.len() < 2 {
            return 1.0;
        }
        
        // Measure smoothness and lack of extreme jumps
        let mut total_variation = 0.0;
        for i in 1..sequence.len() {
            total_variation += (sequence[i] - sequence[i-1]).abs();
        }
        
        let average_variation = total_variation / (sequence.len() - 1) as f64;
        
        // Coherence is inverse of variation
        (1.0 - average_variation).max(0.0)
    }
    
    fn calculate_surprise_factor(sequence: &[f64]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        // Count unexpected values (outliers)
        let mean = sequence.iter().sum::<f64>() / sequence.len() as f64;
        let variance = sequence.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / sequence.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev == 0.0 {
            return 0.0;
        }
        
        let outliers = sequence.iter()
            .filter(|&&x| (x - mean).abs() > 2.0 * std_dev)
            .count();
        
        (outliers as f64 / sequence.len() as f64).min(1.0)
    }
    
    fn calculate_creative_potential(sequence: &[f64]) -> f64 {
        // Creative potential is a combination of novelty and coherence
        let novelty = Self::calculate_novelty_score(sequence);
        let coherence = Self::calculate_coherence_level(sequence);
        
        // Optimal creativity is at moderate levels of both
        let novelty_factor = 1.0 - (novelty - 0.6).abs() * 2.0; // Peak at 0.6
        let coherence_factor = 1.0 - (coherence - 0.7).abs() * 2.0; // Peak at 0.7
        
        (novelty_factor * coherence_factor).max(0.0)
    }
    
    fn calculate_implementation_feasibility(sequence: &[f64]) -> f64 {
        // Feasibility decreases with extreme chaos
        let coherence = Self::calculate_coherence_level(sequence);
        let surprise = Self::calculate_surprise_factor(sequence);
        
        // High coherence, moderate surprise is most feasible
        coherence * (1.0 - surprise * 0.5)
    }
    
    pub fn calculate_similarity(&self, other: &EmergenceIndicators) -> f64 {
        let novelty_sim = 1.0 - (self.novelty_score - other.novelty_score).abs();
        let coherence_sim = 1.0 - (self.coherence_level - other.coherence_level).abs();
        let surprise_sim = 1.0 - (self.surprise_factor - other.surprise_factor).abs();
        let creative_sim = 1.0 - (self.creative_potential - other.creative_potential).abs();
        let feasibility_sim = 1.0 - (self.implementation_feasibility - other.implementation_feasibility).abs();
        
        (novelty_sim + coherence_sim + surprise_sim + creative_sim + feasibility_sim) / 5.0
    }
}

impl PatternRecognizer {
    pub fn new() -> Self {
        Self {
            known_patterns: HashMap::new(),
            similarity_threshold: 0.7,
            pattern_evolution_tracking: Vec::new(),
        }
    }
    
    pub fn add_pattern(&mut self, pattern: ChaosPattern) {
        self.known_patterns.insert(pattern.id, pattern);
    }
    
    pub fn find_similar_patterns(&self, target_signature: &ChaosSignature) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        
        for pattern in self.known_patterns.values() {
            let similarity = pattern.chaos_signature.calculate_similarity(target_signature);
            
            if similarity >= self.similarity_threshold {
                matches.push(PatternMatch {
                    pattern_id: pattern.id,
                    similarity_score: similarity,
                    context_relevance: pattern.effectiveness_score,
                    suggested_adaptations: self.generate_adaptations(&pattern, similarity),
                });
            }
        }
        
        // Sort by similarity score
        matches.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        
        matches
    }
    
    fn generate_adaptations(&self, pattern: &ChaosPattern, similarity: f64) -> Vec<String> {
        let mut adaptations = Vec::new();
        
        if similarity > 0.9 {
            adaptations.push("Use pattern as-is with minor parameter adjustments".to_string());
        } else if similarity > 0.8 {
            adaptations.push("Adapt pattern with moderate modifications".to_string());
            adaptations.push("Consider hybrid approach with other patterns".to_string());
        } else {
            adaptations.push("Extract core principles and reimagine implementation".to_string());
            adaptations.push("Use as inspiration for new pattern development".to_string());
        }
        
        // Context-specific adaptations
        for tag in &pattern.context_tags {
            adaptations.push(format!("Apply {}-specific enhancements", tag));
        }
        
        adaptations
    }
    
    pub fn evolve_pattern(&mut self, pattern_id: Uuid, mutation: String) -> Option<ChaosPattern> {
        if let Some(mut pattern) = self.known_patterns.get(&pattern_id).cloned() {
            let _old_effectiveness = pattern.effectiveness_score;
            
            // Apply mutation (this would be more sophisticated in practice)
            pattern.description = format!("{} [EVOLVED: {}]", pattern.description, mutation);
            
            // Track evolution
            self.pattern_evolution_tracking.push(PatternEvolution {
                pattern_id,
                generation: self.get_pattern_generation(pattern_id) + 1,
                mutation_applied: mutation,
                fitness_change: 0.0, // Will be updated after testing
                timestamp: Utc::now(),
            });
            
            Some(pattern)
        } else {
            None
        }
    }
    
    fn get_pattern_generation(&self, pattern_id: Uuid) -> u32 {
        self.pattern_evolution_tracking
            .iter()
            .filter(|evolution| evolution.pattern_id == pattern_id)
            .map(|evolution| evolution.generation)
            .max()
            .unwrap_or(0)
    }
    
    pub fn get_best_patterns(&self, limit: usize) -> Vec<&ChaosPattern> {
        let mut patterns: Vec<&ChaosPattern> = self.known_patterns.values().collect();
        patterns.sort_by(|a, b| b.effectiveness_score.partial_cmp(&a.effectiveness_score).unwrap());
        patterns.into_iter().take(limit).collect()
    }
}

impl Default for ChaosSignature {
    fn default() -> Self {
        Self {
            entropy_pattern: Vec::new(),
            frequency_distribution: HashMap::new(),
            mathematical_fingerprint: MathematicalFingerprint::default(),
            emergence_indicators: EmergenceIndicators::default(),
        }
    }
}

impl Default for MathematicalFingerprint {
    fn default() -> Self {
        Self {
            fractal_dimension: 1.0,
            lyapunov_exponent: 0.0,
            correlation_sum: 0.0,
            entropy_rate: 0.0,
            complexity_measure: 0.0,
        }
    }
}

impl Default for EmergenceIndicators {
    fn default() -> Self {
        Self {
            novelty_score: 0.5,
            coherence_level: 0.5,
            surprise_factor: 0.0,
            creative_potential: 0.5,
            implementation_feasibility: 0.5,
        }
    }
}

impl Default for PatternRecognizer {
    fn default() -> Self {
        Self::new()
    }
}