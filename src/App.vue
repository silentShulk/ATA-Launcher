<script setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import List from "./Components/List.vue";
import InstallationState from "./Components/InstallationState.vue";
import { useStateStore } from "./stores/state"
import TitleBar from "./TitleBar.vue";



const stateStore = useStateStore()

onMounted(async () => {
    stateStore.installationState = await invoke("check_installation_state");

    stateStore.selectedStyle = await invoke("get_selected_style");
});
</script>



<template>
<TitleBar/>
<div id="ata-app">
    <header class="ata-header">
        <h1 class="ata-title"> ATA Launcher </h1>
    </header>

    <InstallationState :state="stateStore" />
    
    <main class="ata-main ata-flex">
        <button class="ata-btn ata-colors-black ata-border-radius ata-centered-content"></button>
        <div id="style-selection">
            <List :state="stateStore"/>
        </div>
        <button class="ata-btn ata-colors-black ata-border-radius"></button>
    </main>
</div>
</template>



<style lang="scss">
#ata-app {
    display: flex;
    flex-direction: column;    
    
    height: 100vh;
    width: 100vw;
    
    background-color: $ata-main;
    font-family: Jetbrains Mono;

    overflow: hidden;
}
#style-selection {
    width: 60%;
    max-height: 100%;
}
#launch {
    height: 20%;
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
    margin: 5px;
    padding: 15px;

    cursor: pointer;
}
.ata-btn-small {
    margin: 5px;
    padding: 5px;

    cursor: pointer;
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    gap: 10px;
}

.ata-input-text {
    padding: 5px 5px 5px 5px;

    font-size: 1em;
    
    resize: none;
}

.ata-list {
    list-style: none;
    overflow-y: scroll;
}
.ata-list-item {
    border: 3px solid $ata-dark;
}

.ata-truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    word-wrap: normal;
}
.ata-spaceless {
    margin: 0;
    padding: 0;
}
.ata-border-radius {
    border-radius: 15px;
}
.ata-shadow {
    box-shadow: 0 1px 2px 1px $ata-dark;

    &:hover {
        box-shadow: 0 1px 2px 1px $ata-dark, 0 2px 3px 2px $ata-dark-light;
    }
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
.ata-centered-content {
    display: flex;
    align-items: center;
    justify-content: center;
}

// ATA-COLORS
.ata-colors {
    background-color: $ata-main;
    color: $ata-accent;
}
.ata-colors-accent {
    background-color: $ata-accent;
    color: $ata-main;

    border: 5px solid $ata-accent-dark;

    &:hover {
        background-color: $ata-accent-dark;
    }
}
.ata-colors-secondary {
    background-color: $ata-accent-secondary;
    color: $ata-accent-tertiary;
    
    border: 5px solid $ata-accent-secondary-dark;

    &:hover {
        background-color: $ata-accent-secondary-dark;
    }
}
.ata-colors-tertiary {
    background-color: $ata-accent-tertiary;
    color: $ata-accent-secondary;
    
    border: 5px solid $ata-accent-tertiary-dark;

    &:hover {
        background-color: $ata-accent-tertiary-dark;
    }
}
.ata-colors-critical {
    background-color: $ata-accent-secondary;
    color: $ata-dark;

    border: 5px solid $ata-accent-secondary-dark;
    
    box-shadow: $ata-accent-secondary-dark 0 5px 15px 5px;
}
.ata-colors-black {
    background-color: $ata-dark-light;
    color: white;

    border : 5px solid $ata-dark;

    box-shadow: $ata-dark 0 2px 5px 2px;
}
.ata-colors-enabled {
    background-color: $ata-accent-tertiary;
    color: $ata-dark;

    border: 5px solid $ata-accent-tertiary-dark;

    &:hover {
        background-color: $ata-accent-tertiary-dark;
    }
}
.ata-colors-disabled {
    background-color: $ata-accent-secondary;
    color: $ata-dark;

    border: 5px solid $ata-accent-secondary-dark;

    &:hover {
        background-color: $ata-accent-secondary-dark;
    }
}

body, html {
    margin: 0;
    padding: 0;
}
</style>
