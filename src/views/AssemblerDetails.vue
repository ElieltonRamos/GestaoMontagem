<template>
  <div>
    <div class="flex items-center gap-4 mb-6">
      <button
        @click="goBack"
        class="text-gray-400 hover:text-white transition-colors"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
      </button>
      <h2 class="text-2xl font-bold text-white">Detalhes do Montador</h2>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700">
      <div class="flex items-center justify-center">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        <span class="ml-3 text-gray-400">Carregando detalhes...</span>
      </div>
    </div>

    <div v-else-if="assembler" class="space-y-6">
      <!-- Assembler info -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1">Nome</label>
            <p class="text-lg text-white">{{ assembler.name }}</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1">Telefone</label>
            <p class="text-lg text-white">{{ formatPhone(assembler.phone) }}</p>
          </div>

          <div v-if="assembler.cpf">
            <label class="block text-sm font-medium text-gray-400 mb-1">CPF</label>
            <p class="text-lg text-white">{{ formatCPF(assembler.cpf) }}</p>
          </div>

          <div v-if="assembler.address">
            <label class="block text-sm font-medium text-gray-400 mb-1">Endereço</label>
            <p class="text-lg text-white">{{ assembler.address }}</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1">Cadastrado em</label>
            <p class="text-lg text-white">{{ formatDateTime(assembler.createdAt) }}</p>
          </div>
        </div>
      </div>

      <!-- Period filters -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
        <h3 class="text-lg font-semibold text-white mb-4">Período</h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="startDate" class="block text-sm font-medium text-gray-300 mb-2">
              Data inicial
            </label>
            <input
              id="startDate"
              v-model="filters.startDate"
              type="date"
              class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>

          <div>
            <label for="endDate" class="block text-sm font-medium text-gray-300 mb-2">
              Data final
            </label>
            <input
              id="endDate"
              v-model="filters.endDate"
              type="date"
              class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>

        <div class="flex justify-end mt-4">
          <button
            @click="resetToCurrentMonth"
            class="px-4 py-2 text-sm text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors"
          >
            Mês atual
          </button>
        </div>
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
          <p class="text-sm text-gray-400">Total de montagens</p>
          <p class="text-2xl font-bold text-white mt-1">{{ assemblerStats.totalAssemblies }}</p>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
          <p class="text-sm text-gray-400">Total pago</p>
          <p class="text-2xl font-bold text-green-400 mt-1">{{ formatMoney(assemblerStats.totalPaid) }}</p>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
          <p class="text-sm text-gray-400">Média por montagem</p>
          <p class="text-2xl font-bold text-white mt-1">{{ formatMoney(assemblerStats.avgPaid) }}</p>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700">
          <p class="text-sm text-gray-400">Maior / menor</p>
          <p class="text-sm text-white mt-2">
            <span class="text-green-400">{{ formatMoney(assemblerStats.maxPaid) }}</span>
            <span class="text-gray-500"> / </span>
            <span class="text-red-400">{{ formatMoney(assemblerStats.minPaid) }}</span>
          </p>
        </div>
      </div>

      <!-- Assemblies table -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
        <h3 class="text-lg font-semibold text-white mb-4">Montagens do período</h3>

        <div v-if="filteredAssemblerAssemblies.length === 0" class="text-gray-400">
          Nenhuma montagem encontrada no período selecionado.
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full">
            <thead class="bg-gray-700">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                  Número da venda
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
                v-for="assembly in filteredAssemblerAssemblies"
                :key="assembly.id"
                class="hover:bg-gray-750 transition-colors"
              >
                <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                  {{ assembly.orderNumber }}
                </td>

                <td class="px-6 py-4 whitespace-nowrap text-sm text-right text-green-400 font-medium">
                  {{ formatMoney(assembly.amountPaid) }}
                </td>

                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <div class="flex justify-end gap-2">
                    <button
                      @click="viewAssemblyDetails(assembly.id)"
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
                      :disabled="isDeleting"
                      class="text-red-400 hover:text-red-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
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

      <!-- Edit + delete -->
      <EditAssemblyModal
        :is-open="isEditModalOpen"
        :assembly="selectedAssembly"
        @close="closeEditModal"
        @saved="handleAssemblySaved"
      />

      <ConfirmDialog
        :is-open="isDeleteConfirmOpen"
        :is-loading="isDeleting"
        title="Excluir montagem"
        message="Tem certeza que deseja excluir esta montagem? Esta ação não pode ser desfeita."
        @confirm="handleDeleteAssembly"
        @cancel="closeDeleteConfirm"
      />

      <Toast :message="toast.message" :type="toast.type" />
    </div>

    <div v-else class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
      <p class="text-gray-400">Montador não encontrado</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import type { Assembler, Assembly } from '../types'
import { assemblersService } from '../services/assemblers.service.tauri'
import { assembliesService } from '../services/assemblies.service.tauri'
import EditAssemblyModal from '../components/EditAssemblyModal.vue'
import ConfirmDialog from '../components/ConfirmDialog.vue'
import Toast from '../components/Toast.vue'

const router = useRouter()
const route = useRoute()

const assembler = ref<Assembler | null>(null)
const allAssemblies = ref<Assembly[]>([])
const isLoading = ref(true)
const isDeleting = ref(false)

const selectedAssembly = ref<Assembly | null>(null)
const isEditModalOpen = ref(false)
const isDeleteConfirmOpen = ref(false)

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error',
})

const filters = reactive({
  startDate: '',
  endDate: '',
})

function parseYyyyMmDdToLocalDate(dateString: string): Date {
  const [y, m, d] = dateString.split('-').map(Number)
  return new Date(y, m - 1, d)
}

function resetToCurrentMonth(): void {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  filters.startDate = firstDay.toISOString().split('T')[0]
  filters.endDate = lastDay.toISOString().split('T')[0]
}

const assemblerAssemblies = computed(() => {
  if (!assembler.value) return []
  return allAssemblies.value.filter(a => a.assemblerId === assembler.value!.id)
})

const filteredAssemblerAssemblies = computed(() => {
  let result = assemblerAssemblies.value

  if (filters.startDate) {
    const start = parseYyyyMmDdToLocalDate(filters.startDate)
    result = result.filter(a => parseYyyyMmDdToLocalDate(a.date) >= start)
  }

  if (filters.endDate) {
    const end = parseYyyyMmDdToLocalDate(filters.endDate)
    end.setHours(23, 59, 59, 999)
    result = result.filter(a => parseYyyyMmDdToLocalDate(a.date) <= end)
  }

  return [...result].sort((a, b) => parseYyyyMmDdToLocalDate(b.date).getTime() - parseYyyyMmDdToLocalDate(a.date).getTime())
})

const assemblerStats = computed(() => {
  const list = filteredAssemblerAssemblies.value
  const totalAssemblies = list.length
  const totalPaid = list.reduce((sum, a) => sum + a.amountPaid, 0)
  const avgPaid = totalAssemblies > 0 ? totalPaid / totalAssemblies : 0

  const amounts = list.map(a => a.amountPaid)
  const maxPaid = amounts.length ? Math.max(...amounts) : 0
  const minPaid = amounts.length ? Math.min(...amounts) : 0

  return { totalAssemblies, totalPaid, avgPaid, maxPaid, minPaid }
})

const formatPhone = (phone: string): string => {
  const digits = phone.replace(/\D/g, '')
  return digits.replace(/^(\d{2})(\d{5})(\d{4})/, '($1) $2-$3')
}

const formatCPF = (cpf: string): string => {
  const digits = cpf.replace(/\D/g, '')
  return digits.replace(/(\d{3})(\d{3})(\d{3})(\d{2})/, '$1.$2.$3-$4')
}

const formatDateTime = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('pt-BR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const formatMoney = (value: number): string => {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL'
  }).format(value)
}

const goBack = () => router.back()

const viewAssemblyDetails = (id: string) => {
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

const handleAssemblySaved = async () => {
  await loadAssemblies()
  toast.message = 'Montagem atualizada com sucesso!'
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

const handleDeleteAssembly = async () => {
  if (!selectedAssembly.value) {
    closeDeleteConfirm()
    return
  }

  isDeleting.value = true

  try {
    const success = await assembliesService.delete(selectedAssembly.value.id)

    if (success) {
      await loadAssemblies()
      toast.message = 'Montagem excluída com sucesso!'
      toast.type = 'success'
    } else {
      toast.message = 'Erro ao excluir montagem'
      toast.type = 'error'
    }
  } catch (error) {
    toast.message = 'Erro ao excluir montagem'
    toast.type = 'error'
  } finally {
    isDeleting.value = false
    closeDeleteConfirm()
  }
}

const loadAssemblies = async () => {
  try {
    allAssemblies.value = await assembliesService.getAll()
  } catch (error) {
    toast.message = 'Erro ao carregar montagens'
    toast.type = 'error'
  }
}

const loadData = async () => {
  isLoading.value = true

  try {
    const id = route.params.id as string
    
    const [assemblerData, assembliesData] = await Promise.all([
      assemblersService.getById(id),
      assembliesService.getAll()
    ])

    assembler.value = assemblerData
    allAssemblies.value = assembliesData

    if (!assembler.value) {
      toast.message = 'Montador não encontrado'
      toast.type = 'error'
    }
  } catch (error) {
    toast.message = 'Erro ao carregar dados'
    toast.type = 'error'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  resetToCurrentMonth()
  loadData()
})
</script>
