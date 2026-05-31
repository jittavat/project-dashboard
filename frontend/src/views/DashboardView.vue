<script setup lang="ts">
  import { useAuthStore } from '@/stores/auth'
  import { useProjectsStore } from '@/stores/projects'
  import { onMounted } from 'vue'

  const auth = useAuthStore()
  const projects = useProjectsStore()

  onMounted(() => projects.fetchProjects())
</script>

<template>
  <div>
    <h2 class="mb-6 text-2xl font-bold text-gray-900">
      Welcome back, {{ auth.user?.displayName }}
    </h2>
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <div class="rounded-xl border bg-white p-6 shadow-sm">
        <p class="text-sm text-gray-500">Projects</p>
        <p class="mt-1 text-3xl font-bold text-primary-600">{{ projects.projects.length }}</p>
      </div>
    </div>
  </div>
</template>
