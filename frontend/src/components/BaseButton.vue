<template>
    <button :class="classes">
        <slot></slot>
    </button>
</template>
<script setup lang="ts">
import { computed } from 'vue';

interface Props {
    variant?: 'primary' | 'secondary' | 'danger' | 'outline'
    size?: 'sm' | 'md' | 'lg'
    disabled?: boolean
}

const { variant = 'primary', size = 'md', disabled = false,  } = defineProps<Props>()
const classes = computed(() => {
    const variantMap: Record<typeof variant, string> = {
        primary: 'bg-light-main text-light-text focus:ring-blue-500 rounded-full border-1 border-light-border hover:bg-light-main/70 transition-colors',
        secondary: 'bg-light-main text-gray-200 text-gray-800 hover:bg-gray-300 focus:ring-gray-500',
        danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500',
        outline: 'border-2 border-blue-600 text-blue-600 hover:bg-blue-50 focus:ring-blue-500 transition-colors',
    }

    const sizeMap: Record<typeof size, string> = {
        sm: 'px-3 py-2 text-lg',
        md: 'px-4.5 py-2.5 text-lg',
        lg: 'px-7 py-4 text-3xl',
    }

    return [
        variantMap[variant],
        sizeMap[size],
        disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
    ]
})
</script>