use chops_core::PersonaType;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "chops",
    version = "0.1.0",
    about = "🔮 CHOPS: The AI Code Whisperer - Reality-Bending Innovation Engine",
    long_about = "CHOPS is a reality-bending innovation engine that uses AI personas, chaos mathematics, and cognitive architecture to generate wild but implementable ideas for maverick developers."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 🔮 Summon creative demons to generate innovative ideas
    #[command(alias = "s")]
    Summon {
        /// AI persona to invoke
        #[arg(short, long, value_enum)]
        persona: Option<PersonaType>,

        /// Domain to focus on
        #[arg(short, long, default_value = "software")]
        domain: String,

        /// Chaos level (1-11, where 11 breaks reality)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=11))]
        chaos: Option<u8>,

        /// Timeline context (e.g., "2030", "retro-futurism")
        #[arg(short, long)]
        timeline: Option<String>,

        /// Vibe to channel (e.g., "cyberpunk debugging")
        #[arg(short, long)]
        vibe: Option<String>,

        /// Constraints to work within
        #[arg(long)]
        constraints: Vec<String>,

        /// Reality calibration level (0.0-1.0)
        #[arg(short, long)]
        reality_level: Option<f64>,
    },

    /// 🧬 Mutate existing code with personality injection
    #[command(alias = "m")]
    Mutate {
        /// File to mutate
        #[arg(short, long)]
        file: String,

        /// Direction of mutation
        #[arg(short, long)]
        direction: Option<String>,

        /// Inject personality
        #[arg(short, long)]
        personality: bool,

        /// Add easter eggs
        #[arg(short, long)]
        easter_eggs: bool,

        /// Make it weird
        #[arg(short, long)]
        weird: bool,

        /// Keep it functional
        #[arg(long)]
        functional: bool,
    },

    /// 🔮 Generate future prophecies and predictions
    #[command(alias = "p")]
    Prophecy {
        /// Target year for prophecy
        #[arg(short, long)]
        year: Option<u32>,

        /// Domain to prophesy about
        #[arg(short, long, default_value = "technology")]
        domain: String,

        /// Include trend analysis
        #[arg(short, long)]
        trend_analysis: bool,

        /// Focus on emerging tech
        #[arg(short, long)]
        emerging_tech: bool,

        /// What-if scenario
        #[arg(short, long)]
        what_if: Option<String>,
    },

    /// 🤝 Orchestrate AI collaboration and debates
    #[command(alias = "c")]
    Collaborate {
        /// Collaboration mode
        #[arg(short, long, value_enum, default_value = "debate")]
        mode: CollaborationMode,

        /// Topic to collaborate on
        #[arg(short, long)]
        topic: String,

        /// Include human in the loop
        #[arg(long)]
        human: bool,
    },

    /// ⚡ Inject controlled chaos and glitches
    #[command(alias = "g")]
    Glitch {
        /// Probability of glitch injection
        #[arg(short, long)]
        probability: Option<f64>,

        /// Make glitches sentient
        #[arg(long)]
        personality: bool,

        /// Glitch density (low/medium/high/extreme)
        #[arg(short, long)]
        density: Option<String>,
    },

    /// ⏰ Travel through technological eras
    #[command(alias = "tt")]
    TimeTravel {
        /// Era to visit (e.g., "1990s", "2030s", "retro-future")
        #[arg(short, long)]
        era: String,

        /// Modern twist to apply
        #[arg(short, long)]
        twist: Option<String>,
    },

    /// 👻 Channel famous innovators and visionaries
    #[command(alias = "pos")]
    Possession {
        /// Ghost to channel (e.g., "steve-jobs", "tesla", "ada-lovelace")
        #[arg(short, long)]
        ghost: String,

        /// Target to possess/influence
        #[arg(short, long)]
        target: String,
    },

    /// 🌀 Embrace contradictions and paradoxes
    #[command(alias = "par")]
    Paradox {
        /// Contradictory constraints to reconcile
        #[arg(short, long)]
        constraints: Vec<String>,
    },

    /// 🎮 Enter interactive CHOPS mode
    #[command(alias = "i")]
    Interactive,

    /// ⚙️ Configure CHOPS settings
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,

        /// Set configuration value (key=value)
        #[arg(short = 's', long)]
        set: Vec<String>,
    },

    /// 🧠 Manage CHOPS memory and learning
    Memory {
        /// Show memory contents
        #[arg(short, long)]
        show: bool,

        /// Clear memory
        #[arg(short, long)]
        clear: bool,

        /// Export memory to file
        #[arg(short, long)]
        export: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum CollaborationMode {
    /// AI vs AI debate
    Debate,
    /// Collaborative brainstorming
    Brainstorm,
    /// Consensus building
    Consensus,
    /// Devil's advocate mode
    DevilsAdvocate,
    /// Synthesis of ideas
    Synthesis,
}

impl std::fmt::Display for CollaborationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollaborationMode::Debate => write!(f, "debate"),
            CollaborationMode::Brainstorm => write!(f, "brainstorm"),
            CollaborationMode::Consensus => write!(f, "consensus"),
            CollaborationMode::DevilsAdvocate => write!(f, "devils-advocate"),
            CollaborationMode::Synthesis => write!(f, "synthesis"),
        }
    }
}