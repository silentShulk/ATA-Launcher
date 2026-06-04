<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import List from "./Components/List.vue";



const installationState = ref<[boolean, boolean, boolean, boolean]>([false, false, false, false]);
const selectedStyle = ref("");

onMounted(async () => {
    installationState.value = await invoke("check_installation_state");

    selectedStyle.value = await invoke("get_selected_style");
});
</script>



<template>
<div id="app">
    <header class="ata-header">
        <h1 class="ata-title"> ATA Launcher </h1>
    </header>

    <main class="ata-main">
        <div id="features">
            <div id="installation-state" class="ata-flex-column ata-centered-content">
                <h2> State of ATA<br>installation </h2>
                <div id="installation-components" class="ata-grid">
                    <button id="folders" class="ata-btn ata-small" :class="{'btn-enabled': installationState[1]}"> Folders </button>
                    <button id="executable" class="ata-btn ata-small" :class="{'btn-enabled': installationState[0]}"> Executable </button>
                    <button id="data" class="ata-btn ata-small" :class="{'btn-enabled': installationState[2]}"> Data File </button>
                    <button id="settings" class="ata-btn ata-small" :class="{'btn-enabled': installationState[3]}"> Settings File </button>
                </div>
                <p> Click any of the buttons to<br>reinstall/reset that component </p>
            </div>
            <div id="style-selection" class="ata-flex-column ata-centered-content">
                <h2> {{ selectedStyle }} </h2>
                <List />
            </div>
        </div>
    </main>

    <div id="launch" class="ata-centered-content">
        <button class="ata-btn ata-colors-others">
            <h3> LAUNCH ATA </h3>
        </button>
    </div>
        
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

#features {
    display: flex;
    
    width: 100%;
    height: 100%;

    align-items: top;
}

#installation-state {
    width: 50%;
}
#installation-components {
    grid-template-areas: 
        "folders executable"
        "data settings";
}
#executable { grid-area: executable;}
#folders { grid-area: folders;}
#data {grid-area: data;}
#settings {grid-area: settings;}

.btn-enabled {
    background-color: $ata-accent-tertiary;
    color: $ata-black
}
.btn-disalbed {
    background-color: $ata-accent-secondary;
    color: $ata-black
}

#style-selection {
    width: 50%;
}

#launch {
    height: 20%;
}



.ata-page {
    display: flex;
    flex-direction: column;

    width: 100%;
    height: 100%;
    min-height: 0;
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
.ata-title.subtitle {
    font-size: 25px;
}

.ata-main {
    display: flex;
    flex-grow: 1;

    min-height: 0;
    margin: 10px;

    gap: 10px;
}

.ata-footer {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    
    width: 100%;
    
    justify-content: center;
    align-items: center;
    
    background-color: $ata-accent-secondary;
    color: $ata-black;
}

.ata-btn {
    border-radius: 15px;

    cursor: pointer;
}
.ata-btn-back {
    border-radius: 5px;

    position: absolute;
    top: 50px;
    left: 15px;
    
    width: 30px;
    height: 30px;

    background-color: $ata-accent-tertiary;
    color: $ata-black;

    cursor: pointer;   
}

.ata-grid {
    display: grid;
    flex-grow: 1;
    grid-auto-flow: column;

    padding: 5px 5px 5px 5px;
    
    gap: 10px;
}

.ata-input-text {
    padding: 5px 5px 5px 5px;
    
    border-radius: 5px;

    resize: none;
}

.ata-select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;

    padding: 5px 30px 5px 10px;
    border-radius: 5px;

    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'%3E%3Cpolyline points='1,1 6,7 11,1' fill='none' stroke='%23d6cab2' stroke-width='2' stroke-linecap='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 10px center;
    background-size: 12px;
}

.ata-list {
    border: 5px solid transparent;
    border-radius: 15px;
    
    padding: 0;

    list-style: none;
    overflow-y: scroll;
}
.ata-list > * > * {
    border-right: 4px solid $ata-black;
    border-left: 4px solid $ata-black;
    border-top: 2px solid $ata-black;
    border-bottom: 2px solid $ata-black;
}

.ata-checkbox {
    width: 30px;
    height: 30px;
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
.ata-small {
    margin: 5px 5px 5px 5px;
    padding: 5px 5px 5px 5px;
}
.ata-centered-content {
    display: flex;
    align-items: center;
    justify-content: center;
}

.ata-colors {
    background-color: $ata-accent;
    color: $ata-main;
}
.ata-colors-others {
    background-color: $ata-accent-tertiary;
    color: $ata-accent-secondary;
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