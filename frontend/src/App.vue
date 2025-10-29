<template>
  <div class="min-h-screen bg-gray-50 dark:bg-slate-900 transition-colors duration-200">
    <Navigation />
    <main class="container mx-auto px-4 py-8">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import Navigation from './components/Navigation.vue'
import { useThemeStore } from './stores/themeStore'

// Initialize theme store
const themeStore = useThemeStore()
</script>
