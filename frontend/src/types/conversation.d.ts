export interface ChatRole {
    role: "system" | "user" | "assistant"
}

export interface ConversationMessage {
    id: string
    session_id: string
    role: "system" | "user" | "assistant"
    content: string
    created_at: string
    updated_at: string
}

export interface ConversationSession {
    id: string
    user_id: string
    title: string | null
    created_at: string
    updated_at: string
}

export interface ConversationSessionsResponse {
    items: ConversationSession[]
}

export interface ConversationResponse {
    id: string
    items: ConversationMessage[]
}

export interface SendMessageRequest {
    content: string
    provider_model_id: string
}
