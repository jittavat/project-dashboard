import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

export function useAuth() {
  const store = useAuthStore()
  const router = useRouter()

  async function logout() {
    store.logout()
    await router.push({ name: 'login' })
  }

  return { ...store, logout }
}
