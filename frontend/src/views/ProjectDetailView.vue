<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import { useTicketsStore } from '@/stores/tickets'
  import { useEpicsStore } from '@/stores/epics'
  import { useEpicDateGuard } from '@/composables/useEpicDateGuard'
  import { useConfirm } from '@/composables/useConfirm'
  import BaseButton from '@/components/ui/BaseButton.vue'
  import BaseInput from '@/components/ui/BaseInput.vue'
  import BaseModal from '@/components/ui/BaseModal.vue'
  import EpicDetailModal from '@/components/EpicDetailModal.vue'
  import type { TicketPriority } from '@/types'

  const route = useRoute()
  const projectId = route.params.projectId as string

  const store = useTicketsStore()
  const epicStore = useEpicsStore()
  const { checkCreatedTicket } = useEpicDateGuard(projectId, store, epicStore)
  const { confirm } = useConfirm()

  // --- Ticket modal ---
  const showModal = ref(false)
  const form = ref({
    title: '',
    description: '',
    priority: 'medium' as TicketPriority,
    assignee: '',
    startDate: '',
    finishedDate: '',
    tagsInput: '',
    epicId: '',
  })
  const submitting = ref(false)

  // --- Epic management modal ---
  const showEpicsModal = ref(false)
  const showCreateEpicForm = ref(false)
  const epicForm = ref({ title: '', description: '', priority: 'medium' as TicketPriority, startDate: '', finishedDate: '' })
  const epicSubmitting = ref(false)
  const editingEpicId = ref<string | null>(null)
  const showEpicDetailModal = ref(false)

  const STATUS_COLUMNS = [
    { key: 'backlog',     label: 'Backlog' },
    { key: 'todo',        label: 'To Do' },
    { key: 'in_progress', label: 'In Progress' },
    { key: 'in_review',   label: 'In Review' },
    { key: 'done',        label: 'Done' },
  ] as const

  const PRIORITY_COLOURS: Record<TicketPriority, string> = {
    low:      'bg-gray-100 text-gray-600',
    medium:   'bg-blue-100 text-blue-700',
    high:     'bg-orange-100 text-orange-700',
    critical: 'bg-red-100 text-red-700',
  }

  const ticketsByStatus = computed(() => {
    const map: Record<string, typeof store.tickets> = {}
    for (const col of STATUS_COLUMNS) map[col.key] = []
    for (const t of store.tickets) {
      if (map[t.status]) map[t.status].push(t)
    }
    return map
  })

  onMounted(() => {
    store.fetchTickets(projectId)
    epicStore.fetchEpics(projectId)
  })

  function resetForm() {
    form.value = { title: '', description: '', priority: 'medium', assignee: '', startDate: '', finishedDate: '', tagsInput: '', epicId: '' }
  }

  async function handleCreate() {
    submitting.value = true
    try {
      const tags = form.value.tagsInput
        ? form.value.tagsInput.split(',').map((t) => t.trim()).filter(Boolean)
        : []
      const ticket = await store.createTicket(projectId, {
        title: form.value.title,
        description: form.value.description || undefined,
        priority: form.value.priority,
        assignee: form.value.assignee || undefined,
        startDate: form.value.startDate || undefined,
        finishedDate: form.value.finishedDate || undefined,
        tags,
        epicId: form.value.epicId || undefined,
      })
      showModal.value = false
      resetForm()
      await checkCreatedTicket(ticket)
    } finally {
      submitting.value = false
    }
  }

  function resetEpicForm() {
    epicForm.value = { title: '', description: '', priority: 'medium', startDate: '', finishedDate: '' }
    showCreateEpicForm.value = false
  }

  async function handleCreateEpic() {
    epicSubmitting.value = true
    try {
      await epicStore.createEpic(projectId, {
        title: epicForm.value.title,
        description: epicForm.value.description || undefined,
        priority: epicForm.value.priority,
        startDate: epicForm.value.startDate || undefined,
        finishedDate: epicForm.value.finishedDate || undefined,
      })
      resetEpicForm()
    } finally {
      epicSubmitting.value = false
    }
  }

  async function handleDeleteEpic(epicId: string) {
    const ok = await confirm({
      title: 'Delete epic',
      message: 'All tickets assigned to this epic will be unassigned. This cannot be undone.',
      confirmLabel: 'Delete',
      variant: 'danger',
    })
    if (!ok) return
    await epicStore.deleteEpic(projectId, epicId)
  }

  function openEpicEdit(epicId: string) {
    editingEpicId.value = epicId
    showEpicDetailModal.value = true
  }
</script>

<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <h2 class="text-2xl font-bold text-gray-900">Board</h2>
      <div class="flex items-center gap-2">
        <button
          class="rounded-lg border border-indigo-200 bg-indigo-50 px-3 py-2 text-sm font-medium text-indigo-700 hover:bg-indigo-100 transition"
          @click="showEpicsModal = true"
        >Epics</button>
        <BaseButton @click="showModal = true">New Ticket</BaseButton>
      </div>
    </div>

    <div v-if="store.loading" class="text-center text-gray-500">Loading…</div>
    <div v-else class="flex gap-4 overflow-x-auto pb-4">
      <div
        v-for="col in STATUS_COLUMNS"
        :key="col.key"
        class="w-64 flex-shrink-0 rounded-xl bg-gray-100 p-3"
      >
        <h3 class="mb-3 font-semibold text-gray-700">{{ col.label }}</h3>
        <div class="space-y-2">
          <RouterLink
            v-for="ticket in ticketsByStatus[col.key]"
            :key="ticket.id"
            :to="`/projects/${projectId}/tickets/${ticket.id}`"
            class="block rounded-lg bg-white p-3 shadow-sm transition hover:shadow-md"
          >
            <p class="text-sm font-medium text-gray-900">{{ ticket.title }}</p>
            <div class="mt-1 flex flex-wrap items-center gap-1">
              <span v-if="ticket.epicId" class="inline-block rounded px-2 py-0.5 text-xs font-medium bg-indigo-100 text-indigo-700">
                {{ epicStore.epics.find(e => e.id === ticket.epicId)?.title ?? 'Epic' }}
              </span>
            </div>
            <div class="mt-1 flex flex-wrap items-center gap-1">
              <span
                :class="['inline-block rounded px-2 py-0.5 text-xs font-medium', PRIORITY_COLOURS[ticket.priority]]"
              >{{ ticket.priority }}</span>
              <span v-if="ticket.assignee" class="text-xs text-gray-400">· {{ ticket.assignee }}</span>
            </div>
          </RouterLink>
        </div>
      </div>
    </div>

    <!-- New Ticket Modal -->
    <BaseModal title="New Ticket" :show="showModal" @close="showModal = false">
      <form class="space-y-4" @submit.prevent="handleCreate">
        <BaseInput v-model="form.title" label="Title" placeholder="Fix the login bug" required />

        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Description</label>
          <textarea
            v-model="form.description"
            rows="4"
            placeholder="Optional — describe the ticket"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
          />
        </div>

        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Priority</label>
          <select
            v-model="form.priority"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
            <option value="critical">Critical</option>
          </select>
        </div>

        <div v-if="epicStore.epics.length">
          <label class="mb-1 block text-sm font-medium text-gray-700">Epic</label>
          <select
            v-model="form.epicId"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
          >
            <option value="">No Epic</option>
            <option v-for="epic in epicStore.epics" :key="epic.id" :value="epic.id">{{ epic.title }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <BaseInput v-model="form.assignee" label="Assignee" placeholder="e.g. Alice" />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">Start Date</label>
            <input
              v-model="form.startDate"
              type="date"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">Finish Date</label>
            <input
              v-model="form.finishedDate"
              type="date"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            />
          </div>
        </div>

        <BaseInput
          v-model="form.tagsInput"
          label="Tags"
          placeholder="bug, frontend, auth (comma-separated)"
        />

        <div class="flex justify-end gap-2 pt-2">
          <BaseButton variant="secondary" type="button" @click="showModal = false">Cancel</BaseButton>
          <BaseButton type="submit" :loading="submitting">Create</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Epics Management Modal -->
    <BaseModal title="Epics" :show="showEpicsModal" @close="showEpicsModal = false">
      <div class="space-y-3">
        <!-- Epic list -->
        <div v-if="epicStore.epics.length" class="space-y-2">
          <div
            v-for="epic in epicStore.epics"
            :key="epic.id"
            class="flex items-center justify-between rounded-lg border border-gray-100 px-3 py-2 text-sm"
          >
            <div class="min-w-0 flex-1">
              <p class="truncate font-medium text-gray-900">{{ epic.title }}</p>
              <p class="text-xs text-gray-400">{{ epic.status }} · {{ epic.priority }}</p>
            </div>
            <div class="ml-2 flex items-center gap-1 shrink-0">
              <button
                class="rounded px-2 py-1 text-xs text-indigo-600 hover:bg-indigo-50"
                @click="openEpicEdit(epic.id)"
              >Edit</button>
              <button
                class="rounded px-2 py-1 text-xs text-red-500 hover:bg-red-50"
                @click="handleDeleteEpic(epic.id)"
              >Delete</button>
            </div>
          </div>
        </div>
        <p v-else class="text-center text-sm text-gray-400 py-4">No epics yet.</p>

        <!-- Create epic form toggle -->
        <div v-if="!showCreateEpicForm" class="pt-1">
          <button
            class="w-full rounded-lg border border-dashed border-indigo-300 py-2 text-sm text-indigo-600 hover:bg-indigo-50 transition"
            @click="showCreateEpicForm = true"
          >+ New Epic</button>
        </div>

        <form v-else class="space-y-3 border-t pt-3" @submit.prevent="handleCreateEpic">
          <div>
            <label class="mb-1 block text-xs font-medium text-gray-700">Title *</label>
            <input
              v-model="epicForm.title"
              type="text"
              required
              placeholder="Epic title"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
          </div>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="mb-1 block text-xs font-medium text-gray-700">Start Date</label>
              <input v-model="epicForm.startDate" type="date" class="w-full rounded-lg border border-gray-300 px-3 py-1.5 text-sm focus:border-indigo-500 focus:outline-none" />
            </div>
            <div>
              <label class="mb-1 block text-xs font-medium text-gray-700">Finish Date</label>
              <input v-model="epicForm.finishedDate" type="date" class="w-full rounded-lg border border-gray-300 px-3 py-1.5 text-sm focus:border-indigo-500 focus:outline-none" />
            </div>
          </div>
          <div class="flex gap-2">
            <button type="button" class="flex-1 rounded-lg border border-gray-200 py-1.5 text-sm text-gray-600 hover:bg-gray-50" @click="resetEpicForm">Cancel</button>
            <button type="submit" :disabled="epicSubmitting" class="flex-1 rounded-lg bg-indigo-600 py-1.5 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50">
              {{ epicSubmitting ? 'Creating…' : 'Create Epic' }}
            </button>
          </div>
        </form>
      </div>
    </BaseModal>

    <!-- Epic detail/edit modal -->
    <EpicDetailModal
      :epic-id="editingEpicId ?? ''"
      :project-id="projectId"
      :show="showEpicDetailModal"
      @close="showEpicDetailModal = false"
      @deleted="showEpicDetailModal = false"
    />
  </div>
</template>
