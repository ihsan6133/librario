<script lang="ts">
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import { fs, path } from '@tauri-apps/api';
    
    import { BaseDirectory } from '@tauri-apps/api/path';
    import { createEventDispatcher } from 'svelte';


    const dispatch = createEventDispatcher();
    
    function selectAlbum(id: string) {
      dispatch("selectalbum", {id});
        
    }


    
    let loadAlbums = invoke("list_albums");
    
    let isEmpty = null;
    $: loadAlbums.then(albums => {
      isEmpty = Object.keys(albums).length === 0;
    });

    async function imagePath(id: string) {
      const appCacheDir: string = await path.appCacheDir();
      const thumbnailPath: string = await path.join("thumbs", `${id}.webp`);

      if (await fs.exists(thumbnailPath, {dir: BaseDirectory.AppCache})) {
        return convertFileSrc(await path.join(appCacheDir, thumbnailPath));
      } else {
        throw "Path does not exist";
      }

    }

    function onLoad(event: Event) {
      (event.currentTarget as HTMLImageElement).style.opacity = "1";
    }


</script>

<div class="grid-container">
  <div class="grid">
    {#await loadAlbums}
      Loading
    {:then albums} 
      {#each Object.entries(albums) as [id, album]}
      <button class="album" on:click={()=>{selectAlbum(id)}}>
        <div class="image-container">
          {#await imagePath(id) then path}
            <img src="{path}" alt="{album.name}" on:load={onLoad} loading="lazy">
          {/await}
        </div>
        <div class="album-details">
          <div class="album-name">{album.name}</div>        
        </div>
      </button>
      {/each}
    {/await}
  </div>
  <div class="empty-grid" class:visible={isEmpty}>
    No albums found
  </div>
</div>



<style>
    .grid-container {
      width: 100%;
      flex: 1 1 auto;
      position: relative;
    }

    .grid{
      padding: 10px;
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
      gap: 10px;
    }
  
    .album {
      background-color: transparent;
      border: none;
      padding: 0;


      cursor: pointer;
      display: flex;
      flex-direction: column;
      gap: 7px;

    }
    
    .image-container {
      border-radius: 15px;
      width: 100%;

      background-color: rgb(25, 25, 25);
      aspect-ratio: 1;
    }

    .image-container img{
      border-radius: 15px;
      display: block;
      
      width: 100%;
      height: 100%;
      object-fit: cover;
      aspect-ratio: 1;

      transition-duration: 0.3s;
      background-color: black;
      outline: none;
      border: none;
      opacity: 0;
    }

    .album-name {
      font-weight: lighter;
      color: rgba(255, 255, 255, 0.8);
      font-size: 1rem;
    }
    
    .empty-grid {
      visibility: hidden;
      position: absolute;
      width: 100%;
      height: 100%;
      top: 0;
      left: 0;
      display: flex;
      justify-content: center;
      align-items: center;
      
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      
      font-size: 1.5rem;
      font-weight: lighter;
      color: rgba(255, 255, 255, 0.8);
    }

    .visible {
      visibility: visible;
    }
  </style>