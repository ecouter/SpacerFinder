# Rig Creator

GUI for defining required ("must-have") elements, ordering them, and testing them against a spacer inventory. Built with Slint 1.19 and the shared dark UI kit in [`ui-kit/`](../ui-kit/).

## Features

- Ordered required-element list with numbered cards and up/down reordering
- Validated thread (`M54M` style) and length (`mm`) input in add/edit dialogs
- Configurable paths for both `must_have.json` and the spacer inventory
- Chain tester with validated start thread, end thread, and exact target length
- Structured results: each step with thread transition, step length, cumulative length, and a `REQ` marker for required elements
- Save/Load feedback, unsaved-changes indicator, delete confirmation
- Keyboard support: Enter runs an action, Esc cancels a dialog

## Build

From the repository root:

```bash
cargo build --release
```

Run `target/release/rig_creator` from the directory containing your `must_have.json` and `spacers.json`, or edit the paths shown in the UI.

## Interface

- **Header**: element count, Add element (primary), Save, Load
- **Required elements panel**: rows with index badge, thread badges, length, reorder/edit/delete buttons
- **Chain tester panel**: inventory path, start/end threads, target length, Find chain button, result summary and step list
- **Status bar**: current `must_have.json` path, status banner, and saved/unsaved indicator

A chain is only accepted when it connects the start thread to the end thread, sums to the target length, and contains every required element in the listed order. Start/end threads may name either the mating thread (opposite genders connect) or the first/last spacer's own thread, so both common conventions work. The chain finder uses the same [`spacer-core`](../spacer-core/) implementation as the CLI.

## JSON format

```json
[
  { "thread_in": "M54F", "thread_out": "M54M", "length": 5.0 },
  { "thread_in": "M54M", "thread_out": "M54F", "length": 3.0 }
]
```

Order matters: required elements must appear in the chain in this order.

## Architecture

UI markup lives in `ui/app.slint`; `src/main.rs` wires the callbacks. `build.rs` compiles the markup with Slint edition 1.19 and the `fluent-dark` widget style.
