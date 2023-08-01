<!--For each individual album-->
<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    export let id: string;

    
</script>

<div class="container">
    <div class="top">
        {#await invoke('query_album', {id}) then album}
            {album.name}
        {/await}
    </div>
    <div class="grid-container">

    </div>
</div>

<style>
    .container {
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
    }
</style>