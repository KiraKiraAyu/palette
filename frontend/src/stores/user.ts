import { loginApi, registerApi } from "@/api/user";
import type { LoginRequest, RegisterRequest } from "@/types/user";
import { defineStore } from "pinia";

export const useUserStore = defineStore("user", () => {
    let token: string = ''
    const login = (loginForm: LoginRequest) => {
        loginApi(loginForm)
    }

    const logout = () => {
        token = ''
    }

    const register = (registerForm: RegisterRequest) => {
        registerApi(registerForm).then((res => (
            token = res.token
        )))
    }

    return { token, logout, login, register }
})