#!/bin/bash

# CHOPS Usage Demonstration Script
# This script showcases various command combinations and features of the CHOPS CLI tool
# Usage: ./usage_demo.sh [--dry-run] [--section SECTION] [--help]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color

# Configuration
DRY_RUN=false
SPECIFIC_SECTION=""
CHOPS_CMD="cargo run --"
DELAY_BETWEEN_COMMANDS=2

show_help() {
    cat << EOF
🔮 CHOPS Usage Demonstration Script

This script demonstrates various CHOPS CLI command combinations and features.

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --dry-run           Show commands without executing them
    --section SECTION   Run only a specific section (see sections below)
    --help             Show this help message

SECTIONS:
    basic              Basic summon commands with different personas
    chaos              Chaos level demonstrations (1, 3, 5, 7, 11)
    domains            Various domain examples
    advanced           Advanced parameter combinations
    other-commands     Non-summon commands (mutate, prophecy, etc.)
    interactive        Interactive mode demonstration
    config             Configuration and memory management
    error-handling     Error scenarios and edge cases

EXAMPLES:
    $0                          # Run all demonstrations
    $0 --dry-run               # Show all commands without executing
    $0 --section basic         # Run only basic persona demonstrations
    $0 --section chaos --dry-run  # Show chaos level commands without executing

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --section)
            SPECIFIC_SECTION="$2"
            shift 2
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Utility functions
print_header() {
    echo -e "\n${WHITE}======================================================================${NC}"
    echo -e "${WHITE}  $1${NC}"
    echo -e "${WHITE}======================================================================${NC}\n"
}

print_section() {
    echo -e "\n${CYAN}--- $1 ---${NC}\n"
}

print_command() {
    echo -e "${YELLOW}Command:${NC} ${GREEN}$1${NC}"
}

print_description() {
    echo -e "${BLUE}Description:${NC} $1"
}

print_expected() {
    echo -e "${PURPLE}Expected:${NC} $1"
}

execute_command() {
    local cmd="$1"
    local description="$2"
    local expected="$3"
    
    print_command "$cmd"
    print_description "$description"
    if [[ -n "$expected" ]]; then
        print_expected "$expected"
    fi
    
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}[DRY RUN]${NC} Command would be executed here"
    else
        echo -e "${WHITE}Executing...${NC}"
        if eval "$cmd"; then
            echo -e "${GREEN}✓ Command completed${NC}"
        else
            echo -e "${RED}✗ Command failed (exit code: $?)${NC}"
        fi
        sleep $DELAY_BETWEEN_COMMANDS
    fi
    echo
}

# Main demonstration functions
demo_basic_personas() {
    print_section "Basic Persona Demonstrations"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain software --chaos 5" \
        "Mad Scientist persona for software innovation" \
        "Boundary-pushing, ethically flexible suggestions with high creativity"
    
    execute_command \
        "$CHOPS_CMD summon --persona zen-master --domain design --chaos 3" \
        "Zen Master approach to design challenges" \
        "Balanced, harmonious solutions with philosophical depth"
    
    execute_command \
        "$CHOPS_CMD summon --persona punk-hacker --domain security --chaos 7" \
        "Punk Hacker perspective on security" \
        "Unconventional, rebellious approaches to security challenges"
    
    execute_command \
        "$CHOPS_CMD summon --persona empathetic-ai --domain healthcare --chaos 2" \
        "Empathetic AI for healthcare solutions" \
        "Human-centered, ethical solutions with high empathy"
    
    execute_command \
        "$CHOPS_CMD summon --persona chaos-engineer --domain infrastructure --chaos 9" \
        "Chaos Engineer for infrastructure resilience" \
        "Stress-testing approaches with controlled chaos injection"
    
    execute_command \
        "$CHOPS_CMD summon --persona time-traveler --domain research --chaos 6" \
        "Time Traveler perspective on research" \
        "Cross-temporal insights mixing past and future concepts"
    
    execute_command \
        "$CHOPS_CMD summon --persona mind-reader --domain psychology --chaos 4" \
        "Mind Reader approach to psychological insights" \
        "Deep psychological understanding with intuitive solutions"
}

demo_chaos_levels() {
    print_section "Chaos Level Progression (1-11)"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain testing --chaos 1" \
        "Chaos Level 1: Gentle nudge" \
        "Conservative, safe suggestions with minimal risk"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain testing --chaos 3" \
        "Chaos Level 3: Creative spark" \
        "Moderate creativity with some unconventional ideas"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain testing --chaos 5" \
        "Chaos Level 5: Wild ideas" \
        "Significantly creative with boundary-pushing concepts"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain testing --chaos 7" \
        "Chaos Level 7: Reality bending" \
        "Highly unconventional with reality-distortion elements"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain testing --chaos 11" \
        "Chaos Level 11: Transcendent chaos (MAXIMUM)" \
        "Impossible combinations that transcend normal constraints"
}

demo_domains() {
    print_section "Various Domain Examples"
    
    execute_command \
        "$CHOPS_CMD summon --persona chaos-engineer --domain \"web development\" --chaos 5" \
        "Web development with chaos engineering mindset" \
        "Resilient web architectures with chaos testing"
    
    execute_command \
        "$CHOPS_CMD summon --persona zen-master --domain \"user experience\" --chaos 3" \
        "UX design with zen philosophy" \
        "Minimalist, intuitive user experiences"
    
    execute_command \
        "$CHOPS_CMD summon --persona punk-hacker --domain \"mobile apps\" --chaos 6" \
        "Mobile app development with punk attitude" \
        "Disruptive mobile solutions challenging conventions"
    
    execute_command \
        "$CHOPS_CMD summon --persona time-traveler --domain \"artificial intelligence\" --chaos 7" \
        "AI development with temporal perspective" \
        "AI solutions inspired by both historical and futuristic concepts"
    
    execute_command \
        "$CHOPS_CMD summon --persona empathetic-ai --domain \"accessibility\" --chaos 2" \
        "Accessibility solutions with empathetic approach" \
        "Inclusive, human-centered accessibility innovations"
    
    execute_command \
        "$CHOPS_CMD summon --persona mind-reader --domain \"data visualization\" --chaos 4" \
        "Data visualization with psychological insights" \
        "Intuitive visualizations that reveal hidden patterns"
}

demo_advanced_parameters() {
    print_section "Advanced Parameter Combinations"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain blockchain --chaos 8 --timeline \"2030\" --vibe \"cyberpunk debugging\"" \
        "Advanced summon with timeline and vibe parameters" \
        "Futuristic blockchain solutions with cyberpunk aesthetics"
    
    execute_command \
        "$CHOPS_CMD summon --persona zen-master --domain meditation --chaos 2 --timeline \"ancient wisdom\" --vibe \"digital minimalism\"" \
        "Zen approach with temporal and aesthetic context" \
        "Ancient wisdom applied to modern digital meditation"
    
    execute_command \
        "$CHOPS_CMD summon --persona chaos-engineer --domain microservices --chaos 9 --constraints \"budget-limited\" \"kubernetes-only\" --reality-level 0.8" \
        "Complex summon with constraints and reality calibration" \
        "Practical microservices chaos with real-world constraints"
    
    execute_command \
        "$CHOPS_CMD summon --persona time-traveler --domain gaming --chaos 7 --timeline \"retro-futurism\" --vibe \"nostalgia meets innovation\"" \
        "Gaming innovation with retro-futuristic approach" \
        "Games that blend nostalgic elements with futuristic concepts"
    
    execute_command \
        "$CHOPS_CMD summon --persona punk-hacker --domain privacy --chaos 10 --constraints \"no-surveillance\" \"decentralized\" --reality-level 0.6" \
        "Privacy solutions with punk ethics and constraints" \
        "Radical privacy approaches challenging surveillance capitalism"
}

demo_other_commands() {
    print_section "Non-Summon Commands"
    
    # Note: These commands may require actual files or different setup
    execute_command \
        "$CHOPS_CMD mutate --file \"example.rs\" --direction \"more-functional\" --personality --weird" \
        "Code mutation with personality injection" \
        "Transform code to be more functional with personality quirks"
    
    execute_command \
        "$CHOPS_CMD prophecy --year 2030 --domain \"quantum computing\" --trend-analysis --emerging-tech" \
        "Future prophecy about quantum computing" \
        "Predictions about quantum computing developments by 2030"
    
    execute_command \
        "$CHOPS_CMD collaborate --mode debate --topic \"AI ethics\" --human" \
        "AI collaboration on ethics debate" \
        "Multi-AI debate on AI ethics with human moderation"
    
    execute_command \
        "$CHOPS_CMD glitch --probability 0.3 --personality --density medium" \
        "Controlled chaos injection with personality" \
        "Inject medium-density personality-driven glitches"
    
    execute_command \
        "$CHOPS_CMD time-travel --era \"1990s\" --twist \"modern cloud computing\"" \
        "Time travel to 1990s with modern twist" \
        "Reimagine 1990s tech with cloud computing concepts"
    
    execute_command \
        "$CHOPS_CMD possession --ghost \"steve-jobs\" --target \"mobile interface design\"" \
        "Channel Steve Jobs for interface design" \
        "Design mobile interfaces with Jobs' design philosophy"
    
    execute_command \
        "$CHOPS_CMD paradox --constraints \"fast and slow\" \"simple and complex\" \"cheap and premium\"" \
        "Resolve contradictory design constraints" \
        "Solutions that embrace and resolve paradoxical requirements"
}

demo_configuration() {
    print_section "Configuration and Memory Management"
    
    execute_command \
        "$CHOPS_CMD config --show" \
        "Display current CHOPS configuration" \
        "Show API keys, default settings, and preferences"
    
    execute_command \
        "$CHOPS_CMD config --set \"default_chaos_level=7\" --set \"reality_distortion_enabled=true\"" \
        "Update configuration settings" \
        "Modify default chaos level and enable reality distortion"
    
    execute_command \
        "$CHOPS_CMD memory --show" \
        "Display current memory contents" \
        "Show stored patterns, ideas, and learning history"
    
    execute_command \
        "$CHOPS_CMD memory --export \"backup_$(date +%Y%m%d).json\"" \
        "Export memory to backup file" \
        "Create timestamped backup of CHOPS memory"
}

demo_interactive_mode() {
    print_section "Interactive Mode (Simulated)"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        execute_command \
            "$CHOPS_CMD interactive" \
            "Enter interactive CHOPS mode" \
            "Interactive menu with summon, mutate, prophecy, and other options"
    else
        echo -e "${YELLOW}Note:${NC} Interactive mode requires user input and cannot be fully automated."
        echo -e "${BLUE}To try interactive mode manually, run:${NC} ${GREEN}$CHOPS_CMD interactive${NC}"
        echo -e "${BLUE}This provides a menu-driven interface for all CHOPS features.${NC}"
        echo
    fi
}

demo_error_handling() {
    print_section "Error Handling and Edge Cases"
    
    execute_command \
        "$CHOPS_CMD summon --persona invalid-persona --domain software --chaos 5" \
        "Invalid persona type" \
        "Should display error about unknown persona type"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain software --chaos 15" \
        "Invalid chaos level (too high)" \
        "Should display error about chaos level range (1-11)"
    
    execute_command \
        "$CHOPS_CMD summon --persona mad-scientist --domain software --chaos 0" \
        "Invalid chaos level (too low)" \
        "Should display error about chaos level range (1-11)"
    
    execute_command \
        "$CHOPS_CMD mutate --file \"nonexistent.rs\" --direction \"better\"" \
        "Mutate non-existent file" \
        "Should display error about file not found"
    
    execute_command \
        "$CHOPS_CMD collaborate --mode invalid-mode --topic \"test\"" \
        "Invalid collaboration mode" \
        "Should display error about unknown collaboration mode"
}

# Main execution logic
main() {
    print_header "🔮 CHOPS CLI Usage Demonstration"
    
    echo -e "${WHITE}This script demonstrates various CHOPS CLI commands and features.${NC}"
    echo -e "${WHITE}Each command shows the syntax, description, and expected behavior.${NC}"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "\n${YELLOW}Running in DRY-RUN mode - commands will be displayed but not executed.${NC}"
    else
        echo -e "\n${GREEN}Running in LIVE mode - commands will be executed.${NC}"
        echo -e "${RED}Note: Some commands may fail if CHOPS is not properly configured (e.g., missing API keys).${NC}"
    fi
    
    echo -e "\n${CYAN}Sections to demonstrate:${NC}"
    echo -e "  • Basic persona demonstrations (7 personas)"
    echo -e "  • Chaos level progression (1, 3, 5, 7, 11)"
    echo -e "  • Various domain examples"
    echo -e "  • Advanced parameter combinations"
    echo -e "  • Other commands (mutate, prophecy, collaborate, etc.)"
    echo -e "  • Configuration and memory management"
    echo -e "  • Interactive mode overview"
    echo -e "  • Error handling scenarios"
    
    read -p "Press Enter to continue or Ctrl+C to exit..."
    
    # Run specific section or all sections
    case "$SPECIFIC_SECTION" in
        "basic")
            demo_basic_personas
            ;;
        "chaos")
            demo_chaos_levels
            ;;
        "domains")
            demo_domains
            ;;
        "advanced")
            demo_advanced_parameters
            ;;
        "other-commands")
            demo_other_commands
            ;;
        "interactive")
            demo_interactive_mode
            ;;
        "config")
            demo_configuration
            ;;
        "error-handling")
            demo_error_handling
            ;;
        "")
            # Run all sections
            demo_basic_personas
            demo_chaos_levels
            demo_domains
            demo_advanced_parameters
            demo_other_commands
            demo_configuration
            demo_interactive_mode
            demo_error_handling
            ;;
        *)
            echo -e "${RED}Error: Unknown section '$SPECIFIC_SECTION'${NC}"
            echo "Available sections: basic, chaos, domains, advanced, other-commands, interactive, config, error-handling"
            exit 1
            ;;
    esac
    
    print_header "🎉 CHOPS Usage Demonstration Complete"
    
    echo -e "${GREEN}Summary:${NC}"
    echo -e "• Demonstrated all 7 persona types with appropriate use cases"
    echo -e "• Showed chaos level progression from 1 (gentle) to 11 (transcendent)"
    echo -e "• Covered diverse domains: software, design, security, healthcare, etc."
    echo -e "• Illustrated advanced parameter combinations (timeline, vibe, constraints)"
    echo -e "• Showcased non-summon commands: mutate, prophecy, collaborate, glitch, etc."
    echo -e "• Demonstrated configuration and memory management"
    echo -e "• Covered error handling scenarios"
    
    echo -e "\n${WHITE}For more information:${NC}"
    echo -e "• Run ${GREEN}$CHOPS_CMD --help${NC} for command overview"
    echo -e "• Run ${GREEN}$CHOPS_CMD <command> --help${NC} for specific command help"
    echo -e "• Try ${GREEN}$CHOPS_CMD interactive${NC} for menu-driven interface"
    echo -e "• Check ${GREEN}examples/README.md${NC} for detailed documentation"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "\n${YELLOW}To run commands for real, execute this script without --dry-run${NC}"
    fi
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi