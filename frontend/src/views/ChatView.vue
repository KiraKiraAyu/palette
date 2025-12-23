<template>
    <div class="flex-1 h-full flex">
        <SideBar></SideBar>
        <ChatContent class="flex-1"></ChatContent>
    </div>
</template>

<script setup lang="ts">
import { watch, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import ChatContent from '@/components/ChatContent.vue'
import SideBar from '@/components/SideBar.vue'
import { useConversationStore } from '@/stores/conversation'

const route = useRoute()
const conversationStore = useConversationStore()

const handleRouteParam = () => {
    const id = route.params.id as string
    if (id) {
        conversationStore.selectConversation(id)
    } else {
        conversationStore.currentConversationId = null
        conversationStore.messages = []
    }
}

watch(() => route.params.id, handleRouteParam)

onMounted(() => {
    handleRouteParam()
})
</script>
