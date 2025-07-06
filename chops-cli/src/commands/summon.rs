use chops_core::{PersonaType, CHOPSResult};
use crate::{CHOPSSystem, output};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub async fn execute(
    system: &mut CHOPSSystem,
    persona: PersonaType,
    domain: String,
    chaos: u8,
    timeline: Option<String>,
    vibe: Option<String>,
    constraints: Vec<String>,
    reality_level: f64,
) -> CHOPSResult<()> {
    // Create progress bar for the summoning ritual
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    
    pb.set_message(format!("🔮 Summoning {} with chaos level {}...", persona, chaos));
    pb.enable_steady_tick(Duration::from_millis(100));

    // Build the summoning prompt
    let mut prompt = String::new();
    
    if let Some(vibe) = &vibe {
        prompt.push_str(&format!("Channel the vibe of '{}' while ", vibe));
    }
    
    prompt.push_str(&format!(
        "generating innovative ideas for {} development",
        domain
    ));
    
    if let Some(timeline) = &timeline {
        prompt.push_str(&format!(" in the context of {}", timeline));
    }
    
    if !constraints.is_empty() {
        prompt.push_str(&format!(
            " while working within these constraints: {}",
            constraints.join(", ")
        ));
    }
    
    prompt.push_str(". Focus on breakthrough innovations that push boundaries while remaining implementable.");

    pb.set_message("🧠 Activating cognitive architecture...");
    
    // Use the cognitive architecture for complex processing
    let result = system.cognitive_architecture
        .process_complex_idea(&prompt, persona.clone(), &domain, chaos as f64 / 11.0)
        .await?;

    pb.finish_with_message("✨ Summoning complete!");

    // Display the generated idea with rich formatting
    output::display_complex_idea_result(&result)?;

    // Save to memory
    system.memory_system.add_idea(chops_core::GeneratedIdea {
        id: result.base_idea.id,
        title: extract_title_from_content(&result.base_idea.content),
        description: result.base_idea.content.clone(),
        persona_used: persona,
        chaos_level: chaos as f64 / 11.0,
        creativity_score: result.base_idea.creativity_score,
        feasibility_score: result.base_idea.feasibility_score,
        novelty_score: result.base_idea.novelty_score,
        excitement_factor: result.base_idea.excitement_factor,
        tags: extract_tags_from_content(&result.base_idea.content),
        implementation_hints: result.implementation_roadmap.critical_path,
        potential_risks: vec![], // Could be extracted from reality distortion
        experimental_variations: vec![], // Convert from chaos variations
        analogies: result.analogical_insights.into_iter().map(|insight| {
            chops_core::Analogy {
                source_domain: insight.source_domain.clone(),
                source_pattern: insight.analogy_description.clone(),
                target_concept: domain.clone(),
                structural_mapping: chops_core::StructuralMapping {
                    source_domain: insight.source_domain.clone(),
                    target_concept: domain.clone(),
                    source_elements: vec![],
                    target_elements: vec![],
                    relationship_mappings: vec![],
                },
                insight: insight.analogy_description,
                confidence: insight.confidence_score,
                novelty_score: insight.surprise_factor,
                practical_applicability: 0.7, // Default value
            }
        }).collect(),
        timestamp: chrono::Utc::now(),
    });

    // Save memory
    system.save_memory().await?;

    // Show related ideas from memory
    let similar_ideas = system.memory_system.recall_similar_ideas(&domain, 3);
    if !similar_ideas.is_empty() {
        println!("\n{}", "🔗 Related ideas from your memory:".bright_blue());
        for idea in similar_ideas {
            println!("  • {} ({})", 
                idea.title.bright_white(), 
                format!("{}% creative", (idea.creativity_score * 100.0) as u32).green()
            );
        }
    }

    // Suggest next actions
    println!("\n{}", "🚀 Suggested next actions:".bright_yellow());
    println!("  • {}", "chops mutate --file <your-code> --personality --weird".cyan());
    println!("  • {}", format!("chops prophecy --domain {} --year 2030", domain).cyan());
    println!("  • {}", "chops collaborate --mode debate --topic \"implementation approach\"".cyan());

    Ok(())
}

fn extract_title_from_content(content: &str) -> String {
    // Extract the first line or first sentence as title
    if let Some(first_line) = content.lines().next() {
        let title = first_line.trim();
        if title.len() > 100 {
            format!("{}...", &title[..97])
        } else {
            title.to_string()
        }
    } else {
        "Generated Idea".to_string()
    }
}

fn extract_tags_from_content(content: &str) -> Vec<String> {
    let mut tags = Vec::new();
    
    // Extract common technical terms as tags
    let keywords = [
        "ai", "machine learning", "algorithm", "api", "database", "framework",
        "architecture", "performance", "security", "testing", "automation",
        "cloud", "microservices", "blockchain", "quantum", "neural",
        "optimization", "scalability", "user experience", "innovation"
    ];
    
    let content_lower = content.to_lowercase();
    for keyword in &keywords {
        if content_lower.contains(keyword) {
            tags.push(keyword.to_string());
        }
    }
    
    // Limit to 5 most relevant tags
    tags.truncate(5);
    
    if tags.is_empty() {
        tags.push("innovative".to_string());
        tags.push("creative".to_string());
    }
    
    tags
}