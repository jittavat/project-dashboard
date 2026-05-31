<script setup lang="ts">
  import { ref } from 'vue'
  import { useRouter } from 'vue-router'
  import { useAuthStore } from '@/stores/auth'
  import BaseInput from '@/components/ui/BaseInput.vue'
  import BaseButton from '@/components/ui/BaseButton.vue'

  const router = useRouter()
  const store = useAuthStore()

  const email = ref('')
  const cred = ref('')
  const displayName = ref('')
  const mode = ref<'login' | 'register'>('login')
  const loading = ref(false)
  const err = ref('')

  async function submit() {
    loading.value = true
    err.value = ''
    try {
      if (mode.value === 'login') {
        await store.login(email.value, cred.value)
      } else {
        await store.register(email.value, cred.value, displayName.value)
      }
      router.push('/')
    } catch (e: unknown) {
      err.value = (e as Error).message ?? 'Something went wrong'
    } finally {
      loading.value = false
    }
  }
</script>

<template>
  <div class="flex min-h-screen items-center justify-center bg-gray-100">
    <div class="w-full max-w-md rounded-xl bg-white p-8 shadow-lg">
      <h1 class="mb-6 text-2xl font-bold text-gray-900">
        {{ mode === 'login' ? 'Sign in' : 'Create account' }}
      </h1>

      <form class="space-y-4" @submit.prevent="submit">
        <BaseInput v-model="email" label="Email" type="email" placeholder="you@example.com" required />
        <BaseInput
          v-if="mode === 'register'"
          v-model="displayName"
          label="Display name"
          placeholder="Jane Doe"
          required
        />
        <BaseInput v-model="cred" label="Password" type="password" placeholder="••••••••" required />

        <p v-if="err" class="text-sm text-red-600">{{ err }}</p>

        <BaseButton type="submit" class="w-full" :loading="loading">
          {{ mode === 'login' ? 'Sign in' : 'Register' }}
        </BaseButton>
      </form>

      <p class="mt-4 text-center text-sm text-gray-600">
        {{ mode === 'login' ? "Don't have an account?" : 'Already have an account?' }}
        <button
          class="font-medium text-primary-600 hover:underline"
          @click="mode = mode === 'login' ? 'register' : 'login'"
        >
          {{ mode === 'login' ? 'Register' : 'Sign in' }}
        </button>
      </p>
    </div>
  </div>
</template>
