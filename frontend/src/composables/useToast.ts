import { computed, ref } from "vue"

export enum MessageType {
    Success = 'success',
    Error = 'error',
    Warning = 'warning',
}

export interface ToastMessage {
    id: string
    content: string
    type: MessageType
}

const messages = ref<ToastMessage[]>([])

export function useToast() {
    const add = (content: string, type: MessageType) => {
        const id = crypto.randomUUID()
        const msg: ToastMessage = { id, content, type }
        messages.value.push(msg)

        setTimeout(() => {
            remove(id)
        }, 3000)
    }

    const remove = (id: string) => {
        messages.value = messages.value.filter((msg) => msg.id !== id)
    }

    return {
        messages: computed(() => messages.value),
        remove,
        success: (message: string) => add(message, MessageType.Success),
        error: (message: string) => add(message, MessageType.Error),
        warning: (message: string) => add(message, MessageType.Warning)
    }
}