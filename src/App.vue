<script setup>
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Select from "./Components/Select.vue";
import InstallationState from "./Components/InstallationState.vue";
import { useStateStore } from "./stores/state"
import { useStylesStore } from "./stores/styles"
import TitleBar from "./TitleBar.vue";
import "./style/components/button.scss"



const stateStore = useStateStore()
const stylesStore = useStylesStore()

onMounted(async () => {
    stateStore.installationState = await invoke("check_installation_state");

    stylesStore.avaiableStyles = await invoke("scan_for_styles")
    stylesStore.selectedStyle = await invoke("get_selected_style");
});
</script>



<template>
    <TitleBar />
    <div id="ata-app">
        <header class="palette-main">
            <h1 class="spaceless ata-h1"> ATA Launcher </h1>
        </header>

        <InstallationState :state="stateStore"/>

        <main id="style" class="ata-main justify-space-evenly">
            <button class="ata-btn-medium-big palette-dark-bad ata-h2 centered-self-v"> Remove Style</button>
            <div id="style-selector">
                <Select :elements="stylesStore.avaiableStyles" :selectedElement="stylesStore.selectedStyle"/>
            </div>
            <button class="ata-btn-medium-big palette-dark-good ata-h2 centered-self-v"> Add Style </button>
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
    height: 60%;
    width: 100%;
}

.ata-main {
    display: flex;
    flex-grow: 1;

    min-height: 0;
    margin: 10px;

    gap: 10px;
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    place-items: center;

    gap: 10px;
}


.truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    word-wrap: normal;
}
.spaceless {
    margin: 0;
    padding: 0;
}

.flex {
    display: flex;
    align-items: center;
    gap: 10px;
}
.flex-column {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.justify-center {
    justify-content: center;
}
.justify-space-evenly {
    justify-content: space-evenly;
}
.centered-self-v {
    display: flex;
    align-self: center;
}

body,
html {
    margin: 0;
    padding: 0;
}
</style>
