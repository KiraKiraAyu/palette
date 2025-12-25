<template>
  <div>
    <label v-if="label" class="block text-lg font-medium text-light-text mb-1">
      {{ label }}
    </label>
    <div class="relative">
      <select
        v-model="value"
        :class="[
          'w-full',
          'py-3',
          'px-4',
          'pr-10',
          'rounded-full',
          'mb-3',
          'border-none',
          'outline-none',
          'transition-all',
          'appearance-none',
          'bg-gray-100',
          'focus:ring-2',
          'focus:ring-blue-400',
        ]"
      >
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
        >
          {{ option.text }}
        </option>
      </select>
      <div
        class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-light-text"
      >
        <svg
          class="h-5 w-5"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
            clip-rule="evenodd"
          />
        </svg>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface SelectOption {
  text: string
  value: string | number
}

interface Props {
  options: SelectOption[];
  label?: string;
}

defineProps<Props>();
const value = defineModel();
</script>
