<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-3xl font-bold text-gray-900">Market Insights</h1>
      <button @click="handleScrape" class="btn btn-primary" :disabled="loading">
        {{ loading ? 'Scraping...' : 'Refresh Market Data' }}
      </button>
    </div>

    <!-- Analytics Summary -->
    <div v-if="analytics" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Total Properties</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">{{ analytics.total_properties }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Portfolio Value</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">${{ formatCurrency(analytics.total_value) }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Avg Monthly Rent</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">${{ formatCurrency(analytics.average_rent) }}</div>
      </div>
      <div class="card">
        <div class="text-sm font-medium text-gray-600">Occupancy Rate</div>
        <div class="text-3xl font-bold text-gray-900 mt-2">{{ analytics.occupancy_rate.toFixed(1) }}%</div>
      </div>
    </div>

    <!-- Market Trends -->
    <div class="card">
      <h2 class="text-xl font-bold text-gray-900 mb-4">Market Trends</h2>
      <div v-if="loading" class="text-center py-8">
        <p class="text-gray-500">Loading market data...</p>
      </div>
      <div v-else-if="trends.length === 0" class="text-center py-8">
        <p class="text-gray-500">No market data available. Click "Refresh Market Data" to scrape latest data.</p>
      </div>
      <div v-else>
        <div v-for="trend in trends" :key="trend.location" class="mb-6">
          <h3 class="font-semibold text-lg mb-2">{{ trend.location }}</h3>
          <div class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200">
              <thead class="bg-gray-50">
                <tr>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Date</th>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Median Price</th>
                  <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase">Inventory</th>
                </tr>
              </thead>
              <tbody class="bg-white divide-y divide-gray-200">
                <tr v-for="(point, index) in trend.time_series.slice(0, 5)" :key="index">
                  <td class="px-4 py-2 text-sm text-gray-900">{{ formatDate(point.date) }}</td>
                  <td class="px-4 py-2 text-sm text-gray-900">${{ point.median_price?.toLocaleString() || 'N/A' }}</td>
                  <td class="px-4 py-2 text-sm text-gray-900">{{ point.inventory_count?.toLocaleString() || 'N/A' }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useMarketStore } from '@/stores/marketStore'

const marketStore = useMarketStore()

const analytics = computed(() => marketStore.analytics)
const trends = computed(() => marketStore.trends)
const loading = computed(() => marketStore.loading)

onMounted(() => {
  Promise.all([
    marketStore.fetchAnalytics(),
    marketStore.fetchTrends()
  ])
})

async function handleScrape() {
  await marketStore.triggerScrape()
  // Refresh data after scraping
  setTimeout(() => {
    Promise.all([
      marketStore.fetchAnalytics(),
      marketStore.fetchTrends()
    ])
  }, 2000)
}

function formatCurrency(value: number): string {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0
  }).format(value)
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>
