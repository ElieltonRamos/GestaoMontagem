<template>
  <div>
    <h2 class="text-2xl font-bold text-white mb-6">Backup do Sistema</h2>

    <!-- Actions -->
    <div class="flex gap-4 mb-6">
      <button
        @click="handleCreateBackup"
        :disabled="isCreating"
        class="px-6 py-2 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50"
      >
        {{ isCreating ? 'Criando...' : 'Criar Backup' }}
      </button>

      <button
        @click="handleExportBackup"
        :disabled="isExporting"
        class="px-6 py-2 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50"
      >
        {{ isExporting ? 'Exportando...' : 'Exportar para...' }}
      </button>

      <button
        @click="loadBackups"
        :disabled="isLoading"
        class="px-6 py-2 bg-gray-600 text-white font-medium rounded-lg hover:bg-gray-700 transition-colors disabled:opacity-50"
      >
        Atualizar Lista
      </button>
    </div>

    <!-- Loading -->
    <div v-if="isLoading" class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700">
      <div class="flex items-center justify-center">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        <span class="ml-3 text-gray-400">Carregando backups...</span>
      </div>
    </div>

    <!-- Backups List -->
    <div v-else-if="backups.length > 0" class="bg-gray-800 rounded-lg shadow-lg border border-gray-700">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-700">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase">Arquivo</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase">Data</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase">Tamanho</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase">Ações</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-700">
            <tr v-for="backup in backups" :key="backup.path" class="hover:bg-gray-750">
              <td class="px-6 py-4 text-sm text-white">{{ backup.filename }}</td>
              <td class="px-6 py-4 text-sm text-gray-300">{{ formatDate(backup.createdAt) }}</td>
              <td class="px-6 py-4 text-sm text-gray-300">{{ formatSize(backup.size) }}</td>
              <td class="px-6 py-4 text-right">
                <button
                  @click="handleRestore(backup)"
                  class="text-green-400 hover:text-green-300 mr-4"
                  title="Restaurar"
                >
                  Restaurar
                </button>
                <button
                  @click="handleDelete(backup)"
                  class="text-red-400 hover:text-red-300"
                  title="Excluir"
                >
                  Excluir
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700 text-center">
      <p class="text-gray-400">Nenhum backup encontrado</p>
    </div>

    <ConfirmDialog
      :is-open="confirmDialog.isOpen"
      :title="confirmDialog.title"
      :message="confirmDialog.message"
      @confirm="confirmDialog.onConfirm"
      @cancel="confirmDialog.isOpen = false"
    />

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { backupService, type BackupInfo } from '../services/backup.service.tauri'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import Toast from '@/components/Toast.vue'

const backups = ref<BackupInfo[]>([])
const isLoading = ref(false)
const isCreating = ref(false)
const isExporting = ref(false)

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

const confirmDialog = reactive({
  isOpen: false,
  title: '',
  message: '',
  onConfirm: () => {}
})

const loadBackups = async () => {
  isLoading.value = true
  try {
    backups.value = await backupService.listBackups()
  } catch (error) {
    toast.message = 'Erro ao carregar backups'
    toast.type = 'error'
  } finally {
    isLoading.value = false
  }
}

const handleCreateBackup = async () => {
  isCreating.value = true
  try {
    const result = await backupService.createBackup()
    if (result) {
      toast.message = 'Backup criado com sucesso!'
      toast.type = 'success'
      await loadBackups()
    } else {
      toast.message = 'Erro ao criar backup'
      toast.type = 'error'
    }
  } finally {
    isCreating.value = false
  }
}

const handleExportBackup = async () => {
  isExporting.value = true
  try {
    const result = await backupService.exportBackup()
    if (result) {
      toast.message = 'Backup exportado com sucesso!'
      toast.type = 'success'
    } else {
      toast.message = 'Exportação cancelada'
      toast.type = 'error'
    }
  } finally {
    isExporting.value = false
  }
}

const handleRestore = (backup: BackupInfo) => {
  confirmDialog.title = 'Restaurar Backup'
  confirmDialog.message = `Deseja restaurar o backup de ${formatDate(backup.createdAt)}? O banco atual será substituído.`
  confirmDialog.onConfirm = async () => {
    const success = await backupService.restoreBackup(backup.path)
    if (success) {
      toast.message = 'Backup restaurado! Reinicie o aplicativo.'
      toast.type = 'success'
    } else {
      toast.message = 'Erro ao restaurar backup'
      toast.type = 'error'
    }
    confirmDialog.isOpen = false
  }
  confirmDialog.isOpen = true
}

const handleDelete = (backup: BackupInfo) => {
  confirmDialog.title = 'Excluir Backup'
  confirmDialog.message = `Deseja excluir o backup ${backup.filename}?`
  confirmDialog.onConfirm = async () => {
    const success = await backupService.deleteBackup(backup.path)
    if (success) {
      toast.message = 'Backup excluído'
      toast.type = 'success'
      await loadBackups()
    } else {
      toast.message = 'Erro ao excluir backup'
      toast.type = 'error'
    }
    confirmDialog.isOpen = false
  }
  confirmDialog.isOpen = true
}

const formatSize = (bytes: number) => backupService.formatSize(bytes)
const formatDate = (timestamp: number) => backupService.formatDate(timestamp)

onMounted(() => {
  loadBackups()
})
</script>
