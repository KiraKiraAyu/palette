<template>
    <BaseButton 
        :variant="variant"
        :size="size"
        :disabled="disabled"
        class="h-12 aspect-square flex flex-nowrap w-full items-center gap-2 p-0! border-0!"
    >
        <div
            class="flex h-full aspect-square items-center justify-center shrink-0"
            v-if="$slots.default">
            <slot></slot>
        </div>
        <span class="transition-opacity whitespace-nowrap" :class="{ 'opacity-0': folded, 'p-6': !$slots.default}">{{ label }}</span>
    </BaseButton>
</template>

<script setup lang="ts">
import BaseButton from './BaseButton.vue'

interface Props {
    folded: boolean
    label: string
    variant?: 'primary' | 'secondary' | 'danger' | 'outline'
    size?: 'sm' | 'md' | 'lg'
    disabled?: boolean
}

const { 
    folded, 
    label, 
    variant = 'primary', 
    size = 'md', 
    disabled = false,

} = defineProps<Props>()
</script>
