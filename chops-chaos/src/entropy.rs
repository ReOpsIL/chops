use chops_core::{EntropySource, CHOPSResult, CHOPSError};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct EntropyGenerator {
    source: EntropySource,
    quantum_client: Option<QuantumClient>,
    entropy_pool: EntropyPool,
}

#[derive(Debug, Clone)]
pub struct QuantumClient {
    api_endpoint: String,
    api_key: Option<String>,
    client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyPool {
    pub buffer: Vec<u8>,
    pub current_position: usize,
    pub refresh_threshold: usize,
    pub quality_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuantumRandomResponse {
    data: Vec<u8>,
    success: bool,
    error: Option<String>,
}

impl EntropyGenerator {
    #[tracing::instrument(name = "entropy_generator_new", level = "debug")]
    pub fn new() -> Self {
        tracing::debug!("Creating new EntropyGenerator with PseudoRandom source");
        
        let generator = Self {
            source: EntropySource::PseudoRandom,
            quantum_client: None,
            entropy_pool: EntropyPool::new(),
        };
        
        tracing::debug!("EntropyGenerator initialized");
        generator
    }
    
    #[tracing::instrument(name = "set_entropy_source", level = "info")]
    pub fn set_source(&mut self, source: EntropySource) -> CHOPSResult<()> {
        tracing::info!("Setting entropy source to: {:?}", source);
        
        self.source = source.clone();
        
        match source {
            EntropySource::QuantumRandom => {
                tracing::debug!("Initializing quantum client for quantum random source");
                self.quantum_client = Some(QuantumClient::new());
            },
            _ => {
                tracing::debug!("Clearing quantum client for non-quantum source");
                self.quantum_client = None;
            }
        }
        
        tracing::info!("Entropy source set successfully");
        Ok(())
    }
    
    #[tracing::instrument(name = "generate_entropy", level = "debug", skip(self))]
    pub async fn generate_entropy(&mut self) -> CHOPSResult<f64> {
        tracing::debug!("Generating entropy using source: {:?}", self.source);
        
        let entropy = match self.source {
            EntropySource::PseudoRandom => self.generate_pseudo_random(),
            EntropySource::TrueRandom => self.generate_true_random(),
            EntropySource::QuantumRandom => self.generate_quantum_random().await,
            EntropySource::ChaosEquation => self.generate_chaos_equation(),
        };
        
        match entropy {
            Ok(value) => {
                tracing::debug!("Generated entropy value: {:.6}", value);
                Ok(value)
            },
            Err(e) => {
                tracing::error!("Failed to generate entropy: {}", e);
                Err(e)
            }
        }
    }
    
    #[tracing::instrument(name = "generate_pseudo_random", level = "trace")]
    fn generate_pseudo_random(&self) -> CHOPSResult<f64> {
        let value = rand::thread_rng().gen::<f64>();
        tracing::trace!("Generated pseudo-random value: {:.6}", value);
        Ok(value)
    }
    
    #[tracing::instrument(name = "generate_true_random", level = "trace", skip(self))]
    fn generate_true_random(&mut self) -> CHOPSResult<f64> {
        tracing::trace!("Generating true random value from system entropy");
        
        // Use system entropy sources
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf)
            .map_err(|e| {
                tracing::error!("System entropy failed: {}", e);
                CHOPSError::ChaosError(format!("System entropy failed: {}", e))
            })?;
        
        // Add to entropy pool for quality analysis
        self.entropy_pool.add_bytes(&buf);
        tracing::trace!("Added {} bytes to entropy pool", buf.len());
        
        let value = u64::from_le_bytes(buf) as f64 / u64::MAX as f64;
        tracing::trace!("Generated true random value: {:.6}", value);
        Ok(value)
    }
    
    #[tracing::instrument(name = "generate_quantum_random", level = "debug", skip(self))]
    async fn generate_quantum_random(&mut self) -> CHOPSResult<f64> {
        tracing::debug!("Attempting to generate quantum random value");
        
        if let Some(ref client) = self.quantum_client {
            match client.fetch_quantum_bytes(8).await {
                Ok(bytes) => {
                    if bytes.len() >= 8 {
                        tracing::debug!("Received {} quantum bytes", bytes.len());
                        let mut buf = [0u8; 8];
                        buf.copy_from_slice(&bytes[0..8]);
                        
                        // Add to entropy pool
                        self.entropy_pool.add_bytes(&buf);
                        
                        let value = u64::from_le_bytes(buf) as f64 / u64::MAX as f64;
                        tracing::debug!("Generated quantum random value: {:.6}", value);
                        return Ok(value);
                    } else {
                        tracing::warn!("Insufficient quantum bytes received: {}", bytes.len());
                    }
                },
                Err(e) => {
                    tracing::warn!("Quantum entropy source failed: {}, falling back to system entropy", e);
                    return self.generate_true_random();
                }
            }
        } else {
            tracing::warn!("No quantum client available, falling back to system entropy");
        }
        
        // Fallback
        self.generate_true_random()
    }
    
    #[tracing::instrument(name = "generate_chaos_equation", level = "trace")]
    fn generate_chaos_equation(&self) -> CHOPSResult<f64> {
        tracing::trace!("Generating chaos equation entropy");
        
        // Use multiple chaotic maps combined
        let mut x = 0.1;
        let mut y = 0.2;
        
        // Iterate logistic map
        for _ in 0..100 {
            x = 4.0 * x * (1.0 - x);
        }
        tracing::trace!("Logistic map result: {:.6}", x);
        
        // Iterate tent map
        for _ in 0..100 {
            y = if y < 0.5 { 2.0 * y } else { 2.0 * (1.0 - y) };
        }
        tracing::trace!("Tent map result: {:.6}", y);
        
        // Combine results
        let combined = (x + y) / 2.0;
        
        // Add timestamp entropy
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f64;
        
        let timestamp_entropy = (timestamp % 1000000.0) / 1000000.0;
        tracing::trace!("Timestamp entropy: {:.6}", timestamp_entropy);
        
        let final_value = (combined + timestamp_entropy * 0.1) % 1.0;
        tracing::trace!("Final chaos equation value: {:.6}", final_value);
        
        Ok(final_value)
    }
    
    #[tracing::instrument(name = "generate_entropy_sequence", level = "info", skip(self))]
    pub async fn generate_entropy_sequence(&mut self, length: usize) -> CHOPSResult<Vec<f64>> {
        tracing::info!("Generating entropy sequence of length: {}", length);
        
        let mut sequence = Vec::with_capacity(length);
        
        for i in 0..length {
            if i > 0 && i % 50 == 0 {
                tracing::debug!("Generated {} entropy values so far", i);
            }
            sequence.push(self.generate_entropy().await?);
        }
        
        tracing::info!("Successfully generated {} entropy values", sequence.len());
        Ok(sequence)
    }
    
    pub fn analyze_entropy_quality(&self, sequence: &[f64]) -> EntropyQuality {
        EntropyQuality::analyze(sequence)
    }
    
    #[tracing::instrument(name = "reseed_entropy_pool", level = "info", skip(self))]
    pub async fn reseed_entropy_pool(&mut self) -> CHOPSResult<()> {
        tracing::info!("Reseeding entropy pool");
        
        let old_quality = self.entropy_pool.get_quality_score();
        tracing::debug!("Current entropy pool quality: {:.3}", old_quality);
        
        let fresh_entropy = self.generate_entropy_sequence(256).await?;
        
        for value in fresh_entropy {
            let bytes = (value * u64::MAX as f64) as u64;
            self.entropy_pool.add_bytes(&bytes.to_le_bytes());
        }
        
        self.entropy_pool.refresh_quality_score();
        let new_quality = self.entropy_pool.get_quality_score();
        
        tracing::info!("Entropy pool reseeded - quality improved from {:.3} to {:.3}", 
            old_quality, new_quality);
        
        Ok(())
    }
}

impl QuantumClient {
    pub fn new() -> Self {
        Self {
            api_endpoint: "https://qrng.anu.edu.au/API/jsonI.php".to_string(),
            api_key: None,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn fetch_quantum_bytes(&self, count: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Try ANU Quantum Random Numbers Generator API
        let url = format!("{}?length={}&type=uint8", self.api_endpoint, count);
        
        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        
        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            
            if let Some(data) = json["data"].as_array() {
                let bytes: Result<Vec<u8>, _> = data.iter()
                    .map(|v| v.as_u64().map(|n| n as u8))
                    .collect::<Option<Vec<_>>>()
                    .ok_or("Invalid data format");
                
                return Ok(bytes?);
            }
        }
        
        Err("Failed to fetch quantum random data".into())
    }
}

impl EntropyPool {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
            current_position: 0,
            refresh_threshold: 3072, // Refresh when 75% used
            quality_score: 0.0,
        }
    }
    
    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
        
        // Maintain maximum buffer size
        if self.buffer.len() > 4096 {
            self.buffer.drain(0..self.buffer.len() - 4096);
            self.current_position = 0;
        }
    }
    
    pub fn get_bytes(&mut self, count: usize) -> Option<Vec<u8>> {
        if self.current_position + count > self.buffer.len() {
            return None;
        }
        
        let bytes = self.buffer[self.current_position..self.current_position + count].to_vec();
        self.current_position += count;
        
        Some(bytes)
    }
    
    pub fn needs_refresh(&self) -> bool {
        self.current_position >= self.refresh_threshold || 
        self.buffer.len() < self.refresh_threshold
    }
    
    pub fn refresh_quality_score(&mut self) {
        if self.buffer.is_empty() {
            self.quality_score = 0.0;
            return;
        }
        
        // Calculate entropy quality based on statistical tests
        let mut byte_counts = [0u32; 256];
        for &byte in &self.buffer {
            byte_counts[byte as usize] += 1;
        }
        
        // Chi-square test for uniformity
        let expected = self.buffer.len() as f64 / 256.0;
        let mut chi_square = 0.0;
        
        for &count in &byte_counts {
            let diff = count as f64 - expected;
            chi_square += (diff * diff) / expected;
        }
        
        // Convert chi-square to quality score (0.0 to 1.0)
        let critical_value = 293.25; // Chi-square critical value for 255 df at 95% confidence
        self.quality_score = (1.0 - (chi_square / critical_value).min(1.0)).max(0.0);
    }
    
    pub fn get_quality_score(&self) -> f64 {
        self.quality_score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyQuality {
    pub uniformity_score: f64,
    pub independence_score: f64,
    pub compression_ratio: f64,
    pub overall_quality: f64,
}

impl EntropyQuality {
    pub fn analyze(sequence: &[f64]) -> Self {
        let uniformity_score = Self::test_uniformity(sequence);
        let independence_score = Self::test_independence(sequence);
        let compression_ratio = Self::test_compression(sequence);
        
        let overall_quality = (uniformity_score + independence_score + compression_ratio) / 3.0;
        
        Self {
            uniformity_score,
            independence_score,
            compression_ratio,
            overall_quality,
        }
    }
    
    fn test_uniformity(sequence: &[f64]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        let bins = 10;
        let mut histogram = vec![0; bins];
        
        for &value in sequence {
            let bin = (value * bins as f64).floor() as usize;
            let bin = bin.min(bins - 1);
            histogram[bin] += 1;
        }
        
        // Calculate chi-square statistic
        let expected = sequence.len() as f64 / bins as f64;
        let mut chi_square = 0.0;
        
        for count in histogram {
            let diff = count as f64 - expected;
            chi_square += (diff * diff) / expected;
        }
        
        // Convert to quality score
        let critical_value = 16.92; // Chi-square critical value for 9 df at 95% confidence
        (1.0 - (chi_square / critical_value).min(1.0)).max(0.0)
    }
    
    fn test_independence(sequence: &[f64]) -> f64 {
        if sequence.len() < 2 {
            return 0.0;
        }
        
        // Test for serial correlation
        let mut sum_xy = 0.0;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_x2 = 0.0;
        let mut sum_y2 = 0.0;
        let n = (sequence.len() - 1) as f64;
        
        for i in 0..sequence.len()-1 {
            let x = sequence[i];
            let y = sequence[i + 1];
            
            sum_xy += x * y;
            sum_x += x;
            sum_y += y;
            sum_x2 += x * x;
            sum_y2 += y * y;
        }
        
        let correlation = (n * sum_xy - sum_x * sum_y) / 
                         ((n * sum_x2 - sum_x * sum_x).sqrt() * (n * sum_y2 - sum_y * sum_y).sqrt());
        
        // Independence is better when correlation is closer to 0
        1.0 - correlation.abs()
    }
    
    fn test_compression(sequence: &[f64]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        // Convert to bytes for compression test
        let bytes: Vec<u8> = sequence.iter()
            .map(|&x| (x * 255.0) as u8)
            .collect();
        
        // Simple run-length encoding to estimate compressibility
        let mut compressed_size = 0;
        let mut current_byte = bytes[0];
        let mut run_length = 1;
        
        for &byte in &bytes[1..] {
            if byte == current_byte && run_length < 255 {
                run_length += 1;
            } else {
                compressed_size += 2; // byte + count
                current_byte = byte;
                run_length = 1;
            }
        }
        compressed_size += 2; // Final run
        
        let compression_ratio = compressed_size as f64 / bytes.len() as f64;
        
        // Good entropy should not compress well (ratio close to 1.0)
        compression_ratio
    }
}

impl Default for EntropyGenerator {
    fn default() -> Self {
        Self::new()
    }
}