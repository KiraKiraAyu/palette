<template>
    <div class="flex flex-col items-center transition-[width] bg-light-main relative overflow-hidden p-2" :class="folded ? 'w-16' : 'w-64'">
        <BaseButton class="w-12 h-12 rounded-full absolute left-2 top-2 flex justify-center items-center p-0! border-0!" @click="toggleSideBar">
            <i-lucide-text-align-justify />
        </BaseButton>
        <FoldButton class="mt-16" @click="newChat" :folded="folded" label="New Chat">
            <i-lucide-edit />
        </FoldButton>
        <div class="flex flex-col flex-1 min-h-0 overflow-y-auto w-full" v-show="!folded && conversations">
            <div
                v-for="chat in conversations"
                :key="chat.id"
                class="group relative w-full"
            >
                <FoldButton
                    @click="selectChat(chat.id)"
                    :class="{ 'bg-light-blue!': chat.id === conversationStore.currentConversationId }"
                    :folded="folded"
                    :label="chat.title || 'No title'"
                ></FoldButton>
                <button
                    class="absolute right-2 top-1/2 -translate-y-1/2 hidden group-hover:flex items-center justify-center p-2 text-gray-400 hover:text-red-500 transition-colors cursor-pointer z-10 bg-transparent border-none"
                    @click.stop="deleteChat(chat.id)"
                    title="Delete conversation"
                >
                    <i-lucide-trash-2 />
                </button>
            </div>
        </div>
        <FoldButton class="" @click="goToSettings" :folded="folded" label="Settings">
            <i-lucide-cog />
        </FoldButton>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import BaseButton from '@/components/BaseButton.vue'
import FoldButton from './FoldButton.vue'
import { useConversationStore } from '@/stores/conversation'
import { storeToRefs } from 'pinia'

const router = useRouter()
const conversationStore = useConversationStore()
const { conversations } = storeToRefs(conversationStore)
const folded = ref(false)

const toggleSideBar = () => { folded.value = !folded.value }

const newChat = async () => {
    conversationStore.currentConversationId = null
    conversationStore.messages = []
    router.push('/chat')
}

const selectChat = (id: string) => {
    router.push(`/chat/${id}`)
}

const deleteChat = async (id: string) => {
    await conversationStore.deleteConversation(id)
}

const goToSettings = () => { router.push('/settings') }

onMounted(() => {
    conversationStore.fetchConversations()
})
</script>