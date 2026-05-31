import { defineStore } from 'pinia'
import { ref } from 'vue'
import { ticketsApi } from '@/api/tickets'
import type { Ticket } from '@/types'
import type { CreateTicketPayload, UpdateTicketPayload } from '@/api/tickets'

export const useTicketsStore = defineStore('tickets', () => {
  const tickets = ref<Ticket[]>([])
  const current = ref<Ticket | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchTickets(projectId: string) {
    loading.value = true
    error.value = null
    try {
      const { data } = await ticketsApi.list(projectId)
      tickets.value = data
    } catch (e: unknown) {
      error.value = (e as Error).message
    } finally {
      loading.value = false
    }
  }

  async function fetchTicket(projectId: string, ticketId: string) {
    const { data } = await ticketsApi.get(projectId, ticketId)
    current.value = data
    return data
  }

  async function createTicket(projectId: string, payload: CreateTicketPayload) {
    const { data } = await ticketsApi.create(projectId, payload)
    tickets.value.unshift(data)
    return data
  }

  async function updateTicket(projectId: string, ticketId: string, payload: UpdateTicketPayload) {
    const { data } = await ticketsApi.update(projectId, ticketId, payload)
    const idx = tickets.value.findIndex((t) => t.id === ticketId)
    if (idx !== -1) tickets.value[idx] = data
    if (current.value?.id === ticketId) current.value = data
    return data
  }

  async function deleteTicket(projectId: string, ticketId: string) {
    await ticketsApi.delete(projectId, ticketId)
    tickets.value = tickets.value.filter((t) => t.id !== ticketId)
    if (current.value?.id === ticketId) current.value = null
  }

  return {
    tickets,
    current,
    loading,
    error,
    fetchTickets,
    fetchTicket,
    createTicket,
    updateTicket,
    deleteTicket,
  }
})
