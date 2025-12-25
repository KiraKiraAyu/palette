<template>
    <div>
        <label v-if="label" class="block text-lg font-medium text-light-text mb-1">
            {{ label }}
        </label>
        <div class="flex items-center bg-gray-100 rounded-full mb-3 focus-within:ring-2 focus-within:ring-blue-400 transition-all">
            <input :type="type" :placeholder="placeholder" v-model="value" class="bg-transparent border-none w-full py-3 px-4 outline-none">
            <slot></slot>
        </div>
    </div>
</template>

<script setup lang="ts">
interface Props {
    type?: string
    placeholder?: string
    label?: string
}

defineProps<Props>()
const value = defineModel()
</script>