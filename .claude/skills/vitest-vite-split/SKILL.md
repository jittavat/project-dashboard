---
name: vitest-vite-split
description: Vitest 4 + Vite 8 split config pattern. Use when modifying test configuration, adding coverage settings, or diagnosing TypeScript errors in vite.config.ts or vitest.config.ts.
trigger: /vitest-vite-split
---

# /vitest-vite-split

Why this project has two config files and how they work together.

## Current versions

| Package | Version |
|---|---|
| vite | ^8 |
| @vitejs/plugin-vue | ^6 |
| vitest | ^4 |
| @vitest/coverage-v8 | ^4 |
| typescript | ^6 |

## The problem

Vitest bundles its own internal copy of Vite (`node_modules/vitest/node_modules/vite`). When `defineConfig` is imported from `vitest/config` in the same file as `@vitejs/plugin-vue` (which resolves from the top-level `node_modules/vite`), TypeScript sees two incompatible `Plugin` types and throws a cascade of type errors.

This applies to every major Vitest version (2, 3, 4) — the split config pattern is the permanent fix.

## The solution — split config files

**`vite.config.ts`** — owns all build and dev server config, imports only from `vite`:

```typescript
import { defineConfig } from 'vite'              // top-level vite
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  resolve: { alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) } },
  server: { port: 5173, proxy: { '/api': { target: 'http://localhost:8080', changeOrigin: true } } },
})
```

**`vitest.config.ts`** — owns all test config, uses `mergeConfig` to inherit vite config:

```typescript
import { defineConfig, mergeConfig } from 'vitest/config'   // vitest's bundled vite
import viteConfig from './vite.config'

export default mergeConfig(viteConfig, defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html', 'lcov'],
      include: ['src/**/*.{ts,vue}'],
      exclude: ['src/main.ts', 'src/env.d.ts'],
    },
  },
}))
```

`mergeConfig` deep-merges the two configs so plugins, aliases, and proxy settings from `vite.config.ts` are inherited by the test runner.

## TypeScript 6 — `ignoreDeprecations` required

TypeScript 6 deprecated `baseUrl`. The `@/` path alias still requires it (until Vite/TypeScript fully migrate to the new resolution). `tsconfig.json` must include:

```json
"ignoreDeprecations": "6.0"
```

Without this, `vue-tsc --noEmit` (called by `npm run build`) aborts with a TS5101 error.

## Auto-detection

Vitest automatically picks up `vitest.config.ts` when it exists — no `--config` flag needed. The npm scripts work as-is:

```bash
npm run test        # vitest run --passWithNoTests
npm run coverage    # vitest run --coverage
npm run test:watch  # vitest (interactive watch mode)
```

## What NOT to do

- Do not import `defineConfig` from `vitest/config` in `vite.config.ts` — it breaks the `@vitejs/plugin-vue` types
- Do not add a `test:` block to `vite.config.ts` — TypeScript will error on the unknown property
- Do not use `/// <reference types="vitest" />` — the vite version mismatch still causes plugin type conflicts regardless of the reference

## Adding test utilities

Global test utilities (custom matchers, setup files) go in `vitest.config.ts`:

```typescript
test: {
  setupFiles: ['./src/test/setup.ts'],
  globals: true,
}
```
