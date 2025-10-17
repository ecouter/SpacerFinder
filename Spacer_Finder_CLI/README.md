# Spacer Finder

A Rust CLI tool to find the minimal chain of spacers and imaging train elements that connect specified threads with an exact total length.

## Features

- Finds chains of spacers with compatible threads (same size, opposite gender)
- Supports multiple uses of spacers
- Enforces required "must-have" elements in a specific order
- Minimizes the total number of elements in the chain

## Installation

1. Ensure Rust is installed (https://rustup.rs/)
2. Clone or download the project
3. Build with `cargo build --release`

## Usage

```bash
./target/release/spacer_finder <spacers.json> <must_have.json> <start_thread> <end_thread> <total_length>
```

### Parameters

- `spacers.json`: JSON file containing available spacers
- `must_have.json`: JSON file containing required elements in order
- `start_thread`: Starting thread type (e.g., "M54M")
- `end_thread`: Ending thread type (e.g., "M54F")
- `total_length`: Exact total length to achieve (float)

## JSON File Formats

### Spacers JSON

An array of spacer objects:

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

### Must-Have JSON

An array of required elements in the order they must appear:

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

## Thread Compatibility

Threads are specified as size + gender (M for male, F for female), e.g., "M54M".

Connections require:
- Same size (e.g., M54)
- Opposite gender (M connects to F, F to M)

Adapters with length 0 can be used for size changes.

## Examples

### Basic Usage

```bash
./spacer_finder spacers.json must_have.json M54M M54F 8.0
```

Finds a chain from M54M to M54F with total length 8.0, including all must-have elements.

### Output

```
Found chain with 3 elements:
Thread in: M54F, Thread out: M54M, Length: 5
Thread in: M54F, Thread out: M54F, Length: 0
Thread in: M54M, Thread out: M54F, Length: 3
```

## Notes

- Lengths are floating-point for precision
- The tool uses functional programming principles
- Chain length is limited to 10 elements to prevent excessive computation
- Multiple uses of the same spacer are allowed
