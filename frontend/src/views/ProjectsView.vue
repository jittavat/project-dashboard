<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { useProjectsStore } from '@/stores/projects'
  import BaseButton from '@/components/ui/BaseButton.vue'
  import BaseInput from '@/components/ui/BaseInput.vue'
  import BaseModal from '@/components/ui/BaseModal.vue'

  const store = useProjectsStore()
  const showModal = ref(false)
  const form = ref({ name: '', key: '', description: '' })
  const submitting = ref(false)

  onMounted(() => store.fetchProjects())

  async function handleCreate() {
    submitting.value = true
    try {
      await store.createProject(form.value)
      showModal.value = false
      form.value = { name: '', key: '', description: '' }
    } finally {
      submitting.value = false
    }
  }
</script>

<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <h2 class="text-2xl font-bold text-gray-900">Projects</h2>
      <BaseButton @click="showModal = true">New Project</BaseButton>
    </div>

    <div v-if="store.loading" class="text-center text-gray-500">Loading…</div>
    <div v-else-if="store.projects.length === 0" class="text-center text-gray-400">
      No projects yet. Create your first one!
    </div>
    <div v-else class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <RouterLink
        v-for="project in store.projects"
        :key="project.id"
        :to="`/projects/${project.id}`"
        class="rounded-xl border bg-white p-5 shadow-sm transition hover:shadow-md"
      >
        <div class="flex items-start justify-between">
          <div>
            <p class="text-xs font-semibold uppercase tracking-wide text-primary-600">
              {{ project.key }}
            </p>
            <h3 class="mt-1 font-semibold text-gray-900">{{ project.name }}</h3>
            <p v-if="project.description" class="mt-1 text-sm text-gray-500 line-clamp-2">
              {{ project.description }}
            </p>
          </div>
        </div>
      </RouterLink>
    </div>

    <BaseModal title="New Project" :show="showModal" @close="showModal = false">
      <form class="space-y-4" @submit.prevent="handleCreate">
        <BaseInput v-model="form.name" label="Name" placeholder="My Awesome Project" required />
        <BaseInput v-model="form.key" label="Key" placeholder="MAP" required />
        <BaseInput v-model="form.description" label="Description" placeholder="Optional" />
        <div class="flex justify-end gap-2 pt-2">
          <BaseButton variant="secondary" type="button" @click="showModal = false">Cancel</BaseButton>
          <BaseButton type="submit" :loading="submitting">Create</BaseButton>
        </div>
      </form>
    </BaseModal>
  </div>
</template>
