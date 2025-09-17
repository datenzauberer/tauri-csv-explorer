<script lang="ts">
    // Tauri Imports
    import { invoke } from "@tauri-apps/api/core";
    // REQ-002 plugin-dialog is needed
    import { open } from "@tauri-apps/plugin-dialog";

    //Svelte Imports
    import { onMount, onDestroy } from "svelte";

    // @ts-ignore
    import { Grid, Willow } from "wx-svelte-grid";
    // @ts-ignore
    import type { GridColumn } from "wx-svelte-grid/types";
    /*   

//    import FileUploader from "./FileUploader.svelte";
*/
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
</script>

<main class="container">
    <h1>TauriCSVExplorer 5</h1>
    <button type="button" onclick={openFileDialog}>Open File</button>

    <p>filepath: {filepath}</p>

<!--    
    <FileUploader fileChange={handleFileChange} />
    <p>cliParam as rust tauri command: {cliParamInvoke}</p>
    <button onclick={testGetCliParam}>Send File</button>

-->
    <Willow>
        <Grid data={csvData} autoConfig={config} />
    </Willow>
</main>
