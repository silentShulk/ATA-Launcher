<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Fzf } from 'fzf';

const styles = ref<string[]>([]);
const filteredStyles = ref<string[]>([]);
const selectedStyle = ref<string>();

const filterMods = (e: Event) => {
    const fzf = new Fzf(
        styles.value,
        {
            fuzzy: "v2"
        })

    const result = fzf.find((e.target as HTMLInputElement).value)
    filteredStyles.value = result.map(entry => entry.item)
};

const isSelected = (style: string) => selectedStyle.value === style;

const selectStyle = (style: string) => {
    selectedStyle.value = style;
};

onMounted(async () => {
    styles.value = await invoke('scan_for_styles');
    filteredStyles.value = styles.value;
});
</script>



<template>
<main id="selector" class="ata-main ata-flex-column">
    <input 
        class="ata-input-text ata-colors"
        placeholder="Search style..."
        @input="(e) => filterMods(e)"
    />
    <ul class="ata-list" :class="filteredStyles.length > 0 ? 'gradient-border' : ''">
        <li v-for="(style) in filteredStyles" :key="style">
            <label class="ata-btn ata-flex ata-centered-content" :class="isSelected(style) ? 'selected-style' : ''">
                <input 
                    type="radio" 
                    name="style-group"
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
#selector {
    width: 50%;
    max-height: 100%;
}

.gradient-border {
    background: linear-gradient(transparent) padding-box,
                linear-gradient(180deg, $ata-main 0%, $ata-main 20%, $ata-accent 100%) border-box;
}

.selected {
    background-color: $ata-accent-tertiary;
}
</style>