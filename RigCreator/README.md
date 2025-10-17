# Rig Creator

A user-friendly Rust GUI application for managing collections of required "rig" elements and testing spacer chain configurations. Built with Slint for a modern, responsive interface. This tool complements the SpacerFinder suite by providing an intuitive way to create and validate "must-have" element configurations.

## Features

- **Rig Management**: Add, edit, delete, and reorder required elements with full CRUD operations
- **Order Control**: Visual reordering with up/down buttons to set the exact sequence requirements
- **Chain Testing**: Integrated testing interface to validate configurations against spacer inventories
- **Data Persistence**: Save and load rig collections to/from JSON files
- **Functional Programming**: Implemented using functional programming principles for reliability
- **Cross-Platform GUI**: Modern interface built with Slint

## Installation

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- System dependencies for Slint (follow [Slint installation guide](https://slint.dev/docs/rust/getting_started/install))

### Build from Source
```bash
cd RigCreator
cargo build --release
```

The binary will be available at `target/release/rig_creator`.

## Usage

Launch the application:

```bash
./target/release/rig_creator
```

### Main Interface

The application opens with two primary sections:

#### Rig Management Panel

**Adding Rigs:**
- Click "Add Rig" to open the add dialog
- Enter thread specifications and length
- Click "Add" to save

**Managing Existing Rigs:**
- **Edit**: Click "Edit" on any rig row to modify its properties
- **Delete**: Click "Delete" on any rig row to remove it
- **Reorder**: Use ↑ and ↓ buttons to change rig positions (critical for chain finding order)

**Data Operations:**
- **Save**: Export current rig collection to `must_have.json`
- **Load**: Import rig collection from `must_have.json`

Each rig displays its **position number** (1., 2., 3., etc.) indicating the order in which it must appear in spacer chains.

#### Chain Testing Panel

Test your rig configurations against available spacer inventories:

**Configuration Fields:**
- **Spacers File**: Path to JSON file containing available spacers (default: "spacers.json")
- **Start Thread**: Beginning thread specification (e.g., "M54M")
- **End Thread**: Target thread specification (e.g., "M54F")
- **Target Length**: Exact total length required (float)

**Testing:**
- Click "Find Chain" to execute the search
- Results display in the text area below

**Sample Results:**
```
Found chain with 5 elements:
Thread in: M54F, Thread out: M54M, Length: 5
Thread in: M54M, Thread out: M54F, Length: 3
Thread in: M54F, Thread out: M54M, Length: 2
Thread in: M54M, Thread out: M54F, Length: 1
Thread in: M54F, Thread out: M54F, Length: 0
```

## JSON File Formats

### Required Rigs (must_have.json)

Your rig collection is stored as an ordered array of elements that **must** be included in any valid chain:

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
  }
]
```

### Available Spacers (spacers.json)

Reference file containing your spacer inventory (typically managed with SpacerManager):

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
  }
]
```

**Field Definitions:**
- `thread_in`: Input thread specification (e.g., "M54F")
- `thread_out`: Output thread specification (e.g., "M54M")
- `length`: Length in millimeters (float)

## Thread Specifications

Threads use a **size + gender** format:
- **M**: Male thread
- **F**: Female thread

**Example**: `"M54M"` represents a male M54 thread.

### Connection Compatibility

Elements connect when:
- Thread sizes match (e.g., M54 to M54)
- Genders are opposite (M connects to F)

## Integration with SpacerFinder Suite

Rig Creator works seamlessly with other SpacerFinder tools:

- **SpacerManager**: Create and manage your `spacers.json` inventory file
- **Spacer_Finder_CLI**: Use exported `must_have.json` files for command-line chain finding

**Typical Workflow:**
1. Use SpacerManager to build your spacer inventory
2. Use Rig Creator to define required elements and test configurations
3. Export configurations for use with Spacer_Finder_CLI

## Technical Details

- **GUI Framework**: Slint for cross-platform native interfaces
- **Algorithm**: Integrated backtracking chain finder (same as CLI tool)
- **Performance**: Chain search limited to 10 elements maximum
- **Data Format**: JSON for human-readable configuration files
- **Architecture**: Functional programming with immutable data structures

## Troubleshooting

- **Chain not found**: Verify thread compatibility and length constraints
- **GUI not rendering**: Ensure Slint dependencies are properly installed
- **File load errors**: Check JSON syntax in input files
- **Order issues**: Remember that rig order is critical for chain finding

## Contributing

Contributions should follow functional programming principles and include GUI testing where applicable.
