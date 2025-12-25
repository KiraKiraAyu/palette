export interface Model {
    id: string
    name: string
    model_id: string
    provider_id: string
}

export interface CreateProviderModelRequest {
    model_id: string
    name: string
}

export interface UpdateProviderModelRequest {
    model_id?: string
    name?: string
}

export interface ProviderModelIdResponse {
    id: string
}
