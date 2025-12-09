import type { UserProvider } from "@/types/provider";
import request from "@/utils/request";

enum Api {
    UserProviders = "/api/user_providers",
    Check = "/api/user_providers/check"
}

export function getUserProvidersApi() {
    return request.get<UserProvider[]>(Api.UserProviders)
}

export function createUserProviderApi() {
    return request.post<UserProvider>(Api.UserProviders)
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