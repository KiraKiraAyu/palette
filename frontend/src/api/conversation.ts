import request from "@/utils/request"
import { useUserStore } from "@/stores"
import type {
    ConversationSessionsResponse,
    ConversationResponse,
    SendMessageRequest
} from "@/types/conversation"

enum Api {
    Conversations = "/api/conversations",
    ConversationMessages = "/api/conversations/{id}/messages",
    DeleteConversation = "/api/conversations/{id}",
}

export function listConversationsApi() {
    return request.get<ConversationSessionsResponse>(Api.Conversations)
}

export function createConversationApi() {
    return request.post<ConversationResponse>(Api.Conversations, {})
}

export function listMessagesApi(conversationId: string) {
    return request.get<ConversationResponse>(Api.ConversationMessages.replace("{id}", conversationId))
}

export function deleteConversationApi(conversationId: string) {
    return request.delete<ConversationResponse>(Api.DeleteConversation.replace("{id}", conversationId))
}

/**
 * Send a message and receive streaming response via SSE
 * @param conversationId - The conversation ID
 * @param data - The message data containing content and provider_model_id
 * @param onMessage - Callback function to handle each chunk of the streaming response
 * @param onError - Callback function to handle errors
 * @param onComplete - Callback function to handle completion
 * @returns A function to abort the connection
 */
export function sendMessageApi(
    conversationId: string,
    data: SendMessageRequest,
    onMessage: (chunk: string) => void,
    onError?: (error: Error) => void,
    onComplete?: () => void
): () => void {
    const userStore = useUserStore()
    const url = `${import.meta.env.VITE_API_URL}${Api.ConversationMessages.replace("{id}", conversationId)}`

    const controller = new AbortController()

    fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${userStore.token}`,
        },
        body: JSON.stringify(data),
        signal: controller.signal,
    })
        .then(async (response) => {
            if (!response.ok) {
                throw new Error(`HTTP error ${response.status}`)
            }

            const reader = response.body?.getReader()
            const decoder = new TextDecoder()

            if (!reader) {
                throw new Error("Response body is not readable")
            }

            try {
                let buffer = ""
                while (true) {
                    const { done, value } = await reader.read()

                    if (done) {
                        onComplete?.()
                        break
                    }

                    buffer += decoder.decode(value, { stream: true })
                    const lines = buffer.split("\n")
                    buffer = lines.pop() || ""

                    for (const line of lines) {
                        if (line.startsWith("data: ")) {
                            const data = line.slice(6)
                            if (data.trim()) {
                                onMessage(data)
                            }
                        } else if (line.startsWith("event: error")) {
                            console.error("Received error event from stream")
                        }
                    }
                }
            } catch (error) {
                if (error instanceof Error && error.name !== "AbortError") {
                    onError?.(error)
                }
            }
        })
        .catch((error) => {
            if (error.name !== "AbortError") {
                onError?.(error)
            }
        })

    return () => controller.abort()
}
