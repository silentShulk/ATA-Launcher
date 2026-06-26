<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useStateStore } from '../stores/state'



const props = defineProps<{
  state: ReturnType<typeof useStateStore>
}>()

async function createFolders() {
    await invoke("create_folders")
    
    props.state.installationState = await invoke("check_installation_state")
}
async function createDefaultData() {
    await invoke("create_default_data")

    props.state.installationState = await invoke("check_installation_state")
}
async function createDefaultSettings() {
    await invoke("create_default_settings")

    props.state.installationState = await invoke("check_installation_state")
}
</script>



<template>
    <div id="installation-components" class="ata-grid">
        <button id="folders" class="ata-btn ata-border-radius ata-shadow"
            :class="{ 'ata-colors-enabled': props.state.installationState[0], 'ata-colors-disabled': !props.state.installationState[0] }"
            @click="createFolders()"> Folders </button>
        <button id="executable" class="ata-btn ata-border-radius ata-shadow"
            :class="{ 'ata-colors-enabled': props.state.installationState[1], 'ata-colors-disabled': !props.state.installationState[1] }"> Executable
        </button>
        <button id="data" class="ata-btn ata-border-radius ata-shadow"
            :class="{ 'ata-colors-enabled': props.state.installationState[2], 'ata-colors-disabled': !props.state.installationState[2] }"
            @click="createDefaultData()"> Data File </button>
        <button id="settings" class="ata-btn ata-border-radius ata-shadow"
            :class="{ 'ata-colors-enabled': props.state.installationState[3], 'ata-colors-disabled': !props.state.installationState[3] }"
            @click="createDefaultSettings()"> Settings File </button>
    </div>
</template>



<style lang="scss">
#installation-components {
    display: grid;
    grid-auto-flow: row;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(1, 1fr);
    grid-template-areas: 
        "folders executable data settings";
}
#executable { grid-area: executable;}
#folders { grid-area: folders;}
#data {grid-area: data;}
#settings {grid-area: settings;}
</style>
