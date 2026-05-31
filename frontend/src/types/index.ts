export interface User {
  id: string
  email: string
  displayName: string
}

export interface Project {
  id: string
  name: string
  key: string
  description?: string
  ownerId: string
  createdAt: string
  updatedAt: string
}

export type TicketStatus = 'backlog' | 'todo' | 'in_progress' | 'in_review' | 'done'
export type TicketPriority = 'low' | 'medium' | 'high' | 'critical'

export interface Ticket {
  id: string
  projectId: string
  title: string
  description?: string
  status: TicketStatus
  priority: TicketPriority
  assigneeId?: string
  reporterId: string
  createdAt: string
  updatedAt: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags: string[]
  epicId?: string
}

export interface Epic {
  id: string
  projectId: string
  title: string
  description?: string
  status: TicketStatus
  priority: TicketPriority
  assigneeId?: string
  reporterId: string
  createdAt: string
  updatedAt: string
  finishedDate?: string
  startDate?: string
  assignee?: string
  tags: string[]
}

export interface ApiError {
  error: string
}
