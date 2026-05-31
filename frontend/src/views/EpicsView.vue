<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import { useEpicsStore } from '@/stores/epics'
  import { useTicketsStore } from '@/stores/tickets'
  import EpicDetailModal from '@/components/EpicDetailModal.vue'
  import BaseModal from '@/components/ui/BaseModal.vue'
  import { useConfirm } from '@/composables/useConfirm'
  import type { TicketPriority, TicketStatus } from '@/types'

  const route = useRoute()
  const projectId = route.params.projectId as string

  const epicStore = useEpicsStore()
  const ticketStore = useTicketsStore()
  const { confirm } = useConfirm()

  onMounted(async () => {
    await Promise.all([epicStore.fetchEpics(projectId), ticketStore.fetchTickets(projectId)])
  })

  // Ticket count per epic
  const ticketCountByEpic = computed(() => {
    const counts: Record<string, number> = {}
    for (const t of ticketStore.tickets) {
      if (t.epicId) counts[t.epicId] = (counts[t.epicId] ?? 0) + 1
    }
    return counts
  })

  // Create form
  const showCreateModal = ref(false)
  const form = ref({
    title: '',
    description: '',
    priority: 'medium' as TicketPriority,
    startDate: '',
    finishedDate: '',
  })
  const submitting = ref(false)

  function resetForm() {
    form.value = { title: '', description: '', priority: 'medium', startDate: '', finishedDate: '' }
  }

  async function handleCreate() {
    submitting.value = true
    try {
      await epicStore.createEpic(projectId, {
        title: form.value.title,
        description: form.value.description || undefined,
        priority: form.value.priority,
        startDate: form.value.startDate || undefined,
        finishedDate: form.value.finishedDate || undefined,
      })
      showCreateModal.value = false
      resetForm()
    } finally {
      submitting.value = false
    }
  }

  // Edit modal
  const editingEpicId = ref<string | null>(null)
  const showEditModal = ref(false)

  function openEdit(epicId: string) {
    editingEpicId.value = epicId
    showEditModal.value = true
  }

  async function handleDelete(epicId: string) {
    const ok = await confirm({
      title: 'Delete epic',
      message: 'All tickets assigned to this epic will be unassigned. This cannot be undone.',
      confirmLabel: 'Delete',
      variant: 'danger',
    })
    if (!ok) return
    await epicStore.deleteEpic(projectId, epicId)
  }

  const STATUS_LABELS: Record<TicketStatus, string> = {
    backlog: 'Backlog',
    todo: 'To Do',
    in_progress: 'In Progress',
    in_review: 'In Review',
    done: 'Done',
  }

  const STATUS_COLORS: Record<TicketStatus, string> = {
    backlog: 'bg-gray-100 text-gray-600',
    todo: 'bg-blue-100 text-blue-700',
    in_progress: 'bg-amber-100 text-amber-700',
    in_review: 'bg-purple-100 text-purple-700',
    done: 'bg-green-100 text-green-700',
  }

  const PRIORITY_COLORS: Record<TicketPriority, string> = {
    low: 'bg-gray-100 text-gray-600',
    medium: 'bg-blue-100 text-blue-700',
    high: 'bg-orange-100 text-orange-700',
    critical: 'bg-red-100 text-red-700',
  }

  function fmtDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric' })
  }
</script>

<template>
  <div>
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <h2 class="text-2xl font-bold text-gray-900">Epics</h2>
      <button
        class="rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 transition"
        @click="showCreateModal = true"
      >New Epic</button>
    </div>

    <!-- Loading -->
    <div v-if="epicStore.loading" class="text-center text-gray-400 py-16">Loading…</div>

    <!-- Empty state -->
    <div v-else-if="!epicStore.epics.length" class="rounded-xl bg-white shadow-sm p-12 text-center">
      <div class="inline-flex h-14 w-14 items-center justify-center rounded-full bg-indigo-100 mb-4">
        <span class="text-2xl">🗂</span>
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-1">No epics yet</h3>
      <p class="text-sm text-gray-500 mb-4">Group related tickets into epics to track larger pieces of work.</p>
      <button
        class="rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700"
        @click="showCreateModal = true"
      >Create first epic</button>
    </div>

    <!-- Epic list -->
    <div v-else class="rounded-xl bg-white shadow-sm overflow-hidden">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b bg-gray-50 text-left text-xs font-semibold uppercase tracking-wide text-gray-500">
            <th class="px-6 py-3">Title</th>
            <th class="px-4 py-3">Status</th>
            <th class="px-4 py-3">Priority</th>
            <th class="px-4 py-3">Date Range</th>
            <th class="px-4 py-3">Tickets</th>
            <th class="px-4 py-3"></th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100">
          <tr v-for="epic in epicStore.epics" :key="epic.id" class="hover:bg-gray-50 transition">
            <td class="px-6 py-4">
              <div>
                <p class="font-medium text-gray-900">{{ epic.title }}</p>
                <p v-if="epic.description" class="mt-0.5 text-xs text-gray-400 line-clamp-1">{{ epic.description }}</p>
              </div>
            </td>
            <td class="px-4 py-4">
              <span :class="['inline-block rounded-full px-2 py-0.5 text-xs font-medium', STATUS_COLORS[epic.status]]">
                {{ STATUS_LABELS[epic.status] }}
              </span>
            </td>
            <td class="px-4 py-4">
              <span :class="['inline-block rounded-full px-2 py-0.5 text-xs font-medium', PRIORITY_COLORS[epic.priority]]">
                {{ epic.priority }}
              </span>
            </td>
            <td class="px-4 py-4 text-gray-500 whitespace-nowrap">
              <span v-if="epic.startDate || epic.finishedDate">
                {{ epic.startDate ? fmtDate(epic.startDate) : '—' }}
                →
                {{ epic.finishedDate ? fmtDate(epic.finishedDate) : 'ongoing' }}
              </span>
              <span v-else class="text-gray-300">No dates</span>
            </td>
            <td class="px-4 py-4">
              <span class="inline-flex h-6 w-6 items-center justify-center rounded-full bg-indigo-100 text-xs font-semibold text-indigo-700">
                {{ ticketCountByEpic[epic.id] ?? 0 }}
              </span>
            </td>
            <td class="px-4 py-4">
              <div class="flex items-center gap-2 justify-end">
                <button
                  class="rounded px-3 py-1 text-xs font-medium text-indigo-600 hover:bg-indigo-50 transition"
                  @click="openEdit(epic.id)"
                >Edit</button>
                <button
                  class="rounded px-3 py-1 text-xs font-medium text-red-500 hover:bg-red-50 transition"
                  @click="handleDelete(epic.id)"
                >Delete</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Create Epic Modal -->
    <BaseModal title="New Epic" :show="showCreateModal" @close="showCreateModal = false">
      <form class="space-y-4" @submit.prevent="handleCreate">
        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Title *</label>
          <input
            v-model="form.title"
            type="text"
            required
            placeholder="Epic title"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
          />
        </div>

        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Description</label>
          <textarea
            v-model="form.description"
            rows="3"
            placeholder="Describe this epic…"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
          />
        </div>

        <div>
          <label class="mb-1 block text-sm font-medium text-gray-700">Priority</label>
          <select
            v-model="form.priority"
            class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
            <option value="critical">Critical</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">Start Date</label>
            <input
              v-model="form.startDate"
              type="date"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
          </div>
          <div>
            <label class="mb-1 block text-sm font-medium text-gray-700">Finish Date</label>
            <input
              v-model="form.finishedDate"
              type="date"
              class="w-full rounded-lg border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <button
            type="button"
            class="rounded-lg border border-gray-200 px-4 py-2 text-sm text-gray-600 hover:bg-gray-50"
            @click="showCreateModal = false; resetForm()"
          >Cancel</button>
          <button
            type="submit"
            :disabled="submitting"
            class="rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50"
          >{{ submitting ? 'Creating…' : 'Create Epic' }}</button>
        </div>
      </form>
    </BaseModal>

    <!-- Edit Epic Modal -->
    <EpicDetailModal
      :epic-id="editingEpicId ?? ''"
      :project-id="projectId"
      :show="showEditModal"
      @close="showEditModal = false"
      @deleted="showEditModal = false"
    />
  </div>
</template>
