<style>
@import './style.css';
</style>

<script setup lang="ts">
import { onMounted, reactive } from 'vue'
import { backupService } from './services/backup.service.tauri'
import Toast from './components/Toast.vue'

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

// Executar backup automático ao abrir o app (assíncrono, não bloqueia)
onMounted(async () => {
  // Aguardar 2 segundos após o app abrir para não atrasar a inicialização
  setTimeout(async () => {
    try {
      const result = await backupService.createAutoBackup()
      
      if (result.created) {
        toast.message = 'Backup criado automaticamente'
        toast.type = 'success'
      }
    } catch (error) {
      console.error('Erro ao criar backup automático:', error)
    }
  }, 2000)
})
</script>

<template>
  <div id="app">
    <!-- Seu conteúdo do app -->
    <router-view />
    
    <!-- Toast global para backup automático -->
    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>
