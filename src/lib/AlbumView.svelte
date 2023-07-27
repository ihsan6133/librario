<script lang="ts">
    import {convertFileSrc} from '@tauri-apps/api/tauri';
    import { lazyLoad } from './lazyLoad';


    let path = convertFileSrc("D:\\Media\\Pictures\\errorhypixe;.png")
    let thumbnails = [
      String.raw`D:\Media\Pictures\wallpaper-park-ipad.jpg`,
      String.raw`D:\Media\Pictures\DonutRender.png`,
      String.raw`D:\Media\Pictures\Old\pictures 660.jpg`,
    ] 

    $: thumbnails = thumbnails.map((path)=> convertFileSrc(path))
</script>

<div class="grid" >
    {#each thumbnails as thumbnail}
      <div class="album">
        <img use:lazyLoad={thumbnail}  alt="">
      </div>
    {/each}    
</div>


<style>
    .grid{
      padding: 10px;
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
      gap: 10px;
    }
  
    .album {
      width: 100%;
      height: 100%;
      aspect-ratio: 1;
      background-color: gray;
      border-radius: 15px;


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