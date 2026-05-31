import axios from 'axios'
import type { AxiosInstance } from 'axios'

function camelize(s: string): string {
  return s.replace(/_([a-z])/g, (_, c: string) => c.toUpperCase())
}

function camelizeKeys(obj: unknown): unknown {
  if (Array.isArray(obj)) return obj.map(camelizeKeys)
  if (obj !== null && typeof obj === 'object') {
    return Object.fromEntries(
      Object.entries(obj as Record<string, unknown>).map(([k, v]) => [camelize(k), camelizeKeys(v)]),
    )
  }
  return obj
}

const apiClient: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL ?? '/api/v1',
  headers: { 'Content-Type': 'application/json' },
  timeout: 10_000,
})

apiClient.interceptors.request.use((config) => {
  const jwt = localStorage.getItem('access_token')
  if (jwt) {
    config.headers.Authorization = `Bearer ${jwt}`
  }
  return config
})

apiClient.interceptors.response.use(
  (res) => {
    res.data = camelizeKeys(res.data)
    return res
  },
  (err) => {
    if (err.response?.status === 401) {
      localStorage.removeItem('access_token')
      window.location.href = '/login'
    }
    return Promise.reject(err)
  }
)

export default apiClient
