import type { CreateProviderModelRequest, ProviderModelIdResponse, UpdateProviderModelRequest } from "@/types/model"
import request from "@/utils/request"

enum Api {
    Create = "/api/providers/{provider_id}/models",
    Update = "/api/providers/{provider_id}/models/{id}",
    Delete = "/api/providers/{provider_id}/models/{id}",
}

export function createModelApi(providerId: string, data: CreateProviderModelRequest) {
    return request.post<ProviderModelIdResponse>(Api.Create.replace("{provider_id}", providerId), data)
}

export function updateModelApi(providerId: string, id: string, data: UpdateProviderModelRequest) {
    return request.put<never>(Api.Update.replace("{provider_id}", providerId).replace("{id}", id), data)
}

export function deleteModelApi(providerId: string, id: string) {
    return request.delete<never>(Api.Delete.replace("{provider_id}", providerId).replace("{id}", id))
}