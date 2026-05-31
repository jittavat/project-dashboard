import apiClient from './index'
import type { Ticket, TicketPriority, TicketStatus } from '@/types'

export interface CreateTicketPayload {
  title: string
  description?: string
  priority: TicketPriority
  assigneeId?: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags?: string[]
  epicId?: string
}

export interface UpdateTicketPayload {
  title?: string
  description?: string
  status?: TicketStatus
  priority?: TicketPriority
  assigneeId?: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags?: string[]
  epicId?: string
}

export const ticketsApi = {
  list: (projectId: string) => apiClient.get<Ticket[]>(`/projects/${projectId}/tickets`),
  create: (projectId: string, payload: CreateTicketPayload) =>
    apiClient.post<Ticket>(`/projects/${projectId}/tickets`, {
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
      epic_id: payload.epicId,
    }),
  get: (projectId: string, ticketId: string) =>
    apiClient.get<Ticket>(`/projects/${projectId}/tickets/${ticketId}`),
  update: (projectId: string, ticketId: string, payload: UpdateTicketPayload) =>
    apiClient.put<Ticket>(`/projects/${projectId}/tickets/${ticketId}`, {
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
      epic_id: payload.epicId,
    }),
  delete: (projectId: string, ticketId: string) =>
    apiClient.delete(`/projects/${projectId}/tickets/${ticketId}`),
}
