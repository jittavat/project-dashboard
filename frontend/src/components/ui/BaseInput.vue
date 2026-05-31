<script setup lang="ts">
  defineProps<{
    modelValue: string
    label?: string
    placeholder?: string
    type?: string
    error?: string
    required?: boolean
  }>()

  defineEmits<{
    'update:modelValue': [value: string]
  }>()
</script>

<template>
  <div class="flex flex-col gap-1">
    <label v-if="label" class="text-sm font-medium text-gray-700">
      {{ label }} <span v-if="required" class="text-red-500">*</span>
    </label>
    <input
      :value="modelValue"
      :type="type ?? 'text'"
      :placeholder="placeholder"
      :required="required"
      :class="[
        'rounded-md border px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500',
        error ? 'border-red-300 focus:border-red-400' : 'border-gray-300 focus:border-primary-400',
      ]"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <p v-if="error" class="text-xs text-red-600">{{ error }}</p>
  </div>
</template>
