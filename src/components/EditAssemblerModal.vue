<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50"
        @click.self="closeModal"
      >
        <div class="bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
          <!-- Header -->
          <div class="flex items-center justify-between p-6 border-b border-gray-700">
            <h3 class="text-xl font-bold text-white">Edit Assembler</h3>
            <button
              @click="closeModal"
              class="text-gray-400 hover:text-white transition-colors"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Form -->
          <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
            <!-- Name Field -->
            <div>
              <label for="edit-name" class="block text-sm font-medium text-gray-300 mb-2">
                Name <span class="text-red-400">*</span>
              </label>
              <input
                id="edit-name"
                v-model="form.name"
                type="text"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                :class="{ 'border-red-500': errors.name }"
                placeholder="Enter assembler name"
              />
              <p v-if="errors.name" class="mt-1 text-sm text-red-400">{{ errors.name }}</p>
            </div>

            <!-- Phone Field -->
            <div>
              <label for="edit-phone" class="block text-sm font-medium text-gray-300 mb-2">
                Phone <span class="text-red-400">*</span>
              </label>
              <input
                id="edit-phone"
                v-model="form.phone"
                type="tel"
                @input="formatPhone"
                maxlength="15"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                :class="{ 'border-red-500': errors.phone }"
                placeholder="(11) 98765-4321"
              />
              <p v-if="errors.phone" class="mt-1 text-sm text-red-400">{{ errors.phone }}</p>
            </div>

            <!-- CPF Field -->
            <div>
              <label for="edit-cpf" class="block text-sm font-medium text-gray-300 mb-2">
                CPF <span class="text-gray-500 text-xs">(Optional)</span>
              </label>
              <input
                id="edit-cpf"
                v-model="form.cpf"
                type="text"
                @input="formatCPF"
                maxlength="14"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="000.000.000-00"
              />
            </div>

            <!-- Address Field -->
            <div>
              <label for="edit-address" class="block text-sm font-medium text-gray-300 mb-2">
                Address <span class="text-gray-500 text-xs">(Optional)</span>
              </label>
              <textarea
                id="edit-address"
                v-model="form.address"
                rows="3"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                placeholder="Enter address"
              ></textarea>
            </div>

            <!-- Actions -->
            <div class="flex justify-end gap-3 pt-4">
              <button
                type="button"
                @click="closeModal"
                class="px-4 py-2 text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
              >
                Cancel
              </button>
              <button
                type="submit"
                :disabled="isSubmitting"
                class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
              >
                {{ isSubmitting ? 'Saving...' : 'Save Changes' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { Assembler } from '../types';
import { assemblersService } from '../services';

interface Props {
  isOpen: boolean
  assembler: Assembler | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  saved: []
}>()

const form = reactive({
  name: '',
  phone: '',
  cpf: '',
  address: ''
})

const errors = reactive({
  name: '',
  phone: ''
})

const isSubmitting = ref(false)

watch(() => props.assembler, (newAssembler) => {
  if (newAssembler) {
    form.name = newAssembler.name
    form.phone = formatPhoneDisplay(newAssembler.phone)
    form.cpf = newAssembler.cpf ? formatCPFDisplay(newAssembler.cpf) : ''
    form.address = newAssembler.address || ''
  }
})

const formatPhoneDisplay = (phone: string): string => {
  const digits = phone.replace(/\D/g, '')
  return digits.replace(/^(\d{2})(\d{5})(\d{4})/, '($1) $2-$3')
}

const formatCPFDisplay = (cpf: string): string => {
  const digits = cpf.replace(/\D/g, '')
  return digits.replace(/(\d{3})(\d{3})(\d{3})(\d{2})/, '$1.$2.$3-$4')
}

const formatPhone = (event: Event) => {
  const input = event.target as HTMLInputElement
  let value = input.value.replace(/\D/g, '')
  
  if (value.length <= 11) {
    value = value.replace(/^(\d{2})(\d{5})(\d{4}).*/, '($1) $2-$3')
    value = value.replace(/^(\d{2})(\d{4})(\d{0,4}).*/, '($1) $2-$3')
    value = value.replace(/^(\d{2})(\d{0,5})/, '($1) $2')
    value = value.replace(/^(\d*)/, '($1')
  }
  
  form.phone = value
}

const formatCPF = (event: Event) => {
  const input = event.target as HTMLInputElement
  let value = input.value.replace(/\D/g, '')
  
  if (value.length <= 11) {
    value = value.replace(/(\d{3})(\d{3})(\d{3})(\d{2})/, '$1.$2.$3-$4')
    value = value.replace(/(\d{3})(\d{3})(\d{3})/, '$1.$2.$3')
    value = value.replace(/(\d{3})(\d{3})/, '$1.$2')
  }
  
  form.cpf = value
}

const validateForm = (): boolean => {
  let isValid = true
  
  errors.name = ''
  errors.phone = ''
  
  if (!form.name.trim()) {
    errors.name = 'Name is required'
    isValid = false
  } else if (props.assembler && assemblersService.existsByNameExceptId(form.name, props.assembler.id)) {
    errors.name = 'An assembler with this name already exists'
    isValid = false
  }
  
  const phoneDigits = form.phone.replace(/\D/g, '')
  if (!form.phone.trim()) {
    errors.phone = 'Phone is required'
    isValid = false
  } else if (phoneDigits.length < 10) {
    errors.phone = 'Phone must have at least 10 digits'
    isValid = false
  }
  
  return isValid
}

const handleSubmit = () => {
  if (!validateForm() || !props.assembler) return
  
  isSubmitting.value = true
  
  try {
    const cleanPhone = form.phone.replace(/\D/g, '')
    const cleanCPF = form.cpf ? form.cpf.replace(/\D/g, '') : undefined
    
    assemblersService.update(
      props.assembler.id,
      form.name.trim(),
      cleanPhone,
      cleanCPF,
      form.address.trim() || undefined
    )
    
    emit('saved')
    closeModal()
  } finally {
    isSubmitting.value = false
  }
}

const closeModal = () => {
  errors.name = ''
  errors.phone = ''
  emit('close')
}
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
