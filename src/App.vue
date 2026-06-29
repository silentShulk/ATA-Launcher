<script setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import List from "./Components/List.vue";
import InstallationState from "./Components/InstallationState.vue";
import { useStateStore } from "./stores/state"
import TitleBar from "./TitleBar.vue";
import "./style/components/button.scss"



const stateStore = useStateStore()

onMounted(async () => {
    stateStore.installationState = await invoke("check_installation_state");

    stateStore.selectedStyle = await invoke("get_selected_style");
});
</script>



<template>
    <TitleBar />
    <div id="ata-app">
        <header class="ata-h1 palette-main">
            <h1 class="ata-title"> ATA Launcher </h1>
        </header>

        <InstallationState :state="stateStore" />

        <main id="style" class="ata-main ata-flex ata-centered-content">
            <button class="ata-btn-medium-big palette-dark-bad ata-h2"> Remove Style</button>
            <div id="style-selector">
                <List :state="stateStore" />
            </div>
            <button class="ata-btn-medium-big palette-dark-good ata-h2"> Add Style </button>
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

#style {
    height: 50%;
    width: 100%;
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

body,
html {
    margin: 0;
    padding: 0;
}
</style>
