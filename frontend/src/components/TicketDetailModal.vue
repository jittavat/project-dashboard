<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { useTicketsStore } from '@/stores/tickets'
  import { useEpicsStore } from '@/stores/epics'
  import { useEpicDateGuard } from '@/composables/useEpicDateGuard'
  import { useConfirm } from '@/composables/useConfirm'
  import type { TicketStatus, TicketPriority } from '@/types'

  const props = defineProps<{ ticketId: string; projectId: string; show: boolean }>()
  const emit = defineEmits<{ close: []; deleted: [] }>()

  const store = useTicketsStore()
  const epicStore = useEpicsStore()
  const { checkFinishedDate, checkStartDate } = useEpicDateGuard(props.projectId, store, epicStore)
  const { confirm } = useConfirm()
  const loading = ref(false)

  const title = ref('')
  const description = ref('')
  const priority = ref<TicketPriority>('medium')
  const assignee = ref('')
  const startDate = ref('')
  const finishedDate = ref('')
  const epicId = ref('')
  const tags = ref<string[]>([])
  const newTag = ref('')

  const STATUSES: TicketStatus[] = ['backlog', 'todo', 'in_progress', 'in_review', 'done']
  const STATUS_LABELS: Record<TicketStatus, string> = {
    backlog: 'Backlog',
    todo: 'To Do',
    in_progress: 'In Progress',
    in_review: 'In Review',
    done: 'Done',
  }

  watch(
    () => props.show,
    async (val) => {
      if (val && props.ticketId) {
        loading.value = true
        epicStore.fetchEpics(props.projectId)
        await store.fetchTicket(props.projectId, props.ticketId)
        loading.value = false
      }
    },
  )

  watch(
    () => store.current,
    (ticket) => {
      if (!ticket) return
      title.value = ticket.title
      description.value = ticket.description ?? ''
      priority.value = ticket.priority
      assignee.value = ticket.assignee ?? ''
      startDate.value = ticket.startDate ? ticket.startDate.slice(0, 10) : ''
      finishedDate.value = ticket.finishedDate ? ticket.finishedDate.slice(0, 10) : ''
      epicId.value = ticket.epicId ?? ''
      tags.value = [...ticket.tags]
    },
  )

  async function save(patch: Record<string, unknown>) {
    await store.updateTicket(props.projectId, props.ticketId, patch)
  }

  async function onFinishedDateChange() {
    const oldDate = store.current?.finishedDate?.slice(0, 10)
    await save({ finishedDate: finishedDate.value || undefined })
    if (epicId.value)
      await checkFinishedDate(props.ticketId, oldDate, finishedDate.value, epicId.value)
  }

  async function onStartDateChange() {
    const oldDate = store.current?.startDate?.slice(0, 10)
    await save({ startDate: startDate.value || undefined })
    if (epicId.value)
      await checkStartDate(props.ticketId, oldDate, startDate.value, epicId.value)
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
    const ok = await confirm({
      title: 'Delete ticket',
      message: 'This ticket will be permanently deleted. This cannot be undone.',
      confirmLabel: 'Delete',
      variant: 'danger',
    })
    if (!ok) return
    await store.deleteTicket(props.projectId, props.ticketId)
    emit('deleted')
  }
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="show"
        class="fixed inset-0 z-50 flex items-start justify-center overflow-y-auto bg-black/50 p-4 pt-16"
        @click.self="emit('close')"
      >
        <div class="w-full max-w-2xl rounded-xl bg-white shadow-xl mb-8">
          <!-- Header -->
          <div class="flex items-center justify-between border-b px-6 py-4">
            <h2 class="text-lg font-semibold text-gray-900">Edit Ticket</h2>
            <button class="text-xl leading-none text-gray-400 hover:text-gray-600" @click="emit('close')">✕</button>
          </div>

          <div v-if="loading" class="px-6 py-10 text-center text-gray-400">Loading…</div>

          <div v-else-if="store.current" class="px-6 py-4 space-y-5">
            <!-- Title -->
            <div>
              <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Title</label>
              <input
                v-model="title"
                type="text"
                class="w-full rounded-lg border border-gray-200 px-3 py-2 text-base font-bold text-gray-900 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                @blur="save({ title })"
                @keydown.enter.prevent="($event.target as HTMLInputElement).blur()"
              />
            </div>

            <!-- Description -->
            <div>
              <label class="mb-1 block text-xs font-semibold uppercase tracking-wide text-gray-500">Description</label>
              <textarea
                v-model="description"
                rows="3"
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

            <!-- Priority + Assignee + Team + Dates -->
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
                  <button type="button" class="ml-1 text-gray-400 hover:text-gray-600" @click="removeTag(tag)">×</button>
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
                >Add</button>
              </div>
            </div>

            <!-- Footer -->
            <div class="flex items-center justify-between border-t pt-4">
              <p class="text-xs text-gray-400">
                Created {{ new Date(store.current.createdAt).toLocaleDateString() }}
                · Updated {{ new Date(store.current.updatedAt).toLocaleDateString() }}
              </p>
              <button
                type="button"
                class="rounded-lg border border-red-200 px-3 py-1.5 text-sm text-red-600 hover:bg-red-50"
                @click="handleDelete"
              >Delete ticket</button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
  .modal-enter-active,
  .modal-leave-active {
    transition: opacity 0.2s ease;
  }
  .modal-enter-from,
  .modal-leave-to {
    opacity: 0;
  }
</style>
