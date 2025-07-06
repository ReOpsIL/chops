# CHOPS Examples and Usage Demonstrations

This directory contains comprehensive examples and demonstrations of the CHOPS CLI tool capabilities.

## 🔮 Overview

CHOPS is a reality-bending innovation engine that uses AI personas, chaos mathematics, and cognitive architecture to generate wild but implementable ideas. This examples directory helps you understand and explore all the different ways to use CHOPS.

## 📁 Files in This Directory

- **`usage_demo.sh`** - Comprehensive usage demonstration script
- **`README.md`** - This documentation file
- **`sample_outputs/`** - Directory containing sample command outputs (generated when running demos)

## 🚀 Quick Start

### Run the Full Demo

```bash
# From the project root directory
cd examples
./usage_demo.sh
```

### Dry-Run Mode (Recommended First)

```bash
# See all commands without executing them
./usage_demo.sh --dry-run
```

### Run Specific Sections

```bash
# Run only basic persona demonstrations
./usage_demo.sh --section basic

# Run chaos level demonstrations in dry-run mode
./usage_demo.sh --section chaos --dry-run
```

## 📋 Demo Sections

### 1. Basic Personas (`--section basic`)

Demonstrates all 7 CHOPS personas with appropriate use cases:

- **Mad Scientist** 🧪 - Boundary-pushing, ethically flexible innovations
- **Zen Master** 🧘 - Balanced, harmonious solutions with philosophical depth
- **Punk Hacker** 🦾 - Unconventional, rebellious approaches
- **Empathetic AI** 💝 - Human-centered, ethical solutions
- **Chaos Engineer** ⚡ - Resilience through controlled chaos
- **Time Traveler** ⏰ - Cross-temporal insights and concepts
- **Mind Reader** 🧠 - Deep psychological understanding

### 2. Chaos Levels (`--section chaos`)

Shows the progression of chaos levels and their effects:

- **Level 1** - Gentle nudge: Conservative, safe suggestions
- **Level 3** - Creative spark: Moderate creativity with some unconventional ideas
- **Level 5** - Wild ideas: Significantly creative with boundary-pushing concepts
- **Level 7** - Reality bending: Highly unconventional with reality-distortion elements
- **Level 11** - Transcendent chaos: Impossible combinations that transcend normal constraints

### 3. Domains (`--section domains`)

Various domain examples showing CHOPS versatility:

- Web development
- User experience design
- Mobile app development
- Artificial intelligence
- Accessibility
- Data visualization
- And many more...

### 4. Advanced Parameters (`--section advanced`)

Complex command combinations using:

- **Timeline** parameters (e.g., "2030", "retro-futurism")
- **Vibe** settings (e.g., "cyberpunk debugging", "digital minimalism")
- **Constraints** (e.g., "budget-limited", "kubernetes-only")
- **Reality levels** (0.0-1.0 calibration)

### 5. Other Commands (`--section other-commands`)

Non-summon commands:

- **`mutate`** - Transform existing code with personality injection
- **`prophecy`** - Generate future predictions and trend analysis
- **`collaborate`** - Orchestrate AI debates and brainstorming
- **`glitch`** - Inject controlled chaos and personality glitches
- **`time-travel`** - Visit different technological eras
- **`possession`** - Channel famous innovators
- **`paradox`** - Resolve contradictory constraints

### 6. Configuration (`--section config`)

Configuration and memory management:

- View current configuration
- Modify settings
- Export/import memory
- Manage API keys and preferences

### 7. Interactive Mode (`--section interactive`)

Overview of the interactive menu-driven interface.

### 8. Error Handling (`--section error-handling`)

Common error scenarios and edge cases:

- Invalid persona types
- Out-of-range chaos levels
- Missing files
- Invalid parameters

## 🎯 Command Examples

### Basic Summon Commands

```bash
# Mad scientist approach to software with medium chaos
chops summon --persona mad-scientist --domain software --chaos 5

# Zen master approach to design with low chaos
chops summon --persona zen-master --domain design --chaos 3

# Punk hacker security solutions with high chaos
chops summon --persona punk-hacker --domain security --chaos 7
```

### Advanced Summon Commands

```bash
# Futuristic blockchain with cyberpunk aesthetics
chops summon --persona mad-scientist --domain blockchain --chaos 8 \
  --timeline "2030" --vibe "cyberpunk debugging"

# Constrained microservices chaos engineering
chops summon --persona chaos-engineer --domain microservices --chaos 9 \
  --constraints "budget-limited" "kubernetes-only" --reality-level 0.8

# Ancient wisdom for digital meditation
chops summon --persona zen-master --domain meditation --chaos 2 \
  --timeline "ancient wisdom" --vibe "digital minimalism"
```

### Other Command Examples

```bash
# Mutate code with personality
chops mutate --file "example.rs" --direction "more-functional" --personality --weird

# Predict quantum computing future
chops prophecy --year 2030 --domain "quantum computing" --trend-analysis --emerging-tech

# AI ethics debate
chops collaborate --mode debate --topic "AI ethics" --human

# Inject personality glitches
chops glitch --probability 0.3 --personality --density medium

# 1990s tech with modern cloud concepts
chops time-travel --era "1990s" --twist "modern cloud computing"

# Channel Steve Jobs for interface design
chops possession --ghost "steve-jobs" --target "mobile interface design"

# Resolve contradictory constraints
chops paradox --constraints "fast and slow" "simple and complex"
```

## 🔧 Script Options

### Usage Syntax

```bash
./usage_demo.sh [OPTIONS]
```

### Available Options

- **`--dry-run`** - Show commands without executing them (recommended for first run)
- **`--section SECTION`** - Run only a specific section
- **`--help`** - Show detailed help message

### Available Sections

- `basic` - Basic persona demonstrations
- `chaos` - Chaos level progression
- `domains` - Various domain examples
- `advanced` - Advanced parameter combinations
- `other-commands` - Non-summon commands
- `interactive` - Interactive mode overview
- `config` - Configuration management
- `error-handling` - Error scenarios

## 📊 Expected Outputs

When running the demonstrations (without `--dry-run`), you should expect:

### Successful Commands
- ✅ Detailed AI-generated responses based on persona and parameters
- ✅ Structured output with ideas, reasoning, and implementation suggestions
- ✅ Appropriate personality reflection in the response tone and content

### Error Commands (Intentional)
- ❌ Clear error messages for invalid parameters
- ❌ Helpful suggestions for correction
- ❌ Graceful failure without crashing

### Interactive Commands
- 🎮 Menu-driven interfaces for complex workflows
- 🎮 Step-by-step guided experiences

## 🛠️ Prerequisites

### Required Setup

1. **API Keys** - Configure Claude API key:
   ```bash
   export CLAUDE_API_KEY="your-api-key-here"
   # OR add to ~/.config/chops/config.toml
   ```

2. **Build CHOPS** - Ensure CHOPS is built:
   ```bash
   cargo build
   ```

3. **Permissions** - Make script executable:
   ```bash
   chmod +x usage_demo.sh
   ```

### Optional Setup

- **Output Directory** - Script creates `sample_outputs/` for saving results
- **Logging** - Set `RUST_LOG=info` for detailed execution logs

## 🔍 Troubleshooting

### Common Issues

1. **"Command not found"**
   ```bash
   # Ensure you're in the examples directory
   cd examples
   ./usage_demo.sh
   ```

2. **"Permission denied"**
   ```bash
   chmod +x usage_demo.sh
   ```

3. **API key errors**
   ```bash
   # Set your API key
   export CLAUDE_API_KEY="sk-ant-your-key-here"
   ```

4. **Build errors**
   ```bash
   # Build CHOPS first
   cd .. && cargo build
   ```

### Debugging

- Use `--dry-run` to see commands without executing
- Check `RUST_LOG=debug` for detailed logs
- Run specific sections to isolate issues
- Verify API key configuration with `chops config --show`

## 🎨 Customization

### Modifying the Script

The `usage_demo.sh` script is designed to be easily customizable:

```bash
# Change the delay between commands
DELAY_BETWEEN_COMMANDS=5

# Use different CHOPS executable
CHOPS_CMD="./target/release/chops"

# Add your own demonstration sections
demo_my_custom_section() {
    print_section "My Custom Demonstrations"
    execute_command "chops summon ..." "Description" "Expected result"
}
```

### Adding New Examples

1. Edit `usage_demo.sh`
2. Add your function to the appropriate section
3. Use the `execute_command` helper for consistency
4. Test with `--dry-run` first

## 📚 Additional Resources

- **CHOPS Help** - `chops --help` for command overview
- **Command Help** - `chops <command> --help` for specific help
- **Interactive Mode** - `chops interactive` for guided experience
- **Configuration** - `chops config --show` to view settings

## 🤝 Contributing

To add more examples or improve the demonstrations:

1. Fork the repository
2. Add your examples to `usage_demo.sh`
3. Test thoroughly with `--dry-run`
4. Update this README with new sections
5. Submit a pull request

## 📝 Notes

- The script includes 25+ different command combinations
- Error handling examples are intentionally included
- All persona types and chaos levels are demonstrated
- The script is safe to run multiple times
- Use `--dry-run` to preview without executing

---

*Happy reality-bending with CHOPS! 🔮✨*