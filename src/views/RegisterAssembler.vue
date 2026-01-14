<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Register Assembler</h2>
    
    <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Name Field -->
        <div>
          <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
            Name <span class="text-red-400">*</span>
          </label>
          <input
            id="name"
            v-model="form.name"
            type="text"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :class="{ 'border-red-500': errors.name }"
            placeholder="Enter assembler name"
          />
          <p v-if="errors.name" class="mt-1 text-sm text-red-400">{{ errors.name }}</p>
        </div>

        <!-- Phone Field -->
        <div>
          <label for="phone" class="block text-sm font-medium text-gray-300 mb-2">
            Phone <span class="text-red-400">*</span>
          </label>
          <input
            id="phone"
            v-model="form.phone"
            type="tel"
            @input="formatPhone"
            maxlength="15"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :class="{ 'border-red-500': errors.phone }"
            placeholder="(11) 98765-4321"
          />
          <p v-if="errors.phone" class="mt-1 text-sm text-red-400">{{ errors.phone }}</p>
        </div>

        <!-- CPF Field (Optional) -->
        <div>
          <label for="cpf" class="block text-sm font-medium text-gray-300 mb-2">
            CPF <span class="text-gray-500 text-xs">(Optional)</span>
          </label>
          <input
            id="cpf"
            v-model="form.cpf"
            type="text"
            @input="formatCPF"
            maxlength="14"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            placeholder="000.000.000-00"
          />
        </div>

        <!-- Address Field (Optional) -->
        <div>
          <label for="address" class="block text-sm font-medium text-gray-300 mb-2">
            Address <span class="text-gray-500 text-xs">(Optional)</span>
          </label>
          <textarea
            id="address"
            v-model="form.address"
            rows="3"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
            placeholder="Enter address"
          ></textarea>
        </div>

        <!-- Submit Button -->
        <div class="flex justify-end pt-4">
          <button
            type="submit"
            :disabled="isSubmitting"
            class="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isSubmitting ? 'Saving...' : 'Save Assembler' }}
          </button>
        </div>
      </form>
    </div>

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import Toast from '@/components/Toast.vue'
import { assemblersService } from '../services'

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

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

const isSubmitting = ref(false)

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
  
  // Reset errors
  errors.name = ''
  errors.phone = ''
  
  // Validate name
  if (!form.name.trim()) {
    errors.name = 'Name is required'
    isValid = false
  } else if (assemblersService.existsByName(form.name)) {
    errors.name = 'An assembler with this name already exists'
    isValid = false
  }
  
  // Validate phone
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

const handleSubmit = async () => {
  if (!validateForm()) {
    return
  }
  
  isSubmitting.value = true
  
  try {
    // Clean phone and CPF (remove formatting)
    const cleanPhone = form.phone.replace(/\D/g, '')
    const cleanCPF = form.cpf ? form.cpf.replace(/\D/g, '') : undefined
    
    assemblersService.create(
      form.name.trim(),
      cleanPhone,
      cleanCPF,
      form.address.trim() || undefined
    )
    
    // Show success toast
    toast.message = 'Assembler registered successfully!'
    toast.type = 'success'
    
    // Reset form
    form.name = ''
    form.phone = ''
    form.cpf = ''
    form.address = ''
    
  } catch (error) {
    toast.message = 'Error registering assembler'
    toast.type = 'error'
  } finally {
    isSubmitting.value = false
  }
}
</script>
