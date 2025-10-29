<template>
  <nav class="bg-white dark:bg-slate-800 shadow-lg transition-colors duration-200">
    <div class="container mx-auto px-4">
      <div class="flex justify-between items-center h-16">
        <div class="flex items-center space-x-8">
          <router-link to="/" class="text-lg sm:text-xl font-bold text-primary-600 dark:text-primary-400 transition-colors flex items-center gap-2">
            <span>🏠</span>
            <span class="hidden sm:inline">Real Estate Dashboard</span>
            <span class="sm:hidden">RE Dashboard</span>
          </router-link>
          <div class="hidden md:flex space-x-4">
            <router-link 
              to="/" 
              class="nav-link"
              :class="{ 'active': $route.name === 'dashboard' }"
            >
              Dashboard
            </router-link>
            <router-link 
              to="/properties" 
              class="nav-link"
              :class="{ 'active': $route.name === 'properties' }"
            >
              Properties
            </router-link>
            <router-link 
              to="/tenants" 
              class="nav-link"
              :class="{ 'active': $route.name === 'tenants' }"
            >
              Tenants
            </router-link>
            <router-link 
              to="/calendar" 
              class="nav-link"
              :class="{ 'active': $route.name === 'calendar' }"
            >
              Calendar
            </router-link>
            <router-link 
              to="/maintenance" 
              class="nav-link"
              :class="{ 'active': $route.name === 'maintenance' }"
            >
              Maintenance
            </router-link>
            <router-link 
              to="/market" 
              class="nav-link"
              :class="{ 'active': $route.name === 'market' }"
            >
              Market Insights
            </router-link>
          </div>
        </div>
        
        <!-- Theme Toggle & Mobile Menu Button -->
        <div class="flex items-center gap-2">
          <ThemeToggle />
          <button 
            @click="mobileMenuOpen = !mobileMenuOpen"
            class="md:hidden p-2 rounded-md text-gray-700 dark:text-slate-300 hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors"
            aria-label="Toggle mobile menu"
          >
            <svg v-if="!mobileMenuOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
            <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Mobile Menu -->
      <div v-if="mobileMenuOpen" class="md:hidden pb-4 pt-2 space-y-1">
        <router-link 
          to="/" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'dashboard' }"
          @click="mobileMenuOpen = false"
        >
          📊 Dashboard
        </router-link>
        <router-link 
          to="/properties" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'properties' }"
          @click="mobileMenuOpen = false"
        >
          🏘️ Properties
        </router-link>
        <router-link 
          to="/tenants" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'tenants' }"
          @click="mobileMenuOpen = false"
        >
          👥 Tenants
        </router-link>
        <router-link 
          to="/calendar" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'calendar' }"
          @click="mobileMenuOpen = false"
        >
          📅 Calendar
        </router-link>
        <router-link 
          to="/maintenance" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'maintenance' }"
          @click="mobileMenuOpen = false"
        >
          🔧 Maintenance
        </router-link>
        <router-link 
          to="/market" 
          class="mobile-nav-link"
          :class="{ 'active': $route.name === 'market' }"
          @click="mobileMenuOpen = false"
        >
          📈 Market Insights
        </router-link>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ThemeToggle from './ThemeToggle.vue'

const mobileMenuOpen = ref(false)
</script>

<style scoped>
.nav-link {
  @apply px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-slate-300 hover:text-primary-600 dark:hover:text-primary-400 hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors duration-200;
}

.nav-link.active {
  @apply text-primary-600 dark:text-primary-400 bg-primary-50 dark:bg-slate-700;
}

.mobile-nav-link {
  @apply block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-slate-300 hover:text-primary-600 dark:hover:text-primary-400 hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors duration-200;
}

.mobile-nav-link.active {
  @apply text-primary-600 dark:text-primary-400 bg-primary-50 dark:bg-slate-700;
}
</style>
