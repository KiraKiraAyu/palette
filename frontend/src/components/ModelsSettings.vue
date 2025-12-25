<template>
    <div class="flex flex-nowrap h-full">
        <div class="flex flex-col gap-1 w-64 border-r-light-blue border-r pr-4">
            <FoldButton size="sm" @click="onNewProvider" :folded="false" label="Add Provider">
                <i-lucide-circle-plus />
            </FoldButton>
            <div class="flex flex-col gap-1 mt-2">
                <FoldButton v-for="provider in store.providers" :key="provider.id" size="sm"
                    class="transition-opacity"
                    :class="{'opacity-50': store.selectedProviderId === provider.id}"
                    @click="store.selectProvider(provider.id)" :folded="false"
                    :label="provider.name">
                </FoldButton>
            </div>
        </div>
        <div v-if="activeProvider" class="flex-1 px-4 flex flex-col">
            <BaseInput v-model="activeProvider.name" @update:model-value="onDirty" label="Provider Name"></BaseInput>
            <BaseSelect v-model="activeProvider.provider_type" @update:model-value="onDirty" :options="providerCategories"
                label="API Category"></BaseSelect>
            <BaseInput v-model="activeProvider.url" @update:model-value="onDirty" label="API URL"></BaseInput>
            <BaseInput v-model="activeProvider.key" @update:model-value="onDirty" label="API Key">
                <button v-if="!store.isAdding" class="text-light-text text-lg border-l border-gray-300 pl-4 pr-4 h-full cursor-pointer hover:bg-gray-50"
                    @click="checkApiKey">Check</button>
            </BaseInput>
            
            <div v-if="store.isAdding" class="flex gap-2 mt-4">
                <BaseButton class="btn border border-gray-300 rounded px-4 py-2 hover:bg-gray-100" @click="store.cancelAdding()">Cancel</BaseButton>
                <BaseButton class="btn bg-primary rounded px-4 py-2 hover:bg-primary-dark" @click="saveNewProvider">Save</BaseButton>
            </div>

            <div v-else class="flex gap-2 mt-4">
                <BaseButton class="btn bg-primary rounded px-4 py-2 hover:bg-primary-dark" @click="saveProviderInfo">Save</BaseButton>
            </div>

            <template v-if="!store.isAdding && store.selectedProvider">
                <h3 class="text-lg font-semibold mt-6 mb-2">Models</h3>
                <div class="flex-1 overflow-y-auto pr-2">
                    <div v-for="(model, index) in store.selectedProvider.models" :key="index"
                        class="p-4 border rounded-md mb-4 relative bg-gray-50/50">
                        <div class="flex justify-between items-center mb-2">
                            <span></span>
                            <div class="flex gap-2">
                                <button @click="updateExistingModel(model)" class="text-gray-400 hover:text-primary transition-colors" title="Save changes">
                                    <i-lucide-save :size="16" />
                                </button>
                                <button @click="removeModel(index, model)" class="text-gray-400 hover:text-red-500 transition-colors" title="Delete model">
                                    <i-lucide-trash-2 :size="16" />
                                </button>
                            </div>
                        </div>
                        <BaseInput v-model="model.name" label="Display Name" placeholder="e.g. GPT-4 Turbo" />
                        <BaseInput v-model="model.model_id" label="Model ID" placeholder="e.g. gpt-4-turbo-preview" />
                    </div>

                     <div v-if="isAddingModel" class="p-4 border rounded-md mb-4 relative bg-white border-light-main">
                        <h4 class="font-medium mb-2 text-primary">New Model</h4>
                        <BaseInput v-model="newModel.name" label="Display Name" placeholder="e.g. GPT-4 Turbo" />
                        <BaseInput v-model="newModel.model_id" label="Model ID" placeholder="e.g. gpt-4-turbo-preview" />
                        <div class="flex justify-end gap-2 mt-2">
                            <BaseButton size="sm" class="border border-gray-300 rounded px-3 py-1 hover:bg-gray-100" @click="cancelAddModel">Cancel</BaseButton>
                            <BaseButton size="sm" class="bg-primary rounded px-3 py-1 hover:bg-primary-dark" @click="saveNewModel">Save</BaseButton>
                        </div>
                    </div>
                </div>
                
                <BaseButton v-if="!isAddingModel" @click="startAddModel"
                    class="btn btn-primary w-full mt-2 p-2 bg-white border border-dashed border-gray-300 text-gray-600 rounded hover:bg-gray-50 hover:border-gray-400 transition-all flex items-center justify-center gap-2">
                    <i-lucide-plus :size="16" />
                    Add Model
                </BaseButton>
            </template>
        </div>
        <div v-else class="flex-1 px-4 flex items-center justify-center text-gray-500">
            Select a provider or add a new one.
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useProviderStore } from '@/stores/provider'
import FoldButton from './FoldButton.vue'
import BaseInput from './BaseInput.vue'
import BaseSelect from './BaseSelect.vue'
import { useToast } from '@/composables/useToast'
import BaseButton from './BaseButton.vue'
import { createModelApi, updateModelApi, deleteModelApi } from '@/api/model'

const store = useProviderStore()
const toast = useToast()

const providerCategories = [
    { text: 'OpenAI', value: 'OpenAI' },
]

const isDirty = ref(false)
const onDirty = () => { 
    isDirty.value = true 
}

const isAddingModel = ref(false)
const newModel = ref({
    name: '',
    model_id: ''
})

const activeProvider = computed(() => {
    return store.isAdding ? store.draftProvider : store.selectedProvider
})

watch(() => store.selectedProviderId, () => {
    isDirty.value = false
    isAddingModel.value = false
})

const onNewProvider = () => {
    store.newProvider()
    isDirty.value = false
    isAddingModel.value = false
}

const saveNewProvider = async () => {
    await store.saveNewProvider()
    toast.success("Provider created successfully")
}

const checkApiKey = async () => {
    if (store.selectedProviderId) {
        await store.checkProvider(store.selectedProviderId)
        toast.success("Connection successful")
    }
}

const startAddModel = () => {
    newModel.value = { name: '', model_id: '' }
    isAddingModel.value = true
}

const cancelAddModel = () => {
    isAddingModel.value = false
}

const saveNewModel = async () => {
    if (!store.selectedProviderId) return
    if (!newModel.value.name || !newModel.value.model_id) {
        toast.error("Please fill in both Name and Model ID")
        return
    }

    await createModelApi(store.selectedProviderId, {
        name: newModel.value.name,
        model_id: newModel.value.model_id
    })
    await store.fetchProviders()
    store.selectProvider(store.selectedProviderId)
    isAddingModel.value = false
    toast.success("Model added successfully")
}

const updateExistingModel = async (model: any) => {
    if (!store.selectedProviderId || !model.id) return
    await updateModelApi(store.selectedProviderId, model.id, {
        name: model.name,
        model_id: model.model_id
    })
    toast.success("Model updated successfully")
}

const removeModel = async (index: number, model: any) => {
    if (!store.selectedProviderId || !model.id) return

    await deleteModelApi(store.selectedProviderId, model.id)
    await store.fetchProviders()
    store.selectProvider(store.selectedProviderId)
    toast.success("Model deleted successfully")
}

const saveProviderInfo = async () => {
    if (!store.selectedProvider) return

    await store.saveProviders(store.selectedProvider)
    await store.fetchProviders()
    store.selectProvider(store.selectedProvider.id)
    isDirty.value = false
    toast.success("Provider info saved successfully")
}

defineExpose({
    save: saveProviderInfo,
    isDirty
})

</script>