<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
      <button @click="refreshData" class="btn btn-primary">
        Refresh Data
      </button>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Total Properties</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">{{ analytics?.total_properties || 0 }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Total Value</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">${{ formatCurrency(analytics?.total_value || 0) }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Average Rent</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">${{ formatCurrency(analytics?.average_rent || 0) }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Occupancy Rate</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">{{ analytics?.occupancy_rate.toFixed(1) || 0 }}%</div>
      </div>
    </div>

    <!-- Recent Properties -->
    <div class="card">
      <h2 class="text-xl font-bold text-gray-900 mb-4">Recent Properties</h2>
      <div v-if="loading" class="text-center py-8">
        <p class="text-gray-500">Loading...</p>
      </div>
      <div v-else-if="properties.length === 0" class="text-center py-8">
        <p class="text-gray-500">No properties found</p>
      </div>
      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Property</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Location</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Type</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Rent</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="property in properties.slice(0, 5)" :key="property.id" class="hover:bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap">
                <router-link :to="`/properties/${property.id}`" class="text-primary-600 hover:text-primary-800 font-medium">
                  {{ property.title }}
                </router-link>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ property.city }}, {{ property.state }}</td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{{ property.property_type }}</td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full" 
                      :class="getStatusClass(property.status)">
                  {{ property.status }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import { useMarketStore } from '@/stores/marketStore'

const propertyStore = usePropertyStore()
const marketStore = useMarketStore()

const properties = computed(() => propertyStore.properties)
const analytics = computed(() => marketStore.analytics)
const loading = computed(() => propertyStore.loading || marketStore.loading)

onMounted(async () => {
  await Promise.all([
    propertyStore.fetchProperties(),
    marketStore.fetchAnalytics()
  ])
})

function refreshData() {
  Promise.all([
    propertyStore.fetchProperties(),
    marketStore.fetchAnalytics()
  ])
}

function formatCurrency(value: number): string {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0
  }).format(value)
}

function getStatusClass(status: string): string {
  const statusClasses: Record<string, string> = {
    occupied: 'bg-green-100 text-green-800',
    vacant: 'bg-yellow-100 text-yellow-800',
    maintenance: 'bg-red-100 text-red-800'
  }
  return statusClasses[status] || 'bg-gray-100 text-gray-800'
}
</script>
