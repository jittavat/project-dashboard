import type { useTicketsStore } from '@/stores/tickets'
import type { useEpicsStore } from '@/stores/epics'
import type { Ticket } from '@/types'
import { useConfirm } from '@/composables/useConfirm'

function toDay(iso: string | undefined | null): string | undefined {
  return iso ? iso.slice(0, 10) : undefined
}

function fmt(date: string) {
  return new Date(date + 'T00:00:00').toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

export function useEpicDateGuard(
  projectId: string,
  ticketsStore: ReturnType<typeof useTicketsStore>,
  epicsStore: ReturnType<typeof useEpicsStore>,
) {
  const { confirm } = useConfirm()

  function siblings(epicId: string, excludeId?: string): Ticket[] {
    return ticketsStore.tickets.filter(
      (t) => t.epicId === epicId && t.id !== excludeId,
    )
  }

  async function checkFinishedDate(
    ticketId: string | undefined,
    oldDate: string | undefined,
    newDate: string,
    epicId: string,
  ) {
    if (!newDate) return
    const epic = epicsStore.epics.find((e) => e.id === epicId)
    if (!epic) return
    const epicEnd = toDay(epic.finishedDate)

    if (epicEnd && newDate > epicEnd) {
      const ok = await confirm({
        title: 'Extend Epic End Date?',
        message: `"${epic.title}" currently ends on ${fmt(epicEnd)}. Extend it to ${fmt(newDate)}?`,
        confirmLabel: 'Extend',
      })
      if (ok) await epicsStore.updateEpic(projectId, epic.id, { finishedDate: newDate })
      return
    }

    if (epicEnd && oldDate && newDate < oldDate && oldDate === epicEnd) {
      const sibs = siblings(epicId, ticketId)
      const sibMax = sibs
        .map((t) => toDay(t.finishedDate))
        .filter((d): d is string => !!d)
        .sort()
        .at(-1)
      const newMax = sibMax && sibMax > newDate ? sibMax : newDate
      if (newMax < epicEnd) {
        const ok = await confirm({
          title: 'Shrink Epic End Date?',
          message: `"${epic.title}" ends on ${fmt(epicEnd)}. Update the end date to ${fmt(newMax)}?`,
          confirmLabel: 'Update',
        })
        if (ok) await epicsStore.updateEpic(projectId, epic.id, { finishedDate: newMax })
      }
    }
  }

  async function checkStartDate(
    ticketId: string | undefined,
    oldDate: string | undefined,
    newDate: string,
    epicId: string,
  ) {
    if (!newDate) return
    const epic = epicsStore.epics.find((e) => e.id === epicId)
    if (!epic) return
    const epicStart = toDay(epic.startDate)

    if (epicStart && newDate < epicStart) {
      const ok = await confirm({
        title: 'Move Epic Start Date?',
        message: `"${epic.title}" starts on ${fmt(epicStart)}. Move the start date earlier to ${fmt(newDate)}?`,
        confirmLabel: 'Move Earlier',
      })
      if (ok) await epicsStore.updateEpic(projectId, epic.id, { startDate: newDate })
      return
    }

    if (epicStart && oldDate && newDate > oldDate && oldDate === epicStart) {
      const sibs = siblings(epicId, ticketId)
      const sibMin = sibs
        .map((t) => toDay(t.startDate))
        .filter((d): d is string => !!d)
        .sort()
        .at(0)
      const newMin = sibMin && sibMin < newDate ? sibMin : newDate
      if (newMin > epicStart) {
        const ok = await confirm({
          title: 'Advance Epic Start Date?',
          message: `"${epic.title}" starts on ${fmt(epicStart)}. Advance the start date to ${fmt(newMin)}?`,
          confirmLabel: 'Advance',
        })
        if (ok) await epicsStore.updateEpic(projectId, epic.id, { startDate: newMin })
      }
    }
  }

  async function checkCreatedTicket(ticket: Ticket) {
    if (!ticket.epicId) return
    const epic = epicsStore.epics.find((e) => e.id === ticket.epicId)
    if (!epic) return

    if (ticket.finishedDate) {
      const newFinish = toDay(ticket.finishedDate)!
      const epicEnd = toDay(epic.finishedDate)
      if (epicEnd && newFinish > epicEnd) {
        const ok = await confirm({
          title: 'Extend Epic End Date?',
          message: `"${epic.title}" ends on ${fmt(epicEnd)}, but the new ticket finishes on ${fmt(newFinish)}. Extend the epic?`,
          confirmLabel: 'Extend',
        })
        if (ok) await epicsStore.updateEpic(projectId, epic.id, { finishedDate: newFinish })
      }
    }

    if (ticket.startDate) {
      const newStart = toDay(ticket.startDate)!
      const epicStart = toDay(epic.startDate)
      if (epicStart && newStart < epicStart) {
        const ok = await confirm({
          title: 'Move Epic Start Date?',
          message: `"${epic.title}" starts on ${fmt(epicStart)}, but the new ticket starts on ${fmt(newStart)}. Move the epic earlier?`,
          confirmLabel: 'Move Earlier',
        })
        if (ok) await epicsStore.updateEpic(projectId, epic.id, { startDate: newStart })
      }
    }
  }

  return { checkFinishedDate, checkStartDate, checkCreatedTicket }
}
