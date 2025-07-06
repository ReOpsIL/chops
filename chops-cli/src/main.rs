mod cli;
mod commands;
mod output;
mod interactive;

use chops_core::{CHOPSConfig, CHOPSResult, MemorySystem};
use chops_api::{ClaudeClient, CognitiveArchitecture};
use cli::Cli;
use clap::Parser;
use colored::*;
use std::process;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().init();
    
    tracing::info!("CHOPS CLI starting up");

    // Parse command line arguments
    tracing::debug!("Parsing command line arguments");
    let cli = Cli::parse();
    tracing::debug!("Command line arguments parsed successfully");

    // Load configuration
    tracing::debug!("Loading configuration");
    let mut config = match load_configuration().await {
        Ok(config) => {
            tracing::info!("Configuration loaded successfully");
            config
        },
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            eprintln!("{}", format!("❌ Failed to load configuration: {}", e).red());
            process::exit(1);
        }
    };

    // Initialize CHOPS system
    tracing::debug!("Initializing CHOPS system");
    let mut chops_system = match initialize_chops_system(&mut config).await {
        Ok(system) => {
            tracing::info!("CHOPS system initialized successfully");
            system
        },
        Err(e) => {
            tracing::error!("Failed to initialize CHOPS: {}", e);
            eprintln!("{}", format!("❌ Failed to initialize CHOPS: {}", e).red());
            process::exit(1);
        }
    };

    // Welcome message
    print_welcome_banner();

    // Execute command
    tracing::debug!("Executing command");
    if let Err(e) = execute_command(cli, &mut chops_system).await {
        tracing::error!("Command execution failed: {}", e);
        eprintln!("{}", format!("❌ Command execution failed: {}", e).red());
        process::exit(1);
    }
    
    tracing::info!("CHOPS CLI execution completed successfully");
}

#[tracing::instrument(name = "load_configuration", level = "info")]
async fn load_configuration() -> CHOPSResult<CHOPSConfig> {
    tracing::info!("Loading CHOPS configuration");
    
    let config_path = CHOPSConfig::get_config_path();
    tracing::debug!("Configuration path: {}", config_path.display());
    
    let mut config = CHOPSConfig::load_from_file(&config_path)?;
    
    // Merge with environment variables
    tracing::debug!("Merging configuration with environment variables");
    config.merge_with_env();
    
    tracing::info!("Configuration loaded successfully from: {}", config_path.display());
    Ok(config)
}

#[tracing::instrument(name = "initialize_chops_system", level = "info", skip(config))]
async fn initialize_chops_system(config: &mut CHOPSConfig) -> CHOPSResult<CHOPSSystem> {
    tracing::info!("Initializing CHOPS system components");
    
    // Get Claude API key
    tracing::debug!("Retrieving Claude API key from configuration");
    let api_key = config.get_claude_api_key()?.to_string();
    
    // Initialize Claude client
    tracing::debug!("Initializing Claude client");
    let claude_client = ClaudeClient::new(api_key)?;
    
    // Initialize cognitive architecture
    tracing::debug!("Initializing cognitive architecture");
    let cognitive_architecture = CognitiveArchitecture::new(claude_client);
    
    // Load memory system
    let memory_path = std::path::PathBuf::from(".")
        .join("chops")
        .join("memory.json");
    
    tracing::debug!("Loading memory system from: {}", memory_path.display());
    let memory_system = MemorySystem::load_from_file(&memory_path)
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to load memory system, creating new one: {}", e);
            MemorySystem::new()
        });
    
    tracing::info!("CHOPS system initialized successfully with all components");
    
    Ok(CHOPSSystem {
        cognitive_architecture,
        memory_system,
        config: config.clone(),
        memory_path,
    })
}

async fn execute_command(cli: Cli, system: &mut CHOPSSystem) -> CHOPSResult<()> {
    use cli::Commands;
    
    match cli.command {
        Commands::Summon { persona, domain, chaos, timeline, vibe, constraints, reality_level } => {
            commands::summon::execute(
                system,
                persona.unwrap_or_default(),
                domain,
                chaos.unwrap_or(5),
                timeline,
                vibe,
                constraints,
                reality_level.unwrap_or(0.7),
            ).await
        },
        
        Commands::Mutate { file, direction, personality, easter_eggs, weird, functional } => {
            commands::mutate::execute(
                system,
                file,
                direction.unwrap_or_else(|| "creative".to_string()),
                personality,
                easter_eggs,
                weird,
                functional,
            ).await
        },
        
        Commands::Prophecy { year, domain, trend_analysis, emerging_tech, what_if } => {
            commands::prophecy::execute(
                system,
                year,
                domain,
                trend_analysis,
                emerging_tech,
                what_if,
            ).await
        },
        
        Commands::Collaborate { mode, topic, human } => {
            commands::collaborate::execute(
                system,
                mode,
                topic,
                human,
            ).await
        },
        
        Commands::Glitch { probability, personality, density } => {
            commands::glitch::execute(
                system,
                probability.unwrap_or(0.1),
                personality,
                density.unwrap_or_else(|| "medium".to_string()),
            ).await
        },
        
        Commands::TimeTravel { era, twist } => {
            commands::time_travel::execute(
                system,
                era,
                twist,
            ).await
        },
        
        Commands::Possession { ghost, target } => {
            commands::possession::execute(
                system,
                ghost,
                target,
            ).await
        },
        
        Commands::Paradox { constraints } => {
            commands::paradox::execute(
                system,
                constraints,
            ).await
        },
        
        Commands::Interactive => {
            interactive::run_interactive_mode(system).await
        },
        
        Commands::Config { show, set } => {
            commands::config::execute(
                system,
                show,
                set,
            ).await
        },
        
        Commands::Memory { show, clear, export } => {
            commands::memory::execute(
                system,
                show,
                clear,
                export,
            ).await
        },
    }
}

fn print_welcome_banner() {
    println!("{}", "
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  🔮 ██████╗██╗  ██╗ ██████╗ ██████╗ ███████╗                  │
│     ██╔══██╗██║  ██║██╔═══██╗██╔══██╗██╔════╝                  │
│     ██║  ██║███████║██║   ██║██████╔╝███████╗                  │
│     ██║  ██║██╔══██║██║   ██║██╔═══╝ ╚════██║                  │
│     ██████╔╝██║  ██║╚██████╔╝██║     ███████║                  │
│     ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚══════╝                  │
│                                                                 │
│      🌟 THE AI CODE WHISPERER 🌟                               │
│   Reality-Bending Innovation Engine for Maverick Developers    │
│                                                                 │
│   \"Where madness meets method, chaos births order,             │
│    and the impossible becomes inevitable.\"                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
".bright_cyan());

    println!("{}", "Welcome to CHOPS! Type 'chops --help' for command options.".bright_green());
    println!();
}

pub struct CHOPSSystem {
    cognitive_architecture: CognitiveArchitecture,
    memory_system: MemorySystem,
    config: CHOPSConfig,
    memory_path: std::path::PathBuf,
}

impl CHOPSSystem {
    pub async fn save_memory(&self) -> CHOPSResult<()> {
        self.memory_system.save_to_file(&self.memory_path)
    }
}