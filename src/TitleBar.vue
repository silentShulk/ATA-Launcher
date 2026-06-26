<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref, onMounted } from 'vue';



const TITLES: string[] = [
    'Glory to Mankind',
    'This cannot continue',
    'Become as gods',
    'Do you think games are silly little things?',
    'The future is not given to you...',
    '...It is something you must take for yourself',
    'Thank you... Nine... s',
    'It always ends like this',
    'I never quite realized...',
    '...How beautiful this world is',
    'Emotions are prohibited',
    'You want to **** 2B, don\'t you?',
    'The world is full of beautiful things',
    'I\'m not quite sure what it means to mourn',
    'I\'m coming, everyone... I\'m... coming...',
    'Humanity: Love. Family. Religion. War.',
    'Shut up... SHUT UP!',
    'Ha ha ha... ha ha ha hah!',
    'The possibility of a different future also exists',
    'Everything that lives is designed to end...',
    '...We are are perpetually trapped...',
    '...In a never-ending spiral of life and death...',
    '...Is this a curse? Or some kind of punishment?...',
    '...I often think about the god who blessed us with this cryptic puzzle...',
    '...And wonder if we\'ll ever have the chance to kill him',
    '...They are perpetually trapped...',
    '...However, life is all about the struggle within that cycle...',
    '...That is what \"we\" believe',

    'The future doesn\'t belong to you (neither does this phrase belong here)'
]

const appWindow = getCurrentWindow();
const title = ref('');

function randomizeTitle() {
    const randomTitle = TITLES[Math.floor(Math.random() * TITLES.length)];
    appWindow.setTitle(randomTitle)
    title.value = randomTitle;
}

onMounted(async () => {
    randomizeTitle();
});
</script>



<template>
<div data-tauri-drag-region class="titlebar" tabindex="-1">
    <span data-tauri-drag-region tabindex="-1"> Accord's Timeline Alterer </span>
    <button class='title' @click="randomizeTitle()" tabindex="-1">{{ title }}</button>
    <div class="titlebar-buttons" tabindex="-1">
        <button id="minimize" @click="appWindow.minimize()" tabindex="-1">−</button>
        <button id="maximize" @click="appWindow.toggleMaximize()" tabindex="-1">□</button>
        <button id="close" @click="appWindow.close()" tabindex="-1">✕</button>
    </div>
</div>
</template>



<style scoped lang="scss">
.titlebar {
    height: 32px;
    flex-shrink: 0;
    
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    
    user-select: none;
    -webkit-user-select: none;

    background-color: floralwhite;
    padding: 0 10px 0 10px;
}

.title {
    border: none;
    background: none;
    font-family: JetBrains Mono;
    font-size: 1.1em;
    font-style: italic;
    
    justify-self: center;

    user-select: none;
    -webkit-user-select: none;

    cursor: pointer;
}

.titlebar-buttons {
    justify-self: end;
    display: flex;
    gap: 5px;
}

#minimize, #maximize, #close {
    width: 25px;
    height: 25px;

    border: none;
    border-radius: 15px;
}
#minimize {
    background-color: $ata-main;
}
#maximize {
    background-color: $ata-accent;
}
#close {
    background-color: $ata-accent-secondary;
}
</style>