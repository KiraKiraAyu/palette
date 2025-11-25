import { defineStore } from "pinia";

export const useUserStore = defineStore("user", () => {
    let token = ''
    const logout = () => {
        
    }

    const login = (email: string, password: string) => {
        
    }
    return { token, logout, login }
})