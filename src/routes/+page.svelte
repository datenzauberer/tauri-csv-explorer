<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, onDestroy } from "svelte";
    /*   
    // @ts-ignore
    import { Grid, Willow } from "wx-svelte-grid";
    // @ts-ignore
    import type { GridColumn } from "wx-svelte-grid/types";

//    import FileUploader from "./FileUploader.svelte";
*/
    let filepath = $state("");

    let csvData = $state([]);
    const config = { editor: "text", sort: true };
</script>

<main class="container">
    <h1>TauriCSVExplorer 2</h1>
    <p>filepath: {filepath}</p>

    <!--    
    <p>cliParam as rust tauri command: {cliParamInvoke}</p>
    <button onclick={testGetCliParam}>Send File</button>

    <FileUploader fileChange={handleFileChange} />
    <Willow>
        <Grid data={csvData} autoConfig={config} />
    </Willow>
-->
</main>
