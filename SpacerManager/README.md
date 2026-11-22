# Spacer Manager

GUI for maintaining the spacer inventory used by the SpacerFinder suite. Built with Slint 1.19 and the shared dark UI kit in [`ui-kit/`](../ui-kit/).

## Features

- Add, edit, and delete spacers with inline validation
- Thread format checking (`M54M` style) and non-negative length input with `mm` units
- Duplicate detection when adding or editing, with an explicit "Save anyway" option
- Grouping by thread family (M42, M48, M54, …) with per-family counts; sorting applies within each group
- Search by thread or length, plus sorting by thread or length (ascending/descending)
- Configurable JSON path (default `spacers.json`) with Save/Load status messages
- Unsaved-changes indicator and delete confirmation dialog
- Keyboard support: Enter saves from a form, Esc cancels a dialog

## Build

From the repository root:

```bash
cargo build --release
```

Run `target/release/spacer_manager` from the directory where you want `spacers.json` to live (or change the path shown in the status bar).

## Interface

- **Header**: inventory count, Add spacer (primary), Save, Load
- **Toolbar**: filter field and sort buttons; the count of visible entries is shown on the right
- **Table**: family group headers, then aligned columns for thread-in → thread-out badges, length in mm, and row actions. Edit/delete always target the original inventory row, even while filtered.
- **Status bar**: current file path, confirmation/error banner, and saved/unsaved indicator

The add/edit dialog shows helper text for each field. Validation errors appear in the dialog; duplicates prompt you to confirm before a spacer is saved twice.

## JSON format

```json
[
  { "thread_in": "M54F", "thread_out": "M54M", "length": 5.0 }
]
```

`thread_in` and `thread_out` use `M` + size + `M`/`F` (e.g. `M54F`); `length` is a non-negative number in millimetres.

## Architecture

UI markup lives in `ui/app.slint`; `src/main.rs` wires Slint callbacks to the pure operations in [`spacer-core`](../spacer-core/). `build.rs` compiles the markup with Slint edition 1.19 and the `fluent-dark` widget style.
