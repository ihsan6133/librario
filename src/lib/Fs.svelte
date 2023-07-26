<script>
    import { invoke } from "@tauri-apps/api/tauri"
    
    let path = "C:/";
    let loadFiles = Promise.resolve([]);
    
    function handleClick(){
        loadFiles = invoke("readdir", {path})
    }
    
</script>



<div>

    <input type="text" bind:value={path}>
    <button on:click={handleClick}>Search</button>
    {#await loadFiles}
        <p>Loading...</p>
    {:then files} 
        {#each files as file}
            <p>{file}</p>
        {/each}
    {:catch err}
        <p>Error Occured: {err}</p>
    {/await}
    
</div>