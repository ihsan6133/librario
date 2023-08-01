<!--For each individual album-->
<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    export let id: string;

    let query_album: Promise<any> = invoke('query_album', {id});
    let query_album_files: Promise<[string]> = invoke('query_album_files', {id});

</script>

<div class="container">
    <div class="top">
        <button class="back-btn">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"><path color="white" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m5 12 6-6m-6 6 6 6m-6-6h14"/></svg>
        </button>

        <span>
            {#await query_album then album}
                {album.name}
            {/await}
        </span>
    </div>
    <div class="grid-container">
        {#await query_album_files}
            Loading
        {:then files} 
            {#each files as file}
                {file}
            {/each}
        {:catch err}
            {err}
        {/await}
    </div>
</div>

<style>
    .container {
        flex: 1 1 auto;
        display: flex;
        flex-direction: column;
    }

    .top {
        display: flex;
        align-items: center;
        
        gap: 20px;
        padding: 9px;
        font-size: 1.3rem;
        border-bottom: 1px solid rgba(255, 255, 255, 0.3);
    }
    .back-btn {
        display: flex;
        justify-content: center;
        align-items: center;

        padding: 0;

        width: 30px;
        height: 30px;

        border: none;
        border-radius: 50%;

        background-color: transparent;
        cursor: pointer;
        transition-duration: 0.2s;
    }
    
    .back-btn:hover {
        background-color: rgb(80, 80, 80);
    }
    .back-btn:active {
        background-color: rgb(100, 100, 100);
    }
    
    .back-btn svg {
        width: 90%;
    }

</style>