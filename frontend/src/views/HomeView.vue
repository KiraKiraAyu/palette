<template>
  <div class="flex-1 h-full flex justify-center items-center">
    <div class="h-36 w-1/2 flex flex-col justify-center items-center">
      <h1 class="text-8xl mb-4 text-transparent bg-clip-text  font-extrabold bg-linear-to-r from-light-main to-light-blue">Palette</h1>
      <div class="flex justify-center items-center gap-8">
        <RouterLink to="/auth">
          <BaseButton>
            Log In / Sign Up
          </BaseButton>
        </RouterLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import BaseButton from '@/components/BaseButton.vue';
import { RouterLink } from 'vue-router';


</script>

