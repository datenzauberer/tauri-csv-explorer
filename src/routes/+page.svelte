<script lang="ts">
    // Tauri Imports
    import { invoke } from "@tauri-apps/api/core";
    // REQ-004 webviewWindow has draganddrop event
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
    import type { UnlistenFn } from '@tauri-apps/api/event';
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
    let filepath = $state("");
    let csvData = $state([]);

    const config = { editor: "text", sort: true };

    // --- Tauri Dialog File Selection Logic ---
    async function handleFileChange(filePath: string) {
      if (filePath) {
        try {
          const result: string = await invoke("tauri_read_csv_file", {
            path: filePath,
          });

          filepath = filePath;
          csvData = JSON.parse(result);
        } catch (e) {
          // SMTODO: Error Handling (set error from Rust Backend)
          console.error("Error from Tauri backend:", e);
        }
      } else {
        console.warn("No file path provided.");
      }
    }

    async function openFileDialog() {
      const selected = await open({
        multiple: false,
        directory: false,
      });

      if (typeof selected === 'string') {
        await handleFileChange(selected);
      } else {
        console.log("No file selected or dialog was canceled.");
      }
    }

  // REQ-004 Implementation: register on mount and clean up on destroy
    const appWindow = getCurrentWebviewWindow();

    let unlisten: UnlistenFn | null = null;
    onMount(async () => {
        unlisten = await appWindow.onDragDropEvent(async (event) => {
            if (event.payload.type === 'drop') {
            console.log('Files dropped:', event.payload.paths);
            const first = event.payload.paths?.[0];
            if (first) {
                await handleFileChange(first);
            }
            }
        });
    });
    onDestroy(() => {
      if (unlisten) {
          unlisten(); // Remove Tauri listener
      }
    });
</script>

<main class="container">
    <h1>TauriCSVExplorer 5</h1>
    <button type="button" onclick={openFileDialog}>Open File</button>

    <p>filepath: {filepath}</p>

    <Willow>
        <Grid data={csvData} autoConfig={config} />
    </Willow>
</main>
