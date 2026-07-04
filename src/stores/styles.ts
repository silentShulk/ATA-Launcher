import { ref } from "vue";
import { defineStore } from "pinia";

export const useStylesStore = defineStore('style', () => {
    const avaiableStyles = ref<string[]>([]);
    const selectedStyle = ref<string>("");

    return { avaiableStyles, selectedStyle }
})