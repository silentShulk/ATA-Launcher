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
<main id="selector" class="ata-main ata-flex-column">
    <input 
        class="ata-input-text ata-border-radius ata-colors-accent"
        placeholder="Search style..."
        @input="(e) => filter(e)"
    />
    <ul class="ata-list ata-spaceless ata-border-radius" :class="filteredStyles.length > 0 ? 'gradient-border' : ''">
        <li class="ata-colors ata-border" v-for="(style) in filteredStyles" :key="style">
            <label class="ata-btn ata-flex ata-centered-content" :class="isSelected(style) ? 'selected-style' : ''">
                <input 
                    type="radio" 
                    :checked="isSelected(style)" 
                    @change="selectStyle(style)"
                />
                <span>
                    {{ style }}
                </span>
            </label>
        </li>
    </ul>
</main>
</template>



<style scoped lang="scss">
.gradient-border {
    background: linear-gradient(transparent) padding-box,
                linear-gradient(180deg, $ata-main 0%, $ata-main 20%, $ata-accent 100%) border-box;
}
</style>