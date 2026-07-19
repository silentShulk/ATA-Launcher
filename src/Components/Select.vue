<script setup lang="ts">
import { ref, computed } from 'vue';
import { Fzf } from 'fzf';
import "../style/components/option.scss"
import "../style/components/select.scss"
import "../style/components/text-input.scss"

const props = defineProps<{
    elements: string[];
    selectedElement: string;
}>();

const query = ref('');

const filteredElements = computed(() => {
    const base = !query.value
        ? props.elements
        : new Fzf(props.elements, {
            selector: (e: string) => e,
            fuzzy: "v2"
        }).find(query.value).map(entry => entry.item);

    return [...base].sort((a, b) => {
        if (a === props.selectedElement) return -1;
        if (b === props.selectedElement) return 1;
        return 0;
    });
});

const filter = (e: Event) => {
    query.value = (e.target as HTMLInputElement).value;
};

const emit = defineEmits<{
    newSelection: [selection: string]
}>();

let pressTimer: ReturnType<typeof setTimeout> | null = null;
const pressedElement = ref<string | null>(null);

function startPress(e: string) {
    pressedElement.value = e;
    pressTimer = setTimeout(() => {
        emit('newSelection', e);
        pressedElement.value = null;
        pressTimer = null;
    }, 1500);
}
function cancelPress() {
    if (pressTimer) {
        clearTimeout(pressTimer);
        pressTimer = null;
    }
    pressedElement.value = null;
}
</script>



<template>
<main id="selector" class="ata-select-big palette-gradient-main-accent">
    <input
    class="ata-input-big-top palette-accent ata-h1"
    placeholder="Search style"
    @input="filter"
    />
    <ul id="style-list" class="justify-center">
        <li
        v-for="e in filteredElements"
        :key="e"
        class="listless ata-option-big palette-dark-empty"
        :class="{ 'is-pressing': pressedElement === e }"
        @mousedown="startPress(e)"
        @mouseup="cancelPress"
        @mouseleave="cancelPress"
        >
            <span class="ata-h3">{{ e }}</span>
        </li>
    </ul>
</main>
</template>



<style scoped lang="scss">
#selector {
    max-height: 90%;
    display: flex;
    flex-direction: column;
}

#style-list {
    padding:0;
    margin:0;

    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}

.listless {
    list-style: none;
    margin: 0;
    padding: 0;
}

.ata-option-big {
    &.is-pressing {
        box-shadow: inset 0 0 0 175px rgba($ata-dark-light, 0.75);
        transition: box-shadow 1000ms linear;
    }
}
</style>