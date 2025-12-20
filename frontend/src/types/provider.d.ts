export interface CreateProviderRequest {
    name: string
    provider_type: string
    url: string
    key?: string
}

export interface UserProvider {
    id: string
    user_id: string
    name: string
    provider_type: string
    url: string
    key?: string
    created_at: string
    updated_at: string
    models?: ProviderModel[]
}

export interface ProviderModel {
    id: string
    provider_id: string
    model_id: string
    name: string
    input_price_per_million: number
    output_price_per_million: number
    created_at: string
    updated_at: string
}
