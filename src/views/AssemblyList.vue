<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Lista de Montagens</h2>

    <!-- Summary Cards -->
    <div v-if="assemblies.length > 0" class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
      <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
        <p class="text-sm text-gray-400 uppercase tracking-wide">Total de montagens (período)</p>
        <p class="text-2xl font-bold text-white mt-1">{{ filteredAssemblies.length }}</p>
      </div>
      <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
        <p class="text-sm text-gray-400 uppercase tracking-wide">Total pago (período)</p>
        <p class="text-2xl font-bold text-white mt-1">{{ formatCurrency(totalAmountPaid) }}</p>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700 mb-6">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <!-- Order Number Filter -->
        <div>
          <label for="filter-order" class="block text-sm font-medium text-gray-300 mb-2">
            Número da venda
          </label>
          <input
            id="filter-order"
            v-model="filters.orderNumber"
            type="text"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="Buscar..."
          />
        </div>

        <!-- Assembler Filter -->
        <div>
          <label for="filter-assembler" class="block text-sm font-medium text-gray-300 mb-2">
            Montador
          </label>
          <select
            id="filter-assembler"
            v-model="filters.assemblerId"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 appearance-none cursor-pointer"
          >
            <option value="">Todos os montadores</option>
            <option
              v-for="assembler in assemblers"
              :key="assembler.id"
              :value="assembler.id"
            >
              {{ assembler.name }}
            </option>
          </select>
        </div>

        <!-- Start Date Filter -->
        <div>
          <label for="filter-start-date" class="block text-sm font-medium text-gray-300 mb-2">
            Data inicial
          </label>
          <input
            id="filter-start-date"
            v-model="filters.startDate"
            type="date"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <!-- End Date Filter -->
        <div>
          <label for="filter-end-date" class="block text-sm font-medium text-gray-300 mb-2">
            Data final
          </label>
          <input
            id="filter-end-date"
            v-model="filters.endDate"
            type="date"
            class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      <div class="flex justify-end mt-4">
        <button
          @click="clearFilters"
          class="px-4 py-2 text-sm text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
        >
          Limpar filtros
        </button>
      </div>
    </div>

    <!-- Empty State -->
    <div
      v-if="assemblies.length === 0"
      class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700 text-center"
    >
      <svg class="w-16 h-16 text-gray-600 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
      </svg>
      <h3 class="text-xl font-semibold text-gray-300 mb-2">Nenhuma montagem cadastrada</h3>
      <p class="text-gray-500">Comece cadastrando sua primeira montagem</p>
    </div>

    <!-- No Results -->
    <div
      v-else-if="filteredAssemblies.length === 0"
      class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700 text-center"
    >
      <svg class="w-16 h-16 text-gray-600 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <h3 class="text-xl font-semibold text-gray-300 mb-2">Nenhum resultado encontrado</h3>
      <p class="text-gray-500">Tente ajustar os filtros</p>
    </div>

    <!-- Table -->
    <div v-else class="bg-gray-800 rounded-lg shadow-lg border border-gray-700 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-700">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Número da venda
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Montador
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Data
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                Valor pago
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                Ações
              </th>
            </tr>
          </thead>

          <tbody class="divide-y divide-gray-700">
            <tr
              v-for="assembly in filteredAssemblies"
              :key="assembly.id"
              class="hover:bg-gray-750 transition-colors"
            >
              <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                {{ assembly.orderNumber }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                {{ assembly.assemblerName }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                {{ formatDate(assembly.date) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-right text-green-400 font-medium">
                {{ formatCurrency(assembly.amountPaid) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <div class="flex justify-end gap-2">
                  <button
                    @click="viewDetails(assembly.id)"
                    class="text-blue-400 hover:text-blue-300 transition-colors"
                    title="Ver detalhes"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                    </svg>
                  </button>
                  <button
                    @click="openEditModal(assembly)"
                    class="text-yellow-400 hover:text-yellow-300 transition-colors"
                    title="Editar"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    @click="openDeleteConfirm(assembly)"
                    class="text-red-400 hover:text-red-300 transition-colors"
                    title="Excluir"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Edit Modal -->
    <EditAssemblyModal
      :is-open="isEditModalOpen"
      :assembly="selectedAssembly"
      @close="closeEditModal"
      @saved="handleSaved"
    />

    <!-- Delete Confirmation -->
    <ConfirmDialog
      :is-open="isDeleteConfirmOpen"
      title="Excluir montagem"
      message="Tem certeza que deseja excluir esta montagem? Esta ação não pode ser desfeita."
      @confirm="handleDelete"
      @cancel="closeDeleteConfirm"
    />

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import EditAssemblyModal from '@/components/EditAssemblyModal.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import Toast from '@/components/Toast.vue'
import { Assembler, Assembly } from '../types'
import { assemblersService, assembliesService } from '../services'

const router = useRouter()
const assemblies = ref<Assembly[]>([])
const assemblers = ref<Assembler[]>([])
const selectedAssembly = ref<Assembly | null>(null)
const isEditModalOpen = ref(false)
const isDeleteConfirmOpen = ref(false)

const filters = reactive({
  orderNumber: '',
  assemblerId: '',
  startDate: '',
  endDate: ''
})

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

// Set default filter to current month
const setDefaultDateFilter = () => {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)
  
  filters.startDate = firstDay.toISOString().split('T')[0]
  filters.endDate = lastDay.toISOString().split('T')[0]
}

const filteredAssemblies = computed(() => {
  let result = assemblies.value

  // Filter by order number
  if (filters.orderNumber) {
    result = result.filter(a => 
      a.orderNumber.toLowerCase().includes(filters.orderNumber.toLowerCase())
    )
  }

  // Filter by assembler
  if (filters.assemblerId) {
    result = result.filter(a => a.assemblerId === filters.assemblerId)
  }

  // Filter by date range
  if (filters.startDate) {
    const startDate = new Date(filters.startDate)
    result = result.filter(a => new Date(a.date) >= startDate)
  }

  if (filters.endDate) {
    const endDate = new Date(filters.endDate)
    endDate.setHours(23, 59, 59, 999) // End of day
    result = result.filter(a => new Date(a.date) <= endDate)
  }

  // Sort by date (most recent first)
  return result.sort((a, b) => 
    new Date(b.date).getTime() - new Date(a.date).getTime()
  )
})

const totalAmountPaid = computed(() => {
  return filteredAssemblies.value.reduce((sum, assembly) => sum + assembly.amountPaid, 0)
})

const loadData = () => {
  assemblies.value = assembliesService.getAll()
  assemblers.value = assemblersService.getAll()
}

const formatCurrency = (value: number): string => {
  return new Intl.NumberFormat('pt-br', {
    style: 'currency',
    currency: 'BRL'
  }).format(value)
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('pt-br', {
    day: '2-digit',
    month: 'short',
    year: 'numeric'
  })
}

const clearFilters = () => {
  filters.orderNumber = ''
  filters.assemblerId = ''
  setDefaultDateFilter()
}

const viewDetails = (id: string) => {
  router.push(`/assembly/${id}`)
}

const openEditModal = (assembly: Assembly) => {
  selectedAssembly.value = assembly
  isEditModalOpen.value = true
}

const closeEditModal = () => {
  isEditModalOpen.value = false
  selectedAssembly.value = null
}

const handleSaved = () => {
  loadData()
  toast.message = 'Assembly updated successfully!'
  toast.type = 'success'
}

const openDeleteConfirm = (assembly: Assembly) => {
  selectedAssembly.value = assembly
  isDeleteConfirmOpen.value = true
}

const closeDeleteConfirm = () => {
  isDeleteConfirmOpen.value = false
  selectedAssembly.value = null
}

const handleDelete = () => {
  if (selectedAssembly.value) {
    assembliesService.delete(selectedAssembly.value.id)
    loadData()
    toast.message = 'Assembly deleted successfully!'
    toast.type = 'success'
  }
  closeDeleteConfirm()
}

onMounted(() => {
  setDefaultDateFilter()
  loadData()
})
</script>
