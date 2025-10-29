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

    <!-- Charts + ROI -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 card">
        <h2 class="text-xl font-bold text-gray-900 mb-4">Portfolio Performance</h2>
        <PortfolioPerformanceChart :series="portfolioSeries" :categories="portfolioCategories" />
      </div>

      <div class="card">
        <h2 class="text-xl font-bold text-gray-900 mb-4">Rent vs Expenses</h2>
        <div class="mb-4">
          <RentExpenseDonutChart :series="rentExpenseSeries" />
        </div>

        <div class="space-y-3">
          <h3 class="text-lg font-semibold text-gray-900 border-b pb-2">ROI Calculator</h3>
          <div class="grid grid-cols-2 gap-2">
            <label class="text-sm text-gray-600">Purchase Price</label>
            <input v-model.number="purchasePrice" type="number" class="input" placeholder="e.g. 250000" @input="saveInputs" />
            <label class="text-sm text-gray-600">Current Value</label>
            <input v-model.number="currentValue" type="number" class="input" placeholder="e.g. 300000" @input="saveInputs" />
            <label class="text-sm text-gray-600">Annual Rent</label>
            <input v-model.number="annualRent" type="number" class="input" placeholder="e.g. 36000" @input="saveInputs" />
            <label class="text-sm text-gray-600">Annual Expenses</label>
            <input v-model.number="annualExpenses" type="number" class="input" placeholder="e.g. 14400" @input="saveInputs" />
          </div>

          <div class="pt-3 border-t space-y-2">
            <div class="flex items-center justify-between bg-blue-50 p-2 rounded">
              <div>
                <div class="text-xs text-gray-600">ROI (on purchase)</div>
                <div class="text-xl font-bold text-blue-600">{{ roiPercent !== null ? roiPercent.toFixed(2) + '%' : '—' }}</div>
              </div>
              <div>
                <div class="text-xs text-gray-600">Cap Rate</div>
                <div class="text-xl font-bold text-green-600">{{ capRatePercent !== null ? capRatePercent.toFixed(2) + '%' : '—' }}</div>
              </div>
            </div>
            <div v-if="roiPercent !== null" class="text-xs text-gray-500 text-center">
              Net Operating Income: ${{ ((annualRent || 0) - (annualExpenses || 0)).toLocaleString() }}/year
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Cache Info (for debugging) -->
    <div v-if="showCacheInfo" class="card bg-gray-50">
      <div class="flex justify-between items-start">
        <div>
          <h3 class="text-sm font-semibold text-gray-700 mb-2">Free Tier Cache Status</h3>
          <p class="text-xs text-gray-600">Last fetch: {{ lastFetchDate || 'Never' }}</p>
          <p class="text-xs text-gray-600">Next Monday refresh in {{ daysUntilMonday }} days</p>
          <p class="text-xs text-gray-600">Cache expires: {{ cacheExpiresIn }}</p>
        </div>
        <button @click="showCacheInfo = false" class="text-xs text-gray-500 hover:text-gray-700">Hide</button>
      </div>
    </div>
    <button v-else @click="showCacheInfo = true" class="text-xs text-gray-400 hover:text-gray-600">Show cache info</button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref, watch } from 'vue'
import { useMarketStore } from '@/stores/marketStore'
import { usePropertyStore } from '@/stores/propertyStore'
import PortfolioPerformanceChart from '@/components/PortfolioPerformanceChart.vue'
import RentExpenseDonutChart from '@/components/RentExpenseDonutChart.vue'
import { FreeTierCache, saveROIInputs, loadROIInputs } from '@/utils/freeTierCache'

const marketStore = useMarketStore()
const propertyStore = usePropertyStore()

const analytics = computed(() => marketStore.analytics)
const trends = computed(() => marketStore.trends)
const loading = computed(() => marketStore.loading)
const lastFetchDate = computed(() => marketStore.lastFetchDate)

// ROI calculator inputs with persistence
const purchasePrice = ref<number | null>(null)
const currentValue = ref<number | null>(null)
const annualRent = ref<number | null>(null)
const annualExpenses = ref<number | null>(null)
const showCacheInfo = ref(false)

const roiPercent = computed(() => {
  if (!purchasePrice.value || purchasePrice.value === 0 || annualRent.value === null || annualExpenses.value === null) return null
  const noi = (annualRent.value || 0) - (annualExpenses.value || 0)
  return (noi / purchasePrice.value) * 100
})

const capRatePercent = computed(() => {
  if (!currentValue.value || currentValue.value === 0 || annualRent.value === null || annualExpenses.value === null) return null
  const noi = (annualRent.value || 0) - (annualExpenses.value || 0)
  return (noi / currentValue.value) * 100
})

// Chart data
const portfolioSeries = ref<any[]>([])
const portfolioCategories = ref<string[]>([])
const rentExpenseSeries = ref<number[]>([0, 0])

const daysUntilMonday = computed(() => FreeTierCache.daysUntilMonday())
const cacheExpiresIn = computed(() => {
  const meta = FreeTierCache.getMetadata('market_analytics')
  if (!meta) return 'No cache'
  const days = Math.ceil(meta.expiresIn / (1000 * 60 * 60 * 24))
  return `${days} days`
})

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

function saveInputs() {
  saveROIInputs({
    purchasePrice: purchasePrice.value,
    currentValue: currentValue.value,
    annualRent: annualRent.value,
    annualExpenses: annualExpenses.value
  })
}

function loadInputs() {
  const saved = loadROIInputs()
  if (saved) {
    purchasePrice.value = saved.purchasePrice
    currentValue.value = saved.currentValue
    annualRent.value = saved.annualRent
    annualExpenses.value = saved.annualExpenses
  }
}

onMounted(() => {
  // Load saved ROI inputs
  loadInputs()
  
  // Check for Monday auto-refresh
  marketStore.checkAndRefresh()
  
  // Fetch data (will use cache if available)
  Promise.all([
    marketStore.fetchAnalytics(),
    marketStore.fetchTrends()
  ]).then(() => syncCharts())
})

async function handleScrape() {
  await marketStore.triggerScrape()
  // Refresh data after scraping
  setTimeout(() => {
    Promise.all([
      marketStore.fetchAnalytics(),
      marketStore.fetchTrends()
    ]).then(() => syncCharts())
  }, 2000)
}

function syncCharts() {
  const a = analytics.value
  if (a) {
    // Populate ROI inputs with portfolio defaults if not already set
    if (!purchasePrice.value && a.total_value) purchasePrice.value = Math.round(a.total_value * 0.85)
    if (!currentValue.value && a.total_value) currentValue.value = a.total_value
    if (!annualRent.value && a.average_rent) annualRent.value = Math.round(a.average_rent * 12)
    if (!annualExpenses.value && a.average_rent) annualExpenses.value = Math.round(a.average_rent * 12 * 0.4)

    // Update donut chart
    rentExpenseSeries.value = [annualRent.value || 0, annualExpenses.value || 0]
  }

  // Build portfolio performance chart from trends
  if (trends.value && trends.value.length > 0) {
    const primary = trends.value[0]
    portfolioCategories.value = primary.time_series.map(p => formatDate(p.date))
    portfolioSeries.value = [{ 
      name: primary.location + ' Median Price', 
      data: primary.time_series.map(p => p.median_price || 0) 
    }]
  } else if (a) {
    // Fallback: create dummy series using total_value
    const months = Array.from({ length: 6 }, (_, i) => {
      const d = new Date()
      d.setMonth(d.getMonth() - (5 - i))
      return formatDate(d.toISOString())
    })
    portfolioCategories.value = months
    portfolioSeries.value = [{ 
      name: 'Portfolio Value', 
      data: Array(6).fill(a.total_value || 0) 
    }]
  }
}

// Keep donut in sync with input changes
watch([annualRent, annualExpenses], () => {
  rentExpenseSeries.value = [annualRent.value || 0, annualExpenses.value || 0]
})
</script>
