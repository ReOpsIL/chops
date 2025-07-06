use crate::CHOPSSystem;
use chops_core::{PersonaType, CHOPSResult, CHOPSError};
use colored::*;
use dialoguer::{Select, Input, Confirm, MultiSelect};

#[tracing::instrument(name = "run_interactive_mode", level = "info", skip(system))]
pub async fn run_interactive_mode(system: &mut CHOPSSystem) -> CHOPSResult<()> {
    tracing::info!("Starting interactive CHOPS mode");
    
    println!("{}", "🎮 Welcome to Interactive CHOPS Mode!".bright_cyan().bold());
    println!("{}", "   Reality-bending at your fingertips...".bright_blue());
    
    loop {
        println!("\n{}", "─".repeat(50).bright_black());
        
        let actions = vec![
            "🔮 Summon an idea",
            "🧬 Mutate existing code", 
            "🔮 Generate prophecy",
            "🤝 Start AI collaboration",
            "⚡ Inject chaos glitch",
            "🧠 View memory",
            "⚙️ Configure settings",
            "🚪 Exit"
        ];
        
        tracing::debug!("Presenting interactive menu with {} options", actions.len());
        
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&actions)
            .default(0)
            .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?;
            
        tracing::debug!("User selected option: {} ({})", selection, actions[selection]);
            
        match selection {
            0 => {
                tracing::info!("Starting interactive summon");
                interactive_summon(system).await?
            },
            1 => {
                tracing::info!("Starting interactive mutate");
                interactive_mutate(system).await?
            },
            2 => {
                tracing::info!("Starting interactive prophecy");
                interactive_prophecy(system).await?
            },
            3 => {
                tracing::info!("Starting interactive collaboration");
                interactive_collaborate(system).await?
            },
            4 => {
                tracing::info!("Starting interactive glitch");
                interactive_glitch(system).await?
            },
            5 => {
                tracing::info!("Viewing memory");
                interactive_memory(system).await?
            },
            6 => {
                tracing::info!("Configuring settings");
                interactive_config(system).await?
            },
            7 => {
                tracing::info!("User exiting interactive mode");
                println!("{}", "👋 Reality returns to normal. Goodbye!".bright_green());
                break;
            },
            _ => unreachable!(),
        }
    }
    
    tracing::info!("Interactive mode session completed");
    Ok(())
}

#[tracing::instrument(name = "interactive_summon", level = "info", skip(system))]
async fn interactive_summon(system: &mut CHOPSSystem) -> CHOPSResult<()> {
    tracing::info!("Starting interactive idea summoning");
    
    println!("\n{}", "🔮 IDEA SUMMONING RITUAL".bright_cyan().bold());
    
    // Select persona
    let personas = vec![
        "Mad Scientist 🧪",
        "Zen Master 🧘",
        "Punk Hacker 🦾", 
        "Empathetic AI 💝",
        "Chaos Engineer ⚡",
        "Time Traveler ⏰",
        "Mind Reader 🧠"
    ];
    
    tracing::debug!("Presenting persona selection with {} options", personas.len());
    
    let persona_idx = Select::new()
        .with_prompt("Choose your persona")
        .items(&personas)
        .default(0)
        .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?;
        
    let persona = match persona_idx {
        0 => PersonaType::MadScientist,
        1 => PersonaType::ZenMaster,
        2 => PersonaType::PunkHacker,
        3 => PersonaType::EmpatheticAI,
        4 => PersonaType::ChaosEngineer,
        5 => PersonaType::TimeTraveler,
        6 => PersonaType::MindReader,
        _ => PersonaType::MadScientist,
    };
    
    tracing::debug!("Selected persona: {:?}", persona);
    
    // Get domain
    let domain: String = Input::new()
        .with_prompt("What domain are you working in?")
        .default("software development".to_string())
        .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?;
    
    // Get chaos level
    let chaos_options = vec!["1 - Gentle nudge", "3 - Creative spark", "5 - Wild ideas", "7 - Reality bending", "11 - Transcendent chaos"];
    let chaos_idx = Select::new()
        .with_prompt("Choose chaos level")
        .items(&chaos_options)
        .default(2)
        .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?;
        
    let chaos = match chaos_idx {
        0 => 1,
        1 => 3,
        2 => 5,
        3 => 7,
        4 => 11,
        _ => 5,
    };
    
    // Optional vibe
    let add_vibe = Confirm::new()
        .with_prompt("Add a specific vibe?")
        .default(false)
        .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?;
        
    let vibe = if add_vibe {
        Some(Input::<String>::new()
            .with_prompt("Describe the vibe")
            .interact().map_err(|e| CHOPSError::UnexpectedError(e.to_string()))?)
    } else {
        None
    };
    
    // Execute summon
    crate::commands::summon::execute(
        system,
        persona,
        domain,
        chaos,
        None, // timeline
        vibe,
        vec![], // constraints
        0.7, // reality level
    ).await
}

async fn interactive_mutate(_system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("{}", "🧬 Code mutation coming soon...".bright_yellow());
    Ok(())
}

async fn interactive_prophecy(_system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("{}", "🔮 Prophecy generation coming soon...".bright_yellow());
    Ok(())
}

async fn interactive_collaborate(_system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("{}", "🤝 AI collaboration coming soon...".bright_yellow());
    Ok(())
}

async fn interactive_glitch(_system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("{}", "⚡ Chaos injection coming soon...".bright_yellow());
    Ok(())
}

async fn interactive_memory(system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("\n{}", "🧠 CHOPS MEMORY SYSTEM".bright_blue().bold());
    
    let recent_ideas = system.memory_system.recall_similar_ideas("", 10);
    
    if recent_ideas.is_empty() {
        println!("{}", "📝 No ideas in memory yet. Start summoning some!".bright_yellow());
    } else {
        println!("{}", format!("Found {} ideas in memory:", recent_ideas.len()).bright_green());
        
        for (i, idea) in recent_ideas.iter().enumerate() {
            println!("\n{} {}", 
                format!("{}.", i + 1).bright_cyan(),
                idea.title.bright_white()
            );
            println!("   {} {}", 
                "Persona:".bright_black(),
                format!("{}", idea.persona_used).bright_green()
            );
            println!("   {} {}", 
                "Quality:".bright_black(),
                format!("{}% creative, {}% feasible", 
                    (idea.creativity_score * 100.0) as u32,
                    (idea.feasibility_score * 100.0) as u32
                ).bright_blue()
            );
            if !idea.tags.is_empty() {
                println!("   {} {}", 
                    "Tags:".bright_black(),
                    idea.tags.join(", ").bright_magenta()
                );
            }
        }
    }
    
    Ok(())
}

async fn interactive_config(_system: &mut CHOPSSystem) -> CHOPSResult<()> {
    println!("{}", "⚙️ Configuration options coming soon...".bright_yellow());
    Ok(())
}