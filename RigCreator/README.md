# Rig Creator

A Rust GUI application for managing rig collections and testing spacer chain configurations using Slint.

## Features

- Add, edit, and delete rigs with thread specifications and lengths
- Save and load rig collections to/from JSON files
- Test spacer chain finding with your rig configurations
- User-friendly graphical interface for rig management and chain testing

## Installation

1. Ensure Rust is installed (https://rustup.rs/)
2. Clone or download the project
3. Build with `cargo build --release`

## Usage

Run the application:

```bash
./target/release/rig_creator
```

The GUI will open with two main sections:

### Rig Management

- Add new rigs by clicking "Add Rig"
- Edit existing rigs by clicking "Edit" on a rig row
- Delete rigs by clicking "Delete" on a rig row
- **Reorder rigs** using the ↑ and ↓ buttons to change their position (important for chain finding order)
- Save the current collection to `must_have.json` by clicking "Save"
- Load a collection from `must_have.json` by clicking "Load"

Each rig displays its position number (1., 2., 3., etc.) to show the order in which they will be required in spacer chains.

### Test Chain Finding

Test your rig configurations against available spacers:

- **Spacers File**: Path to a JSON file containing available spacers (defaults to "spacers.json")
- **Start Thread**: Starting thread specification (e.g., "M54M")
- **End Thread**: Ending thread specification (e.g., "M54F")
- **Target Length**: Exact total length to achieve (float)

Click "Find Chain" to test the configuration. The results will show:
- The minimal chain of spacers that includes all your required rigs
- Or a message indicating no valid chain was found

## JSON File Formats

### Must-Have Rigs (must_have.json)

Rigs are stored in `must_have.json` as an array of rig objects:

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

### Available Spacers

The spacers file should contain available spacer elements in the same format:

```json
[
  {
    "thread_in": "M54F",
    "thread_out": "M54M",
    "length": 5.0
  }
]
```

Each element has:
- `thread_in`: Input thread specification (e.g., "M54F")
- `thread_out`: Output thread specification (e.g., "M54M")
- `length`: Length in millimeters (float)

## Thread Specifications

Threads are specified as size + gender:
- M for male
- F for female

Example: "M54M" means a male M54 thread.

## Notes

- The application uses functional programming principles
- Data is persisted in JSON format for easy editing and sharing
- The created `must_have.json` files can be used with Spacer_Finder_CLI
- Chain finding is limited to 10 elements to prevent excessive computation
- The algorithm finds the minimal chain that includes all required rigs in order
