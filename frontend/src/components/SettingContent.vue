<template>
    <div class="flex flex-col flex-1 p-16 h-full">
        <div class="flex justify-between items-center mb-4 pl-4">
            <h2 class="text-2xl font-bold">Settings</h2>
            <div class="flex items-center">
                <span v-if="showSaved" class="text-green-500 mr-4">Saved!</span>
                <button v-if="isDirty" @click="onSave"
                    class="btn btn-primary bg-primary text-white py-2 px-4 rounded hover:bg-primary-dark">
                    Save
                </button>
            </div>
        </div>
        <HorizontalTab class="w-full" v-model="selectedOption" :options="options"></HorizontalTab>
        <div class="overflow-x-hidden overflow-y-auto flex-1 pt-8 pl-4">
            <GeneralSettings v-if="selectedOption === 'General'" ref="generalSettingsRef" />
            <ModelsSettings v-else-if="selectedOption === 'Models'" ref="modelsSettingsRef" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import HorizontalTab from './HorizontalTab.vue'
import GeneralSettings from './GeneralSettings.vue'
import ModelsSettings from './ModelsSettings.vue'

type Option = 'General' | 'Models'

const options: Option[] = ['General', 'Models']
const selectedOption = ref<Option>('General')

const generalSettingsRef = ref<{ isDirty: boolean, save: () => Promise<void> } | null>(null);
const modelsSettingsRef = ref<{ isDirty: boolean, save: () => Promise<void> } | null>(null);

const showSaved = ref(false);

const isDirty = computed(() => {
    if (selectedOption.value === 'General') {
        return generalSettingsRef.value?.isDirty ?? false;
    } else {
        return modelsSettingsRef.value?.isDirty ?? false;
    }
});

const onSave = async () => {
    let savePromise;
    if (selectedOption.value === 'General') {
        savePromise = generalSettingsRef.value?.save();
    } else {
        savePromise = modelsSettingsRef.value?.save();
    }

    if (savePromise) {
        await savePromise;
        showSaved.value = true;
        setTimeout(() => {
            showSaved.value = false;
        }, 2000);
    }
};
</script>
