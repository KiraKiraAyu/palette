<template>
    <div class="flex flex-col items-center transition-[width] bg-light-main relative overflow-hidden p-2" :class="folded ? 'w-16' : 'w-64'">
        <BaseButton class="w-12 h-12 rounded-full absolute left-2 top-2 flex justify-center items-center p-0! border-0!" @click="toggleSideBar">
            <i-lucide-text-align-justify />
        </BaseButton>
        <FoldButton class="mt-16" @click="newChat" :folded="folded" label="New Chat">
            <i-lucide-edit />
        </FoldButton>
        <ul class="overflow-scroll" v-show="!folded && chats">
            <li><BaseButton v-for="chat in chats" :key="chat.id" @click="selectChat(chat.id)"></BaseButton></li>
        </ul>
        <FoldButton class="absolute bottom-2 left-2" @click="goToSettings" :folded="folded" label="Settings">
            <i-lucide-settings />
        </FoldButton>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import BaseButton from '@/components/BaseButton.vue'
import FoldButton from './FoldButton.vue'

interface Chat {
    id: string
    title: string
}

const router = useRouter()
const folded = ref(false)
const chats = ref<Chat[]>([])
const toggleSideBar = () => { folded.value = !folded.value }

const newChat = () => {
    router.push('/chat')
}
const selectChat = (id: string) => {
    router.push(`/chat/${id}`)
}
const goToSettings = () => { router.push('/settings') }
</script>