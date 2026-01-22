<template>
  <div class="flex h-screen bg-slate-950">
    <!-- Sidebar -->
    <aside class="w-64 bg-slate-900 shadow-lg flex flex-col border-r border-slate-800">
      <div class="p-6 border-b border-slate-800">
  <div class="flex items-center gap-3">
    <img
      src="/logo-app.png"
      alt="Logo Gestão de Montagens"
      class="h-20 w-auto object-contain"
    />

    <div>
      <h1 class="text-2xl font-bold text-white">Gestão de Montagens</h1>
      <p class="text-sm text-slate-300 mt-1">Sistema de Controle</p>
    </div>
  </div>
</div>


      <nav class="flex-1 p-4 space-y-2">
        <!-- Dashboard -->
        <router-link
          to="/"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          Painel Inicial
        </router-link>

        <!-- Register Assembler -->
        <router-link
          to="/register-assembler"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-sky-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          Cadastrar Montador
        </router-link>

        <!-- Register Assembly -->
        <router-link
          to="/register-assembly"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-sky-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Cadastrar Montagem
        </router-link>

        <!-- Assemblers List -->
        <router-link
          to="/listing"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          Lista de Montadores
        </router-link>

        <!-- Assemblies List -->
        <router-link
          to="/assembly-list"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
          </svg>
          Lista de Montagens
        </router-link>

        <!-- Manager Backupt -->
        <router-link
          to="/backups"
          class="flex items-center px-4 py-3 text-slate-200 hover:bg-slate-800 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-slate-800 text-white border border-orange-500/30"
        >
          <svg class="w-5 h-5 mr-3 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01" />
          </svg>
          Backups
        </router-link>
      </nav>

      <div class="p-4 border-t border-slate-800">
        <p class="text-xs text-slate-400 text-center">v0.3.0 | © 2026</p>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Top Navbar -->
      <header class="h-16 flex items-center justify-between px-6 border-b border-slate-800 bg-linear-to-r from-slate-900 via-blue-950 to-slate-900">
        <div class="flex items-center gap-3">
          <h2 class="text-xl font-semibold text-white">Bem Vindo</h2>
            <img
              src="/logo.png"
              alt="Logo da Rede Norte"
              class="h-12 w-auto object-contain"
            />
        </div>

        <div class="flex items-center">
          <span class="text-sm text-slate-200">{{ currentDate }}</span>
        </div>
      </header>

      <!-- Router Outlet -->
      <main class="flex-1 overflow-y-auto bg-slate-950 p-6">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const currentDate = computed(() => {
  return new Date().toLocaleDateString('pt-BR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
  })
})
</script>
