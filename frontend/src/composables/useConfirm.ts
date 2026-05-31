import { ref } from 'vue'

export interface ConfirmOptions {
  title: string
  message: string
  confirmLabel?: string
  cancelLabel?: string
  variant?: 'default' | 'danger'
}

interface ConfirmState extends ConfirmOptions {
  visible: boolean
  resolve: ((value: boolean) => void) | null
}

const state = ref<ConfirmState>({
  visible: false,
  title: '',
  message: '',
  confirmLabel: 'Confirm',
  cancelLabel: 'Cancel',
  variant: 'default',
  resolve: null,
})

export function useConfirm() {
  function confirm(options: ConfirmOptions): Promise<boolean> {
    return new Promise((resolve) => {
      state.value = {
        ...options,
        confirmLabel: options.confirmLabel ?? 'Confirm',
        cancelLabel: options.cancelLabel ?? 'Cancel',
        variant: options.variant ?? 'default',
        visible: true,
        resolve,
      }
    })
  }

  function onConfirm() {
    state.value.resolve?.(true)
    state.value.visible = false
    state.value.resolve = null
  }

  function onCancel() {
    state.value.resolve?.(false)
    state.value.visible = false
    state.value.resolve = null
  }

  return { confirm, state, onConfirm, onCancel }
}
