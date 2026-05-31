<script setup lang="ts">
  import { useAuth } from '@/composables/useAuth'

  const { user, logout } = useAuth()
</script>

<template>
  <header class="flex h-16 items-center justify-between border-b bg-white px-6 shadow-sm">
    <h1 class="text-lg font-semibold text-gray-800">Project Dashboard</h1>
    <div class="flex items-center gap-4">
      <span class="text-sm text-gray-600">{{ user?.displayName }}</span>
      <button
        class="rounded-md bg-gray-100 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-200"
        @click="logout"
      >
        Sign out
      </button>
    </div>
  </header>
</template>
