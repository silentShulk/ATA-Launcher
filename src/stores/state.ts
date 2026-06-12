import { ref } from "vue";
import { defineStore } from "pinia";

export const useStateStore = defineStore('state', () => {
    const installationState = ref<[boolean, boolean, boolean, boolean]>([false, false, false, false]);
    const selectedStyle = ref("");

    return { installationState, selectedStyle }
})