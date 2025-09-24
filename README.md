#  tauri-csv-explorer

This is a demonstration for using Tauri (with SvelteKit and TypeScript) for desktop applications.
It's a csv viewer.
I've tested it on Mac Os and Linux (Ubuntu).

# Architecture

We have a clear separation of concern.
The backend is implemented in Rust, the frontend in Svelte,
In other words: The business logic is implemented (and should be tested) in Rust in an own lib create.
the UI (in Typescript).

In the frontend TypeScript(JavaScript) based Framework is used, because there are many components for handling DataTables.
We as Rust developers trust in Typing so we use TypeScript (instead of untyped JavaScript).
For Reading the CSV-file polars is used (for this used case it's oversized, but the integration between Rust/polars and Typescript has a lot of potential not only in DataScience).
Svelte is used because of Speed and Simplicity.

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

In `src-csv`:

```sh
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

If no `defaultPath` is set, the Path is set first on `$HOME/Documents` and after one selection to the latest used directory.

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

## Svelte UI: DataGrid

For visualization the [Svelte Datagrid](https://svar.dev/svelte/datagrid/) is used.

Error Handling 
https://v2.tauri.app/develop/calling-rust/#error-handling
Errors from the Rust backend are propagated to the Frontend.

Alternative:
https://github.com/joaquimnetocel/svelte-datatables-net

## Application Icon

Demo to use a new icon 

```sh
pnpm tauri icon taurirc/assets/icon_csv_text.png
```

After changing the icon in `src-tauri` a `cargo clean` is needed to reflect the changes.

## File Association (`.csv`)

Files with Extension `.csv` should opened with the application.

Implementation: tauri.conf.json
``` 
  "bundle": {
    "fileAssociations": [
      {
        "ext": [
          "csv"
        ],
        "name": "CSV File",
        "role": "Editor",
        "mimeType": "text/csv",
        "description": "A comma-separated values file."
      }
    ],
```


# Distribute Cross-Platform Application

## Release Creation

implemented with Github Actions, see: [.github/workflows/publish-tauri-app.yml](.github/workflows/publish-tauri-app.yml)

## REQ-102 Update Process

Demonstration with AppImage on Ubuntu ARM.
For Mac/Windows a signed release must be created.

```sh
cargo tauri add updater
```

SMTODO:

Tauri internal signs the updates:
So create Tauri Keys
pnpm tauri signer generate -w ~/.tauri/myapp.key

This Keys have to be added to your:

## ./src-tauri/tauri.conf.json : configure updater
SMTODO: Code Formatter

```
createUpdaterArtifacts true
  "bundle": {
...  
    "createUpdaterArtifacts": true
  },
  "plugins": {
...
  "updater": {
    "pubkey": "TODO: insert",
    "endpoints": [
        "https://github.com/TODO:user/TODO:repo/releases/latest/download/latest.json"
    ]
  }
```

more:
https://v2.tauri.app/plugin/updater/#tauri-configuration

To create the 
In Github `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` have to be configured as secrets to sign a update release, e.g.:

```sh
echo $TAURI_SIGNING_PRIVATE_KEY | gh secret set TAURI_SIGNING_PRIVATE_KEY
echo $TAURI_SIGNING_PRIVATE_KEY_PASSWORD | gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD
```

## SMTOD opener entfernen !!!!

# Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

# License: MIT

SMTODO: Code Formatter
```
        "fix": "prettier -w \"**/*.svelte\" && biome check --write .",
        "verify": "prettier -c \"**/*.svelte\" && biome check . && svelte-check"
```

SMTODO: vscode settings

