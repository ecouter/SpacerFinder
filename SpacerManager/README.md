# Spacer Manager

A robust Rust GUI application for managing collections of spacer elements with built-in duplicate prevention. Built with Slint for a clean, intuitive interface. This tool serves as the foundation of the SpacerFinder suite by providing reliable spacer inventory management.

## Features

- **Complete CRUD Operations**: Add, edit, and delete spacers with full validation
- **Duplicate Prevention**: Intelligent warnings when attempting to add existing spacers
- **Data Persistence**: Save and load spacer collections to/from JSON files
- **User-Friendly Interface**: Clean GUI designed for efficient spacer catalog management
- **Functional Programming**: Implemented using functional programming principles
- **Cross-Platform**: Native GUI built with Slint

## Installation

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- System dependencies for Slint (follow [Slint installation guide](https://slint.dev/docs/rust/getting_started/install))

### Build from Source
```bash
cd SpacerManager
cargo build --release
```

The binary will be available at `target/release/spacer_manager`.

## Usage

Launch the application:

```bash
./target/release/spacer_manager
```

### Main Interface

The application provides a straightforward interface for spacer management:

#### Spacer Operations

**Adding Spacers:**
- Click "Add Spacer" to open the input dialog
- Enter thread specifications and length
- The system checks for duplicates before allowing addition
- If a duplicate is detected, you can choose to add anyway or cancel

**Managing Existing Spacers:**
- **Edit**: Click "Edit" on any spacer row to modify its properties
- **Delete**: Click "Delete" on any spacer row to remove it from inventory

#### Data Management

**File Operations:**
- **Save**: Export current spacer collection to `spacers.json`
- **Load**: Import spacer collection from `spacers.json`

### Duplicate Detection

When adding a new spacer, the system automatically checks if an identical spacer already exists (matching thread_in, thread_out, and length). If found:

- A warning message appears: *"A spacer with these exact specifications already exists!"*
- You can choose "Add Anyway" to proceed or "Cancel" to abort
- This prevents accidental duplicates while allowing intentional additions

## JSON File Format

### Spacer Collection (spacers.json)

Your spacer inventory is stored as an array of spacer objects:

```json
[
  {
    "thread_in": "M54F",
    "thread_out": "M54M",
    "length": 5.0
  },
  {
    "thread_in": "M54M",
    "thread_out": "M54F",
    "length": 3.0
  },
  {
    "thread_in": "M42F",
    "thread_out": "M54F",
    "length": 0.0
  }
]
```

**Field Definitions:**
- `thread_in`: Input thread specification (e.g., "M54F")
- `thread_out`: Output thread specification (e.g., "M54M")
- `length`: Length in millimeters (float)

## Thread Specifications

Threads are specified using **size + gender** notation:
- **M**: Male thread
- **F**: Female thread

**Example**: `"M54M"` denotes a male M54 thread.

### Connection Rules

For compatibility between spacers:
- Thread sizes must match (e.g., M54 connects to M54)
- Genders must be opposite (M connects to F, F connects to M)

## Integration with SpacerFinder Suite

Spacer Manager is designed to work seamlessly with other SpacerFinder components:

- **RigCreator**: Use your `spacers.json` file for testing rig configurations
- **Spacer_Finder_CLI**: Provide spacer inventory for command-line chain finding

**Recommended Workflow:**
1. Use SpacerManager to build and maintain your spacer inventory
2. Export `spacers.json` for use with RigCreator and Spacer_Finder_CLI
3. Keep your inventory up-to-date as you acquire new spacers

## Technical Details

- **GUI Framework**: Slint for native cross-platform interfaces
- **Duplicate Detection**: Exact matching on all three fields (thread_in, thread_out, length)
- **Data Validation**: Input validation for thread specifications and numeric lengths
- **File Format**: JSON for human-readable and editable data storage
- **Architecture**: Functional programming with pure functions for data operations

## Troubleshooting

- **Duplicate warnings**: This is normal behavior - review if you truly need another identical spacer
- **GUI display issues**: Verify Slint installation and system graphics drivers
- **File operation errors**: Ensure write permissions for the target directory
- **Thread format errors**: Use the size+gender format (e.g., "M54F")

## Best Practices

- **Regular Backups**: Keep backups of your `spacers.json` file
- **Consistent Naming**: Use standardized thread specifications
- **Inventory Audit**: Periodically review and clean up your spacer collection
- **Documentation**: Keep physical records matching your digital inventory

## Contributing

Contributions should maintain functional programming principles and include tests for duplicate detection logic.
