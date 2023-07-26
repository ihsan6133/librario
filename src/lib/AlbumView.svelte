<script lang="ts">
    import {convertFileSrc} from '@tauri-apps/api/tauri';

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
      <img src="{thumbnail}" alt="" class="album">
    {/each}    
</div>


<style>
    .grid{
      padding: 10px;
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(125px, 1fr));
      gap: 10px;
    }
  
    .album{
      width: 100%;
      height: 100%;
      object-fit: cover;
      aspect-ratio: 1;
  
      display: flex;
      justify-content: center;
      align-items: center;
      color: black;
      font-size: 2rem;
  
      background-color: gray;
      border-radius: 15px;
    }
  </style>