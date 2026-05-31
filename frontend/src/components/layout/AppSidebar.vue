<script setup lang="ts">
  import { computed } from 'vue'
  import { RouterLink, useRoute } from 'vue-router'

  const route = useRoute()
  const projectId = computed(() => route.params.projectId as string | undefined)
</script>

<template>
  <aside class="flex w-60 flex-col border-r bg-white shadow-sm">
    <div class="flex h-16 items-center px-6 border-b">
      <span class="text-xl font-bold text-primary-600">Dashboard</span>
    </div>
    <nav class="flex-1 space-y-1 p-4">
      <RouterLink
        to="/"
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
        active-class="bg-primary-50 text-primary-700"
        :exact="true"
      >
        Home
      </RouterLink>
      <RouterLink
        to="/projects"
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
        active-class="bg-primary-50 text-primary-700"
      >
        Projects
      </RouterLink>

      <template v-if="projectId">
        <div class="mt-4 mb-1 px-3 text-xs font-semibold uppercase tracking-wider text-gray-400">
          This Project
        </div>
        <RouterLink
          :to="`/projects/${projectId}`"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
          active-class="bg-primary-50 text-primary-700"
          :exact="true"
        >
          Board
        </RouterLink>
        <RouterLink
          :to="`/projects/${projectId}/epics`"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
          active-class="bg-indigo-50 text-indigo-700"
        >
          Epics
        </RouterLink>
        <RouterLink
          :to="`/projects/${projectId}/timeline`"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100"
          active-class="bg-primary-50 text-primary-700"
        >
          Timeline
        </RouterLink>
      </template>
    </nav>
  </aside>
</template>
