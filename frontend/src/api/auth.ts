import apiClient from './index'
import type { User } from '@/types'

export interface LoginPayload {
  email: string
  pass: string
}

export interface RegisterPayload {
  email: string
  pass: string
  displayName: string
}

export interface AuthResponse {
  token: string
  user: User
}

export const authApi = {
  login: (payload: LoginPayload) =>
    apiClient.post<AuthResponse>('/auth/login', {
      email: payload.email,
      password: payload.pass,
    }),
  register: (payload: RegisterPayload) =>
    apiClient.post<AuthResponse>('/auth/register', {
      email: payload.email,
      password: payload.pass,
      display_name: payload.displayName,
    }),
}
