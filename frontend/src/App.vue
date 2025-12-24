<template>
  <main class="w-full min-h-screen relative flex flex-col">
    <RouterView />
  </main>
  <MessageToast />
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import MessageToast from './components/MessageToast.vue'
</script>
