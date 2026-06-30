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
<main id="selector" class="ata-select-big">
    <input 
        id="style-filterer"
        class="ata-input-big-top palette-accent"
        placeholder="Search style..."
        @input="(e) => filter(e)"
    />
    <ul id="style-list"class="ata-list ata-spaceless ata-border-radius">
        <li id="style" class="ata-colors ata-centered-content ata-border-radius ata-list-item" v-for="(style) in filteredStyles" :key="style">
            <label id="style" class="ata-btn ata-flex ata-centered-content ata-border" :class="isSelected(style) ? 'selected-style' : ''">
                <input
                    class="ata-button"
                    type="radio" 
                    :checked="isSelected(style)" 
                    @change="selectStyle(style)"
                />
                <span class="ata-truncate">
                    {{ style }}
                </span>
            </label>
        </li>
    </ul>
</main>
</template>



<style scoped lang="scss">
#selector {
    height: 90%;
}

#style-filterer {
    margin-bottom: 0;
    padding-bottom: 0;

    width: 95%
}
#style-list {
    margin-top: 0;
    padding-top: 0;

    width: 100%
}
</style>