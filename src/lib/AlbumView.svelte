<script lang="ts">
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
    import { fs, path } from '@tauri-apps/api';

    
    import { lazyLoad } from './lazyLoad';
    import { BaseDirectory } from '@tauri-apps/api/path';

    
    let long_func = invoke("long_function");

    async function imagePath(id: string) {
      const appDataPath: string = await path.appDataDir();
      const thumbnailPath: string = await path.join("thumbnails", `${id}.webp`);

      if (await fs.exists(thumbnailPath, {dir: BaseDirectory.AppData})) {
        return await path.join(appDataPath, "thumbnails", `${id}.webp`);
      } else {
        return null;
      }

    }

    function loadThumbnail(image: HTMLImageElement, id: string) {
      imagePath(id).then(path=>{
        if (path === null) {
          lazyLoad(image, path); 
        }
      })
    }

</script>

<div class="grid" >
    {#await long_func}
      Loading
    {:then albums} 
      {#each Object.entries(albums) as [id, album]}
        <div class="album">
          <img use:loadThumbnail={id} alt="">        
        </div>
      {/each}
    {/await}

</div>


<style>
    .grid{
      padding: 10px;
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
      gap: 10px;
    }
  
    .album {
      display: flex;
      justify-content: center;
      align-items: center;

      width: 100%;
      height: 100%;
      aspect-ratio: 1;
      background-color: gray;
      border-radius: 15px;
    }
    .album-name {
      color: black;
    }

    .album img{
      border-radius: inherit;
      display: block;

      width: 100%;
      height: 100%;
      object-fit: cover;
      aspect-ratio: 1;
  
      transition-duration: 0.3s;
      opacity: 0;
    }
  </style>