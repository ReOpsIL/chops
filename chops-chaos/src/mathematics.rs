use chops_core::CHOPSResult;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct ChaosMathematics {
    lorenz_state: LorenzAttractor,
    henon_state: HenonMap,
    mandelbrot_explorer: MandelbrotExplorer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LorenzAttractor {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
    pub dt: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HenonMap {
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, Clone)]
pub struct MandelbrotExplorer {
    pub max_iterations: u32,
    pub escape_radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosMetrics {
    pub lyapunov_exponent: f64,
    pub correlation_dimension: f64,
    pub entropy_rate: f64,
    pub predictability_horizon: f64,
}

impl ChaosMathematics {
    pub fn new() -> Self {
        Self {
            lorenz_state: LorenzAttractor::new(),
            henon_state: HenonMap::new(),
            mandelbrot_explorer: MandelbrotExplorer::new(),
        }
    }
    
    pub fn lorenz_chaos_value(&mut self) -> f64 {
        // Evolve the Lorenz attractor
        for _ in 0..100 {
            self.lorenz_state.iterate();
        }
        
        // Normalize x coordinate to [0,1]
        ((self.lorenz_state.x + 20.0) / 40.0).max(0.0).min(1.0)
    }
    
    pub fn henon_chaos_value(&mut self) -> f64 {
        // Evolve the Hénon map
        for _ in 0..50 {
            self.henon_state.iterate();
        }
        
        // Normalize x coordinate to [0,1]
        ((self.henon_state.x + 1.5) / 3.0).max(0.0).min(1.0)
    }
    
    pub fn mandelbrot_chaos_value(&self, real: f64, imag: f64) -> f64 {
        let iterations = self.mandelbrot_explorer.escape_time(real, imag);
        iterations as f64 / self.mandelbrot_explorer.max_iterations as f64
    }
    
    pub fn generate_chaotic_sequence(&mut self, length: usize) -> Vec<f64> {
        let mut sequence = Vec::with_capacity(length);
        
        for i in 0..length {
            let value = match i % 3 {
                0 => self.lorenz_chaos_value(),
                1 => self.henon_chaos_value(),
                2 => {
                    let real = rand::thread_rng().gen_range(-2.0..2.0);
                    let imag = rand::thread_rng().gen_range(-2.0..2.0);
                    self.mandelbrot_chaos_value(real, imag)
                },
                _ => unreachable!(),
            };
            sequence.push(value);
        }
        
        sequence
    }
    
    pub fn calculate_chaos_metrics(&mut self, sequence_length: usize) -> CHOPSResult<ChaosMetrics> {
        let sequence = self.generate_chaotic_sequence(sequence_length);
        
        Ok(ChaosMetrics {
            lyapunov_exponent: self.calculate_lyapunov_exponent(&sequence),
            correlation_dimension: self.calculate_correlation_dimension(&sequence),
            entropy_rate: self.calculate_entropy_rate(&sequence),
            predictability_horizon: self.calculate_predictability_horizon(&sequence),
        })
    }
    
    fn calculate_lyapunov_exponent(&self, sequence: &[f64]) -> f64 {
        if sequence.len() < 10 {
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
    
    fn calculate_correlation_dimension(&self, sequence: &[f64]) -> f64 {
        if sequence.len() < 20 {
            return 1.0;
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
        
        if total_pairs > 0 {
            let correlation_integral = correlations as f64 / total_pairs as f64;
            if correlation_integral > 0.0 {
                -correlation_integral.ln() / epsilon.ln()
            } else {
                1.0
            }
        } else {
            1.0
        }
    }
    
    fn calculate_entropy_rate(&self, sequence: &[f64]) -> f64 {
        if sequence.is_empty() {
            return 0.0;
        }
        
        // Discretize the sequence into bins
        let bins = 10;
        let mut histogram = vec![0; bins];
        
        for &value in sequence {
            let bin = (value * bins as f64).floor() as usize;
            let bin = bin.min(bins - 1);
            histogram[bin] += 1;
        }
        
        // Calculate Shannon entropy
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
    
    fn calculate_predictability_horizon(&self, sequence: &[f64]) -> f64 {
        if sequence.len() < 5 {
            return 1.0;
        }
        
        // Simple predictability measure based on local correlation
        let mut predictable_steps = 0;
        let threshold = 0.1;
        
        for i in 2..sequence.len() {
            let predicted = 2.0 * sequence[i-1] - sequence[i-2];
            let actual = sequence[i];
            let error = (predicted - actual).abs();
            
            if error < threshold {
                predictable_steps += 1;
            } else {
                break;
            }
        }
        
        predictable_steps as f64
    }
    
    pub fn apply_chaotic_transformation(&mut self, input: f64) -> f64 {
        // Apply multiple chaotic transformations
        let lorenz_factor = self.lorenz_chaos_value();
        let henon_factor = self.henon_chaos_value();
        
        // Combine transformations non-linearly
        let transformed = input * lorenz_factor + (1.0 - input) * henon_factor;
        let final_value = (transformed + 0.1 * (transformed * 2.0 * std::f64::consts::PI).sin()).abs();
        
        final_value.min(1.0)
    }
    
    pub fn generate_fractal_noise(&self, x: f64, y: f64, octaves: u32) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let persistence = 0.5;
        
        for _ in 0..octaves {
            // Use Mandelbrot set as basis for fractal noise
            let noise_value = self.mandelbrot_chaos_value(x * frequency, y * frequency);
            value += noise_value * amplitude;
            
            amplitude *= persistence;
            frequency *= 2.0;
        }
        
        value.min(1.0)
    }
}

impl LorenzAttractor {
    pub fn new() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0/3.0,
            dt: 0.01,
        }
    }
    
    pub fn iterate(&mut self) {
        let dx = self.sigma * (self.y - self.x) * self.dt;
        let dy = (self.x * (self.rho - self.z) - self.y) * self.dt;
        let dz = (self.x * self.y - self.beta * self.z) * self.dt;
        
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
    
    pub fn set_parameters(&mut self, sigma: f64, rho: f64, beta: f64) {
        self.sigma = sigma;
        self.rho = rho;
        self.beta = beta;
    }
}

impl HenonMap {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            a: 1.4,
            b: 0.3,
        }
    }
    
    pub fn iterate(&mut self) {
        let new_x = 1.0 - self.a * self.x * self.x + self.y;
        let new_y = self.b * self.x;
        
        self.x = new_x;
        self.y = new_y;
    }
    
    pub fn set_parameters(&mut self, a: f64, b: f64) {
        self.a = a;
        self.b = b;
    }
}

impl MandelbrotExplorer {
    pub fn new() -> Self {
        Self {
            max_iterations: 100,
            escape_radius: 2.0,
        }
    }
    
    pub fn escape_time(&self, c_real: f64, c_imag: f64) -> u32 {
        let mut z_real = 0.0;
        let mut z_imag = 0.0;
        
        for iteration in 0..self.max_iterations {
            let z_real_sq = z_real * z_real;
            let z_imag_sq = z_imag * z_imag;
            
            if z_real_sq + z_imag_sq > self.escape_radius * self.escape_radius {
                return iteration;
            }
            
            let new_z_real = z_real_sq - z_imag_sq + c_real;
            let new_z_imag = 2.0 * z_real * z_imag + c_imag;
            
            z_real = new_z_real;
            z_imag = new_z_imag;
        }
        
        self.max_iterations
    }
    
    pub fn is_in_set(&self, c_real: f64, c_imag: f64) -> bool {
        self.escape_time(c_real, c_imag) == self.max_iterations
    }
    
    pub fn generate_fractal_value(&self, x: f64, y: f64, zoom: f64) -> f64 {
        let c_real = (x - 0.5) * zoom - 0.7;
        let c_imag = (y - 0.5) * zoom;
        
        let iterations = self.escape_time(c_real, c_imag);
        
        if iterations == self.max_iterations {
            0.0
        } else {
            // Smooth coloring
            let z_real = c_real;
            let z_imag = c_imag;
            let log_zn = (z_real * z_real + z_imag * z_imag).ln() * 0.5;
            let nu = (log_zn / 2.0_f64.ln()).ln() / 2.0_f64.ln();
            
            (iterations as f64 + 1.0 - nu) / self.max_iterations as f64
        }
    }
}

impl Default for ChaosMathematics {
    fn default() -> Self {
        Self::new()
    }
}