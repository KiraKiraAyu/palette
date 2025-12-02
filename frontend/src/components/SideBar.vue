<template>
    <div class="flex flex-col items-center transition-[width] bg-light-main relative overflow-hidden p-2" :class="folded ? 'w-16' : 'w-64'">
        <BaseButton class="w-12 h-12 rounded-full absolute left-2 top-2 flex justify-center items-center p-0! border-0!" @click="toggleSideBar">
            <i-lucide-text-align-justify />
        </BaseButton>
        <BaseButton @click="newChat" class="mt-16 h-12 aspect-square flex flex-nowrap w-full items-center gap-2 p-0! border-0!">
            <div class="flex h-full aspect-square justify-center items-center shrink-0">
                <i-lucide-edit />
            </div>
            <span class="transition-opacity whitespace-nowrap" :class="folded ? 'opacity-0' : ''">New Chat</span>
        </BaseButton>
        <ul v-show="!folded && chats" class="overflow-scroll">
            <li><BaseButton v-for="chat in chats" :key="chat.id" @click="selectChat(chat.id)"></BaseButton></li>
        </ul>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import BaseButton from '@/components/BaseButton.vue'

interface Chat {
    id: string
    title: string
}

const folded = ref(false)
const chats = ref<Chat[]>([])
const toggleSideBar = () => { folded.value = !folded.value }

const newChat = () => {}
const selectChat = (id: string) => {}
</script>