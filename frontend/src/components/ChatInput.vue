<template>
  <div
    class="flex items-end gap-2 p-3 border border-gray-300 rounded-xl shadow-sm bg-white"
  >
    <textarea
      ref="textareaRef"
      v-model="content"
      rows="1"
      placeholder="Input your instructions"
      class="flex-1 max-h-48 min-h-8 resize-none border-0 bg-transparent p-0 focus:ring-0 outline-none overflow-y-auto leading-6 text-gray-800 placeholder:text-gray-400 disabled:text-gray-400 disabled:cursor-not-allowed"
      @input="handleInput"
      @keydown="handleKeydown"
      :disabled="disabled"
    ></textarea>

    <BaseButton
      @click="handleSend"
      :disabled="disabled || !content.trim()"
      class="w-10 h-10 relative flex items-center justify-center rounded-full transition-all duration-200 shrink-0 mb-0.5 p-0!"
      :class="
        !disabled && content.trim()
          ? 'bg-blue-600 text-white hover:bg-blue-700'
          : 'bg-gray-100 text-gray-400 cursor-not-allowed'
      "
    >
      <i-lucide-send-horizontal class="absolute transition-opacity" :class="content ? '' : 'opacity-0'"></i-lucide-send-horizontal>
      <i-lucide-square class="absolute transition-opacity"></i-lucide-square>
    </BaseButton>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import BaseButton from './BaseButton.vue'

const props = defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  (e: 'send', text: string): void
}>()

const content = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)

const autoResize = () => {
  const el = textareaRef.value
  if (!el) return

  el.style.height = 'auto'

  el.style.height = el.scrollHeight + 'px'
}

const handleInput = () => {
  autoResize()
}

const handleSend = async () => {
  if (props.disabled) return
  const text = content.value.trim()
  if (!text) return

  emit('send', text)

  content.value = ''

  await nextTick()
  autoResize()
}

const handleKeydown = (e: KeyboardEvent) => {
  if (e.isComposing) return

  if (e.key === 'Enter') {
    if (!e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }
}

onMounted(() => {
  autoResize()
})
</script>
