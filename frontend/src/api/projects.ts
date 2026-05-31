import apiClient from './index'
import type { Project } from '@/types'

export interface CreateProjectPayload {
  name: string
  key: string
  description?: string
}

export const projectsApi = {
  list: () => apiClient.get<Project[]>('/projects'),
  create: (payload: CreateProjectPayload) => apiClient.post<Project>('/projects', payload),
  delete: (projectId: string) => apiClient.delete(`/projects/${projectId}`),
}
