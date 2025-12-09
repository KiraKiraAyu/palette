<template>
    <div class="border-b-light-main flex flex-nowrap gap-4 relative">
        <div class="w-24 h-7 text-center text-light-text border-b-light-main cursor-pointer hover:text-light-text/50 transition-colors"
            v-for ="(option, index) in options" :key="index"
            @click="selectOption(option)"
            :class="{'': selectedOption === option}">
            {{ option }}
        </div>
        <div class="absolute w-24 h-0.5 bg-light-blue bottom-0 transition-[left]" :style="`left: ${offset}rem`"></div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
    options: Array<string>
}>()

const selectedOption = defineModel<string>({ required: true })
const selectOption = (option: string) => {
    selectedOption.value = option
}

const offset = computed(() => {
    const index = props.options.indexOf(selectedOption.value)
    return index * 7
})
</script>