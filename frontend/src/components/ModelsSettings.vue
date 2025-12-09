<template>
    <div class="flex flex-nowrap h-full">
        <div class="flex flex-col gap-1 w-64 border-r-light-blue border-r pr-4">
            <FoldButton size="sm" @click="onNewProvider" :folded="false" label="Add Provider">
                <i-lucide-circle-plus />
            </FoldButton>
            <div class="flex flex-col gap-1 mt-2">
                <FoldButton v-for="provider in store.providers" :key="provider.id" size="sm"
                    @click="store.selectProvider(provider.id)" :folded="store.selectedProviderId !== provider.id"
                    :label="provider.name">
                </FoldButton>
            </div>
        </div>
        <div v-if="store.selectedProvider" class="flex-1 px-4 flex flex-col">
            <BaseInput v-model="store.selectedProvider.name" label="Provider Name"></BaseInput>
            <BaseSelect v-model="store.selectedProvider.provider_type" :options="providerCategories"
                label="API Category"></BaseSelect>
            <BaseInput v-model="store.selectedProvider.url" label="API URL"></BaseInput>
            <BaseInput v-model="store.selectedProvider.key" label="API Key">
                <button class="text-light-text text-lg border-l border-gray-300 pl-4 pr-4 h-full cursor-pointer"
                    @click="checkApiKey">Check</button>
            </BaseInput>
            
            <h3 class="text-lg font-semibold mt-6 mb-2">Models</h3>
            <div class="flex-1 overflow-y-auto">
                <div v-for="(model, index) in store.selectedProvider.models" :key="index"
                    class="p-4 border rounded-md mb-4">
                    <div class="flex justify-between items-center mb-2">
                        <span class="font-bold">Model {{ index + 1 }}</span>
                        <button @click="removeModel(index)" class="text-red-500 hover:text-red-700">
                            <i-lucide-trash-2 />
                        </button>
                    </div>
                    <BaseInput v-model="model.name" label="Model Name" />
                    <BaseInput v-model="model.model_id" label="Model ID" />
                    <div class="grid grid-cols-2 gap-4">
                        <BaseInput v-model.number="model.input_price_per_million" type="number"
                            label="Input Price / Million Tokens" />
                        <BaseInput v-model.number="model.output_price_per_million" type="number"
                            label="Output Price / Million Tokens" />
                    </div>
                </div>
                <button @click="addModel"
                    class="btn btn-primary w-full mt-2 p-2 bg-primary text-white rounded hover:bg-primary-dark">
                    Add Model
                </button>
            </div>
        </div>
        <div v-else class="flex-1 px-4 flex items-center justify-center text-gray-500">
            Select a provider or add a new one.
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useProviderStore } from '@/stores/provider'
import FoldButton from './FoldButton.vue'
import BaseInput from './BaseInput.vue'
import BaseSelect from './BaseSelect.vue'

const store = useProviderStore()

const providerCategories = [
    { text: 'OpenAI', value: 'openai' },
]

onMounted(() => {
    store.fetchProviders()
})

const onNewProvider = () => {
    store.newProvider()
}

const checkApiKey = () => {

}

const addModel = () => {
    
}

const removeModel = (index: number) => {
    
}

defineExpose({
    isDirty: store.isDirty,
    save: store.saveProviders,
})

</script>