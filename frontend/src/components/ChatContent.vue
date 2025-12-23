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
                    <div v-for="msg in messages" :key="msg.id" class="flex flex-col gap-2">
                        <div class="font-bold text-sm text-gray-600 uppercase">{{ msg.role }}</div>
                        <div 
                            class="bg-gray-50 p-4 rounded-2xl shadow-sm prose prose-sm max-w-none prose-pre:bg-gray-800 prose-pre:text-gray-100"
                            :class="{ 'bg-blue-50': msg.role === 'user' }"
                        >
                            <div v-html="renderMarkdown(msg.content)"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="absolute w-full flex justify-center p-4 bg-linear-to-t pt-12 transition-all duration-500"
            :class="{ 'bottom-0': messages.length > 0 || conversationStore.streaming, 'bottom-1/2': messages.length === 0 && !conversationStore.streaming }"
        >
            <ChatInput 
                class="w-5xl border border-light-blue rounded-2xl shadow-lg bg-white" 
                @send="handleSend"
                :disabled="conversationStore.streaming || !providerStore.selectedModelId"
            ></ChatInput>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, nextTick, watch } from 'vue'
import ChatInput from './ChatInput.vue'
import BaseSelect from './BaseSelect.vue'
import { useProviderStore } from '@/stores/provider'
import { useConversationStore } from '@/stores/conversation'
import { storeToRefs } from 'pinia'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'

const providerStore = useProviderStore()
const conversationStore = useConversationStore()
const { messages } = storeToRefs(conversationStore)
const messagesContainer = ref<HTMLElement | null>(null)

const md = new MarkdownIt({
    html: false,
    linkify: true,
    typographer: true,
    breaks: true
})

md.set({
    highlight: function (str, lang) {
        if (lang && hljs.getLanguage(lang)) {
            try {
                return '<pre class="hljs"><code>' +
                    hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
                    '</code></pre>';
            } catch (__) {}
        }

        return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>';
    }
})

const renderMarkdown = (content: string) => {
    return md.render(content)
}

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
</script>