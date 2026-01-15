<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50"
        @click.self="cancel"
      >
        <div class="bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <div class="p-6">
            <div class="flex items-start gap-4">
              <div class="shrink-0 w-10 h-10 rounded-full bg-red-500 bg-opacity-20 flex items-center justify-center">
                <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
              </div>
              <div class="flex-1">
                <h3 class="text-lg font-semibold text-white mb-2">{{ title }}</h3>
                <p class="text-gray-400 text-sm">{{ message }}</p>
              </div>
            </div>
          </div>

          <div class="flex justify-end gap-3 px-6 pb-6">
            <button
              @click="cancel"
              class="px-4 py-2 text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
            >
              Cancel
            </button>
            <button
              @click="confirm"
              class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  isOpen: boolean
  title?: string
  message?: string
}

withDefaults(defineProps<Props>(), {
  title: 'Confirmar Acão',
  message: 'Tem certeza de que deseja prosseguir?'
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const confirm = () => emit('confirm')
const cancel = () => emit('cancel')
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
