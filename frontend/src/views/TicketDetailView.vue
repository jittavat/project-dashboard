<script setup lang="ts">
  import { ref, watch, onMounted } from 'vue'
  import { useRoute, useRouter } from 'vue-router'
  import { useTicketsStore } from '@/stores/tickets'
  import { useEpicsStore } from '@/stores/epics'
  import { useEpicDateGuard } from '@/composables/useEpicDateGuard'
  import type { TicketStatus, TicketPriority } from '@/types'

  const route = useRoute()
  const router = useRouter()
  const projectId = route.params.projectId as string
  const ticketId = route.params.ticketId as string

  const store = useTicketsStore()
  const epicStore = useEpicsStore()
  const { checkFinishedDate, checkStartDate } = useEpicDateGuard(projectId, store, epicStore)

  onMounted(() => {
    store.fetchTicket(projectId, ticketId)
    epicStore.fetchEpics(projectId)
  })

  const title = ref('')
  const description = ref('')
  const priority = ref<TicketPriority>('medium')
  const assignee = ref('')
  const finishedDate = ref('')
  const startDate = ref('')
  const epicId = ref('')
  const tags = ref<string[]>([])
  const newTag = ref('')

  watch(
    () => store.current,
    (ticket) => {
      if (!ticket) return
      title.value = ticket.title
      description.value = ticket.description ?? ''
      priority.value = ticket.priority
      assignee.value = ticket.assignee ?? ''
      finishedDate.value = ticket.finishedDate ? ticket.finishedDate.slice(0, 10) : ''
      startDate.value = ticket.startDate ? ticket.startDate.slice(0, 10) : ''
      epicId.value = ticket.epicId ?? ''
      tags.value = [...ticket.tags]
    },
    { immediate: true },
  )

  const STATUSES: TicketStatus[] = ['backlog', 'todo', 'in_progress', 'in_review', 'done']
  const STATUS_LABELS: Record<TicketStatus, string> = {
    backlog: 'Backlog',
    todo: 'To Do',
    in_progress: 'In Progress',
    in_review: 'In Review',
    done: 'Done',
  }

  async function save(patch: Record<string, unknown>) {
    await store.updateTicket(projectId, ticketId, patch)
  }

  async function onFinishedDateChange() {
    const oldDate = store.current?.finishedDate?.slice(0, 10)
    await save({ finishedDate: finishedDate.value || undefined })
    if (epicId.value)
      await checkFinishedDate(ticketId, oldDate, finishedDate.value, epicId.value)
  }

  async function onStartDateChange() {
    const oldDate = store.current?.startDate?.slice(0, 10)
    await save({ startDate: startDate.value || undefined })
    if (epicId.value)
      await checkStartDate(ticketId, oldDate, startDate.value, epicId.value)
  }

  function addTag() {
    const t = newTag.value.trim()
    if (t && !tags.value.includes(t)) {
      tags.value.push(t)
      save({ tags: tags.value })
    }
    newTag.value = ''
  }

  function removeTag(tag: string) {
    tags.value = tags.value.filter((t) => t !== tag)
    save({ tags: tags.value })
  }

  function onTagKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault()
      addTag()
    }
  }

  async function handleDelete() {
    if (!window.confirm('Delete this ticket? This cannot be undone.')) return
    await store.deleteTicket(projectId, ticketId)
    router.push(`/projects/${projectId}`)
  }
</script>

<template>
  <div v-if="store.current" class="mx-auto max-w-2xl">
    <RouterLink
      :to="`/projects/${projectId}`"
      class="mb-4 inline-flex items-center text-sm text-gray-500 hover:text-gray-700"
    >
      ← Back to board
    </RouterLink>

    <div class="rounded-xl bg-white p-6 shadow-sm space-y-6">
      <!-- Title -->
      <div>
        <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Title</label>
        <input
          v-model="title"
          type="text"
          class="w-full rounded-lg border border-gray-200 px-3 py-2 text-lg font-bold text-gray-900 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
          @blur="save({ title })"
          @keydown.enter.prevent="($event.target as HTMLInputElement).blur()"
        />
      </div>

      <!-- Description -->
      <div>
        <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Description</label>
        <textarea
          v-model="description"
          rows="5"
          placeholder="Add a description…"
          class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
          @blur="save({ description })"
        />
      </div>

      <!-- Status -->
      <div>
        <p class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500">Status</p>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="s in STATUSES"
            :key="s"
            :class="[
              'rounded-full px-3 py-1 text-xs font-medium transition',
              store.current.status === s
                ? 'bg-primary-600 text-white'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
            ]"
            @click="save({ status: s })"
          >
            {{ STATUS_LABELS[s] }}
          </button>
        </div>
      </div>

      <!-- Priority + Assignee + Team + Finish Date (2-col grid) -->
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Priority</label>
          <select
            v-model="priority"
            class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @change="save({ priority })"
          >
            <option value="low">Low</option>
            <option value="medium">Medium</option>
            <option value="high">High</option>
            <option value="critical">Critical</option>
          </select>
        </div>

        <div>
          <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Assignee</label>
          <input
            v-model="assignee"
            type="text"
            placeholder="Unassigned"
            class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @blur="save({ assignee })"
          />
        </div>

        <div>
          <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Start Date</label>
          <input
            v-model="startDate"
            type="date"
            class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @change="onStartDateChange"
          />
        </div>

        <div>
          <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Finish Date</label>
          <input
            v-model="finishedDate"
            type="date"
            class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @change="onFinishedDateChange"
          />
        </div>

        <div class="col-span-2">
          <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Epic</label>
          <select
            v-model="epicId"
            class="w-full rounded-lg border border-gray-200 px-3 py-2 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @change="epicId ? save({ epicId }) : undefined"
          >
            <option value="">No Epic</option>
            <option v-if="!epicStore.epics.length" value="" disabled>No epics yet — create one from Epics page</option>
            <option v-for="epic in epicStore.epics" :key="epic.id" :value="epic.id">{{ epic.title }}</option>
          </select>
        </div>
      </div>

      <!-- Tags -->
      <div>
        <label class="mb-2 block text-xs font-semibold uppercase tracking-wide text-gray-500">Tags</label>
        <div class="flex flex-wrap gap-2 mb-2">
          <span
            v-for="tag in tags"
            :key="tag"
            class="inline-flex items-center gap-1 rounded-full bg-gray-100 px-3 py-1 text-xs text-gray-700"
          >
            {{ tag }}
            <button
              type="button"
              class="ml-1 text-gray-400 hover:text-gray-600"
              @click="removeTag(tag)"
            >×</button>
          </span>
        </div>
        <div class="flex gap-2">
          <input
            v-model="newTag"
            type="text"
            placeholder="Add tag…"
            class="flex-1 rounded-lg border border-gray-200 px-3 py-1.5 text-sm text-gray-700 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            @keydown="onTagKeydown"
          />
          <button
            type="button"
            class="rounded-lg bg-gray-100 px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-200"
            @click="addTag"
          >
            Add
          </button>
        </div>
      </div>

      <!-- Read-only metadata + delete -->
      <div class="flex items-center justify-between border-t pt-4">
        <dl class="grid grid-cols-2 gap-4 text-sm">
          <div>
            <dt class="font-medium text-gray-500">Created</dt>
            <dd class="mt-1 text-gray-900">{{ new Date(store.current.createdAt).toLocaleDateString() }}</dd>
          </div>
          <div>
            <dt class="font-medium text-gray-500">Updated</dt>
            <dd class="mt-1 text-gray-900">{{ new Date(store.current.updatedAt).toLocaleDateString() }}</dd>
          </div>
        </dl>
        <button
          type="button"
          class="rounded-lg border border-red-200 px-3 py-1.5 text-sm text-red-600 hover:bg-red-50"
          @click="handleDelete"
        >Delete ticket</button>
      </div>
    </div>
  </div>
  <div v-else class="text-center text-gray-400">Loading ticket…</div>
</template>
