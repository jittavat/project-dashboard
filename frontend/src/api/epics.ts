import apiClient from './index'
import type { Epic, TicketPriority, TicketStatus } from '@/types'

export interface CreateEpicPayload {
  title: string
  description?: string
  priority: TicketPriority
  assigneeId?: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags?: string[]
}

export interface UpdateEpicPayload {
  title?: string
  description?: string
  status?: TicketStatus
  priority?: TicketPriority
  assigneeId?: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags?: string[]
}

export const epicsApi = {
  list: (projectId: string) => apiClient.get<Epic[]>(`/projects/${projectId}/epics`),
  create: (projectId: string, payload: CreateEpicPayload) =>
    apiClient.post<Epic>(`/projects/${projectId}/epics`, {
      title: payload.title,
      description: payload.description,
      priority: payload.priority,
      assignee_id: payload.assigneeId,
      finished_date: payload.finishedDate
        ? new Date(payload.finishedDate).toISOString()
        : undefined,
      start_date: payload.startDate
        ? new Date(payload.startDate).toISOString()
        : undefined,
      assignee: payload.assignee,
      tags: payload.tags,
    }),
  get: (projectId: string, epicId: string) =>
    apiClient.get<Epic>(`/projects/${projectId}/epics/${epicId}`),
  update: (projectId: string, epicId: string, payload: UpdateEpicPayload) =>
    apiClient.put<Epic>(`/projects/${projectId}/epics/${epicId}`, {
      title: payload.title,
      description: payload.description,
      status: payload.status,
      priority: payload.priority,
      assignee_id: payload.assigneeId,
      finished_date: payload.finishedDate
        ? new Date(payload.finishedDate).toISOString()
        : undefined,
      start_date: payload.startDate
        ? new Date(payload.startDate).toISOString()
        : undefined,
      assignee: payload.assignee,
      tags: payload.tags,
    }),
  delete: (projectId: string, epicId: string) =>
    apiClient.delete(`/projects/${projectId}/epics/${epicId}`),
}
