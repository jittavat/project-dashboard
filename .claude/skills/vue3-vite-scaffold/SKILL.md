---
name: vue3-vite-scaffold
description: Add a new Vue 3 view, store, or API module to the frontend. Use when creating a new page, feature, or Pinia store.
trigger: /vue3-vite-scaffold
---

# /vue3-vite-scaffold

Add a new feature to the Vue 3 frontend (`frontend/src/`).

## Stack already in place

| Layer | Choice |
|---|---|
| Framework | Vue 3 Composition API |
| State | Pinia (`defineStore` with setup function) |
| Router | Vue Router 4 — lazy-loaded views |
| HTTP | Axios with JWT + 401 interceptors in `src/api/index.ts` |
| Styles | Tailwind CSS 3 |
| Tests | Vitest 4 + `@vue/test-utils` + jsdom |
| Types | TypeScript strict mode |

## New view checklist

1. **Create view** at `src/views/<Name>View.vue`
2. **Add route** in `src/router/index.ts` — use lazy import:
   ```typescript
   { path: '/path', name: 'name', component: () => import('@/views/<Name>View.vue') }
   ```
   Routes inside the `AppLayout` wrapper require auth automatically. Add `meta: { requiresAuth: false }` to opt out.
3. **Add nav link** in `src/components/layout/AppSidebar.vue` if it needs sidebar entry.

## New Pinia store pattern

```typescript
// src/stores/<resource>.ts
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { resourceApi } from '@/api/resource'
import type { Resource } from '@/types'

export const useResourceStore = defineStore('<resource>', () => {
  const items = ref<Resource[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchAll() {
    loading.value = true
    error.value = null
    try {
      const { data } = await resourceApi.list()
      items.value = data
    } catch (e: unknown) {
      error.value = (e as Error).message
    } finally {
      loading.value = false
    }
  }

  return { items, loading, error, fetchAll }
})
```

## New API module pattern

```typescript
// src/api/<resource>.ts
import apiClient from './index'
import type { Resource } from '@/types'

export const resourceApi = {
  list:   ()                   => apiClient.get<Resource[]>('/resources'),
  get:    (id: string)         => apiClient.get<Resource>(`/resources/${id}`),
  create: (payload: unknown)   => apiClient.post<Resource>('/resources', payload),
  update: (id: string, p: unknown) => apiClient.put<Resource>(`/resources/${id}`, p),
  delete: (id: string)         => apiClient.delete(`/resources/${id}`),
}
```

The axios client in `src/api/index.ts` automatically attaches the JWT from `localStorage`, redirects to `/login` on 401, and **transforms all snake_case response keys to camelCase** via a `camelizeKeys` response interceptor. Do not add manual key mapping in stores or views — the interceptor handles it globally.

## Adding a type

Add to `src/types/index.ts`. Always use **camelCase** field names (e.g. `startDate`, `finishedDate`, `createdAt`) — the `camelizeKeys` interceptor automatically converts the backend's snake_case JSON keys before they reach store or view code.

Outgoing payloads (POST/PUT request bodies built in `src/api/*.ts`) must still use snake_case to match the backend: `start_date`, `finished_date`, etc. Only responses are auto-converted.

## Writing a test

Tests live alongside source files or in `__tests__/`. Run with `npm run test`. Coverage with `npm run coverage` (outputs to `coverage/`).

```typescript
// src/stores/__tests__/resource.spec.ts
import { setActivePinia, createPinia } from 'pinia'
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { useResourceStore } from '../resource'
import * as api from '@/api/resource'

describe('resource store', () => {
  beforeEach(() => setActivePinia(createPinia()))

  it('fetchAll populates items', async () => {
    vi.spyOn(api.resourceApi, 'list').mockResolvedValue({ data: [{ id: '1', name: 'test' }] } as any)
    const store = useResourceStore()
    await store.fetchAll()
    expect(store.items).toHaveLength(1)
  })
})
```
