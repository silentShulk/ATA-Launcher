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

const emit = defineEmits<{
    select: [element: string];
}>();

const query = ref('');

const LONG_PRESS_MS = 750;
let pressTimer: ReturnType<typeof setTimeout> | null = null;
const pressedElement = ref<string | null>(null);

const onPressStart = (e: string) => {
    pressedElement.value = e;
    pressTimer = setTimeout(() => {
        pressTimer = null;
        pressedElement.value = null;
        emit('select', e);
    }, LONG_PRESS_MS);
};

const onPressCancel = () => {
    pressedElement.value = null;
    if (pressTimer) {
        clearTimeout(pressTimer);
        pressTimer = null;
    }
};

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
        class="listless ata-option-big"
        >
            <button
            class="palette-dark-empty li-btn"
            :class="{ selecting: pressedElement === e }"
            :style="{ '--press-duration': LONG_PRESS_MS + 'ms' }"
            @pointerdown="onPressStart(e)"
            @pointerup="onPressCancel"
            @pointerleave="onPressCancel"
            @pointercancel="onPressCancel"
            >
                <span class="ata-h3">{{ e }}</span>
            </button>
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

.li-btn {
    width: 100%;
    height: 100%;

    border: none;
    background-color: transparent;
    transition: background-color var(--press-duration, 0.5s) ease-out;
}

.selecting {
    background-color: rgba($ata-dark-light, 0.5);
}
</style>