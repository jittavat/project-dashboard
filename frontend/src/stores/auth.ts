import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/api/auth'
import type { User } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const jwt = ref<string | null>(null)

  const isAuthenticated = computed(() => !!jwt.value)

  function initialize() {
    const stored = localStorage.getItem('access_token')
    if (stored) jwt.value = stored
  }

  async function login(email: string, pass: string) {
    const { data } = await authApi.login({ email, pass })
    jwt.value = data.token
    user.value = data.user
    localStorage.setItem('access_token', data.token)
  }

  async function register(email: string, pass: string, displayName: string) {
    const { data } = await authApi.register({ email, pass, displayName })
    jwt.value = data.token
    user.value = data.user
    localStorage.setItem('access_token', data.token)
  }

  function logout() {
    jwt.value = null
    user.value = null
    localStorage.removeItem('access_token')
  }

  return { user, jwt, isAuthenticated, initialize, login, register, logout }
})
