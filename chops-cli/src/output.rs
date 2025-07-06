use chops_api::ComplexIdeaResult;
use chops_core::CHOPSResult;
use colored::*;

pub fn display_complex_idea_result(result: &ComplexIdeaResult) -> CHOPSResult<()> {
    println!("\n{}", "═══════════════════════════════════════════════════════".bright_cyan());
    println!("{}", "🔮 IDEA SUMMONED SUCCESSFULLY 🔮".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════".bright_cyan());

    // Main idea content
    println!("\n{}", "💡 Generated Idea:".bright_yellow().bold());
    println!("{}", format_content_with_boxes(&result.base_idea.content));

    // Quality metrics
    println!("\n{}", "📊 Quality Metrics:".bright_blue().bold());
    display_metrics_bar("Creativity", result.base_idea.creativity_score);
    display_metrics_bar("Feasibility", result.base_idea.feasibility_score);
    display_metrics_bar("Novelty", result.base_idea.novelty_score);
    display_metrics_bar("Excitement", result.base_idea.excitement_factor);
    display_metrics_bar("Coherence", result.base_idea.coherence_score);
    
    let overall = result.base_idea.calculate_overall_score();
    let tier = result.base_idea.get_quality_tier();
    println!("  {} {}", "Overall:".bright_white(), format!("{} ({})", format_score(overall), tier).bright_green());

    // Chaos effects
    if result.base_idea.chaos_level > 0.1 {
        println!("\n{}", "⚡ Chaos Effects:".bright_magenta().bold());
        println!("  {} {}", "Chaos Level:".white(), format_score(result.base_idea.chaos_level).bright_red());
        
        if !result.base_idea.unexpected_elements.is_empty() {
            println!("  {} {}", "Unexpected Elements:".white(), result.base_idea.unexpected_elements.len().to_string().bright_yellow());
            for element in result.base_idea.unexpected_elements.iter().take(3) {
                println!("    • {}", element.bright_yellow());
            }
        }
    }

    // Analogical insights
    if !result.analogical_insights.is_empty() {
        println!("\n{}", "🔗 Analogical Insights:".bright_green().bold());
        for insight in result.analogical_insights.iter().take(3) {
            println!("  {} {} → {}", 
                "•".bright_white(),
                insight.source_domain.bright_cyan(), 
                insight.target_domain.bright_green()
            );
            println!("    {}", insight.analogy_description.white());
        }
    }

    // Implementation roadmap
    println!("\n{}", "🗺️ Implementation Roadmap:".bright_blue().bold());
    println!("  {} {} weeks", "Duration:".white(), result.implementation_roadmap.total_duration_weeks.to_string().bright_green());
    println!("  {} {}%", "Success Probability:".white(), format!("{:.0}", result.implementation_roadmap.success_probability * 100.0).bright_green());
    
    if !result.implementation_roadmap.critical_path.is_empty() {
        println!("  {} {}", "Critical Path:".white(), result.implementation_roadmap.critical_path.join(" → ").bright_yellow());
    }

    println!("\n{}", "═══════════════════════════════════════════════════════".bright_cyan());

    Ok(())
}

fn format_content_with_boxes(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    
    for line in lines {
        if line.trim().is_empty() {
            result.push('\n');
        } else {
            result.push_str(&format!("  📝 {}\n", line.bright_white()));
        }
    }
    
    result
}

fn display_metrics_bar(label: &str, value: f64) {
    let bar_length = 20;
    let filled = (value * bar_length as f64) as usize;
    let empty = bar_length - filled;
    
    let bar = format!("{}{}",
        "█".repeat(filled).bright_green(),
        "░".repeat(empty).bright_black()
    );
    
    println!("  {} {} {}",
        format!("{:>12}:", label).white(),
        bar,
        format_score(value).bright_green()
    );
}

fn format_score(value: f64) -> String {
    format!("{:.1}%", value * 100.0)
}