import { loginApi, registerApi } from "@/api/user";
import type { LoginRequest, RegisterRequest } from "@/types/user";
import { defineStore } from "pinia";
import { ref } from "vue"

export const useUserStore = defineStore("user", () => {
    const token = ref(localStorage.getItem("token") || '')

    const updateToken = (newToken: string) => {
        token.value = newToken
        localStorage.setItem("token", newToken)
    }

    const login = async (loginForm: LoginRequest) => {
        try {
            const res = await loginApi(loginForm)
            updateToken(res.token)
        } catch (error) {
            throw error
        }   
    }

    const logout = () => {
        updateToken('')
    }

    const register = async (registerForm: RegisterRequest) => {
        try {
            const res = await registerApi(registerForm)
            updateToken(res.token)
        } catch (error) {
            throw error
        }
    }

    return { token, logout, login, register }
})