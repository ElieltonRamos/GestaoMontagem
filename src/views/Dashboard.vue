<template>
  <div>
    <!-- Dashboard Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <!-- Card 1: Total Montagens -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-blue-500 transition-colors">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-400 uppercase tracking-wide">Total Montagens</p>
            <p class="text-3xl font-bold text-white mt-2">{{ stats.totalMontagens }}</p>
            <p class="text-xs text-green-400 mt-1">+12% este mês</p>
          </div>
          <div class="p-3 bg-blue-500 bg-opacity-20 rounded-lg">
            <svg class="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Card 2: Valor Total -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-green-500 transition-colors">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-400 uppercase tracking-wide">Valor Total</p>
            <p class="text-3xl font-bold text-white mt-2">{{ formatCurrency(stats.valorTotal) }}</p>
            <p class="text-xs text-green-400 mt-1">+8% este mês</p>
          </div>
          <div class="p-3 bg-green-500 bg-opacity-20 rounded-lg">
            <svg class="w-8 h-8 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Card 3: Montadores Ativos -->
      <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-purple-500 transition-colors">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-400 uppercase tracking-wide">Montadores Ativos</p>
            <p class="text-3xl font-bold text-white mt-2">{{ stats.montadoresAtivos }}</p>
            <p class="text-xs text-gray-400 mt-1">3 novos este mês</p>
          </div>
          <div class="p-3 bg-purple-500 bg-opacity-20 rounded-lg">
            <svg class="w-8 h-8 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
            </svg>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
      <h3 class="text-lg font-semibold text-white mb-4">Atividades Recentes</h3>
      <div class="space-y-3">
        <div class="flex items-center justify-between p-3 bg-gray-700 rounded-lg hover:bg-gray-650 transition-colors">
          <div class="flex items-center space-x-3">
            <div class="w-2 h-2 bg-green-400 rounded-full"></div>
            <p class="text-sm text-gray-300">Nova montagem registrada - Pedido #14388</p>
          </div>
          <span class="text-xs text-gray-500">Há 2 horas</span>
        </div>
        <div class="flex items-center justify-between p-3 bg-gray-700 rounded-lg hover:bg-gray-650 transition-colors">
          <div class="flex items-center space-x-3">
            <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
            <p class="text-sm text-gray-300">Montador cadastrado - IVANILDO</p>
          </div>
          <span class="text-xs text-gray-500">Há 5 horas</span>
        </div>
        <div class="flex items-center justify-between p-3 bg-gray-700 rounded-lg hover:bg-gray-650 transition-colors">
          <div class="flex items-center space-x-3">
            <div class="w-2 h-2 bg-yellow-400 rounded-full"></div>
            <p class="text-sm text-gray-300">Montagem concluída - Pedido #14322</p>
          </div>
          <span class="text-xs text-gray-500">Ontem</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const stats = ref({
  totalMontagens: 143,
  valorTotal: 285670.50,
  montadoresAtivos: 12
})

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL'
  }).format(value)
}
</script>
