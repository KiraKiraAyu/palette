<script setup lang="ts">
import { ref } from 'vue';

const isDirty = ref(false);
const save = () => {
    // a snyc function
    return Promise.resolve();
}

defineExpose({
    isDirty,
    save
})
</script>
<template>
    <div></div>
</template>