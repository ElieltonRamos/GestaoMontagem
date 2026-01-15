<template>
  <div class="space-y-6">
    <!-- Loading State -->
    <div v-if="isLoading" class="bg-gray-800 rounded-lg shadow-lg p-12 border border-gray-700">
      <div class="flex items-center justify-center">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
        <span class="ml-3 text-gray-400">Carregando dados...</span>
      </div>
    </div>

    <template v-else>
      <!-- Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-blue-500 transition-colors">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-400 uppercase tracking-wide">Total de montagens</p>
              <p class="text-3xl font-bold text-white mt-2">{{ periodStats.totalAssemblies }}</p>
            </div>
            <div class="p-3 bg-blue-500 bg-opacity-20 rounded-lg">
              <svg class="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-green-500 transition-colors">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-400 uppercase tracking-wide">Total pago</p>
              <p class="text-3xl font-bold text-white mt-2">{{ formatBRL(periodStats.totalPaid) }}</p>
            </div>
            <div class="p-3 bg-green-500 bg-opacity-20 rounded-lg">
              <svg class="w-8 h-8 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700 hover:border-purple-500 transition-colors">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-400 uppercase tracking-wide">Montadores cadastrados</p>
              <p class="text-3xl font-bold text-white mt-2">{{ periodStats.totalAssemblers }}</p>
            </div>
            <div class="p-3 bg-purple-500 bg-opacity-20 rounded-lg">
              <svg class="w-8 h-8 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- Filtro compacto (1 linha) -->
      <div class="bg-gray-800 rounded-lg shadow-lg px-4 py-3 border border-gray-700">
        <div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3 w-full">
            <div>
              <label class="block text-xs font-medium text-gray-300 mb-1">Data inicial</label>
              <input
                v-model="filters.startDate"
                type="date"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label class="block text-xs font-medium text-gray-300 mb-1">Data final</label>
              <input
                v-model="filters.endDate"
                type="date"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>

          <button
            @click="resetToCurrentMonth"
            class="px-4 py-2 text-sm text-gray-300 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors md:ml-3 whitespace-nowrap"
          >
            Mês atual
          </button>
        </div>
      </div>

      <!-- Gráficos -->
      <div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
          <h3 class="text-lg font-semibold text-white mb-4">Total pago por montador (Top 10)</h3>
          <canvas ref="paidByAssemblerCanvas"></canvas>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
          <h3 class="text-lg font-semibold text-white mb-4">Quantidade de montagens por montador (Top 10)</h3>
          <canvas ref="countByAssemblerCanvas"></canvas>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
          <h3 class="text-lg font-semibold text-white mb-4">Total pago por dia</h3>
          <canvas ref="paidByDayCanvas"></canvas>
        </div>

        <div class="bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-700">
          <h3 class="text-lg font-semibold text-white mb-4">Quantidade de montagens por dia</h3>
          <canvas ref="countByDayCanvas"></canvas>
        </div>
      </div>
    </template>

    <Toast :message="toast.message" :type="toast.type" />
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, reactive, ref, watch } from 'vue'
import Chart from 'chart.js/auto'
import type { Assembly, Assembler } from '../types'
import { assemblersService } from '../services/assemblers.service.tauri'
import Toast from '@/components/Toast.vue'
import { assembliesService } from '../services/assemblies.service.tauri'

const TOP_N = 10

const filters = reactive({
  startDate: '',
  endDate: '',
})

const allAssemblies = ref<Assembly[]>([])
const allAssemblers = ref<Assembler[]>([])
const isLoading = ref(true)

const toast = reactive({
  message: '',
  type: 'success' as 'success' | 'error'
})

function formatBRL(value: number): string {
  return new Intl.NumberFormat('pt-BR', {
    style: 'currency',
    currency: 'BRL',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value)
}

function parseYyyyMmDdToLocalDate(dateString: string): Date {
  const [y, m, d] = dateString.split('-').map(Number)
  return new Date(y, m - 1, d)
}

function formatDateLabelPtBr(dateString: string): string {
  const d = parseYyyyMmDdToLocalDate(dateString)
  return d.toLocaleDateString('pt-BR', { day: '2-digit', month: '2-digit' })
}

function toYyyyMmDd(date: Date): string {
  return date.toISOString().split('T')[0]
}

function resetToCurrentMonth(): void {
  const now = new Date()
  const firstDay = new Date(now.getFullYear(), now.getMonth(), 1)
  const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  filters.startDate = toYyyyMmDd(firstDay)
  filters.endDate = toYyyyMmDd(lastDay)
}

const filteredAssemblies = computed(() => {
  let result = allAssemblies.value

  if (filters.startDate) {
    const start = parseYyyyMmDdToLocalDate(filters.startDate)
    result = result.filter(a => parseYyyyMmDdToLocalDate(a.date) >= start)
  }

  if (filters.endDate) {
    const end = parseYyyyMmDdToLocalDate(filters.endDate)
    end.setHours(23, 59, 59, 999)
    result = result.filter(a => parseYyyyMmDdToLocalDate(a.date) <= end)
  }

  return result
})

const periodStats = computed(() => {
  const totalAssemblies = filteredAssemblies.value.length
  const totalPaid = filteredAssemblies.value.reduce((sum, a) => sum + a.amountPaid, 0)
  const totalAssemblers = allAssemblers.value.length

  return { totalAssemblies, totalPaid, totalAssemblers }
})

const paidByAssemblerSeriesTop = computed(() => {
  const totalPaidMap = new Map<string, number>()
  for (const assembler of allAssemblers.value) totalPaidMap.set(assembler.id, 0)

  for (const assembly of filteredAssemblies.value) {
    totalPaidMap.set(
      assembly.assemblerId,
      (totalPaidMap.get(assembly.assemblerId) || 0) + assembly.amountPaid
    )
  }

  const rows = allAssemblers.value.map(a => ({
    assemblerId: a.id,
    assemblerName: a.name,
    totalPaid: totalPaidMap.get(a.id) || 0,
  }))

  return rows
    .sort((a, b) => b.totalPaid - a.totalPaid)
    .slice(0, TOP_N)
})

const countByAssemblerSeriesTop = computed(() => {
  const countMap = new Map<string, number>()
  for (const assembler of allAssemblers.value) countMap.set(assembler.id, 0)

  for (const assembly of filteredAssemblies.value) {
    countMap.set(assembly.assemblerId, (countMap.get(assembly.assemblerId) || 0) + 1)
  }

  const rows = allAssemblers.value.map(a => ({
    assemblerId: a.id,
    assemblerName: a.name,
    totalCount: countMap.get(a.id) || 0,
  }))

  return rows
    .sort((a, b) => b.totalCount - a.totalCount)
    .slice(0, TOP_N)
})

function listDaysInclusive(startYmd: string, endYmd: string): string[] {
  const start = parseYyyyMmDdToLocalDate(startYmd)
  const end = parseYyyyMmDdToLocalDate(endYmd)
  end.setHours(23, 59, 59, 999)

  const days: string[] = []
  const cursor = new Date(start.getFullYear(), start.getMonth(), start.getDate())

  while (cursor.getTime() <= end.getTime()) {
    days.push(toYyyyMmDd(cursor))
    cursor.setDate(cursor.getDate() + 1)
  }

  return days
}

const dayAxis = computed(() => {
  if (!filters.startDate || !filters.endDate) return []
  return listDaysInclusive(filters.startDate, filters.endDate)
})

const paidByDaySeries = computed(() => {
  const map = new Map<string, number>()
  for (const day of dayAxis.value) map.set(day, 0)

  for (const assembly of filteredAssemblies.value) {
    map.set(assembly.date, (map.get(assembly.date) || 0) + assembly.amountPaid)
  }

  return dayAxis.value.map(day => ({
    date: day,
    totalPaid: map.get(day) || 0,
  }))
})

const countByDaySeries = computed(() => {
  const map = new Map<string, number>()
  for (const day of dayAxis.value) map.set(day, 0)

  for (const assembly of filteredAssemblies.value) {
    map.set(assembly.date, (map.get(assembly.date) || 0) + 1)
  }

  return dayAxis.value.map(day => ({
    date: day,
    totalCount: map.get(day) || 0,
  }))
})

const paidByAssemblerCanvas = ref<HTMLCanvasElement | null>(null)
const countByAssemblerCanvas = ref<HTMLCanvasElement | null>(null)
const paidByDayCanvas = ref<HTMLCanvasElement | null>(null)
const countByDayCanvas = ref<HTMLCanvasElement | null>(null)

let paidChart: Chart | null = null
let countChart: Chart | null = null
let paidByDayChart: Chart | null = null
let countByDayChart: Chart | null = null

async function renderCharts(): Promise<void> {
  await nextTick()

  if (paidChart) paidChart.destroy()
  if (countChart) countChart.destroy()
  if (paidByDayChart) paidByDayChart.destroy()
  if (countByDayChart) countByDayChart.destroy()

  const colors = {
    blueBorder: 'rgb(59, 130, 246)',
    blueBg: 'rgba(59, 130, 246, 0.35)',

    purpleBorder: 'rgb(168, 85, 247)',
    purpleBg: 'rgba(168, 85, 247, 0.35)',

    greenBorder: 'rgb(34, 197, 94)',
    greenBg: 'rgba(34, 197, 94, 0.20)',

    orangeBorder: 'rgb(249, 115, 22)',
    orangeBg: 'rgba(249, 115, 22, 0.35)',
  }

  if (paidByAssemblerCanvas.value) {
    paidChart = new Chart(paidByAssemblerCanvas.value, {
      type: 'bar',
      data: {
        labels: paidByAssemblerSeriesTop.value.map(x => x.assemblerName),
        datasets: [
          {
            label: 'Total pago (R$)',
            data: paidByAssemblerSeriesTop.value.map(x => x.totalPaid),
            backgroundColor: colors.blueBg,
            borderColor: colors.blueBorder,
            borderWidth: 1,
          },
        ],
      },
      options: { responsive: true, maintainAspectRatio: true },
    })
  }

  if (countByAssemblerCanvas.value) {
    countChart = new Chart(countByAssemblerCanvas.value, {
      type: 'bar',
      data: {
        labels: countByAssemblerSeriesTop.value.map(x => x.assemblerName),
        datasets: [
          {
            label: 'Quantidade',
            data: countByAssemblerSeriesTop.value.map(x => x.totalCount),
            backgroundColor: colors.purpleBg,
            borderColor: colors.purpleBorder,
            borderWidth: 1,
          },
        ],
      },
      options: { responsive: true, maintainAspectRatio: true },
    })
  }

  if (paidByDayCanvas.value) {
    paidByDayChart = new Chart(paidByDayCanvas.value, {
      type: 'line',
      data: {
        labels: paidByDaySeries.value.map(x => formatDateLabelPtBr(x.date)),
        datasets: [
          {
            label: 'Total pago (R$)',
            data: paidByDaySeries.value.map(x => x.totalPaid),
            borderColor: colors.greenBorder,
            backgroundColor: colors.greenBg,
            fill: true,
            tension: 0.3,
            pointBackgroundColor: colors.greenBorder,
            pointBorderColor: colors.greenBorder,
          },
        ],
      },
      options: { responsive: true, maintainAspectRatio: true },
    })
  }

  if (countByDayCanvas.value) {
    countByDayChart = new Chart(countByDayCanvas.value, {
      type: 'bar',
      data: {
        labels: countByDaySeries.value.map(x => formatDateLabelPtBr(x.date)),
        datasets: [
          {
            label: 'Quantidade',
            data: countByDaySeries.value.map(x => x.totalCount),
            backgroundColor: colors.orangeBg,
            borderColor: colors.orangeBorder,
            borderWidth: 1,
          },
        ],
      },
      options: { responsive: true, maintainAspectRatio: true },
    })
  }
}

const loadData = async () => {
  isLoading.value = true
  
  try {
    const [assemblers, assemblies] = await Promise.all([
      assemblersService.getAll(),
      assembliesService.getAll()
    ])
    
    allAssemblers.value = assemblers
    allAssemblies.value = assemblies
  } catch (error) {
    toast.message = 'Erro ao carregar dados do dashboard'
    toast.type = 'error'
  } finally {
    isLoading.value = false
    // Aguardar o DOM atualizar (canvas aparecer) antes de renderizar
    await nextTick()
    await renderCharts()
  }
}

onMounted(() => {
  resetToCurrentMonth()
  loadData()
})

watch(
  [paidByAssemblerSeriesTop, countByAssemblerSeriesTop, paidByDaySeries, countByDaySeries],
  () => renderCharts()
)

onUnmounted(() => {
  if (paidChart) paidChart.destroy()
  if (countChart) countChart.destroy()
  if (paidByDayChart) paidByDayChart.destroy()
  if (countByDayChart) countByDayChart.destroy()
})
</script>
