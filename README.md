# SpacerFinder

A comprehensive Rust-based suite of tools for finding optimal chains of spacers and imaging train elements with exact length requirements. Designed for optics and microscopy applications where precise spacer configurations are critical.

## Overview

SpacerFinder provides three complementary tools that work together to solve complex spacer chain optimization problems:

- **[Spacer_Finder_CLI](Spacer_Finder_CLI/)** - Core command-line algorithm for finding minimal spacer chains
- **[RigCreator](RigCreator/)** - GUI application for managing required "rig" elements and testing configurations
- **[SpacerManager](SpacerManager/)** - GUI application for managing spacer inventories with duplicate prevention

## Key Features

- **Optimal Chain Finding**: Finds the shortest possible spacer chains meeting exact length constraints
- **Required Elements**: Enforces inclusion of mandatory elements in specific orders
- **Thread Compatibility**: Automatic validation of thread connections (size matching, gender opposition)
- **Functional Programming**: All tools implemented using functional programming principles
- **Cross-Platform GUIs**: Modern interfaces built with Slint
- **JSON Data Format**: Human-readable configuration files
- **Duplicate Prevention**: Intelligent inventory management

## Quick Start

### Prerequisites
- Rust toolchain ([install from rustup.rs](https://rustup.rs/))
- For GUI tools: Slint system dependencies ([installation guide](https://slint.dev/docs/rust/getting_started/install))

### Build All Components
```bash
# Clone the repository
git clone https://github.com/ecouter/SpacerFinder.git
cd SpacerFinder

# Build all components
cargo build --release
```

### Typical Workflow

1. **Set up your spacer inventory** with SpacerManager:
   ```bash
   ./target/release/spacer_manager
   ```
   - Add your available spacers
   - Save to `spacers.json`

2. **Define required elements** with RigCreator:
   ```bash
   ./target/release/rig_creator
   ```
   - Create your "must-have" rig configuration
   - Test against your spacer inventory
   - Save to `must_have.json`

3. **Find optimal chains** with Spacer_Finder_CLI:
   ```bash
   ./target/release/spacer_finder spacers.json must_have.json M54M M54F 8.0
   ```

## Architecture

### Data Flow
```
SpacerManager → spacers.json → RigCreator → must_have.json → Spacer_Finder_CLI
                                      ↓
                               Chain Results
```

### Thread Specifications
- Format: `SIZE + GENDER` (e.g., `"M54M"`, `"M54F"`)
- **M**: Male thread
- **F**: Female thread
- Connection requires: same size + opposite gender

### JSON File Formats

#### spacers.json (Available Inventory)
```json
[
  {
    "thread_in": "M54F",
    "thread_out": "M54M",
    "length": 5.0
  }
]
```

#### must_have.json (Required Elements)
```json
[
  {
    "thread_in": "M54F",
    "thread_out": "M54M",
    "length": 5.0
  }
]
```

## Component Details

### Spacer_Finder_CLI
**Purpose**: Core algorithm implementation
- Command-line interface for batch processing
- Backtracking search with performance optimizations
- Chain length limited to 10 elements
- Functional programming architecture

### RigCreator
**Purpose**: Required element management and testing
- GUI for creating ordered "rig" configurations
- Integrated chain testing against spacer inventories
- Visual reordering of required elements
- Configuration export for CLI usage

### SpacerManager
**Purpose**: Spacer inventory management
- GUI for maintaining spacer collections
- Duplicate detection and prevention
- Clean interface for CRUD operations
- Foundation for the entire workflow

## Technical Specifications

- **Language**: Rust 2021 edition
- **GUI Framework**: Slint
- **Serialization**: Serde with JSON
- **Algorithm**: Backtracking with pruning
- **Architecture**: Functional programming
- **Platform**: Cross-platform (Linux, macOS, Windows)

## Use Cases

- **Microscopy**: Configuring imaging train spacers
- **Optics**: Building precise optical assemblies
- **Engineering**: Any application requiring exact-length threaded connections
- **Research**: Optimizing spacer configurations for experimental setups

## Contributing

We welcome contributions! Please:

1. Follow functional programming principles
2. Include tests for new functionality
3. Update documentation for API changes
4. Ensure cross-platform compatibility

### Development Setup
```bash
# Clone and build
git clone https://github.com/ecouter/SpacerFinder.git
cd SpacerFinder
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

This project is under the GPLv3 license.

## Support

- **Issues**: [GitHub Issues](https://github.com/ecouter/SpacerFinder/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ecouter/SpacerFinder/discussions)

## Related Projects

- [Slint](https://slint.dev/) - GUI framework
- [Serde](https://serde.rs/) - Serialization framework
- [Clap](https://docs.rs/clap/) - CLI argument parsing
