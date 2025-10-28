<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Market Data & Analytics</h1>
      <button @click="scrapeData" :disabled="scraping"
              class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 disabled:bg-gray-400">
        {{ scraping ? 'Scraping...' : 'Scrape New Data' }}
      </button>
    </div>

    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <h2 class="text-xl font-bold mb-4">Market Trends</h2>
      <div v-if="chartData" class="h-96">
        <Line :data="chartData" :options="chartOptions" />
      </div>
      <div v-else class="text-center py-12 text-gray-500">
        No market data available. Click "Scrape New Data" to fetch market information.
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <div v-for="location in uniqueLocations" :key="location"
           class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-bold mb-4">{{ location }}</h3>
        <div v-if="getLatestDataForLocation(location)" class="space-y-2">
          <div class="flex justify-between">
            <span class="text-gray-600">Median Price:</span>
            <span class="font-bold text-green-600">
              ${{ getLatestDataForLocation(location)?.median_price.toLocaleString() }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Price/Sq Ft:</span>
            <span class="font-medium">
              ${{ getLatestDataForLocation(location)?.average_price_per_sqft }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Inventory:</span>
            <span class="font-medium">
              {{ getLatestDataForLocation(location)?.inventory_count }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Days on Market:</span>
            <span class="font-medium">
              {{ getLatestDataForLocation(location)?.days_on_market }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-lg shadow p-6">
      <h2 class="text-xl font-bold mb-4">Historical Data</h2>
      <div v-if="marketData.length === 0" class="text-center py-12 text-gray-500">
        No historical data available.
      </div>
      <div v-else class="overflow-x-auto">
        <table class="min-w-full">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Location</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Date</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Median Price</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Price/Sq Ft</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Inventory</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Days on Market</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="data in marketData" :key="data.id">
              <td class="px-6 py-4 whitespace-nowrap">{{ data.location }}</td>
              <td class="px-6 py-4 whitespace-nowrap">{{ data.data_date }}</td>
              <td class="px-6 py-4 whitespace-nowrap">${{ data.median_price.toLocaleString() }}</td>
              <td class="px-6 py-4 whitespace-nowrap">${{ data.average_price_per_sqft }}</td>
              <td class="px-6 py-4 whitespace-nowrap">{{ data.inventory_count }}</td>
              <td class="px-6 py-4 whitespace-nowrap">{{ data.days_on_market }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'
import { marketDataApi, scraperApi, type MarketData } from '@/services/api'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
)

const marketData = ref<MarketData[]>([])
const scraping = ref(false)

const uniqueLocations = computed(() => {
  return [...new Set(marketData.value.map(d => d.location))]
})

const getLatestDataForLocation = (location: string) => {
  const locationData = marketData.value.filter(d => d.location === location)
  return locationData.length > 0 ? locationData[0] : null
}

const chartData = computed(() => {
  if (marketData.value.length === 0) return null

  const locations = uniqueLocations.value
  const datasets = locations.map((location, index) => {
    const locationData = marketData.value
      .filter(d => d.location === location)
      .sort((a, b) => a.data_date.localeCompare(b.data_date))

    const colors = [
      { border: 'rgb(59, 130, 246)', bg: 'rgba(59, 130, 246, 0.1)' },
      { border: 'rgb(16, 185, 129)', bg: 'rgba(16, 185, 129, 0.1)' },
      { border: 'rgb(249, 115, 22)', bg: 'rgba(249, 115, 22, 0.1)' },
    ]
    const color = colors[index % colors.length]!

    return {
      label: location,
      data: locationData.map(d => d.median_price),
      borderColor: color.border,
      backgroundColor: color.bg,
      tension: 0.4
    }
  })

  const allDates = [...new Set(marketData.value.map(d => d.data_date))].sort()

  return {
    labels: allDates,
    datasets
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'top' as const,
    },
    title: {
      display: true,
      text: 'Median Home Prices Over Time'
    },
    tooltip: {
      callbacks: {
        label: function(context: any) {
          return `${context.dataset.label}: $${context.parsed.y.toLocaleString()}`
        }
      }
    }
  },
  scales: {
    y: {
      ticks: {
        callback: function(value: any) {
          return '$' + value.toLocaleString()
        }
      }
    }
  }
}

const loadMarketData = async () => {
  try {
    const response = await marketDataApi.getAll()
    marketData.value = response.data
  } catch (error) {
    console.error('Error loading market data:', error)
  }
}

const scrapeData = async () => {
  scraping.value = true
  try {
    await scraperApi.scrapeMarketData()
    await loadMarketData()
  } catch (error) {
    console.error('Error scraping market data:', error)
  } finally {
    scraping.value = false
  }
}

onMounted(loadMarketData)
</script>
