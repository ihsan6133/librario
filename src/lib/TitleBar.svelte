<script lang="ts">
    import {appWindow} from "@tauri-apps/api/window";
    import AppIcon from "../../src-tauri/icons/32x32.png";

    function close() {  
        appWindow.close();
    }

    async function getTitle() {
        return await appWindow.title();
    }

    setInterval(()=>{
        getTitle().then(t=>{title = t});
    }, 1000);

    let title;
    getTitle().then(t=>{title = t});

    appWindow.onResized(({payload: size})=>{
        appWindow.isMaximized().then(v => {maximized = v})
    })
    
    let maximized = false;
    appWindow.isMaximized().then(v => {maximized = v})
</script>

<div data-tauri-drag-region class="flex" >
    <div data-tauri-drag-region class="title-container">
        <img data-tauri-drag-region src="{AppIcon}" alt="" class="icon">
        <div data-tauri-drag-region class="title">
            {title}
        </div>
    </div>
    <div class="button-container">
        <button class="button minimize" on:click={appWindow.minimize}>
            <svg x="0px" y="0px" viewBox="0 0 10.2 1"><rect x="0" y="50%" width="10.2" height="1" /></svg>
        </button>
        <button class="button maximize" on:click={appWindow.toggleMaximize}>
            {#if maximized}
                <svg width="10" height="10" viewBox="0 0 10 10">    
                    <path fill="#000" d="M1.667 0v1.667H2.5V.833h6.667V7.5h-.834v.833H10V0H1.667Z"/>
                    <path fill="#000" fill-rule="evenodd" d="M0 1.667V10h8.333V1.667H0Zm.833 7.5H7.5V2.5H.833v6.667Z" clip-rule="evenodd"/>
                </svg>
            {:else}
                <svg viewBox="0 0 10 10"><path d="M0,0v10h10V0H0z M9,9H1V1h8V9z" /></svg>
            {/if}
            
        </button>
        <button class="button close" on:click={appWindow.close}>
            <svg viewBox="0 0 10 10"><polygon points="10.2,0.7 9.5,0 5.1,4.4 0.7,0 0,0.7 4.4,5.1 0,9.5 0.7,10.2 5.1,5.8 9.5,10.2 10.2,9.5 5.8,5.1" /></svg>
        </button>
    </div>
</div>

<style>

    .flex {
        display: flex;
        height: 32px;
        background-color: rgb(15, 15, 15);
        justify-content: space-between;
    }

    .title-container {
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        gap: 10px;

        padding-left: 7px;

        color: rgb(200, 200, 200);
    }

    .icon {
        height: 18px;
    }
    .title {
        cursor: default;
    }

    .button-container {
        display: flex;
        align-items: center;
    }
    .button {
        margin: 0;
        width: 48px;
        height:32px;
        border: 0;
        outline: 0;
        background: transparent;
    }
    .button svg {
        width: 10px;
        height: 10px;
    }

    svg path, 
    svg rect, 
    svg polygon {
        fill: #fff;
    }

    .button:hover {
        background: rgba(255,255,255,.1);
    }

    .close:hover {
        background-color: #e81123;
    }
</style>