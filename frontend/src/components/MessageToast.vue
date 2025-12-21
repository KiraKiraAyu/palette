<template>
    <Teleport to="body">
        <TransitionGroup 
            name="toast" 
            tag="div" 
            class="fixed bottom-4 right-4 z-99 flex flex-col gap-3"
        >
            <div 
                v-for="message in messages" 
                :key="message.id" 
                class="w-108 p-4 bg-white rounded-lg border border-light-border backdrop-blur-sm bg-opacity-95 overflow-hidden relative"
            >
                <p
                    class="text-md font-semibold flex-1 wrap-break-word"
                    :class="TOAST_STYLES[message.type].text"
                >{{ message.content }}</p>
                <div
                    class="absolute h-1 left-0 bottom-0 toast-progress"
                    :class="TOAST_STYLES[message.type].bg"
                ></div>
            </div>
        </TransitionGroup>
    </Teleport>
</template>

<script setup lang="ts">
import { MessageType, useToast } from '@/composables/useToast'

const { messages } = useToast()

const TOAST_STYLES: Record<MessageType, { text: string; bg: string }> = {
    [MessageType.Success]: { text: 'text-green-400', bg: 'bg-green-400' },
    [MessageType.Error]: { text: 'text-red-400', bg: 'bg-red-400' },
    [MessageType.Warning]: { text: 'text-yellow-400', bg: 'bg-yellow-400' }
}
</script>

<style scoped>
.toast-move,
.toast-enter-active,
.toast-leave-active {
    transition:
        transform 0.4s cubic-bezier(0.16, 1, 0.3, 1),
        opacity 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.toast-enter-from,
.toast-leave-to {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
}

@keyframes progress {
    from {
        width: 100%;
    }
    to {
        width: 0%;
    }
}

.toast-progress {
    animation: progress 3000ms linear forwards;
}
</style>
