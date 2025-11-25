import router from "@/router";
import { useUserStore } from "@/stores";
import type { ApiResponse } from "@/types/api";
import type { AxiosError, AxiosInstance, AxiosRequestConfig, AxiosResponse, InternalAxiosRequestConfig } from "axios";
import axios from "axios";

const service: AxiosInstance = axios.create({
    baseURL: import.meta.env.VITE_API_URL,
    headers: { 'Content-Type': 'application/json' },
    timeout: 10000,
})

service.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
        const userStore = useUserStore()
        if (userStore.token) {
            config.headers.Authorization = `Bearer ${userStore.token}`
        }
        return config
    },
    (error: AxiosError) => {
        return Promise.reject(error)
    }
)

service.interceptors.response.use(
    (response: AxiosResponse<ApiResponse>) => {
        const res = response.data
        if (res.success) return res.data as unknown as AxiosResponse
        return Promise.reject(new Error(res.error || 'Unknown error'))
    },
    (error: AxiosError) => {
        if (error.status == 401) {
            const userStore = useUserStore()
            userStore.logout()
            router.replace('/auth')
        }
        return Promise.reject(error)
    }
)

const request = {
    get<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
        return service.get(url, config) as Promise<T>;
    },
    post<T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
        return service.post(url, data, config) as Promise<T>;
    },
    put<T = unknown>(url: string, data?: unknown, config?: AxiosRequestConfig): Promise<T> {
        return service.put(url, data, config) as Promise<T>;
    },
    delete<T = unknown>(url: string, config?: AxiosRequestConfig): Promise<T> {
        return service.delete(url, config) as Promise<T>;
    }
}

export default request;