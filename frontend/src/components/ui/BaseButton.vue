<script setup lang="ts">
  withDefaults(
    defineProps<{
      variant?: 'primary' | 'secondary' | 'danger'
      size?: 'sm' | 'md' | 'lg'
      loading?: boolean
      disabled?: boolean
    }>(),
    { variant: 'primary', size: 'md', loading: false, disabled: false }
  )
</script>

<template>
  <button
    :disabled="disabled || loading"
    :class="[
      'inline-flex items-center justify-center rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2',
      {
        'bg-primary-600 text-white hover:bg-primary-700 focus:ring-primary-500': variant === 'primary',
        'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-primary-500': variant === 'secondary',
        'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500': variant === 'danger',
        'px-2.5 py-1.5 text-xs': size === 'sm',
        'px-4 py-2 text-sm': size === 'md',
        'px-6 py-3 text-base': size === 'lg',
        'opacity-50 cursor-not-allowed': disabled || loading,
      },
    ]"
  >
    <span v-if="loading" class="mr-2 h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent" />
    <slot />
  </button>
</template>
