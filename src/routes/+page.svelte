<script lang="ts">
    // Tauri Imports
    import { invoke } from "@tauri-apps/api/core";

    // REQ-004 webviewWindow has draganddrop event
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    // REG-005
    import { setCustomMenu } from "$lib/menu";
    // REG-006
    import { getDisplayShortcutFileOpen } from "$lib/menu";

    // REQ-005 for handling custom menu items
    import { listen } from "@tauri-apps/api/event";
    // REQ-006 get Application Name
    import { getName } from "@tauri-apps/api/app";
    import { basename } from "@tauri-apps/api/path";

    // REQ-002 plugin-dialog is needed
    import { open } from "@tauri-apps/plugin-dialog";

    // Svelte Imports
    // REQ-004 svelte onMount and Destroy needed
    import { onMount, onDestroy } from "svelte";

    // @ts-ignore
    import { Grid, Willow } from "wx-svelte-grid";
    // @ts-ignore
    import type { GridColumn } from "wx-svelte-grid/types";

    // Svelte states
    let error = $state<string | null>(null);
    let filepath = $state("");
    let csvData = $state([]);

    const config = { editor: "text", sort: true };

    // --- Tauri Dialog File Selection Logic ---
    async function handleFileChange(filePath: string) {
        if (filePath) {
            // Clear previous error and data
            error = null;
            csvData = [];

            invoke("tauri_read_csv_file", {
                path: filePath,
            })
                .then((result: unknown) => {
                    // The invoke call was successful, so parse the result and update the states.
                    const resultString = result as string;
                    filepath = filePath;
                    csvData = JSON.parse(resultString);
                })
                .catch((e: string) => {
                    // The invoke call returned an Err from Rust, so handle the error here.
                    error = e;
                    console.error("Error from Tauri backend:", e);
                });
            // REQ-006 set application title with filename
            const fileName = await basename(filePath);
            await getCurrentWindow().setTitle(
                (await getName()) + `: ${fileName}`
            );
        } else {
            console.warn("No file path provided.");
        }
    }

    async function openFileDialog() {
        const selected = await open({
            multiple: false,
            directory: false,
        });

        if (typeof selected === "string") {
            await handleFileChange(selected);
        } else {
            console.log("No file selected or dialog was canceled.");
        }
    }

    const appWindow = getCurrentWebviewWindow();

    let unlistenDragAndDrop: UnlistenFn | null = null;
    let unlistenFileOpen: UnlistenFn | null = null;
    onMount(async () => {
        // REQ-006 set default Application Name
        await getCurrentWindow().setTitle(
            (await getName()) +
                `: ${getDisplayShortcutFileOpen()} to open a file`
        );

        // REQ-004 Implementation: register onDragDropEvent clean up on destroy
        unlistenDragAndDrop = await appWindow.onDragDropEvent(async (event) => {
            if (event.payload.type === "drop") {
                const first = event.payload.paths?.[0];
                if (first) {
                    await handleFileChange(first);
                }
            }
        });
        // REQ-005 set Menu and listen to File Open Event
        await setCustomMenu();
        unlistenFileOpen = await listen(
            "menu_action_open_file",
            async (event) => {
                await openFileDialog();
            }
        );
    });
    onDestroy(() => {
        if (unlistenDragAndDrop) {
            unlistenDragAndDrop();
        }
        if (unlistenFileOpen) {
            unlistenFileOpen();
        }
    });
</script>

<main class="container">
    <h1>TauriCSVExplorer</h1>
    <button type="button" onclick={openFileDialog}>Open File</button>

    <p>Absolute filename: {filepath}</p>

    {#if error}
        <div class="error-box">
            <p>Error: {error}</p>
        </div>
    {:else}
        <Willow>
            <Grid data={csvData} autoConfig={config} />
        </Willow>
    {/if}
</main>

<style>
    .error-box {
        background-color: #ffeaea; /* Light red background */
        border: 1px solid #ff4d4d; /* Red border */
        color: #ff4d4d; /* Red text */
        padding: 1rem;
        border-radius: 8px;
        margin-top: 1rem;
        font-family: monospace;
    }
</style>
