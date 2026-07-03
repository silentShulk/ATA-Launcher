<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Fzf } from 'fzf';



const props = defineProps<{
    elements: string[];
    selectedElement: string;
}>();

const filteredElements = ref<string[]>(props.elements);

const filter = (e: Event) => {
    const query = (e.target as HTMLInputElement).value;
    if (!query) {
        filteredElements.value = props.elements;
        return;
    }
    const fzf = new Fzf(props.elements, { fuzzy: 'v2' });
    filteredElements.value = fzf.find(query).map((entry) => entry.item);
};

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();
const select = (item: string) => emit('update:modelValue', item);

onMounted(async () => {
})
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
        v-for="item in filteredElements"
        :key="item"
        class="listless ata-option-big palette-dark-empty"
      >
        <input
          type="radio"
          name="selector-group"
          :checked="item === selectedElement"
          @change="select(item)"
        />
        <span class="ata-h3">{{ item }}</span>
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