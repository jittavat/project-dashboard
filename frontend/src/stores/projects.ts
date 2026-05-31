import { defineStore } from 'pinia'
import { ref } from 'vue'
import { projectsApi } from '@/api/projects'
import type { Project } from '@/types'
import type { CreateProjectPayload } from '@/api/projects'

export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchProjects() {
    loading.value = true
    error.value = null
    try {
      const { data } = await projectsApi.list()
      projects.value = data
    } catch (e: unknown) {
      error.value = (e as Error).message
    } finally {
      loading.value = false
    }
  }

  async function createProject(payload: CreateProjectPayload) {
    const { data } = await projectsApi.create(payload)
    projects.value.unshift(data)
    return data
  }

  async function deleteProject(projectId: string) {
    await projectsApi.delete(projectId)
    projects.value = projects.value.filter((p) => p.id !== projectId)
  }

  return { projects, loading, error, fetchProjects, createProject, deleteProject }
})
