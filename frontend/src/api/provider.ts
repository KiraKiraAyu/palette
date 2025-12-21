import type { CreateProviderRequest, UserProvider, ProviderModel } from "@/types/provider"
import request from "@/utils/request"

enum Api {
    UserProviders = "/api/providers",
    UserProvider = "/api/providers/{id}",
    Check = "/api/providers/check/{id}"
}

interface ProviderWithModels {
    provider: UserProvider
    models: ProviderModel[]
}

interface ProviderListResponse {
    items: ProviderWithModels[]
}

export async function getUserProvidersApi() {
    const data = await request.get<ProviderListResponse>(Api.UserProviders)
    return data.items.map(item => ({
        ...item.provider,
        models: item.models
    }))
}

export function createUserProviderApi(data: CreateProviderRequest) {
    return request.post<UserProvider>(Api.UserProviders, data)
}

export function updateUserProviderApi(id: string, data: Partial<UserProvider>) {
    return request.put<UserProvider>(Api.UserProvider.replace("{id}", id), data)
}

export function deleteUserProviderApi(id: string) {
    return request.delete<never>(Api.UserProvider.replace("{id}", id))
}

export function checkUserProviderApi(id: string) {
    return request.post<never>(Api.Check.replace("{id}", id))
}