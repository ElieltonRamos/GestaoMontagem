<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="isOpen"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50 overflow-y-auto"
        @click.self="closeModal"
      >
        <div class="bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full my-8">
          <!-- Header -->
          <div class="flex items-center justify-between p-6 border-b border-gray-700">
            <h3 class="text-xl font-bold text-white">Editar Montagem</h3>
            <button
              @click="closeModal"
              class="text-gray-400 hover:text-white transition-colors"
              :disabled="isSubmitting"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Form -->
          <form @submit.prevent="handleSubmit" class="p-6 space-y-4 max-h-[calc(90vh-8rem)] overflow-y-auto">
            <!-- Order Number -->
            <div>
              <label for="edit-orderNumber" class="block text-sm font-medium text-gray-300 mb-2">
                Nº do Pedido <span class="text-red-400">*</span>
              </label>
              <div class="relative">
                <input
                  id="edit-orderNumber"
                  v-model="form.orderNumber"
                  type="text"
                  class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{ 'border-red-500': errors.orderNumber }"
                  :disabled="isValidatingOrderNumber || isSubmitting"
                  placeholder="Digite o número do pedido"
                />
                <!-- Loading da validação de orderNumber -->
                <div v-if="isValidatingOrderNumber" class="absolute right-3 top-1/2 -translate-y-1/2">
                  <svg
                    class="animate-spin h-5 w-5 text-blue-500"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                  >
                    <circle
                      class="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      stroke-width="4"
                    ></circle>
                    <path
                      class="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                  </svg>
                </div>
              </div>
              <p v-if="errors.orderNumber" class="mt-1 text-sm text-red-400">
                {{ errors.orderNumber }}
              </p>
            </div>

            <!-- Date -->
            <div>
              <label for="edit-date" class="block text-sm font-medium text-gray-300 mb-2">
                Data <span class="text-red-400">*</span>
              </label>
              <input
                id="edit-date"
                v-model="form.date"
                type="date"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                :disabled="isSubmitting"
              />
            </div>

            <!-- Assembler -->
            <div>
              <label for="edit-assembler" class="block text-sm font-medium text-gray-300 mb-2">
                Montador <span class="text-red-400">*</span>
              </label>
              <select
                id="edit-assembler"
                v-model="form.assemblerId"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 appearance-none cursor-pointer"
                :disabled="isSubmitting"
              >
                <option value="">Selecione um montador</option>
                <option
                  v-for="assembler in assemblers"
                  :key="assembler.id"
                  :value="assembler.id"
                >
                  {{ assembler.name }}
                </option>
              </select>
            </div>

            <!-- Furniture Description -->
            <div>
              <label for="edit-furnitureDescription" class="block text-sm font-medium text-gray-300 mb-2">
                Descrição do Móvel <span class="text-red-400">*</span>
              </label>
              <textarea
                id="edit-furnitureDescription"
                v-model="form.furnitureDescription"
                rows="3"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                :disabled="isSubmitting"
                placeholder="Digite a descrição do móvel"
              ></textarea>
            </div>

            <!-- Order Value -->
            <div>
              <label for="edit-orderValue" class="block text-sm font-medium text-gray-300 mb-2">
                Valor do Pedido <span class="text-red-400">*</span>
              </label>
              <div class="relative">
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">R$</span>
                <input
                  id="edit-orderValue"
                  v-model.number="form.orderValue"
                  type="number"
                  step="0.01"
                  min="0"
                  class="w-full pl-8 pr-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  @input="calculateAmountPaid"
                  :disabled="isSubmitting"
                />
              </div>
            </div>

            <!-- Percentage Paid -->
            <div>
              <label for="edit-percentagePaid" class="block text-sm font-medium text-gray-300 mb-2">
                % Pago <span class="text-red-400">*</span>
              </label>
              <div class="relative">
                <input
                  id="edit-percentagePaid"
                  v-model.number="form.percentagePaid"
                  type="number"
                  step="0.01"
                  min="0"
                  max="100"
                  class="w-full pr-8 pl-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  @input="calculateAmountPaid"
                  :disabled="isSubmitting"
                />
                <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400">%</span>
              </div>
            </div>

            <!-- Amount Paid -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">Valor Pago</label>
              <div class="relative">
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">R$</span>
                <input
                  :value="amountPaid.toFixed(2)"
                  type="text"
                  readonly
                  class="w-full pl-8 pr-4 py-2 bg-gray-600 border border-gray-600 rounded-lg text-white cursor-not-allowed"
                />
              </div>
            </div>

            <!-- Actions -->
            <div class="flex justify-end gap-3 pt-4">
              <button
                type="button"
                @click="closeModal"
                class="px-4 py-2 text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
                :disabled="isSubmitting"
              >
                Cancelar
              </button>
              <button
                type="submit"
                :disabled="isSubmitting || isValidatingOrderNumber"
                class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
              >
                {{ isSubmitting ? 'Salvando...' : 'Salvar Alterações' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { reactive, ref, watch, computed, onMounted } from 'vue'
import { Assembler, Assembly } from '../types'
import { assemblersService } from '../services/assemblers.service.tauri'
import { assembliesService } from '../services/assemblies.service.tauri'

interface Props {
  isOpen: boolean
  assembly: Assembly | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  close: []
  saved: []
}>()

const assemblers = ref<Assembler[]>([])
const isSubmitting = ref(false)
const isValidatingOrderNumber = ref(false)

const form = reactive({
  orderNumber: '',
  assemblerId: '',
  orderValue: 0,
  percentagePaid: 0,
  furnitureDescription: '',
  date: ''
})

const errors = reactive({
  orderNumber: ''
})

const amountPaid = computed(() => {
  return (form.orderValue * form.percentagePaid) / 100
})

const calculateAmountPaid = () => {
  // Trigger reactivity
}

watch(
  () => props.assembly,
  (newAssembly) => {
    if (newAssembly) {
      form.orderNumber = newAssembly.orderNumber
      form.assemblerId = newAssembly.assemblerId
      form.orderValue = newAssembly.orderValue
      form.percentagePaid = newAssembly.percentagePaid
      form.furnitureDescription = newAssembly.furnitureDescription
      form.date = newAssembly.date
    } else {
      resetForm()
    }
    errors.orderNumber = ''
  }
)

const resetForm = () => {
  form.orderNumber = ''
  form.assemblerId = ''
  form.orderValue = 0
  form.percentagePaid = 0
  form.furnitureDescription = ''
  form.date = ''
}

const validateForm = async (): Promise<boolean> => {
  errors.orderNumber = ''

  if (!form.orderNumber.trim()) {
    errors.orderNumber = 'O número do pedido é obrigatório'
    return false
  }

  if (props.assembly) {
    try {
      isValidatingOrderNumber.value = true
      const orderExists = await assembliesService.existsByOrderNumberExceptId(
        form.orderNumber.trim(),
        props.assembly.id
      )
      if (orderExists) {
        errors.orderNumber = 'Já existe uma montagem com este número de pedido'
        return false
      }
    } catch (error) {
      console.error('Erro ao validar número do pedido:', error)
      errors.orderNumber = 'Erro ao validar número do pedido'
      return false
    } finally {
      isValidatingOrderNumber.value = false
    }
  }

  // Validações obrigatórias
  if (!form.assemblerId) {
    errors.orderNumber = 'Selecione um montador'
    return false
  }
  if (form.orderValue <= 0) {
    errors.orderNumber = 'O valor do pedido deve ser maior que zero'
    return false
  }
  if (form.percentagePaid < 0 || form.percentagePaid > 100) {
    errors.orderNumber = 'O % pago deve estar entre 0 e 100'
    return false
  }
  if (!form.furnitureDescription.trim()) {
    errors.orderNumber = 'A descrição do móvel é obrigatória'
    return false
  }
  if (!form.date) {
    errors.orderNumber = 'A data é obrigatória'
    return false
  }

  return true
}

const handleSubmit = async () => {
  if (!props.assembly) return

  const isValid = await validateForm()
  if (!isValid) return

  isSubmitting.value = true

  try {
    const selectedAssembler = assemblers.value.find((a) => a.id === form.assemblerId)
    if (!selectedAssembler) {
      errors.orderNumber = 'Montador não encontrado'
      return
    }

    const updatedAssembly: Assembly = {
      id: props.assembly.id,
      assemblerId: form.assemblerId,
      assemblerName: selectedAssembler.name,
      orderNumber: form.orderNumber.trim(),
      orderValue: form.orderValue,
      percentagePaid: form.percentagePaid,
      amountPaid: amountPaid.value,
      furnitureDescription: form.furnitureDescription.trim(),
      date: form.date,
      createdAt: props.assembly.createdAt // Mantém a data original de criação
    }

    await assembliesService.update(props.assembly.id, updatedAssembly)

    emit('saved')
    closeModal()
  } catch (error) {
    console.error('Erro ao salvar montagem:', error)
    errors.orderNumber = 'Erro ao salvar montagem'
  } finally {
    isSubmitting.value = false
  }
}

const closeModal = () => {
  errors.orderNumber = ''
  isValidatingOrderNumber.value = false
  emit('close')
}

onMounted(async () => {
  try {
    assemblers.value = await assemblersService.getAll()
  } catch (error) {
    console.error('Erro ao carregar montadores:', error)
  }
})
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
