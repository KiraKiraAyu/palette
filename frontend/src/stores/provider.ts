import { defineStore } from "pinia"
import { ref, watch } from "vue"
import type { UserProvider, CreateProviderRequest } from "@/types/provider"
import { getUserProvidersApi, updateUserProviderApi, createUserProviderApi, checkUserProviderApi } from "@/api/provider"

export const useProviderStore = defineStore("provider", () => {
    const providers = ref<UserProvider[]>([])
    const selectedProviderId = ref<string | null>(null)
    const selectedProvider = ref<UserProvider | null>(null)
    const isDirty = ref(false)
    const isAdding = ref(false)
    const draftProvider = ref<CreateProviderRequest>({
        name: '',
        provider_type: 'OpenAI',
        url: '',
        key: ''
    })
    let originalState: string = '[]'

    const fetchProviders = async () => {
        const data = await getUserProvidersApi()
        providers.value = data
        originalState = JSON.stringify(providers.value)
        // For type checking
        const firstProvider = providers.value[0]
        if (firstProvider) {
            selectedProviderId.value = firstProvider.id
            selectedProvider.value = firstProvider
        }
    }

    const selectProvider = (id: string) => {
        isAdding.value = false
        selectedProviderId.value = id
        selectedProvider.value = providers.value.find(p => p.id === id) || null
    }

    const newProvider = () => {
        isAdding.value = true
        selectedProviderId.value = null
        selectedProvider.value = null
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
        if (!payload.key) {
            delete payload.key
        }
        const newProviderData = await createUserProviderApi(payload)
        await fetchProviders()
        selectProvider(newProviderData.id)
        isAdding.value = false
    }

    const saveProviders = async () => {
        for (const provider of providers.value) {
            const originalProvider = JSON.parse(originalState).find((p: UserProvider) => p.id === provider.id)
            if (!originalProvider) {
                return
            } else if (JSON.stringify(provider) !== JSON.stringify(originalProvider)) {
                await updateUserProviderApi(provider.id, provider);
            }
        }
        await fetchProviders()
        isDirty.value = false
    }

    watch(providers, (newValue) => {
        if (JSON.stringify(newValue) !== originalState) {
            isDirty.value = true;
        } else {
            isDirty.value = false;
        }
        // update selected provider
        if (selectedProviderId.value) {
            selectedProvider.value = providers.value.find(p => p.id === selectedProviderId.value) || null
        }
    }, { deep: true })

    const checkProvider = async (id: string) => {
        await checkUserProviderApi(id)
    }

    return {
        providers,
        selectedProviderId,
        selectedProvider,
        isDirty,
        isAdding,
        draftProvider,
        fetchProviders,
        selectProvider,
        newProvider,
        cancelAdding,
        saveNewProvider,
        saveProviders,
        checkProvider
    }
})