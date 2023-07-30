<script lang="ts">
  import AlbumView from "./lib/AlbumView.svelte";
  import WindowsTitleBar from "./lib/WindowsTitleBar.svelte";

  import { appWindow } from "@tauri-apps/api/window";
  
  let hasBorder = true;
  
  appWindow.onResized(async () => {
    hasBorder = await appWindow.isMaximized() ? false : true;
  });


</script>

<main class="container" class:has-border={hasBorder}>
  <WindowsTitleBar/>
  <AlbumView/>
</main>

<style>
  /*container should be full height and width of window, minus 1 px because of a tauri bug.*/
  .container {
    width: 100%;
    height: 100%;
    box-sizing: border-box;
  }
  
  .has-border {
    border: 1px solid rgba(255, 255, 255, 0.2);
 
    
    /* To prevent a bug where, if the width/height in pixels is not a whole number, 
       the remaining pixel is clipped, and border-right and border-bottom do not
       appear */
    width: calc(100% - 1px);
    height: calc(100% - 1px);
    max-height: calc(100% - 1px);
  }
</style>