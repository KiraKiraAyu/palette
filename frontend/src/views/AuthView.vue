<template>
  <div class="min-h-screen bg-gray-100 flex items-center justify-center p-4">
    <div class="bg-white rounded-4xl shadow-light-glow relative overflow-hidden w-full max-w-4xl min-h-[600px]">
      <div
        class="absolute top-0 left-0 h-full w-1/2 transition-all duration-700 ease-in-out z-2"
        :class="{ 'translate-x-full': isSignUp }"
      >
        <form
          class="bg-white flex flex-col items-center justify-center h-full px-12 text-center"
          @submit.prevent="handleLogin"
        >
          <h1 class="font-bold text-3xl mb-4 text-gray-800">Welcome back</h1>
          <div class="social-container mb-4 text-gray-400 text-sm">
            <span>Please log in</span>
          </div>

          <BaseInput
            type="email"
            placeholder="Email"
            class="w-full"
            v-model="loginForm.email"
          />
          <BaseInput
            type="password"
            placeholder="Password"
            class="w-full"
            v-model="loginForm.password"
          />

          <a
            href="#"
            class="text-sm text-gray-500 hover:text-gray-800 my-4 border-b border-transparent hover:border-gray-500 transition-colors"
            >Forgot password?</a
          >

          <BaseButton
            variant="primary"
            size="md"
            class="mt-2 font-semibold tracking-wider uppercase"
            :disabled="isAuthenticating"
          >
            Log in
          </BaseButton>
        </form>
      </div>

      <div
        class="absolute top-0 left-0 h-full w-1/2 transition-all duration-700 ease-in-out"
        :class="isSignUp ? 'translate-x-full opacity-100 z-3' : 'z-1 opacity-0'"
      >
        <form
          class="bg-white flex flex-col items-center justify-center h-full px-12 text-center"
          @submit.prevent="handleRegister"
        >
          <h1 class="font-bold text-3xl mb-4 text-gray-800">Create a account</h1>

          <BaseInput
            type="text"
            placeholder="Username"
            class="w-full"
            v-model="signUpForm.name"
          />
          <BaseInput
            type="email"
            placeholder="Email"
            class="w-full"
            v-model="signUpForm.email"
          />
          <BaseInput
            type="password"
            placeholder="Password"
            class="w-full"
            v-model="signUpForm.password"
          />

          <BaseButton
            variant="primary"
            size="md"
            class="mt-2 font-semibold tracking-wider uppercase"
          >
            Sign in
          </BaseButton>
        </form>
      </div>

      <div
        class="absolute top-0 left-1/2 w-1/2 h-full overflow-hidden transition-transform duration-700 ease-in-out z-4"
        :class="{ '-translate-x-full': isSignUp }"
      >
        <div
          class="bg-linear-to-t from-light-main to-light-blue text-white relative -left-full h-full w-[200%] transform transition-transform duration-700 ease-in-out flex flex-row"
          :class="{ 'translate-x-[50%]': isSignUp }"
        >
          <div
            class="flex flex-col items-center justify-center text-center w-1/2 h-full px-10 transition-transform duration-700 ease-in-out transform translate-x-0"
          >
            <h1 class="font-bold text-4xl mb-4">Already have a account?</h1>
            <BaseButton
              variant="outline"
              size="md"
              class="border-white text-white hover:bg-white hover:text-light-text font-semibold tracking-wider uppercase"
              @click="toggleMode"
              :disabled="isAuthenticating"
            >
              Go to Log in
            </BaseButton>
          </div>

          <div
            class="flex flex-col items-center justify-center text-center w-1/2 h-full px-10 transition-transform duration-700 ease-in-out transform translate-x-0"
          >
            <h1 class="font-bold text-4xl mb-4">Not have a account yet?</h1>
            <BaseButton
              variant="outline"
              size="md"
              class="border-white text-white hover:bg-white hover:text-light-text font-semibold tracking-wider uppercase"
              @click="toggleMode"
            >
              Go to sign up
            </BaseButton>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import BaseButton from '@/components/BaseButton.vue'
import BaseInput from '@/components/BaseInput.vue'
import type { LoginRequest, RegisterRequest } from '@/types/user'
import { useUserStore } from '@/stores'
import { useRouter } from 'vue-router'
import { useToast } from '@/composables/useToast'

const isSignUp = ref(false)
const isAuthenticating = ref(false)
const router = useRouter()

const toggleMode = () => { isSignUp.value = !isSignUp.value }

const userStore = useUserStore()
const toast = useToast()

const loginForm = reactive<LoginRequest>({
  email: "",
  password: "",
})

const signUpForm = reactive<RegisterRequest>({
  name: "",
  email: "",
  password: "",
})

const handleLogin = async () => {
  isAuthenticating.value = true
  try {
    await userStore.login(loginForm)
    toast.success("Login successful")
    router.push('/chat')
  } catch (error) {
    toast.error("Login failed: " + error)
  } finally {
    isAuthenticating.value = false
  }
}

const handleRegister = async () => {
  isAuthenticating.value = true
  try {
    await userStore.register(signUpForm)
    toast.success("Register successful")
    router.push('/chat')
  } catch (error) {
    toast.error("Register failed: " + error)
  } finally {
    isAuthenticating.value = false
  }
}
</script>
