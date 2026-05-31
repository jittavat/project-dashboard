<script setup lang="ts">
  import AppHeader from './AppHeader.vue'
  import AppSidebar from './AppSidebar.vue'
  import ConfirmModal from '@/components/ui/ConfirmModal.vue'
</script>

<template>
  <div class="flex h-screen bg-gray-50">
    <AppSidebar />
    <div class="flex flex-1 flex-col overflow-hidden">
      <AppHeader />
      <main class="flex-1 overflow-y-auto p-6">
        <RouterView />
      </main>
    </div>
    <ConfirmModal />
  </div>
</template>
