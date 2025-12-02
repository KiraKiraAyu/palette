<template>
    <div class="flex justify-center relative">
        <div class="w-5xl">
            <div class="overflow-scroll"></div>
        </div>
        <div class="absolute w-full bottom-0 flex justify-center p-4">
            <ChatInput class="w-5xl border border-light-blue rounded-2xl"></ChatInput>
        </div>
    </div>
</template>

<script setup lang="ts">
import ChatInput from './ChatInput.vue'

</script>