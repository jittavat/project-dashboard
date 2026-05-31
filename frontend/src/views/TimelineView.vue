<script setup lang="ts">
  import { ref, computed, watch, onMounted, onBeforeUnmount } from 'vue'
  import { useRoute } from 'vue-router'
  import { useTicketsStore } from '@/stores/tickets'
  import { useEpicsStore } from '@/stores/epics'
  import type { Ticket, Epic, TicketPriority, TicketStatus } from '@/types'
  import TicketDetailModal from '@/components/TicketDetailModal.vue'
  import EpicDetailModal from '@/components/EpicDetailModal.vue'

  const route = useRoute()
  const projectId = route.params.projectId as string
  const store = useTicketsStore()
  const epicStore = useEpicsStore()

  type ViewMode = 'ticket' | 'assignee' | 'epic'
  const viewMode = ref<ViewMode>('ticket')

  const containerRef = ref<HTMLElement>()
  const svgWidth = ref(900)
  let ro: ResizeObserver | null = null

  onMounted(async () => {
    await Promise.all([store.fetchTickets(projectId), epicStore.fetchEpics(projectId)])
    ro = new ResizeObserver((entries) => {
      svgWidth.value = entries[0].contentRect.width
    })
    if (containerRef.value) ro.observe(containerRef.value)
  })

  onBeforeUnmount(() => ro?.disconnect())

  const ROW_H = 38
  const EPIC_ROW_H = 44
  const LABEL_W = 200
  const HEADER_H = 60
  const MONTH_H = 28
  const PAD = 12
  const TEAM_GAP = 6
  const MIN_BAR_W = 10

  const PRIORITY_COLORS: Record<TicketPriority, string> = {
    low: '#9ca3af',
    medium: '#3b82f6',
    high: '#f97316',
    critical: '#ef4444',
  }

  const STATUS_COLORS: Record<TicketStatus, string> = {
    backlog: '#9ca3af',
    todo: '#3b82f6',
    in_progress: '#f59e0b',
    in_review: '#8b5cf6',
    done: '#22c55e',
  }

  const STATUS_LABELS: Record<TicketStatus, string> = {
    backlog: 'Backlog',
    todo: 'To Do',
    in_progress: 'In Progress',
    in_review: 'In Review',
    done: 'Done',
  }

  const today = new Date()

  function effectiveStart(t: Ticket): string {
    return t.startDate || t.createdAt || new Date().toISOString()
  }

  const rangeStart = computed(() => {
    const dates = store.tickets
      .map((t) => new Date(effectiveStart(t)).getTime())
      .filter((n) => !isNaN(n))
    const epicDates = epicStore.epics
      .filter((e) => e.startDate)
      .map((e) => new Date(e.startDate!).getTime())
    const all = [...dates, ...epicDates].filter((n) => !isNaN(n))
    const min = all.length ? Math.min(...all) : today.getTime()
    const d = new Date(min)
    d.setDate(d.getDate() - 3)
    return d
  })

  const rangeEnd = computed(() => {
    const dates = store.tickets
      .map((t) => (t.finishedDate ? new Date(t.finishedDate).getTime() : today.getTime()))
      .filter((n) => !isNaN(n))
    const epicDates = epicStore.epics
      .filter((e) => e.finishedDate)
      .map((e) => new Date(e.finishedDate!).getTime())
    const all = [...dates, ...epicDates].filter((n) => !isNaN(n))
    const max = all.length ? Math.max(...all) : today.getTime()
    const d = new Date(Math.max(max, today.getTime()))
    d.setDate(d.getDate() + 7)
    return d
  })

  const totalMs = computed(() => rangeEnd.value.getTime() - rangeStart.value.getTime())
  const chartW = computed(() => Math.max(svgWidth.value - LABEL_W - PAD * 2, 200))

  function xPos(d: string | Date): number {
    const ms = new Date(d).getTime() - rangeStart.value.getTime()
    return LABEL_W + PAD + (ms / totalMs.value) * chartW.value
  }

  function ticketBarX(t: Ticket): number {
    return xPos(effectiveStart(t))
  }

  function ticketBarW(t: Ticket): number {
    const startMs = new Date(effectiveStart(t)).getTime()
    const endMs = t.finishedDate ? new Date(t.finishedDate).getTime() : today.getTime()
    const w = ((endMs - startMs) / totalMs.value) * chartW.value
    return Math.max(w, MIN_BAR_W)
  }

  // Month bands for the top header row
  const monthBands = computed(() => {
    const bands: { label: string; x1: number; x2: number }[] = []
    const d = new Date(rangeStart.value.getFullYear(), rangeStart.value.getMonth(), 1)
    while (d <= rangeEnd.value) {
      const monthEnd = new Date(d.getFullYear(), d.getMonth() + 1, 1)
      const x1 = Math.max(xPos(d), LABEL_W + PAD)
      const x2 = Math.min(xPos(monthEnd), svgWidth.value - PAD)
      if (x2 > x1) {
        const label = d.toLocaleDateString('en-GB', { month: 'long', year: 'numeric' })
        bands.push({ label, x1, x2 })
      }
      d.setMonth(d.getMonth() + 1)
    }
    return bands
  })

  // Day ticks every 7 days for the bottom header row
  const dateTicks = computed(() => {
    const ticks: { x: number; label: string }[] = []
    const d = new Date(rangeStart.value)
    d.setHours(0, 0, 0, 0)
    while (d <= rangeEnd.value) {
      ticks.push({
        x: xPos(d),
        label: d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' }),
      })
      d.setDate(d.getDate() + 7)
    }
    return ticks
  })

  // --- By-Ticket view ---
  const ticketRows = computed(() => {
    const sorted = [...store.tickets].sort(
      (a, b) =>
        new Date(a.startDate ?? a.createdAt).getTime() -
        new Date(b.startDate ?? b.createdAt).getTime(),
    )
    return sorted.map((t, i) => ({
      y: HEADER_H + i * ROW_H,
      ticket: t,
      rowLabel: clip(t.title, 26),
      barText: t.assignee?.trim() || '—',
      x: ticketBarX(t),
      w: ticketBarW(t),
      color: STATUS_COLORS[t.status],
    }))
  })

  const ticketSvgH = computed(() => HEADER_H + store.tickets.length * ROW_H + PAD)

  // --- By-Assignee view ---
  interface TeamRow {
    assignee: string
    tickets: Ticket[]
    y: number
    height: number
  }

  const teamRows = computed((): TeamRow[] => {
    const groups = new Map<string, Ticket[]>()
    for (const t of store.tickets) {
      const key = t.assignee?.trim() || 'Unassigned'
      if (!groups.has(key)) groups.set(key, [])
      groups.get(key)!.push(t)
    }
    const sortedGroups = [...groups.entries()].sort(([, a], [, b]) => {
      const minA = Math.min(...a.map((t) => new Date(t.startDate ?? t.createdAt).getTime()).filter((n) => !isNaN(n)))
      const minB = Math.min(...b.map((t) => new Date(t.startDate ?? t.createdAt).getTime()).filter((n) => !isNaN(n)))
      return minA - minB
    })
    let y = HEADER_H
    const rows: TeamRow[] = []
    for (const [assignee, tickets] of sortedGroups) {
      const sorted = [...tickets].sort(
        (a, b) =>
          new Date(a.startDate ?? a.createdAt).getTime() -
          new Date(b.startDate ?? b.createdAt).getTime(),
      )
      const height = sorted.length * ROW_H
      rows.push({ assignee, tickets: sorted, y, height })
      y += height + TEAM_GAP
    }
    return rows
  })

  const teamSvgH = computed(
    () => teamRows.value.reduce((s, r) => s + r.height + TEAM_GAP, HEADER_H) + PAD,
  )

  // --- By-Epic view ---
  interface EpicGroup {
    key: string
    epic: Epic | null
    tickets: Ticket[]
    isExpanded: boolean
  }

  // All epic groups start expanded
  const expandedEpics = ref<Set<string>>(new Set(['__none__']))

  watch(
    () => epicStore.epics,
    (epics) => {
      for (const e of epics) expandedEpics.value.add(e.id)
    },
    { immediate: true },
  )

  function toggleEpic(key: string) {
    if (expandedEpics.value.has(key)) expandedEpics.value.delete(key)
    else expandedEpics.value.add(key)
  }

  const epicGroups = computed((): EpicGroup[] => {
    const map = new Map<string, { epic: Epic | null; tickets: Ticket[] }>()
    map.set('__none__', { epic: null, tickets: [] })
    for (const e of epicStore.epics) map.set(e.id, { epic: e, tickets: [] })

    for (const t of store.tickets) {
      const key = t.epicId ?? '__none__'
      if (!map.has(key)) map.set(key, { epic: null, tickets: [] })
      map.get(key)!.tickets.push(t)
    }

    return [...map.entries()]
      .filter(([, g]) => g.epic !== null || g.tickets.length > 0)
      .map(([key, g]) => ({
        key,
        epic: g.epic,
        isExpanded: expandedEpics.value.has(key),
        tickets: [...g.tickets].sort(
          (a, b) =>
            new Date(a.startDate ?? a.createdAt).getTime() -
            new Date(b.startDate ?? b.createdAt).getTime(),
        ),
      }))
  })

  // Cumulative y positions for each epic group
  const epicRowPositions = computed(() => {
    let y = HEADER_H
    return epicGroups.value.map((g) => {
      const epicY = y
      y += EPIC_ROW_H + (g.isExpanded ? g.tickets.length * ROW_H : 0) + TEAM_GAP
      return { key: g.key, epicY }
    })
  })

  const epicSvgH = computed(
    () =>
      epicGroups.value.reduce(
        (sum, g) => sum + EPIC_ROW_H + (g.isExpanded ? g.tickets.length * ROW_H : 0) + TEAM_GAP,
        HEADER_H,
      ) + PAD,
  )

  function epicBarStart(g: EpicGroup): string {
    if (g.epic?.startDate) return g.epic.startDate
    const ms = g.tickets.map((t) => new Date(effectiveStart(t)).getTime()).filter((n) => !isNaN(n))
    return ms.length ? new Date(Math.min(...ms)).toISOString() : today.toISOString()
  }

  function epicBarEnd(g: EpicGroup): string {
    if (g.epic?.finishedDate) return g.epic.finishedDate
    const ms = g.tickets
      .map((t) => (t.finishedDate ? new Date(t.finishedDate).getTime() : today.getTime()))
      .filter((n) => !isNaN(n))
    return new Date(Math.max(...ms, today.getTime())).toISOString()
  }

  function epicBarX(g: EpicGroup): number {
    return xPos(epicBarStart(g))
  }

  function epicBarW(g: EpicGroup): number {
    const startMs = new Date(epicBarStart(g)).getTime()
    const endMs = new Date(epicBarEnd(g)).getTime()
    const w = ((endMs - startMs) / totalMs.value) * chartW.value
    return Math.max(w, MIN_BAR_W)
  }

  function clip(s: string, n: number): string {
    return s.length > n ? s.slice(0, n) + '…' : s
  }

  function fitLabel(text: string, width: number): string {
    const chars = Math.max(Math.floor(width / 7) - 1, 0)
    return chars < 3 ? '' : clip(text, chars)
  }

  const todayX = computed(() => xPos(today))

  // --- Tooltip ---
  const tooltip = ref<{ ticket: Ticket; x: number; y: number } | null>(null)

  function showTooltip(event: MouseEvent, ticket: Ticket) {
    tooltip.value = { ticket, x: event.clientX, y: event.clientY }
  }

  function moveTooltip(event: MouseEvent) {
    if (tooltip.value) {
      tooltip.value = { ...tooltip.value, x: event.clientX, y: event.clientY }
    }
  }

  function hideTooltip() {
    tooltip.value = null
  }

  const tooltipStyle = computed(() => {
    if (!tooltip.value) return {}
    const { x, y } = tooltip.value
    const flip = x > window.innerWidth - 290
    return {
      left: (flip ? x - 272 : x + 16) + 'px',
      top: Math.min(y - 8, window.innerHeight - 300) + 'px',
    }
  })

  function fmtDate(iso: string): string {
    return new Date(iso).toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric' })
  }

  // --- Edit ticket modal ---
  const editingTicketId = ref<string | null>(null)
  const showEditModal = ref(false)

  function openEdit(ticket: Ticket) {
    hideTooltip()
    editingTicketId.value = ticket.id
    showEditModal.value = true
  }

  // --- Edit epic modal ---
  const editingEpicId = ref<string | null>(null)
  const showEpicEditModal = ref(false)

  function openEditEpic(epic: Epic) {
    editingEpicId.value = epic.id
    showEpicEditModal.value = true
  }


</script>

<template>
  <div>
    <div class="mb-6 flex items-center justify-between">
      <h2 class="text-2xl font-bold text-gray-900">Timeline</h2>
      <div class="flex rounded-lg border border-gray-200 overflow-hidden">
        <button
          :class="['px-4 py-2 text-sm font-medium transition', viewMode === 'ticket' ? 'bg-primary-600 text-white' : 'bg-white text-gray-600 hover:bg-gray-50']"
          @click="viewMode = 'ticket'"
        >By Ticket</button>
        <button
          :class="['px-4 py-2 text-sm font-medium transition border-l border-gray-200', viewMode === 'assignee' ? 'bg-primary-600 text-white' : 'bg-white text-gray-600 hover:bg-gray-50']"
          @click="viewMode = 'assignee'"
        >By Assignee</button>
        <button
          :class="['px-4 py-2 text-sm font-medium transition border-l border-gray-200', viewMode === 'epic' ? 'bg-indigo-600 text-white' : 'bg-white text-gray-600 hover:bg-gray-50']"
          @click="viewMode = 'epic'"
        >By Epic</button>
      </div>
    </div>

    <div v-if="store.loading" class="text-center text-gray-400">Loading…</div>
    <div v-else-if="!store.tickets.length" class="text-center text-gray-400">No tickets yet.</div>

    <div v-else ref="containerRef" class="overflow-x-auto rounded-xl bg-white shadow-sm">

      <!-- ====== By Ticket ====== -->
      <svg v-if="viewMode === 'ticket'" :width="svgWidth" :height="ticketSvgH" class="block">
        <line v-for="tick in dateTicks" :key="tick.x" :x1="tick.x" y1="0" :x2="tick.x" :y2="ticketSvgH" stroke="#f3f4f6" stroke-width="1" />
        <g v-for="band in monthBands" :key="band.label">
          <rect :x="band.x1" y="0" :width="band.x2 - band.x1" :height="MONTH_H" fill="#f8fafc" stroke="#e5e7eb" stroke-width="0.5" />
          <text :x="(band.x1 + band.x2) / 2" :y="MONTH_H / 2 + 4" font-size="11" font-weight="600" fill="#374151" text-anchor="middle" class="select-none">{{ band.label }}</text>
        </g>
        <g v-for="tick in dateTicks" :key="`day-${tick.x}`">
          <line :x1="tick.x" :y1="MONTH_H" :x2="tick.x" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
          <text :x="tick.x + 3" :y="HEADER_H - 5" font-size="10" fill="#9ca3af" class="select-none">{{ tick.label }}</text>
        </g>
        <line x1="0" :y1="HEADER_H" :x2="svgWidth" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
        <line :x1="todayX" y1="0" :x2="todayX" :y2="ticketSvgH" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4 3" opacity="0.5" />
        <g v-for="row in ticketRows" :key="row.ticket.id"
           @mouseenter="showTooltip($event, row.ticket)"
           @mousemove="moveTooltip"
           @mouseleave="hideTooltip"
           @click="openEdit(row.ticket)"
           class="cursor-pointer"
        >
          <rect x="0" :y="row.y" :width="svgWidth" :height="ROW_H" fill="transparent" />
          <line x1="0" :y1="row.y" :x2="svgWidth" :y2="row.y" stroke="#f3f4f6" stroke-width="1" />
          <text x="8" :y="row.y + ROW_H / 2 + 4" font-size="12" fill="#374151" class="select-none">{{ row.rowLabel }}</text>
          <defs>
            <clipPath :id="`clip-t-${row.ticket.id}`">
              <rect :x="row.x" :y="row.y + 6" :width="row.w" :height="ROW_H - 12" rx="4" />
            </clipPath>
          </defs>
          <rect :x="row.x" :y="row.y + 6" :width="row.w" :height="ROW_H - 12" :fill="row.color" rx="4" opacity="0.85" />
          <text :x="row.x + 6" :y="row.y + ROW_H / 2 + 4" font-size="11" fill="white" :clip-path="`url(#clip-t-${row.ticket.id})`" class="select-none pointer-events-none">{{ fitLabel(row.barText, row.w) }}</text>
        </g>
      </svg>

      <!-- ====== By Assignee ====== -->
      <svg v-else-if="viewMode === 'assignee'" :width="svgWidth" :height="teamSvgH" class="block">
        <line v-for="tick in dateTicks" :key="tick.x" :x1="tick.x" y1="0" :x2="tick.x" :y2="teamSvgH" stroke="#f3f4f6" stroke-width="1" />
        <g v-for="band in monthBands" :key="band.label">
          <rect :x="band.x1" y="0" :width="band.x2 - band.x1" :height="MONTH_H" fill="#f8fafc" stroke="#e5e7eb" stroke-width="0.5" />
          <text :x="(band.x1 + band.x2) / 2" :y="MONTH_H / 2 + 4" font-size="11" font-weight="600" fill="#374151" text-anchor="middle" class="select-none">{{ band.label }}</text>
        </g>
        <g v-for="tick in dateTicks" :key="`day-${tick.x}`">
          <line :x1="tick.x" :y1="MONTH_H" :x2="tick.x" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
          <text :x="tick.x + 3" :y="HEADER_H - 5" font-size="10" fill="#9ca3af" class="select-none">{{ tick.label }}</text>
        </g>
        <line x1="0" :y1="HEADER_H" :x2="svgWidth" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
        <line :x1="todayX" y1="0" :x2="todayX" :y2="teamSvgH" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4 3" opacity="0.5" />
        <g v-for="row in teamRows" :key="row.assignee">
          <line x1="0" :y1="row.y" :x2="svgWidth" :y2="row.y" stroke="#e5e7eb" stroke-width="1" />
          <text x="8" :y="row.y + row.height / 2 + 4" font-size="12" font-weight="600" fill="#374151" class="select-none">{{ clip(row.assignee, 24) }}</text>
          <g v-for="(ticket, ti) in row.tickets" :key="ticket.id"
             @mouseenter="showTooltip($event, ticket)"
             @mousemove="moveTooltip"
             @mouseleave="hideTooltip"
             @click="openEdit(ticket)"
             class="cursor-pointer"
          >
            <line v-if="ti > 0" x1="0" :y1="row.y + ti * ROW_H" :x2="svgWidth" :y2="row.y + ti * ROW_H" stroke="#f9fafb" stroke-width="1" />
            <defs>
              <clipPath :id="`clip-tm-${ticket.id}`">
                <rect :x="ticketBarX(ticket)" :y="row.y + ti * ROW_H + 6" :width="ticketBarW(ticket)" :height="ROW_H - 12" rx="4" />
              </clipPath>
            </defs>
            <rect :x="ticketBarX(ticket)" :y="row.y + ti * ROW_H + 6" :width="ticketBarW(ticket)" :height="ROW_H - 12" :fill="STATUS_COLORS[ticket.status]" rx="4" opacity="0.85" />
            <text :x="ticketBarX(ticket) + 6" :y="row.y + ti * ROW_H + ROW_H / 2 + 4" font-size="11" fill="white" :clip-path="`url(#clip-tm-${ticket.id})`" class="select-none pointer-events-none">{{ fitLabel(ticket.title, ticketBarW(ticket)) }}</text>
          </g>
        </g>
        <line x1="0" :y1="teamSvgH - PAD" :x2="svgWidth" :y2="teamSvgH - PAD" stroke="#e5e7eb" stroke-width="1" />
      </svg>

      <!-- ====== By Epic ====== -->
      <svg v-else :width="svgWidth" :height="epicSvgH" class="block">
        <line v-for="tick in dateTicks" :key="tick.x" :x1="tick.x" y1="0" :x2="tick.x" :y2="epicSvgH" stroke="#f3f4f6" stroke-width="1" />
        <g v-for="band in monthBands" :key="band.label">
          <rect :x="band.x1" y="0" :width="band.x2 - band.x1" :height="MONTH_H" fill="#f8fafc" stroke="#e5e7eb" stroke-width="0.5" />
          <text :x="(band.x1 + band.x2) / 2" :y="MONTH_H / 2 + 4" font-size="11" font-weight="600" fill="#374151" text-anchor="middle" class="select-none">{{ band.label }}</text>
        </g>
        <g v-for="tick in dateTicks" :key="`day-${tick.x}`">
          <line :x1="tick.x" :y1="MONTH_H" :x2="tick.x" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
          <text :x="tick.x + 3" :y="HEADER_H - 5" font-size="10" fill="#9ca3af" class="select-none">{{ tick.label }}</text>
        </g>
        <line x1="0" :y1="HEADER_H" :x2="svgWidth" :y2="HEADER_H" stroke="#e5e7eb" stroke-width="1" />
        <line :x1="todayX" y1="0" :x2="todayX" :y2="epicSvgH" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4 3" opacity="0.5" />

        <!-- Epic groups -->
        <g v-for="(g, gi) in epicGroups" :key="g.key">
          <!-- Epic header row -->
          <g @click="toggleEpic(g.key)" class="cursor-pointer">
            <rect
              x="0" :y="epicRowPositions[gi].epicY"
              :width="svgWidth" :height="EPIC_ROW_H"
              :fill="g.epic ? '#eef2ff' : '#f9fafb'"
              stroke="#e5e7eb" stroke-width="0.5"
            />
            <!-- Chevron -->
            <text
              x="8" :y="epicRowPositions[gi].epicY + EPIC_ROW_H / 2 + 5"
              font-size="11" fill="#6366f1" class="select-none"
            >{{ g.isExpanded ? '▼' : '▶' }}</text>
            <!-- Epic label -->
            <text
              x="22" :y="epicRowPositions[gi].epicY + EPIC_ROW_H / 2 + 5"
              font-size="13" font-weight="600"
              :fill="g.epic ? '#4338ca' : '#9ca3af'"
              class="select-none"
            >{{ g.epic ? clip(g.epic.title, 22) : 'No Epic' }}</text>
            <!-- Ticket count badge -->
            <text
              x="22" :y="epicRowPositions[gi].epicY + EPIC_ROW_H / 2 + 18"
              font-size="9" fill="#9ca3af" class="select-none"
            >{{ g.tickets.length }} ticket{{ g.tickets.length !== 1 ? 's' : '' }}</text>
          </g>

          <!-- Epic bar (only for named epics) -->
          <g v-if="g.epic && g.tickets.length" @click.stop="openEditEpic(g.epic)" class="cursor-pointer">
            <defs>
              <clipPath :id="`clip-epic-${g.key}`">
                <rect :x="epicBarX(g)" :y="epicRowPositions[gi].epicY + 8" :width="epicBarW(g)" :height="EPIC_ROW_H - 16" rx="5" />
              </clipPath>
            </defs>
            <rect
              :x="epicBarX(g)" :y="epicRowPositions[gi].epicY + 8"
              :width="epicBarW(g)" :height="EPIC_ROW_H - 16"
              fill="#6366f1" rx="5" opacity="0.9"
            />
            <text
              :x="epicBarX(g) + 8" :y="epicRowPositions[gi].epicY + EPIC_ROW_H / 2 + 4"
              font-size="11" font-weight="600" fill="white"
              :clip-path="`url(#clip-epic-${g.key})`"
              class="select-none pointer-events-none"
            >{{ fitLabel(g.epic.title, epicBarW(g)) }}</text>
          </g>

          <!-- Ticket sub-rows (visible when expanded, sorted by start date) -->
          <g v-if="g.isExpanded">
            <g v-for="(ticket, ti) in g.tickets" :key="ticket.id"
               @mouseenter="showTooltip($event, ticket)"
               @mousemove="moveTooltip"
               @mouseleave="hideTooltip"
               @click="openEdit(ticket)"
               class="cursor-pointer"
            >
              <rect
                x="0"
                :y="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H"
                :width="svgWidth" :height="ROW_H"
                fill="transparent"
              />
              <line
                x1="0" :y1="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H"
                :x2="svgWidth" :y2="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H"
                stroke="#f3f4f6" stroke-width="1"
              />
              <!-- Indent line -->
              <line
                :x1="LABEL_W - 4" :y1="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H"
                :x2="LABEL_W - 4" :y2="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H + ROW_H"
                stroke="#e0e7ff" stroke-width="2"
              />
              <text
                x="16"
                :y="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H + ROW_H / 2 + 4"
                font-size="11" fill="#6b7280" class="select-none"
              >{{ clip(ticket.title, 24) }}</text>
              <defs>
                <clipPath :id="`clip-ep-t-${ticket.id}`">
                  <rect
                    :x="ticketBarX(ticket)"
                    :y="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H + 6"
                    :width="ticketBarW(ticket)"
                    :height="ROW_H - 12"
                    rx="4"
                  />
                </clipPath>
              </defs>
              <rect
                :x="ticketBarX(ticket)"
                :y="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H + 6"
                :width="ticketBarW(ticket)"
                :height="ROW_H - 12"
                :fill="STATUS_COLORS[ticket.status]"
                rx="4" opacity="0.85"
              />
              <text
                :x="ticketBarX(ticket) + 6"
                :y="epicRowPositions[gi].epicY + EPIC_ROW_H + ti * ROW_H + ROW_H / 2 + 4"
                font-size="11" fill="white"
                :clip-path="`url(#clip-ep-t-${ticket.id})`"
                class="select-none pointer-events-none"
              >{{ fitLabel(ticket.assignee?.trim() || ticket.title, ticketBarW(ticket)) }}</text>
            </g>
          </g>

          <!-- Group separator -->
          <line
            x1="0" :y1="epicRowPositions[gi].epicY + EPIC_ROW_H + (g.isExpanded ? g.tickets.length * ROW_H : 0)"
            :x2="svgWidth" :y2="epicRowPositions[gi].epicY + EPIC_ROW_H + (g.isExpanded ? g.tickets.length * ROW_H : 0)"
            stroke="#e5e7eb" stroke-width="1"
          />
        </g>
      </svg>

      <!-- Legend -->
      <div class="flex flex-wrap items-center gap-4 px-4 py-3 border-t text-xs text-gray-500">
        <span class="font-medium">Status:</span>
        <span v-for="(color, s) in STATUS_COLORS" :key="s" class="flex items-center gap-1">
          <span class="inline-block h-3 w-3 rounded-sm" :style="{ background: color }" />
          {{ STATUS_LABELS[s as TicketStatus] }}
        </span>
        <span v-if="viewMode === 'epic'" class="flex items-center gap-1 border-l pl-4">
          <span class="inline-block h-3 w-3 rounded-sm bg-indigo-500"></span>
          Epic
        </span>
        <span class="flex items-center gap-1 ml-2">
          <svg width="20" height="12"><line x1="0" y1="6" x2="20" y2="6" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4 3" /></svg>
          Today
        </span>
        <span v-if="viewMode !== 'epic'" class="ml-2 text-gray-400">· Click a bar to edit · By Ticket: label = assignee · By Assignee: label = task</span>
        <span v-else class="ml-2 text-gray-400">· Click ▼/▶ to expand/collapse · Click bar to edit</span>
      </div>
    </div>
  </div>

  <!-- Edit ticket modal -->
  <TicketDetailModal
    :ticket-id="editingTicketId ?? ''"
    :project-id="projectId"
    :show="showEditModal"
    @close="showEditModal = false"
    @deleted="showEditModal = false"
  />

  <!-- Edit epic modal -->
  <EpicDetailModal
    :epic-id="editingEpicId ?? ''"
    :project-id="projectId"
    :show="showEpicEditModal"
    @close="showEpicEditModal = false"
    @deleted="showEpicEditModal = false"
  />

  <!-- Hover tooltip (teleported to body to escape overflow-x-auto clipping) -->
  <Teleport to="body">
    <div
      v-if="tooltip"
      class="fixed z-50 pointer-events-none bg-white rounded-xl shadow-xl border border-gray-200 p-4 text-sm w-64"
      :style="tooltipStyle"
    >
      <div class="flex items-start justify-between gap-2 mb-3">
        <p class="font-semibold text-gray-900 leading-snug">{{ tooltip.ticket.title }}</p>
        <span
          class="shrink-0 rounded-full px-2 py-0.5 text-xs font-medium"
          :style="{ background: PRIORITY_COLORS[tooltip.ticket.priority] + '22', color: PRIORITY_COLORS[tooltip.ticket.priority] }"
        >{{ tooltip.ticket.priority }}</span>
      </div>
      <dl class="space-y-1.5 text-xs">
        <div class="flex justify-between">
          <dt class="text-gray-500">Status</dt>
          <dd class="font-medium text-gray-800">{{ STATUS_LABELS[tooltip.ticket.status] }}</dd>
        </div>
        <div v-if="tooltip.ticket.assignee" class="flex justify-between">
          <dt class="text-gray-500">Assignee</dt>
          <dd class="font-medium text-gray-800">{{ tooltip.ticket.assignee }}</dd>
        </div>
        <div v-if="tooltip.ticket.startDate" class="flex justify-between">
          <dt class="text-gray-500">Start</dt>
          <dd class="font-medium text-gray-800">{{ fmtDate(tooltip.ticket.startDate) }}</dd>
        </div>
        <div v-if="tooltip.ticket.finishedDate" class="flex justify-between">
          <dt class="text-gray-500">Finish</dt>
          <dd class="font-medium text-gray-800">{{ fmtDate(tooltip.ticket.finishedDate) }}</dd>
        </div>
        <div v-if="tooltip.ticket.description" class="pt-1 border-t border-gray-100">
          <dt class="text-gray-500 mb-0.5">Description</dt>
          <dd class="text-gray-700 line-clamp-3">{{ tooltip.ticket.description }}</dd>
        </div>
        <div v-if="tooltip.ticket.tags?.length" class="flex flex-wrap gap-1 pt-1 border-t border-gray-100">
          <span v-for="tag in tooltip.ticket.tags" :key="tag" class="rounded-full bg-gray-100 px-2 py-0.5 text-gray-600">{{ tag }}</span>
        </div>
      </dl>
    </div>
  </Teleport>
</template>
