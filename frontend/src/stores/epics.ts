import { defineStore } from 'pinia'
import { ref } from 'vue'
import { epicsApi } from '@/api/epics'
import type { Epic } from '@/types'
import type { CreateEpicPayload, UpdateEpicPayload } from '@/api/epics'

export const useEpicsStore = defineStore('epics', () => {
  const epics = ref<Epic[]>([])
  const current = ref<Epic | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchEpics(projectId: string) {
    loading.value = true
    error.value = null
    try {
      const { data } = await epicsApi.list(projectId)
      epics.value = data
    } catch (e: unknown) {
      error.value = (e as Error).message
    } finally {
      loading.value = false
    }
  }

  async function fetchEpic(projectId: string, epicId: string) {
    const { data } = await epicsApi.get(projectId, epicId)
    current.value = data
    return data
  }

  async function createEpic(projectId: string, payload: CreateEpicPayload) {
    const { data } = await epicsApi.create(projectId, payload)
    epics.value.unshift(data)
    return data
  }

  async function updateEpic(projectId: string, epicId: string, payload: UpdateEpicPayload) {
    const { data } = await epicsApi.update(projectId, epicId, payload)
    const idx = epics.value.findIndex((e) => e.id === epicId)
    if (idx !== -1) epics.value[idx] = data
    if (current.value?.id === epicId) current.value = data
    return data
  }

  async function deleteEpic(projectId: string, epicId: string) {
    await epicsApi.delete(projectId, epicId)
    epics.value = epics.value.filter((e) => e.id !== epicId)
    if (current.value?.id === epicId) current.value = null
  }

  return {
    epics,
    current,
    loading,
    error,
    fetchEpics,
    fetchEpic,
    createEpic,
    updateEpic,
    deleteEpic,
  }
})
