<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Assemblers List</h2>

    <!-- Empty State -->
    <div
      v-if="assemblers.length === 0"
      class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700 text-center"
    >
      <svg class="w-16 h-16 text-gray-600 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
      </svg>
      <h3 class="text-xl font-semibold text-gray-300 mb-2">No assemblers registered</h3>
      <p class="text-gray-500">Start by adding your first assembler</p>
    </div>

    <!-- Table -->
    <div v-else class="bg-gray-800 rounded-lg shadow-lg border border-gray-700 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-700">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Name
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                Phone
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-700">
            <tr
              v-for="assembler in assemblers"
              :key="assembler.id"
              class="hover:bg-gray-750 transition-colors"
            >
              <td class="px-6 py-4 whitespace-nowrap text-sm text-white">
                {{ assembler.name }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-300">
                {{ formatPhone(assembler.phone) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <div class="flex justify-end gap-2">
                  <button
                    @click="viewDetails(assembler.id)"
                    class="text-blue-400 hover:text-blue-300 transition-colors"
                    title="View Details"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                    </svg>
                  </button>
                  <button
                    @click="openEditModal(assembler)"
                    class="text-yellow-400 hover:text-yellow-300 transition-colors"
                    title="Edit"
                  >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    @click="openDeleteConfirm(assembler)"
                    class="text-red-400 hover:text-red-300 transition-colors"
                    title="Delete"
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
    <EditAssemblerModal
      :is-open="isEditModalOpen"
      :assembler="selectedAssembler"
      @close="closeEditModal"
      @saved="handleSaved"
    />

    <!-- Delete Confirmation -->
    <ConfirmDialog
      :is-open="isDeleteConfirmOpen"
      title="Delete Assembler"
      message="Are you sure you want to delete this assembler? This action cannot be undone."
      @confirm="handleDelete"
      @cancel="closeDeleteConfirm"
    />

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { storageService } from '../services/database.service'
import EditAssemblerModal from '@/components/EditAssemblerModal.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import Toast from '@/components/Toast.vue'
import { Assembler } from '../types'

const router = useRouter()
const assemblers = ref<Assembler[]>([])
const selectedAssembler = ref<Assembler | null>(null)
const isEditModalOpen = ref(false)
const isDeleteConfirmOpen = ref(false)

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

const loadAssemblers = () => {
  assemblers.value = storageService.getAssemblers()
}

const formatPhone = (phone: string): string => {
  const digits = phone.replace(/\D/g, '')
  return digits.replace(/^(\d{2})(\d{5})(\d{4})/, '($1) $2-$3')
}

const viewDetails = (id: string) => {
  router.push(`/assembler/${id}`)
}

const openEditModal = (assembler: Assembler) => {
  selectedAssembler.value = assembler
  isEditModalOpen.value = true
}

const closeEditModal = () => {
  isEditModalOpen.value = false
  selectedAssembler.value = null
}

const handleSaved = () => {
  loadAssemblers()
  toast.message = 'Assembler updated successfully!'
  toast.type = 'success'
}

const openDeleteConfirm = (assembler: Assembler) => {
  selectedAssembler.value = assembler
  isDeleteConfirmOpen.value = true
}

const closeDeleteConfirm = () => {
  isDeleteConfirmOpen.value = false
  selectedAssembler.value = null
}

const handleDelete = () => {
  if (selectedAssembler.value) {
    storageService.deleteAssembler(selectedAssembler.value.id)
    loadAssemblers()
    toast.message = 'Assembler deleted successfully!'
    toast.type = 'success'
  }
  closeDeleteConfirm()
}

onMounted(() => {
  loadAssemblers()
})
</script>
