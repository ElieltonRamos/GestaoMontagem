<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Cadastrar Montagem</h2>
    
    <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Order Number -->
        <div>
          <label for="orderNumber" class="block text-sm font-medium text-gray-300 mb-2">
            Número da venda <span class="text-red-400">*</span>
          </label>
          <input
            id="orderNumber"
            v-model="form.orderNumber"
            type="text"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :class="{ 'border-red-500': errors.orderNumber }"
            placeholder="Digite o número da venda"
          />
          <p v-if="errors.orderNumber" class="mt-1 text-sm text-red-400">{{ errors.orderNumber }}</p>
        </div>

        <!-- Date -->
        <div>
          <label for="date" class="block text-sm font-medium text-gray-300 mb-2">
            Data <span class="text-red-400">*</span>
          </label>
          <input
            id="date"
            v-model="form.date"
            type="date"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            :class="{ 'border-red-500': errors.date }"
          />
          <p v-if="errors.date" class="mt-1 text-sm text-red-400">{{ errors.date }}</p>
        </div>

        <!-- Assembler Selection -->
        <div>
          <label for="assembler" class="block text-sm font-medium text-gray-300 mb-2">
            Montador <span class="text-red-400">*</span>
          </label>
          <select
            id="assembler"
            v-model="form.assemblerId"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none cursor-pointer"
            :class="{ 'border-red-500': errors.assemblerId }"
          >
            <option value="" disabled>Selecione um montador</option>
            <option
              v-for="assembler in assemblers"
              :key="assembler.id"
              :value="assembler.id"
            >
              {{ assembler.name }}
            </option>
          </select>
          <p v-if="errors.assemblerId" class="mt-1 text-sm text-red-400">{{ errors.assemblerId }}</p>
        </div>

        <!-- Furniture Description -->
        <div>
          <label for="furnitureDescription" class="block text-sm font-medium text-gray-300 mb-2">
            Descrição do móvel <span class="text-red-400">*</span>
          </label>
          <textarea
            id="furnitureDescription"
            v-model="form.furnitureDescription"
            rows="3"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
            :class="{ 'border-red-500': errors.furnitureDescription }"
            placeholder="Descreva o móvel"
          ></textarea>
          <p v-if="errors.furnitureDescription" class="mt-1 text-sm text-red-400">{{ errors.furnitureDescription }}</p>
        </div>

        <!-- Order Value -->
        <div>
          <label for="orderValue" class="block text-sm font-medium text-gray-300 mb-2">
            Valor da venda <span class="text-red-400">*</span>
          </label>
          <div class="relative">
            <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">R$</span>
            <input
              id="orderValue"
              v-model.number="form.orderValue"
              type="number"
              step="0.01"
              min="0"
              class="w-full pl-12 pr-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              :class="{ 'border-red-500': errors.orderValue }"
              placeholder="0,00"
              @input="calculateAmountPaid"
            />
          </div>
          <p v-if="errors.orderValue" class="mt-1 text-sm text-red-400">{{ errors.orderValue }}</p>
        </div>

        <!-- Percentage Paid -->
        <div>
          <label for="percentagePaid" class="block text-sm font-medium text-gray-300 mb-2">
            Percentual pago <span class="text-red-400">*</span>
          </label>
          <div class="relative">
            <input
              id="percentagePaid"
              v-model.number="form.percentagePaid"
              type="number"
              step="0.01"
              min="0"
              max="100"
              class="w-full pr-8 pl-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              :class="{ 'border-red-500': errors.percentagePaid }"
              placeholder="0,00"
              @input="calculateAmountPaid"
            />
            <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400">%</span>
          </div>
          <p v-if="errors.percentagePaid" class="mt-1 text-sm text-red-400">{{ errors.percentagePaid }}</p>
        </div>

        <!-- Amount Paid (Read-only, calculated) -->
        <div>
          <label for="amountPaid" class="block text-sm font-medium text-gray-300 mb-2">
            Valor pago
          </label>
          <div class="relative">
            <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">R$</span>
            <input
              id="amountPaid"
              :value="formatCurrency(amountPaid)"
              type="text"
              readonly
              class="w-full pl-12 pr-4 py-2 bg-gray-600 border border-gray-600 rounded-lg text-white cursor-not-allowed"
            />
          </div>
          <p class="mt-1 text-xs text-gray-400">Calculado automaticamente com base no valor da venda e no percentual</p>
        </div>

        <!-- Submit Button -->
        <div class="flex justify-end pt-4">
          <button
            type="submit"
            :disabled="isSubmitting"
            class="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isSubmitting ? 'Salvando...' : 'Salvar Montagem' }}
          </button>
        </div>
      </form>
    </div>

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import Toast from '@/components/Toast.vue'
import { Assembler, Assembly } from '../types'
import { assemblersService, assembliesService } from '../services'

const assemblers = ref<Assembler[]>([])

const form = reactive({
  orderNumber: '',
  assemblerId: '',
  orderValue: 0,
  percentagePaid: 0,
  furnitureDescription: '',
  date: new Date().toISOString().split('T')[0]
})

const errors = reactive({
  orderNumber: '',
  assemblerId: '',
  orderValue: '',
  percentagePaid: '',
  furnitureDescription: '',
  date: ''
})

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

const isSubmitting = ref(false)

const amountPaid = computed(() => {
  return (form.orderValue * form.percentagePaid) / 100
})

const calculateAmountPaid = () => {}

const formatCurrency = (value: number): string => {
  return value.toFixed(2)
}

const validateForm = (): boolean => {
  let isValid = true
  
  errors.orderNumber = ''
  errors.assemblerId = ''
  errors.orderValue = ''
  errors.percentagePaid = ''
  errors.furnitureDescription = ''
  errors.date = ''
  
  if (!form.orderNumber.trim()) {
    errors.orderNumber = 'Número da venda é obrigatório'
    isValid = false
  } else if (assembliesService.existsByOrderNumber(form.orderNumber)) {
    errors.orderNumber = 'Já existe uma montagem com este número de venda'
    isValid = false
  }
  
  if (!form.date) {
    errors.date = 'Data é obrigatória'
    isValid = false
  }
  
  if (!form.assemblerId) {
    errors.assemblerId = 'Montador é obrigatório'
    isValid = false
  }
  
  if (!form.furnitureDescription.trim()) {
    errors.furnitureDescription = 'Descrição do móvel é obrigatória'
    isValid = false
  }
  
  if (form.orderValue <= 0) {
    errors.orderValue = 'O valor da venda deve ser maior que 0'
    isValid = false
  }
  
  if (form.percentagePaid < 0 || form.percentagePaid > 100) {
    errors.percentagePaid = 'O percentual deve estar entre 0 e 100'
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
    const selectedAssembler = assemblers.value.find(a => a.id === form.assemblerId)
    
    if (!selectedAssembler) {
      toast.message = 'Montador selecionado não encontrado'
      toast.type = 'error'
      return
    }

    const newAssembly: Assembly = {
      id: '1',
      amountPaid: amountPaid.value,
      assemblerId: form.assemblerId,
      assemblerName: selectedAssembler.name,
      createdAt: form.date,
      date: form.date,
      furnitureDescription: form.furnitureDescription.trim(),
      orderNumber: form.orderNumber.trim(),
      orderValue: form.orderValue,
      percentagePaid: form.percentagePaid,
    }

    assembliesService.create(newAssembly)
    
    toast.message = 'Montagem cadastrada com sucesso!'
    toast.type = 'success'
    
    form.orderNumber = ''
    form.assemblerId = ''
    form.orderValue = 0
    form.percentagePaid = 0
    form.furnitureDescription = ''
    form.date = new Date().toISOString().split('T')[0]
    
  } catch (error) {
    toast.message = 'Erro ao cadastrar montagem'
    toast.type = 'error'
  } finally {
    isSubmitting.value = false
  }
}

const loadAssemblers = () => {
  assemblers.value = assemblersService.getAll()
}

onMounted(() => {
  loadAssemblers()
})
</script>
