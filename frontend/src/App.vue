<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-blue-600 text-white shadow-lg">
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center space-x-8">
            <h1 class="text-2xl font-bold">Real Estate Dashboard</h1>
            <div class="hidden md:flex space-x-4">
              <RouterLink to="/" class="hover:bg-blue-700 px-3 py-2 rounded">
                Dashboard
              </RouterLink>
              <RouterLink to="/properties" class="hover:bg-blue-700 px-3 py-2 rounded">
                Properties
              </RouterLink>
              <RouterLink to="/reminders" class="hover:bg-blue-700 px-3 py-2 rounded">
                Reminders
              </RouterLink>
              <RouterLink to="/tenants" class="hover:bg-blue-700 px-3 py-2 rounded">
                Tenants
              </RouterLink>
              <RouterLink to="/market-data" class="hover:bg-blue-700 px-3 py-2 rounded">
                Market Data
              </RouterLink>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <main class="container mx-auto px-4 py-8">
      <RouterView />
    </main>
  </div>
</template>
