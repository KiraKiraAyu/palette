import { defineStore } from "pinia"
import { ref, watch } from "vue"
import type { UserProvider, CreateProviderRequest } from "@/types/provider"
import { getUserProvidersApi, updateUserProviderApi, createUserProviderApi, checkUserProviderApi } from "@/api/provider"

export const useProviderStore = defineStore("provider", () => {
    const providers = ref<UserProvider[]>([])
    const selectedProviderId = ref<string | null>(null)
    const selectedProvider = ref<UserProvider | null>(null)
    const isAdding = ref(false)
    const draftProvider = ref<CreateProviderRequest>({
        name: '',
        provider_type: 'OpenAI',
        url: '',
        key: ''
    })
    const selectedModelId = ref<string | null>(null)
    const selectedModel = ref<string | null>(null)


    const fetchProviders = async () => {
        const data = await getUserProvidersApi()
        providers.value = data
        // For type checking
        const firstProvider = providers.value[0]
        if (firstProvider) {
            selectedProviderId.value = firstProvider.id
            selectedProvider.value = firstProvider
            const models = firstProvider.models
            if (models && models.length > 0) {
                const firstModel = models[0]
                if (firstModel) {
                    selectedModelId.value = firstModel.id
                    selectedModel.value = firstModel.model_id
                }
            }
        }
    }

    const selectProvider = (id: string) => {
        isAdding.value = false
        selectedProviderId.value = id
        selectedProvider.value = providers.value.find(p => p.id === id) || null
        // Reset model when provider changes to the first available model
        const models = selectedProvider.value?.models
        if (models && models.length > 0) {
            const firstModel = models[0]
            if (firstModel) {
                selectedModelId.value = firstModel.id
                selectedModel.value = firstModel.model_id
            }
        } else {
            selectedModelId.value = null
            selectedModel.value = null
        }
    }

    const selectModel = (id: string) => {
        selectedModelId.value = id
        const models = selectedProvider.value?.models
        if (models) {
            const model = models.find(m => m.id === id)
            selectedModel.value = model ? model.model_id : null
        }
    }

    const newProvider = () => {
        isAdding.value = true
        selectedProviderId.value = null
        selectedProvider.value = null
        selectedModelId.value = null
        selectedModel.value = null
        draftProvider.value = {
            name: 'New Provider',
            provider_type: 'OpenAI',
            url: 'https://api.openai.com/v1',
            key: ''
        }
    }

    const cancelAdding = () => {
        isAdding.value = false
        const firstProvider = providers.value[0]
        if (firstProvider) {
            selectProvider(firstProvider.id)
        }
    }

    const saveNewProvider = async () => {
        const payload = { ...draftProvider.value }
        const newProviderData = await createUserProviderApi(payload)
        await fetchProviders()
        selectProvider(newProviderData.id)
        isAdding.value = false
    }

    const saveProviders = async (provider: UserProvider) => {
        await updateUserProviderApi(provider.id, {
            name: provider.name,
            provider_type: provider.provider_type,
            url: provider.url,
            key: provider.key,
        })
        await fetchProviders()
    }

    const checkProvider = async (id: string) => {
        await checkUserProviderApi(id)
    }

    return {
        providers,
        selectedProviderId,
        selectedProvider,
        selectedModelId,
        selectedModel,
        isAdding,
        draftProvider,
        fetchProviders,
        selectProvider,
        selectModel,
        newProvider,
        cancelAdding,
        saveNewProvider,
        saveProviders,
        checkProvider
    }
})