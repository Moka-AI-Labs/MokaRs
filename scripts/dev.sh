#!/bin/bash
# Development helper script for MokaRs

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

echo_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

echo_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo_error "$1 is not installed. Please install it first."
        exit 1
    fi
}

# Function to install cargo tools if not present
install_tool() {
    local tool=$1
    local package=${2:-$tool}
    
    if ! command -v $tool &> /dev/null; then
        echo_info "Installing $tool..."
        cargo install $package
    else
        echo_info "$tool is already installed"
    fi
}

# Main development setup
setup() {
    echo_info "Setting up development environment..."
    
    # Check for required tools
    check_command "cargo"
    check_command "rustc"
    
    # Install development tools
    install_tool "cargo-audit"
    install_tool "cargo-outdated"
    install_tool "cargo-tarpaulin"
    
    echo_success "Development environment setup complete!"
}

# Format code
format() {
    echo_info "Formatting code..."
    cargo fmt --all
    echo_success "Code formatting complete!"
}

# Run linter
lint() {
    echo_info "Running clippy lints..."
    cargo clippy --all-targets --all-features -- -D warnings
    echo_success "Linting complete!"
}

# Run tests
test() {
    echo_info "Running tests..."
    cargo test --all-features --verbose
    echo_success "Tests complete!"
}

# Run security audit
audit() {
    echo_info "Running security audit..."
    cargo audit
    echo_success "Security audit complete!"
}

# Check for outdated dependencies
outdated() {
    echo_info "Checking for outdated dependencies..."
    cargo outdated
}

# Generate coverage report
coverage() {
    echo_info "Generating coverage report..."
    cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out html
    echo_success "Coverage report generated in tarpaulin-report.html"
}

# Build project
build() {
    echo_info "Building project..."
    cargo build --release
    echo_success "Build complete!"
}

# Run the application
run() {
    echo_info "Running application..."
    cargo run
}

# Full pipeline check (what CI runs)
check() {
    echo_info "Running full pipeline check..."
    
    format
    lint
    test
    audit
    build
    
    echo_success "All checks passed! ✨"
}

# Help function
help() {
    echo "MokaRs Development Script"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  setup      - Set up development environment"
    echo "  format     - Format code with rustfmt"
    echo "  lint       - Run clippy lints"
    echo "  test       - Run all tests"
    echo "  audit      - Run security audit"
    echo "  outdated   - Check for outdated dependencies"
    echo "  coverage   - Generate test coverage report"
    echo "  build      - Build the project in release mode"
    echo "  run        - Run the application"
    echo "  check      - Run all checks (format, lint, test, audit, build)"
    echo "  help       - Show this help message"
}

# Main script logic
case "${1:-help}" in
    setup)
        setup
        ;;
    format)
        format
        ;;
    lint)
        lint
        ;;
    test)
        test
        ;;
    audit)
        audit
        ;;
    outdated)
        outdated
        ;;
    coverage)
        coverage
        ;;
    build)
        build
        ;;
    run)
        run
        ;;
    check)
        check
        ;;
    help|*)
        help
        ;;
esac