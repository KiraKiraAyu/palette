import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import { useUserStore } from '@/stores/user'
import { useProviderStore } from '@/stores/provider'
import { useToast } from '@/composables/useToast'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/auth',
      name: 'auth',
      component: () => import('../views/AuthView.vue'),
      meta: { requiresGuest: true }
    },
    {
      path: '/chat/:id?',
      name: 'chat',
      component: () => import('../views/ChatView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsView.vue'),
      meta: { requiresAuth: true }
    }
  ],
})

router.beforeEach(async (to, from, next) => {
  const userStore = useUserStore()
  const providerStore = useProviderStore()
  const toast = useToast()

  const isAuthenticated = !!userStore.token
  const requiresAuth = to.meta.requiresAuth
  const requiresGuest = to.meta.requiresGuest

  if (isAuthenticated && providerStore.providers.length === 0) {
    try {
      await providerStore.fetchProviders()
    } catch (error) {
      userStore.logout()
      toast.error(String(error))
      
      if (requiresAuth) {
        return next({ name: 'auth' })
      }
    }
  }

  if (requiresAuth && !isAuthenticated) {
    return next({ name: 'auth' })
  }

  if (requiresGuest && isAuthenticated) {
    return next({ name: 'chat' })
  }

  if (to.name === 'home') {
    if (isAuthenticated) {
      return next({ name: 'chat' })
    }
  }

  next()
})

export default router
