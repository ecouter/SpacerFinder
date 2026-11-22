# SpacerFinder

A Rust workspace of tools for finding optimal chains of spacers and imaging-train elements with exact length requirements. Designed for optics and microscopy applications where precise spacer configurations are critical.

## Overview

- **[Spacer_Finder_CLI](Spacer_Finder_CLI/)** — command-line chain finder
- **[RigCreator](RigCreator/)** — GUI for required "rig" elements and chain testing
- **[SpacerManager](SpacerManager/)** — GUI for managing spacer inventories
- **[spacer-core](spacer-core/)** — shared domain model, validation, and chain-finding logic

All three tools use the same rules for thread compatibility, validation, and chain results because those rules live in `spacer-core`. The GUIs are built with **Slint 1.19** and share a dark "instrument panel" UI kit in [`ui-kit/`](ui-kit/).

## Key Features

- Optimal chain finding: shortest chain satisfying an exact total length
- Required elements enforced in a specific order
- Thread validation (`M54M` style: metric size + M/F gender) and connection checking
- Chain endpoints may name either the mating thread or the first/last spacer's own thread, so both common conventions work
- Search, filter, and sorting in SpacerManager
- Structured chain results with cumulative lengths and required-element markers in RigCreator
- Save/load status feedback, unsaved-changes indicator, and configurable JSON paths in both GUIs
- Keyboard shortcuts: Enter submits a dialog or action, Esc cancels
- Functional, pure collection operations with unit tests in `spacer-core`

## Quick Start

### Prerequisites
- Rust toolchain ([rustup.rs](https://rustup.rs/))
- For the GUI tools: Slint system dependencies ([installation guide](https://slint.dev/docs/rust/getting_started/install))

### Build all components

From the repository root:

```bash
cargo build --release
```

Binaries are written to `target/release/` (`spacer_manager`, `rig_creator`, `spacer_finder`).

### Typical workflow

1. Build your inventory with SpacerManager and save it as `spacers.json`.
2. Define required elements with RigCreator and save them as `must_have.json`.
3. Find chains with the CLI:
   ```bash
   ./target/release/spacer_finder spacers.json must_have.json M54M M54F 8.0
   ```

## Architecture

```
SpacerManager → spacers.json ─┐
                              ├→ spacer-core → chain results (CLI + RigCreator)
RigCreator    → must_have.json┘
```

`spacer-core` owns:
- `Element` (serde model) and pure add/update/delete/reorder operations
- `Thread` parsing, validation, and `can_connect`
- `find_minimal_chain` returning a structured `ChainResult`
- JSON load/save helpers

The GUI crates contain only Slint markup (`ui/app.slint`) and Rust callback wiring (`src/main.rs`). Their `build.rs` scripts compile the markup with Slint edition 1.19 and the `fluent-dark` widget style; the shared design tokens and components live in `ui-kit/`. You can preview the `.slint` files with `slint-viewer`.

## JSON formats

Both files are arrays of elements:

```json
[
  {
    "thread_in": "M54F",
    "thread_out": "M54M",
    "length": 5.0
  }
]
```

Thread format is `M` + size digits + `M`/`F` gender, e.g. `M54M` (male M54 thread). Two elements connect when the sizes match and the genders are opposite. Lengths are millimetres; adapters with length `0.0` are allowed.

## Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo build --release
```

## License

GPLv3 — see [LICENSE](LICENSE).
