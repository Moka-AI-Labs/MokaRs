# MokaRs

[![CI Pipeline](https://github.com/Moka-AI-Labs/MokaRs/actions/workflows/ci.yml/badge.svg)](https://github.com/Moka-AI-Labs/MokaRs/actions/workflows/ci.yml)
[![Release Pipeline](https://github.com/Moka-AI-Labs/MokaRs/actions/workflows/release.yml/badge.svg)](https://github.com/Moka-AI-Labs/MokaRs/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/Moka-AI-Labs/MokaRs/branch/main/graph/badge.svg)](https://codecov.io/gh/Moka-AI-Labs/MokaRs)

A Rust project with a comprehensive CI/CD pipeline demonstrating best practices for code quality, testing, and deployment automation.

## Features

- **Task Management**: Simple task management system with async operations
- **Comprehensive CI/CD**: Multi-platform builds, testing, and deployment
- **Code Quality**: Automated formatting, linting, and security auditing
- **Documentation**: Automated documentation generation and deployment

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70.0 or later)
- [Git](https://git-scm.com/)

### Installation

```bash
# Clone the repository
git clone https://github.com/Moka-AI-Labs/MokaRs.git
cd MokaRs

# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test
```

### Development

We provide a convenient development script to help with common tasks:

```bash
# Set up development environment
./scripts/dev.sh setup

# Format code
./scripts/dev.sh format

# Run lints
./scripts/dev.sh lint

# Run tests
./scripts/dev.sh test

# Run security audit
./scripts/dev.sh audit

# Build release version
./scripts/dev.sh build

# Run full pipeline check (what CI runs)
./scripts/dev.sh check
```

## Code Pipeline

Our comprehensive CI/CD pipeline includes:

### Continuous Integration (CI)

The CI pipeline runs on every push and pull request, performing:

1. **Code Formatting Check** (`cargo fmt`)
   - Ensures consistent code style across the project
   - Uses rustfmt with custom styling rules

2. **Linting** (`cargo clippy`)
   - Static code analysis to catch common mistakes
   - Enforces best practices and idiomatic Rust code
   - Treats warnings as errors to maintain code quality

3. **Multi-Platform Testing**
   - Tests on Ubuntu, Windows, and macOS
   - Tests against stable and beta Rust versions
   - Runs unit tests and integration tests
   - Tests with all feature flags enabled

4. **Security Audit** (`cargo audit`)
   - Checks dependencies for known security vulnerabilities
   - Runs weekly on schedule to catch new issues
   - Fails the build if critical vulnerabilities are found

5. **Code Coverage**
   - Generates test coverage reports using `cargo tarpaulin`
   - Uploads coverage data to Codecov
   - Tracks coverage trends over time

6. **Documentation Generation**
   - Builds and deploys documentation to GitHub Pages
   - Ensures all public APIs are documented
   - Only runs on main branch pushes

7. **Dependency Management**
   - Checks for outdated dependencies
   - Provides notifications for available updates

### Release Pipeline

The release pipeline triggers on git tags and includes:

1. **Multi-Platform Builds**
   - Linux (x86_64, including musl)
   - Windows (x86_64)
   - macOS (x86_64 and ARM64)

2. **Release Artifacts**
   - Pre-compiled binaries for all supported platforms
   - Packaged as tar.gz (Unix) or zip (Windows)
   - Automatic GitHub release creation

3. **Changelog Generation**
   - Automatically generates changelogs from git commits
   - Links to GitHub releases for easy tracking

4. **Registry Publishing**
   - Optional publishing to crates.io registry
   - Controlled via GitHub secrets

### Development Tools Configuration

- **rustfmt.toml**: Code formatting configuration
- **.clippy.toml**: Linting rules and settings
- **.gitignore**: Comprehensive ignore rules for Rust projects
- **scripts/dev.sh**: Development helper script

### Pipeline Features

- **Caching**: Intelligent caching of dependencies to speed up builds
- **Matrix Testing**: Tests across multiple OS and Rust versions
- **Security**: Automated security auditing and vulnerability scanning
- **Quality Gates**: Prevents merges if quality checks fail
- **Documentation**: Automated API documentation generation
- **Release Automation**: One-click releases with multi-platform binaries

## Project Structure

```
MokaRs/
├── .github/
│   └── workflows/          # CI/CD pipeline definitions
│       ├── ci.yml         # Continuous integration pipeline
│       └── release.yml    # Release and deployment pipeline
├── scripts/
│   └── dev.sh            # Development helper script
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library root with tests
│   └── task_manager.rs   # Task management implementation
├── Cargo.toml            # Project configuration and dependencies
├── rustfmt.toml          # Code formatting configuration
├── .clippy.toml          # Linting configuration
└── .gitignore           # Git ignore rules
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the development checks (`./scripts/dev.sh check`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Quality Standards

- All code must pass formatting checks (`cargo fmt`)
- All code must pass linting without warnings (`cargo clippy`)
- All tests must pass (`cargo test`)
- Security audit must pass (`cargo audit`)
- Code coverage should not decrease significantly

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CI/CD powered by [GitHub Actions](https://github.com/features/actions)
- Code coverage by [Codecov](https://codecov.io/)
- Documentation hosted on [GitHub Pages](https://pages.github.com/)
