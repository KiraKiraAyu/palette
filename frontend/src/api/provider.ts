import type { CreateProviderRequest, UserProvider } from "@/types/provider"
import request from "@/utils/request"

enum Api {
    UserProviders = "/api/providers",
    Check = "/api/providers/check"
}

export function getUserProvidersApi() {
    return request.get<UserProvider[]>(Api.UserProviders)
}

export function createUserProviderApi(data: CreateProviderRequest) {
    return request.post<UserProvider>(Api.UserProviders, data)
}

export function updateUserProviderApi(id: string, data: Partial<UserProvider>) {
    return request.put<UserProvider>(`${Api.UserProviders}/${id}`, data)
}

export function deleteUserProviderApi(id: string) {
    return request.delete<never>(`${Api.UserProviders}/${id}`)
}

export function checkUserProviderApi(id: string) {
    return request.post<never>(`${Api.Check}/${id}`)
}