<script setup lang="ts">
  import { useConfirm } from '@/composables/useConfirm'

  const { state, onConfirm, onCancel } = useConfirm()
</script>

<template>
  <Teleport to="body">
    <Transition name="confirm-backdrop">
      <div
        v-if="state.visible"
        class="fixed inset-0 z-[100] flex items-center justify-center p-4"
        @click.self="onCancel"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-gray-900/60 backdrop-blur-sm" @click="onCancel" />

        <!-- Card -->
        <Transition name="confirm-card">
          <div
            v-if="state.visible"
            class="relative w-full max-w-sm rounded-2xl bg-white shadow-2xl ring-1 ring-gray-900/10"
          >
            <!-- Icon strip -->
            <div
              :class="[
                'flex h-12 w-12 items-center justify-center rounded-full mx-auto mt-6',
                state.variant === 'danger'
                  ? 'bg-red-100'
                  : 'bg-indigo-100',
              ]"
            >
              <svg
                v-if="state.variant === 'danger'"
                class="h-6 w-6 text-red-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z" />
              </svg>
              <svg
                v-else
                class="h-6 w-6 text-indigo-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2"
              >
                <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
              </svg>
            </div>

            <!-- Content -->
            <div class="px-6 pb-2 pt-4 text-center">
              <h3 class="text-base font-semibold text-gray-900">{{ state.title }}</h3>
              <p class="mt-2 text-sm leading-relaxed text-gray-500">{{ state.message }}</p>
            </div>

            <!-- Actions -->
            <div class="flex gap-3 px-6 pb-6 pt-4">
              <button
                type="button"
                class="flex-1 rounded-xl border border-gray-200 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-gray-400"
                @click="onCancel"
              >
                {{ state.cancelLabel }}
              </button>
              <button
                type="button"
                :class="[
                  'flex-1 rounded-xl px-4 py-2.5 text-sm font-medium text-white transition focus:outline-none focus-visible:ring-2',
                  state.variant === 'danger'
                    ? 'bg-red-600 hover:bg-red-700 focus-visible:ring-red-500'
                    : 'bg-indigo-600 hover:bg-indigo-700 focus-visible:ring-indigo-500',
                ]"
                @click="onConfirm"
              >
                {{ state.confirmLabel }}
              </button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
  .confirm-backdrop-enter-active,
  .confirm-backdrop-leave-active {
    transition: opacity 0.2s ease;
  }
  .confirm-backdrop-enter-from,
  .confirm-backdrop-leave-to {
    opacity: 0;
  }

  .confirm-card-enter-active {
    transition: opacity 0.2s ease, transform 0.2s ease;
  }
  .confirm-card-leave-active {
    transition: opacity 0.15s ease, transform 0.15s ease;
  }
  .confirm-card-enter-from {
    opacity: 0;
    transform: scale(0.95) translateY(8px);
  }
  .confirm-card-leave-to {
    opacity: 0;
    transform: scale(0.95) translateY(8px);
  }
</style>
