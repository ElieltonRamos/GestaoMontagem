<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Cadastrar Montador</h2>
    
    <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Name Field -->
        <div>
          <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
            Nome <span class="text-red-400">*</span>
          </label>
          <input
            id="name"
            v-model="form.name"
            type="text"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :class="{ 'border-red-500': errors.name }"
            placeholder="Digite o nome do montador"
          />
          <p v-if="errors.name" class="mt-1 text-sm text-red-400">{{ errors.name }}</p>
        </div>

        <!-- Phone Field -->
        <div>
          <label for="phone" class="block text-sm font-medium text-gray-300 mb-2">
            Telefone <span class="text-red-400">*</span>
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
            CPF <span class="text-gray-500 text-xs">(Opcional)</span>
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
            Endereço <span class="text-gray-500 text-xs">(Opcional)</span>
          </label>
          <textarea
            id="address"
            v-model="form.address"
            rows="3"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
            placeholder="Digite o endereço"
          ></textarea>
        </div>

        <!-- Submit Button -->
        <div class="flex justify-end pt-4">
          <button
            type="submit"
            :disabled="isSubmitting"
            class="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isSubmitting ? 'Salvando...' : 'Salvar Montador' }}
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
import { assemblersService } from '../services/assemblers.service.tauri'

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
  
  // Limita a 11 dígitos
  if (value.length > 11) {
    value = value.substring(0, 11)
  }
  
  // Formata conforme a quantidade de dígitos
  if (value.length <= 2) {
    value = value.replace(/^(\d*)/, '($1')
  } else if (value.length <= 7) {
    // (38) 9886
    value = value.replace(/^(\d{2})(\d{0,5})/, '($1) $2')
  } else if (value.length <= 10) {
    // (38) 9886-635 (telefone fixo)
    value = value.replace(/^(\d{2})(\d{4})(\d{0,4})/, '($1) $2-$3')
  } else {
    // (38) 98866-3580 (celular com 9 dígitos)
    value = value.replace(/^(\d{2})(\d{5})(\d{4})/, '($1) $2-$3')
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

const validateForm = async (): Promise<boolean> => {
  let isValid = true
  
  errors.name = ''
  errors.phone = ''
  
  if (!form.name.trim()) {
    errors.name = 'Nome é obrigatório'
    isValid = false
  } else {
    const exists = await assemblersService.existsByName(form.name)
    if (exists) {
      errors.name = 'Já existe um montador com este nome'
      isValid = false
    }
  }
  
  const phoneDigits = form.phone.replace(/\D/g, '')
  if (!form.phone.trim()) {
    errors.phone = 'Telefone é obrigatório'
    isValid = false
  } else if (phoneDigits.length < 10) {
    errors.phone = 'O telefone deve ter pelo menos 10 dígitos'
    isValid = false
  }
  
  return isValid
}

const handleSubmit = async () => {
  const isValid = await validateForm()
  
  if (!isValid) {
    return
  }
  
  isSubmitting.value = true
  
  try {
    const cleanPhone = form.phone.replace(/\D/g, '')
    const cleanCPF = form.cpf ? form.cpf.replace(/\D/g, '') : undefined
    
    const newAssembler = await assemblersService.create(
      form.name.trim(),
      cleanPhone,
      cleanCPF,
      form.address.trim() || undefined
    )
    
    if (!newAssembler) {
      toast.message = 'Erro ao cadastrar montador'
      toast.type = 'error'
      return
    }
    
    toast.message = 'Montador cadastrado com sucesso!'
    toast.type = 'success'
    
    form.name = ''
    form.phone = ''
    form.cpf = ''
    form.address = ''
    
  } catch (error) {
    toast.message = 'Erro ao cadastrar montador'
    toast.type = 'error'
  } finally {
    isSubmitting.value = false
  }
}
</script>
