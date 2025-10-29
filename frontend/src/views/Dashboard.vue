<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex justify-between items-center">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-slate-100">Dashboard</h1>
        <p class="text-gray-600 dark:text-slate-400 mt-1">Welcome back! Here's your property portfolio overview.</p>
      </div>
      <button @click="refreshData" class="btn btn-primary flex items-center gap-2">
        <span>🔄</span>
        Refresh Data
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-12">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 dark:border-primary-400"></div>
      <p class="text-gray-500 dark:text-slate-400 mt-4">Loading dashboard data...</p>
    </div>

    <template v-else>
      <!-- KPI Cards -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <KpiCard
          title="Total Properties"
          :value="analytics?.total_properties || 0"
          :trend="5.2"
          icon="🏠"
          color="primary"
        />
        <KpiCard
          title="Portfolio Value"
          :value="formatCurrency(analytics?.total_value || 0)"
          :trend="3.8"
          icon="💰"
          color="success"
        />
        <KpiCard
          title="Monthly Revenue"
          :value="formatCurrency(totalMonthlyRent)"
          :trend="2.1"
          icon="📈"
          color="success"
        />
        <KpiCard
          title="Occupancy Rate"
          :value="`${(analytics?.occupancy_rate || 0).toFixed(1)}%`"
          :trend="-1.3"
          icon="📊"
          color="warning"
        />
      </div>

      <!-- Charts Row -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Revenue & Expenses Chart -->
        <div class="card">
          <h2 class="text-xl font-bold text-gray-900 dark:text-slate-100 mb-4">Revenue vs Expenses</h2>
          <RevenueExpenseChart
            :labels="last12Months"
            :revenue="revenueData"
            :expenses="expenseData"
          />
        </div>

        <!-- Property Status Chart -->
        <div class="card">
          <h2 class="text-xl font-bold text-gray-900 dark:text-slate-100 mb-4">Property Status Distribution</h2>
          <PropertyStatusChart
            :occupied="occupiedCount"
            :vacant="vacantCount"
            :maintenance="maintenanceCount"
          />
        </div>
      </div>

      <!-- Activity & Properties Row -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Recent Activity Feed -->
        <div class="lg:col-span-2">
          <ActivityFeed :activities="recentActivities" :max-items="8" />
        </div>

        <!-- Quick Actions -->
        <div class="card">
          <h2 class="text-xl font-bold text-gray-900 dark:text-slate-100 mb-4">Quick Actions</h2>
          <div class="space-y-3">
            <router-link
              to="/properties"
              class="block w-full btn btn-primary text-left"
            >
              <span class="mr-2">🏠</span> View All Properties
            </router-link>
            <router-link
              to="/tenants"
              class="block w-full btn btn-secondary text-left"
            >
              <span class="mr-2">👥</span> Manage Tenants
            </router-link>
            <router-link
              to="/maintenance"
              class="block w-full btn btn-secondary text-left"
            >
              <span class="mr-2">🔧</span> Maintenance Requests
            </router-link>
            <router-link
              to="/calendar"
              class="block w-full btn btn-secondary text-left"
            >
              <span class="mr-2">📅</span> View Calendar
            </router-link>
            <router-link
              to="/market"
              class="block w-full btn btn-secondary text-left"
            >
              <span class="mr-2">📈</span> Market Insights
            </router-link>
          </div>
        </div>
      </div>

      <!-- Recent Properties Table -->
      <div class="card">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-xl font-bold text-gray-900 dark:text-slate-100">Recent Properties</h2>
          <router-link to="/properties" class="text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 font-medium">
            View All →
          </router-link>
        </div>
        
        <div v-if="properties.length === 0" class="text-center py-8">
          <p class="text-gray-500 dark:text-slate-400">No properties found</p>
        </div>
        <div v-else class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200 dark:divide-slate-700">
            <thead class="bg-gray-50 dark:bg-slate-800">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Property</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Location</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Type</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Status</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Rent</th>
              </tr>
            </thead>
            <tbody class="bg-white dark:bg-slate-900 divide-y divide-gray-200 dark:divide-slate-700">
              <tr v-for="property in properties.slice(0, 5)" :key="property.id" class="hover:bg-gray-50 dark:hover:bg-slate-800">
                <td class="px-6 py-4 whitespace-nowrap">
                  <router-link :to="`/properties/${property.id}`" class="text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 font-medium">
                    {{ property.title }}
                  </router-link>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">{{ property.city }}, {{ property.state }}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">{{ property.property_type }}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full" 
                        :class="getStatusClass(property.status)">
                    {{ property.status }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">
                  ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import { useMarketStore } from '@/stores/marketStore'
import KpiCard from '@/components/KpiCard.vue'
import RevenueExpenseChart from '@/components/RevenueExpenseChart.vue'
import PropertyStatusChart from '@/components/PropertyStatusChart.vue'
import ActivityFeed from '@/components/ActivityFeed.vue'

const propertyStore = usePropertyStore()
const marketStore = useMarketStore()

const properties = computed(() => propertyStore.properties)
const analytics = computed(() => marketStore.analytics)
const loading = computed(() => propertyStore.loading || marketStore.loading)

// Computed property counts
const occupiedCount = computed(() => 
  properties.value.filter(p => p.status === 'occupied').length
)
const vacantCount = computed(() => 
  properties.value.filter(p => p.status === 'vacant').length
)
const maintenanceCount = computed(() => 
  properties.value.filter(p => p.status === 'maintenance').length
)

const totalMonthlyRent = computed(() => 
  properties.value.reduce((sum, p) => sum + (p.monthly_rent || 0), 0)
)

// Generate last 12 months labels
const last12Months = computed(() => {
  const months = []
  const now = new Date()
  for (let i = 11; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1)
    months.push(d.toLocaleDateString('en-US', { month: 'short', year: '2-digit' }))
  }
  return months
})

// Generate sample revenue data (would come from backend in production)
const revenueData = computed(() => {
  const baseRevenue = totalMonthlyRent.value || 5000
  return last12Months.value.map((_, i) => {
    const variation = Math.sin(i * 0.5) * 500 + Math.random() * 300
    return Math.round(baseRevenue + variation)
  })
})

// Generate sample expense data
const expenseData = computed(() => {
  return revenueData.value.map(rev => Math.round(rev * (0.3 + Math.random() * 0.2)))
})

// Generate recent activities (sample data - would come from backend)
const recentActivities = computed(() => {
  const activities = []
  const now = new Date()
  
  // Add some sample activities
  if (properties.value.length > 0) {
    activities.push({
      id: 1,
      type: 'payment' as const,
      title: 'Rent Payment Received',
      description: `Payment of $${properties.value[0]?.monthly_rent || 0} from ${properties.value[0]?.title}`,
      timestamp: new Date(now.getTime() - 2 * 60 * 60 * 1000),
      icon: '💰'
    })
    
    activities.push({
      id: 2,
      type: 'maintenance' as const,
      title: 'Maintenance Request Completed',
      description: 'HVAC repair completed at property',
      timestamp: new Date(now.getTime() - 5 * 60 * 60 * 1000),
      icon: '🔧'
    })
    
    activities.push({
      id: 3,
      type: 'tenant' as const,
      title: 'New Tenant Move-in',
      description: 'Lease signed for 12 months',
      timestamp: new Date(now.getTime() - 24 * 60 * 60 * 1000),
      icon: '👥'
    })
    
    activities.push({
      id: 4,
      type: 'property' as const,
      title: 'Property Inspection Scheduled',
      description: 'Annual inspection set for next week',
      timestamp: new Date(now.getTime() - 48 * 60 * 60 * 1000),
      icon: '🔍'
    })
    
    activities.push({
      id: 5,
      type: 'payment' as const,
      title: 'Rent Payment Received',
      description: `Payment received from tenant`,
      timestamp: new Date(now.getTime() - 72 * 60 * 60 * 1000),
      icon: '💰'
    })
  }
  
  return activities
})

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
    style: 'currency',
    currency: 'USD',
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
