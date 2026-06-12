<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Fzf } from 'fzf';
import { useStateStore } from '../stores/state';

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
<main id="selector" class="ata-main ata-flex-column ata-border-radius ata-centered-content" :class="filteredStyles.length > 0 ? 'gradient-border' : ''">
    <input 
        id="style-filterer"
        class="ata-input-text ata-border-radius ata-colors-accent"
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

.gradient-border {
    border: 5px solid transparent; 

    background-image: 
        linear-gradient($ata-main, $ata-accent-dark), 
        linear-gradient(180deg, $ata-main 0%, $ata-accent-dark 100%);
    
    background-clip: padding-box, border-box;
    
    background-origin: padding-box, border-box;
}

#style {
    padding: 2px 0 2px 0;
}
</style>