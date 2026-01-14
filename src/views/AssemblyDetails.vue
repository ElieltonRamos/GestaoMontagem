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
      <h2 class="text-2xl font-bold text-white">Assembly Details</h2>
    </div>

    <div v-if="assembly" class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 max-w-2xl">
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Order Number</label>
          <p class="text-lg text-white">{{ assembly.orderNumber }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Assembler</label>
          <p class="text-lg text-white">{{ assembly.assemblerName }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Date</label>
          <p class="text-lg text-white">{{ formatDate(assembly.date) }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-400 mb-1">Furniture Description</label>
          <p class="text-lg text-white whitespace-pre-wrap">{{ assembly.furnitureDescription }}</p>
        </div>

        <div class="grid grid-cols-2 gap-4 pt-4">
          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1">Order Value</label>
            <p class="text-lg text-white">{{ formatCurrency(assembly.orderValue) }}</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-400 mb-1">Percentage Paid</label>
            <p class="text-lg text-white">{{ assembly.percentagePaid }}%</p>
          </div>
        </div>

        <div class="border-t border-gray-700 pt-4">
          <label class="block text-sm font-medium text-gray-400 mb-1">Amount Paid</label>
          <p class="text-2xl font-bold text-green-400">{{ formatCurrency(assembly.amountPaid) }}</p>
        </div>

        <div class="border-t border-gray-700 pt-4">
          <label class="block text-sm font-medium text-gray-400 mb-1">Registered At</label>
          <p class="text-sm text-gray-400">{{ formatDateTime(assembly.createdAt) }}</p>
        </div>
      </div>
    </div>

    <div v-else class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
      <p class="text-gray-400">Assembly not found</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Assembly } from '../types'
import { storageService } from '../services/database.service'

const router = useRouter()
const route = useRoute()
const assembly = ref<Assembly | null>(null)

const formatCurrency = (value: number): string => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(value)
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('en-US', {
    day: '2-digit',
    month: 'long',
    year: 'numeric'
  })
}

const formatDateTime = (dateString: string): string => {
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
  assembly.value = storageService.getAssemblyById(id)
})
</script>
