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
            <h3 class="text-xl font-bold text-white">Edit Assembly</h3>
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
          <form @submit.prevent="handleSubmit" class="p-6 space-y-4 max-h-[calc(90vh-8rem)] overflow-y-auto">
            <!-- Order Number -->
            <div>
              <label for="edit-orderNumber" class="block text-sm font-medium text-gray-300 mb-2">
                Order Number <span class="text-red-400">*</span>
              </label>
              <input
                id="edit-orderNumber"
                v-model="form.orderNumber"
                type="text"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                :class="{ 'border-red-500': errors.orderNumber }"
              />
              <p v-if="errors.orderNumber" class="mt-1 text-sm text-red-400">{{ errors.orderNumber }}</p>
            </div>

            <!-- Date -->
            <div>
              <label for="edit-date" class="block text-sm font-medium text-gray-300 mb-2">
                Date <span class="text-red-400">*</span>
              </label>
              <input
                id="edit-date"
                v-model="form.date"
                type="date"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <!-- Assembler -->
            <div>
              <label for="edit-assembler" class="block text-sm font-medium text-gray-300 mb-2">
                Assembler <span class="text-red-400">*</span>
              </label>
              <select
                id="edit-assembler"
                v-model="form.assemblerId"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 appearance-none cursor-pointer"
              >
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
                Furniture Description <span class="text-red-400">*</span>
              </label>
              <textarea
                id="edit-furnitureDescription"
                v-model="form.furnitureDescription"
                rows="3"
                class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
              ></textarea>
            </div>

            <!-- Order Value -->
            <div>
              <label for="edit-orderValue" class="block text-sm font-medium text-gray-300 mb-2">
                Order Value <span class="text-red-400">*</span>
              </label>
              <div class="relative">
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">$</span>
                <input
                  id="edit-orderValue"
                  v-model.number="form.orderValue"
                  type="number"
                  step="0.01"
                  class="w-full pl-8 pr-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  @input="calculateAmountPaid"
                />
              </div>
            </div>

            <!-- Percentage Paid -->
            <div>
              <label for="edit-percentagePaid" class="block text-sm font-medium text-gray-300 mb-2">
                Percentage Paid <span class="text-red-400">*</span>
              </label>
              <div class="relative">
                <input
                  id="edit-percentagePaid"
                  v-model.number="form.percentagePaid"
                  type="number"
                  step="0.01"
                  class="w-full pr-8 pl-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                  @input="calculateAmountPaid"
                />
                <span class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400">%</span>
              </div>
            </div>

            <!-- Amount Paid -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">Amount Paid</label>
              <div class="relative">
                <span class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400">$</span>
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
import { reactive, ref, watch, computed, onMounted } from 'vue'
import { Assembler, Assembly } from '../types';
import { storageService } from '../services/database.service';

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

const isSubmitting = ref(false)

const amountPaid = computed(() => {
  return (form.orderValue * form.percentagePaid) / 100
})

const calculateAmountPaid = () => {
  // Trigger reactivity
}

watch(() => props.assembly, (newAssembly) => {
  if (newAssembly) {
    form.orderNumber = newAssembly.orderNumber
    form.assemblerId = newAssembly.assemblerId
    form.orderValue = newAssembly.orderValue
    form.percentagePaid = newAssembly.percentagePaid
    form.furnitureDescription = newAssembly.furnitureDescription
    form.date = newAssembly.date
  }
})

const validateForm = (): boolean => {
  errors.orderNumber = ''
  
  if (!form.orderNumber.trim()) {
    errors.orderNumber = 'Order number is required'
    return false
  }
  
  if (props.assembly && storageService.assemblyExistsExcept(form.orderNumber, props.assembly.id)) {
    errors.orderNumber = 'An assembly with this order number already exists'
    return false
  }
  
  return true
}

const handleSubmit = () => {
  if (!validateForm() || !props.assembly) return
  
  isSubmitting.value = true
  
  try {
    const selectedAssembler = assemblers.value.find(a => a.id === form.assemblerId)
    
    if (!selectedAssembler) return
    
    storageService.updateAssembly(
      props.assembly.id,
      form.assemblerId,
      selectedAssembler.name,
      form.orderNumber.trim(),
      form.orderValue,
      form.percentagePaid,
      amountPaid.value,
      form.furnitureDescription.trim(),
      form.date
    )
    
    emit('saved')
    closeModal()
  } finally {
    isSubmitting.value = false
  }
}

const closeModal = () => {
  errors.orderNumber = ''
  emit('close')
}

onMounted(() => {
  assemblers.value = storageService.getAssemblers()
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
