# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


Kopiere Vorlage aus


# Architecture

We have a clear separation of concern.
The backend is implemented in Rust, the frontend in Svelte,
In other words: The business logic is implemented (and should be tested) in Rust in an own lib create.
the UI (in Typescript).

In the frontend TypeScript(JavaScript) based Framework is used, because there are many components for handling DataTables.
SMTODO: Show a small overview.
We as Rust developers trust in Typing so we use TypeScript (instead of untyped JavaScript).
For Reading the CSV-file polars is used (for this used case it's oversized, but the integration between Rust/polars and Typescript has a lot of potential not only in DataScience).
Svelte is used because of Speed and Simplicity.

## SMTODO: Frontend TS vs. Rust
Demonstrate Hot Module Reload (in Rust) zum Vergleich mit TS
Hot Reload

## "Write once, run anywhere"

# Requirements

## REQ-0001 CSV Logic Lib

The logic for extracting the metadata (like delimeter) must be in a separate crate.
There should be a cli version that use the lib crate independently.

## REQ-002 Button Open CSV-File

# CSV-Demo Implementation

TODO: Full Picture

# Develop Cross-Platform Application

## REQ-001 Logic src-csv (CSV Processing) in an own crate

logic is in an own lib create, so that we can demonstrate
how to make a UI for a existing library.

``sh
echo target > .gitignore
```

## REQ-002 UI (with button load csv)

Plugin dialog is needed, more see:
https://v2.tauri.app/reference/javascript/dialog/

```sh
pnpm tauri add dialog
```

see: src-tauri/capabilities/default.json

    "dialog:allow-open"

Component for visualizing the CSV-Data

```sh
pnpm install wx-svelte-grid
```

If no `defaultPath` is set, the Path is set first on $HOME/Documents and after one selection to the latest used directory.

## REQ-003 CLI Argument FileName

When the Tauri App is started from CLI there should be a way to pass the csv file.

Implementation:
Because we use an already existing clap logic, we do not need the tauri cli-plugin !

## REQ-004 Drag & Drop

## REQ-005 Menue

## REQ-006 Application Title

To set the Application Title following permission must be set:

```sh
pnpm tauri permission add "core:window:allow-set-title"
```

Otherwise we get:

```
Unhandled Promise Rejection: window.set_title not allowed. Permissions associated with this command: core:window:allow-set-title
```

## Application Icon

# Distribute Cross-Platform Application

## Release Creation

SMTODO: Test : FileOpenDialog

## Update Process

# Error Handling (errots from rust should be shown in svelte frontend)

SMTODO: https://v2.tauri.app/develop/calling-rust/#error-handling

# SMTODO: OSS-LICENCE: MIT
