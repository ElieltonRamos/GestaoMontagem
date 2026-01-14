<template>
  <div class="flex h-screen bg-gray-900">
    <!-- Sidebar -->
    <aside class="w-64 bg-gray-800 shadow-lg flex flex-col">
      <div class="p-6 border-b border-gray-700">
        <h1 class="text-2xl font-bold text-white">Assembly Management</h1>
        <p class="text-sm text-gray-400 mt-1">Control System</p>
      </div>

      <nav class="flex-1 p-4 space-y-2">
        <router-link
          to="/"
          class="flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-gray-700 text-white"
        >
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          Dashboard
        </router-link>

        <router-link
          to="/listing"
          class="flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-gray-700 text-white"
        >
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          Listing
        </router-link>

        <router-link
          to="/register-assembler"
          class="flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-gray-700 text-white"
        >
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          Register Assembler
        </router-link>

        <router-link
          to="/register-assembly"
          class="flex items-center px-4 py-3 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition-colors duration-200"
          exact-active-class="bg-gray-700 text-white"
        >
          <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Register Assembly
        </router-link>
      </nav>

      <div class="p-4 border-t border-gray-700">
        <p class="text-xs text-gray-500 text-center">v1.0.0 | © 2026</p>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Top Navbar -->
      <header class="bg-gray-800 shadow-lg h-16 flex items-center justify-between px-6 border-b border-gray-700">
        <div class="flex items-center">
          <h2 class="text-xl font-semibold text-white">Welcome</h2>
        </div>

        <div class="flex items-center">
          <span class="text-sm text-gray-400">{{ currentDate }}</span>
        </div>
      </header>

      <!-- Router Outlet -->
      <main class="flex-1 overflow-y-auto bg-gray-900 p-6">
        <router-view />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const currentDate = computed(() => {
  return new Date().toLocaleDateString('en-US', { 
    day: '2-digit', 
    month: 'long', 
    year: 'numeric' 
  })
})
</script>
