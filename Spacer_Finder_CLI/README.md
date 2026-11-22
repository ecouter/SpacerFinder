# Spacer Finder CLI

A high-performance Rust command-line tool for finding the minimal chain of spacers and imaging train elements that connect specified threads with an exact total length. This is the core algorithm component of the SpacerFinder suite.

## Features

- **Optimal Chain Finding**: Finds the shortest possible chain of spacers that meets exact length requirements
- **Thread Compatibility**: Automatically ensures compatible thread connections (same size, opposite gender)
- **Required Elements**: Enforces inclusion of "must-have" elements in a specific order
- **Flexible Usage**: Supports multiple uses of the same spacer
- **Functional Programming**: Pure chain-finding logic lives in the shared [`spacer-core`](../spacer-core/) crate
- **JSON Data Format**: Uses human-readable JSON files for input and output
- **Shared results**: Reports steps, cumulative lengths, and required-element markers just like RigCreator

## Installation

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build from Source
From the repository root:
```bash
cargo build --release
```

The binary will be available at `target/release/spacer_finder`.

## Usage

```bash
spacer_finder <spacers.json> <must_have.json> <start_thread> <end_thread> <total_length>
```

### Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `spacers.json` | JSON file containing available spacer elements | `spacers.json` |
| `must_have.json` | JSON file containing required elements in order | `must_have.json` |
| `start_thread` | Starting thread specification | `M54M` |
| `end_thread` | Ending thread specification | `M54F` |
| `total_length` | Exact total length to achieve (float) | `8.0` |

### Command Line Help

Run `spacer_finder --help` for detailed usage information.

## JSON File Formats

### Available Spacers (spacers.json)

An array of spacer objects representing your available inventory:

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

### Required Elements (must_have.json)

An array of elements that **must** be included in the chain, in the specified order:

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

Each element has:
- `thread_in`: Input thread specification (e.g., "M54F")
- `thread_out`: Output thread specification (e.g., "M54M")
- `length`: Length in millimeters (float)

## Thread Specifications

Threads are specified as **size + gender**:
- **M**: Male thread
- **F**: Female thread

**Example**: `"M54M"` means a male M54 thread.

### Connection Rules

For two elements to connect:
- Thread sizes must match (e.g., M54 connects to M54)
- Genders must be opposite (M connects to F, F connects to M)

The `start_thread` and `end_thread` arguments may be given either as the mating
thread on the external component or as the thread on the first/last spacer
itself. Both conventions are accepted; the reported chain always shows the
spacers' actual `thread_in`/`thread_out` values.

**Note**: Adapters with `length: 0.0` can be used for size transitions.

## Examples

### Basic Chain Finding

Find a chain from M54M to M54F with total length 8.0:

```bash
spacer_finder spacers.json must_have.json M54M M54F 8.0
```

**Sample Output:**
```
Found chain with 3 elements (total 8 mm):
  1. M54F -> M54M   5 mm   (cumulative 5 mm)  [required]
  2. M54F -> M54F   0 mm   (cumulative 5 mm)
  3. M54M -> M54F   3 mm   (cumulative 8 mm)
```

### With Required Elements

If `must_have.json` contains required elements, they will be included in order in the final chain.

### No Solution Found

If no valid chain exists:
```
No chain found from M54M to M54F that sums to 8 mm with the required elements.
```

## Integration with GUI Tools

This CLI tool is designed to work seamlessly with the SpacerFinder GUI applications:

- **SpacerManager**: Create and manage your `spacers.json` file
- **RigCreator**: Create and manage your `must_have.json` file and test configurations

## Technical Details

- **Algorithm**: Backtracking search with pruning for efficiency
- **Performance**: Chain length limited to 10 elements to prevent excessive computation
- **Precision**: Uses floating-point arithmetic with epsilon comparison for length matching
- **Memory**: Minimal memory footprint, suitable for large spacer inventories

## Troubleshooting

- **No chain found**: Verify thread compatibility and length requirements
- **Invalid JSON**: Ensure proper JSON syntax in input files
- **Performance issues**: Large spacer inventories may take time; consider filtering your input files

## Contributing

This tool follows functional programming principles. Contributions should maintain this paradigm and include appropriate tests.
