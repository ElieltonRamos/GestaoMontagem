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
      <h2 class="text-2xl font-bold text-white">Assembler Details</h2>
    </div>

    <div v-if="assembler" class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Name</label>
          <p class="text-lg text-white">{{ assembler.name }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Phone</label>
          <p class="text-lg text-white">{{ formatPhone(assembler.phone) }}</p>
        </div>

        <div v-if="assembler.cpf">
          <label class="block text-sm font-medium text-gray-400 mb-1">CPF</label>
          <p class="text-lg text-white">{{ formatCPF(assembler.cpf) }}</p>
        </div>

        <div v-if="assembler.address">
          <label class="block text-sm font-medium text-gray-400 mb-1">Address</label>
          <p class="text-lg text-white">{{ assembler.address }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Registered At</label>
          <p class="text-lg text-white">{{ formatDate(assembler.createdAt) }}</p>
        </div>
      </div>
    </div>

    <div v-else class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
      <p class="text-gray-400">Assembler not found</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { storageService } from '../services/database.service'
import { Assembler } from '../types'

const router = useRouter()
const route = useRoute()
const assembler = ref<Assembler | null>(null)

const formatPhone = (phone: string): string => {
  const digits = phone.replace(/\D/g, '')
  return digits.replace(/^(\d{2})(\d{5})(\d{4})/, '($1) $2-$3')
}

const formatCPF = (cpf: string): string => {
  const digits = cpf.replace(/\D/g, '')
  return digits.replace(/(\d{3})(\d{3})(\d{3})(\d{2})/, '$1.$2.$3-$4')
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('en-US', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const goBack = () => {
  router.back()
}

onMounted(() => {
  const id = route.params.id as string
  assembler.value = storageService.getAssemblerById(id)
})
</script>
