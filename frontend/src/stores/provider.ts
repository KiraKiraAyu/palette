import { defineStore } from "pinia"
import { ref, watch } from "vue"
import type { UserProvider } from "@/types/provider"
import { getUserProvidersApi, updateUserProviderApi, createUserProviderApi } from "@/api/provider"

export const useProviderStore = defineStore("provider", () => {
    const providers = ref<UserProvider[]>([])
    const selectedProviderId = ref<string | null>(null)
    const selectedProvider = ref<UserProvider | null>(null)
    const isDirty = ref(false)
    let originalState: string = ''

    const fetchProviders = async () => {
        const data = await getUserProvidersApi()
        providers.value = data
        originalState = JSON.stringify(providers.value)
        if (providers.value.length > 0) {
            selectedProviderId.value = providers.value[0].id
        }
    }

    const selectProvider = (id: string) => {
        selectedProviderId.value = id
        selectedProvider.value = providers.value.find(p => p.id === id) || null
    }

    const newProvider = async () => {
        const newProviderData = await createUserProviderApi()
        providers.value.unshift(newProviderData)
        selectProvider(newProviderData.id)
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
        if(selectedProviderId.value) {
            selectProvider(selectedProviderId.value)
        }
    }, { deep: true })

    return { 
        providers, 
        selectedProviderId, 
        selectedProvider,
        isDirty, 
        fetchProviders, 
        selectProvider,
        newProvider,
        saveProviders
    }
})