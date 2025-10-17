# Spacer Manager

A Rust GUI application for managing spacer collections using Slint.

## Features

- Add, edit, and delete spacers with thread specifications and lengths
- Save and load spacer collections to/from JSON files
- User-friendly graphical interface for spacer management

## Installation

1. Ensure Rust is installed (https://rustup.rs/)
2. Clone or download the project
3. Build with `cargo build --release`

## Usage

Run the application:

```bash
./target/release/spacer_manager
```

The GUI will open, allowing you to:
- Add new spacers by clicking "Add Spacer"
- Edit existing spacers by clicking "Edit" on a spacer row
- Delete spacers by clicking "Delete" on a spacer row
- Save the current collection to `spacers.json` by clicking "Save"
- Load a collection from `spacers.json` by clicking "Load"

## JSON File Format

Spacers are stored in `spacers.json` as an array of spacer objects:

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

Each spacer has:
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
