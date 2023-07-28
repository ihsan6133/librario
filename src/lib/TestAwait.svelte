<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
let long_func = invoke("long_function");
</script>

<div>
    {#await long_func}
        loading
    {:then _} 
        loaded
    {/await}
    
</div>

<style>

</style>