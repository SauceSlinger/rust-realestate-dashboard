<template>
  <nav class="bg-white shadow-lg">
    <div class="container mx-auto px-4">
      <div class="flex justify-between items-center h-16">
        <div class="flex items-center space-x-8">
          <router-link to="/" class="text-xl font-bold text-primary-600">
            🏠 Real Estate Dashboard
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
      </div>
    </div>
  </nav>
</template>

<style scoped>
.nav-link {
  @apply px-3 py-2 rounded-md text-sm font-medium text-gray-700 hover:text-primary-600 hover:bg-gray-100 transition-colors duration-200;
}

.nav-link.active {
  @apply text-primary-600 bg-primary-50;
}
</style>
