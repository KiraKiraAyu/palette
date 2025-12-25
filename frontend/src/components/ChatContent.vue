<template>
    <div class="flex flex-col h-screen relative">
        <div class="flex justify-center items-end gap-4 p-4 border-b border-gray-100 bg-white/80 backdrop-blur-sm z-10">
            <div class="w-64">
                <BaseSelect 
                    :options="providerOptions" 
                    v-model="selectedProviderId" 
                    label="Provider"
                />
            </div>
            <div class="w-64">
                <BaseSelect 
                    :options="modelOptions" 
                    v-model="selectedModelId" 
                    label="Model" 
                />
            </div>
        </div>

        <div class="flex-1 overflow-y-auto flex justify-center w-full" ref="messagesContainer">
            <div class="w-5xl p-4 pb-32">
                <div v-if="!providerStore.selectedModelId" class="text-center text-gray-400 mt-20">
                    Select a model to start chatting
                </div>
                
                <div v-else-if="messages.length === 0" class="text-center text-gray-400 mt-20">
                    Start a conversation with {{ providerStore.selectedModel }}
                </div>

                <div v-else class="flex flex-col gap-6">
                    <div v-for="(msg, index) in messages" :key="msg.id" class="flex flex-col gap-2" :class="{ 'items-end': msg.role === ChatRole.User }">
                        <div
                            class="font-bold text-sm text-gray-600 uppercase"
                            :class="{ 'text-right': msg.role === ChatRole.User }"
                        >
                            {{ msg.role }}
                        </div>
                        <div
                            class="p-4 prose prose-sm prose-pre:bg-gray-800 prose-pre:text-gray-100"
                            :class="msg.role === ChatRole.User ? 'bg-blue-50 rounded-full max-w-[50%]': 'max-w-none'"
                        >
                            <div v-if="msg.role === ChatRole.Assistant">
                                <StreamMarkdown 
                                    v-if="msg.content" 
                                    :content="msg.content" 
                                    :animate="conversationStore.streaming && index === messages.length - 1"
                                />
                                <BaseLoading v-else-if="conversationStore.streaming" class="w-4 h-4" />
                            </div>
                            <div v-else>{{ msg.content }}</div>
                        </div>
                    </div>
                    <div class="h-16"></div>
                </div>
            </div>

            <Transition name="fade">
                <div v-if="conversationStore.loading" class="absolute inset-0 bg-white/50 backdrop-blur-sm flex items-center justify-center z-20">
                    <BaseLoading class="w-8 h-8" />
                </div>
            </Transition>
        </div>
        
        <div class="absolute w-full flex justify-center p-4 bg-linear-to-t pt-12 transition-all duration-500"
            :class="{ 'bottom-0': messages.length > 0 || conversationStore.streaming, 'bottom-1/2': messages.length === 0 && !conversationStore.streaming }"
        >
            <ChatInput 
                class="w-5xl border border-light-blue rounded-2xl shadow-lg bg-white" 
                @send="handleSend"
                @stop="handleStop"
                :disabled="conversationStore.streaming || !providerStore.selectedModelId"
                :streaming="conversationStore.streaming"
            ></ChatInput>
        </div>
    </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>

<script setup lang="ts">
import { computed, ref, nextTick, watch } from 'vue'
import ChatInput from './ChatInput.vue'
import BaseSelect from './BaseSelect.vue'
import BaseLoading from './BaseLoading.vue'
import StreamMarkdown from './StreamMarkdown.vue'
import { useProviderStore } from '@/stores/provider'
import { useConversationStore } from '@/stores/conversation'
import { storeToRefs } from 'pinia'
import { ChatRole } from '@/types/conversation'

const providerStore = useProviderStore()
const conversationStore = useConversationStore()
const { messages } = storeToRefs(conversationStore)
const messagesContainer = ref<HTMLElement | null>(null)

const providerOptions = computed(() => {
    return providerStore.providers.map(p => ({
        text: p.name,
        value: p.id
    }))
})

const modelOptions = computed(() => {
    const models = providerStore.selectedProvider?.models || []
    return models.map(m => ({
        text: m.name,
        value: m.id
    }))
})

const selectedProviderId = computed({
    get: () => providerStore.selectedProviderId,
    set: (val) => {
        if (val) providerStore.selectProvider(val as string)
    }
})

const selectedModelId = computed({
    get: () => providerStore.selectedModelId,
    set: (val) => {
        if (val) providerStore.selectModel(val as string)
    }
})

const scrollToBottom = async () => {
    await nextTick()
    if (messagesContainer.value) {
        messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
}

watch(() => messages.value.length, scrollToBottom)
watch(() => messages.value[messages.value.length - 1]?.content, scrollToBottom, { deep: true })

const handleSend = async (text: string) => {
    if (!providerStore.selectedModelId) return
    
    await conversationStore.sendMessage(text, providerStore.selectedModelId)
}

const handleStop = () => {
    conversationStore.stopStreaming()
}
</script>