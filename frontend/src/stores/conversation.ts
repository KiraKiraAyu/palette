import { defineStore } from "pinia"
import { ref } from "vue"
import { type ConversationSession, type ConversationMessage, ChatRole } from "@/types/conversation"
import { 
    listConversationsApi, 
    createConversationApi, 
    listMessagesApi, 
    sendMessageApi,
    deleteConversationApi
} from "@/api/conversation"

export const useConversationStore = defineStore("conversation", () => {
    const conversations = ref<ConversationSession[]>([])
    const currentConversationId = ref<string | null>(null)
    const messages = ref<ConversationMessage[]>([])
    const isLoading = ref(false)
    const isStreaming = ref(false)
    const abortStream = ref<(() => void) | null>(null)

    const fetchConversations = async () => {
        isLoading.value = true
        try {
            conversations.value = (await listConversationsApi()).items
        } finally {
            isLoading.value = false
        }
    }

    const createConversation = async () => {
        const res = await createConversationApi()
        conversations.value.unshift(res as unknown as ConversationSession)
        return res.id
    }
 
    const deleteConversation = async (id: string) => {
        await deleteConversationApi(id)
        conversations.value = conversations.value.filter(c => c.id !== id)
        if (currentConversationId.value === id) {
            currentConversationId.value = null
            messages.value = []
        }
    }

    const selectConversation = async (id: string) => {
        currentConversationId.value = id
        isLoading.value = true
        try {
            messages.value = (await listMessagesApi(id)).items
        } finally {
            isLoading.value = false
        }
    }

    const sendMessage = async (content: string, modelId: string) => {
        if (!currentConversationId.value) {
            currentConversationId.value = await createConversation()
        }

        const conversationId = currentConversationId.value!

        const userMsg: ConversationMessage = {
            id: 'temp-' + Date.now(),
            session_id: conversationId,
            role: ChatRole.User,
            content: content,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
        }
        messages.value.push(userMsg)

        const assistantMsg: ConversationMessage = {
            id: 'temp-ai-' + Date.now(),
            session_id: conversationId,
            role: ChatRole.Assistant,
            content: '',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
        }
        messages.value.push(assistantMsg)

        const assistantMsgReactive = messages.value[messages.value.length - 1]
        isStreaming.value = true

        if (!assistantMsgReactive) {
            isStreaming.value = false
            return
        }

        abortStream.value = sendMessageApi(
            conversationId,
            { content, provider_model_id: modelId },
            (chunk) => {
                try {
                    if (chunk.startsWith('"') && chunk.endsWith('"')) {
                        try {
                        const parsed = JSON.parse(chunk)
                        assistantMsgReactive.content += parsed
                        } catch {
                        assistantMsgReactive.content += chunk
                        }
                    } else {
                        assistantMsgReactive.content += chunk
                    }
                } catch (e) {
                    assistantMsgReactive.content += chunk
                }
            },
            (error) => {
                console.error("Stream error", error)
                isStreaming.value = false
                abortStream.value = null
                assistantMsgReactive.content += "\n[Error generating response]"
            },
            () => {
                isStreaming.value = false
                abortStream.value = null
                fetchConversations()
            }
        )
    }

    const stopStreaming = () => {
        if (abortStream.value) {
            abortStream.value()
            abortStream.value = null
            isStreaming.value = false
        }
    }

    return {
        conversations,
        currentConversationId,
        messages,
        loading: isLoading,
        streaming: isStreaming,
        fetchConversations,
        createConversation,
        deleteConversation,
        selectConversation,
        sendMessage,
        stopStreaming
    }
})
