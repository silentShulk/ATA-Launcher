<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Fzf } from 'fzf';
import { useStateStore } from '../stores/state';
import "../style/components/select.scss"
import "../style/components/text-input.scss"



const styles = ref<string[]>([]);
const filteredStyles = ref<string[]>([]);

const props = defineProps<{
    state: ReturnType<typeof useStateStore>
}>()

const filter = (e: Event) => {
    const fzf = new Fzf(
        styles.value,
        {
            fuzzy: "v2"
        })

    const result = fzf.find((e.target as HTMLInputElement).value)
    filteredStyles.value = result.map(entry => entry.item)
};

const isSelected = (style: string) => props.state.selectedStyle === style;

async function selectStyle(style: string) {
    await invoke("set_selected_style", {selectedStyle: style})

    props.state.selectedStyle = style;
};

onMounted(async () => {
    styles.value = await invoke('scan_for_styles');
    filteredStyles.value = styles.value;
});
</script>



<template>
<main id="selector" class="ata-select-big palette-gradient-main-accent">
    <input 
        class="ata-input-big-top palette-accent ata-h1"
        placeholder="Search style"
        @input="(e) => filter(e)"
    />
    <ul id="style-list" class="justify-center">
        <li v-for="style in styles" class="listless ata-option-big palette-dark-empty">
            <input
                type="radio" 
                :checked="isSelected(style)" 
                @change="selectStyle(style)"
            />
            <span class="ata-h3"> {{style}} </span>
        </li>
    </ul>
</main>
</template>



<style scoped lang="scss">
#selector {
    max-height: 90%;
}

#style-list {
    padding:0;
    margin:0;
}

.listless {
    list-style: none;
    margin: 0;
    padding: 0;
}
</style>