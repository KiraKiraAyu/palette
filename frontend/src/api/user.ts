import type { AuthResponse, LoginRequest } from "@/types/user"
import request from "@/utils/request"

enum Api {
    Register = "/api/auth/register",
    Login = "/api/auth/login",
    Logout = "/api/auth/logout",
}

export function registerApi(data: LoginRequest) {
    return request.post<AuthResponse>(Api.Register, data)
}

export function loginApi(data: LoginRequest) {
    return request.post<AuthResponse>(Api.Login, data)
}

export function logoutApi() {
    return request.post<AuthResponse>(Api.Logout)
}
