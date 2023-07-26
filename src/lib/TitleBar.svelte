<script lang="ts">
    import {appWindow} from "@tauri-apps/api/window";
    
    function close() {
        appWindow.close();
    }
</script>

<div data-tauri-drag-region class="flex" >
    <div class="title-container">
        Librario
    </div>
    <div class="button-container">
        <button class="button minimize" on:click={appWindow.minimize}></button>
        <button class="button maximize"></button>
        <button class="button close"></button>
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
        padding-left: 10px;

        color: rgb(200, 200, 200);
    }
    .button-container {
        display: flex;
        width: 140px;
        align-items: center;

    }
    .button {

        background-color: transparent;
        border: none;

        display: flex;
        justify-content: center;
        align-items: center;
        flex: 1;
        background-image: url('/minimize-maximize-close-icons.svg');
        filter: invert(80%);
        background-repeat: no-repeat;
        background-size: 30%;
        background-position: center;
        height: 100%;
        transition-duration: 0.1s;
    }

    .button:hover {
        background-color: rgb(255, 255, 255);
    }

    .minimize {
        background-image: url('/minimize-icon.svg');
    }
    .maximize {
        background-image: url('/maximize-icon.svg');
    }

    .close {
        background-image: url('/close-icon.svg');
    }

    .close:hover {
        background-color: cyan;
    }
</style>