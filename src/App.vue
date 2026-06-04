<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";



const installationState = ref<string>("");
const selectedUI = ref("");

onMounted(async () => {
    const result: string = await invoke("check_installation_state");
    installationState.value = result;

    const uiResult: string = await invoke("get_selected_ui");
    selectedUI.value = uiResult;
});
</script>



<template>
<div id="app">
    <header class="ata-header">
        <h1 class="ata-title"> ATA Launcher </h1>
    </header>

    <main class="ata-main ata-flex-column">
        <div id="features" class="ata-flex ata-centered-content">
            <div id="installation" class="ata-flex-column ata-centered-content">
                <button id="install" class="ata-btn"> Install ATA </button>
                <p>{{ installationState }}</p>
            </div>
            <div id="ui-selection" class="ata-flex-column ata-centered-content">
                <button id="select" class="ata-btn">
                    Select UI
                </button>
                <p>{{ selectedUI }}</p>
            </div>
        </div>
        <div id="launcher" class="ata-centered-content"
        v-if="installationState === 'Everything is Installed'">
            <button id="launch" class="ata-btn">
                LAUNCH ATA
            </button>
        </div>
        <div v-else class="ata-centered-content">
            <h2 class="ata-colors-critical"> SOMETHING IS WRONG<br>FIX YOUR ATA INSTALLATION </h2>
        </div>
    </main>
</div>
</template>



<style lang="scss">
#app {
    display: flex;
    flex-direction: column;    
    
    height: 100vh;
    width: 100%;
    
    background-color: $ata-main;
    font-family: Jetbrains Mono;

    overflow: hidden;
}

#install, #select, #launch {
    background-color: transparent;
    color: $ata-black;
}

#features {
    width: 100%;
    height: 50vh;
}
.installation, #ui-selection {
    width: 50%;
}
#install, #select {
    padding: 15px;
    
    border: 2px solid $ata-accent;

    font-size: 1.25em;
}
#install:hover, #select:hover {
    background-color: $ata-accent;
    color: $ata-main;
    border: 2px solid $ata-black;
}

#launcher {
    height: 50vh;
}
#launch {
    width: 40%;
    height: 40%;

    border: 2px solid $ata-accent-secondary;

    font-size: 1.5em;
    font-weight: bold;
}
#launch:hover {
    background-color: $ata-accent-secondary;
    color: $ata-accent-tertiary;
    border: 2px solid $ata-black;
}



.ata-header {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    
    text-align: center;

    color: $ata-accent;
}
.ata-title {
    font-size: 40px;
}

.ata-main {
    display: flex;
    flex-grow: 1;

    min-height: 0;
    margin: 10px;

    gap: 10px;
}

.ata-btn {
    border-radius: 5px;

    cursor: pointer;
}

.ata-truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
.ata-spaceless {
    margin: 0;
    padding: 0;
}
.ata-flex {
    display: flex;
    align-items: center;
    gap: 10px;
}
.ata-flex-column {
    display: flex;
    flex-direction: column;
    gap: 5px;
}
.ata-centered {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
.ata-centered-content {
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
}

.ata-colors {
    background-color: $ata-accent;
    color: $ata-main;
}
.ata-colors-secondary {
    background-color: $ata-accent-secondary;
    color: $ata-accent-tertiary;
}
.ata-colors-critical {
    background-color: $ata-accent-secondary;
    color: $ata-black;
    
    box-shadow: red 0px 5px 15px;
}

body, html {
    margin: 0;
    padding: 0;
}
</style>