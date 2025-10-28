<template>
  <div class="space-y-6">
    <!-- Loading State -->
    <div v-if="loading" class="text-center py-12">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      <p class="text-gray-500 mt-4">Loading property details...</p>
    </div>

    <!-- Property Not Found -->
    <div v-else-if="!property" class="text-center py-12 card">
      <div class="text-6xl mb-4">🏠</div>
      <h2 class="text-2xl font-bold text-gray-900 mb-2">Property Not Found</h2>
      <p class="text-gray-600 mb-6">The property you're looking for doesn't exist.</p>
      <router-link to="/properties" class="btn btn-primary">
        ← Back to Properties
      </router-link>
    </div>

    <!-- Property Details -->
    <template v-else>
      <!-- Header -->
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div class="flex items-center gap-4">
          <router-link to="/properties" class="text-gray-600 hover:text-gray-900">
            <span class="text-2xl">←</span>
          </router-link>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">{{ property.title }}</h1>
            <p class="text-gray-600 mt-1">
              {{ property.address }}, {{ property.city }}, {{ property.state }} {{ property.zip_code }}
            </p>
          </div>
        </div>
        <div class="flex gap-2">
          <button class="btn btn-secondary">
            <span class="mr-1">✏️</span> Edit
          </button>
          <button class="btn btn-primary">
            <span class="mr-1">📄</span> Generate Report
          </button>
        </div>
      </div>

      <!-- Photo Gallery Section -->
      <div class="card">
        <div class="relative h-96 bg-gradient-to-br from-blue-100 via-purple-100 to-pink-100 rounded-lg overflow-hidden">
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="text-center">
              <div class="text-8xl mb-4">🏠</div>
              <p class="text-gray-600">Photo gallery coming soon</p>
              <button class="mt-4 btn btn-secondary">
                <span class="mr-1">📸</span> Upload Photos
              </button>
            </div>
          </div>
          <div class="absolute top-4 right-4">
            <span class="px-4 py-2 text-sm font-semibold rounded-full shadow-lg" :class="getStatusClass(property.status)">
              {{ property.status }}
            </span>
          </div>
        </div>
      </div>

      <!-- Quick Stats -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div class="card text-center">
          <p class="text-sm text-gray-600">Monthly Rent</p>
          <p class="text-2xl font-bold text-primary-600 mt-1">
            ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
          </p>
        </div>
        <div class="card text-center">
          <p class="text-sm text-gray-600">Purchase Price</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">
            ${{ property.purchase_price?.toLocaleString() || 'N/A' }}
          </p>
        </div>
        <div class="card text-center">
          <p class="text-sm text-gray-600">Square Feet</p>
          <p class="text-2xl font-bold text-gray-900 mt-1">
            {{ property.square_feet?.toLocaleString() || 'N/A' }}
          </p>
        </div>
        <div class="card text-center">
          <p class="text-sm text-gray-600">ROI (Est.)</p>
          <p class="text-2xl font-bold text-green-600 mt-1">{{ estimatedROI }}%</p>
        </div>
      </div>

      <!-- Tabs -->
      <div class="card">
        <div class="border-b border-gray-200">
          <nav class="-mb-px flex space-x-8 overflow-x-auto">
            <button
              v-for="tab in tabs"
              :key="tab.id"
              @click="activeTab = tab.id"
              :class="[
                'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors',
                activeTab === tab.id
                  ? 'border-primary-500 text-primary-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              ]"
            >
              {{ tab.icon }} {{ tab.label }}
            </button>
          </nav>
        </div>

        <div class="p-6">
          <!-- Overview Tab -->
          <div v-if="activeTab === 'overview'" class="space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <h3 class="text-lg font-semibold mb-4">Property Information</h3>
                <dl class="space-y-2">
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Property Type</dt>
                    <dd class="font-medium">{{ property.property_type }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Bedrooms</dt>
                    <dd class="font-medium">{{ property.bedrooms || 'N/A' }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Bathrooms</dt>
                    <dd class="font-medium">{{ property.bathrooms || 'N/A' }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Square Feet</dt>
                    <dd class="font-medium">{{ property.square_feet?.toLocaleString() || 'N/A' }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Current Value</dt>
                    <dd class="font-medium">${{ property.current_value?.toLocaleString() || 'N/A' }}</dd>
                  </div>
                </dl>
              </div>

              <div>
                <h3 class="text-lg font-semibold mb-4">Financial Details</h3>
                <dl class="space-y-2">
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Purchase Price</dt>
                    <dd class="font-medium">${{ property.purchase_price?.toLocaleString() || 'N/A' }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Monthly Rent</dt>
                    <dd class="font-medium text-primary-600">${{ property.monthly_rent?.toLocaleString() || 'N/A' }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Created Date</dt>
                    <dd class="font-medium">{{ formatDate(property.created_at) }}</dd>
                  </div>
                  <div class="flex justify-between py-2 border-b">
                    <dt class="text-gray-600">Status</dt>
                    <dd>
                      <span class="px-2 py-1 text-xs font-semibold rounded-full" :class="getStatusClass(property.status)">
                        {{ property.status }}
                      </span>
                    </dd>
                  </div>
                </dl>
              </div>
            </div>

            <div v-if="property.notes" class="pt-4">
              <h3 class="text-lg font-semibold mb-2">Notes</h3>
              <p class="text-gray-600 leading-relaxed">{{ property.notes }}</p>
            </div>
          </div>

          <!-- Financial Tab -->
          <div v-if="activeTab === 'financial'" class="space-y-6">
            <div>
              <h3 class="text-lg font-semibold mb-4">Cash Flow (Last 6 Months)</h3>
              <CashFlowChart :income="monthlyIncome" :expenses="monthlyExpenses" :labels="last6Months" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="card bg-green-50">
                <p class="text-sm text-gray-600">Total Income (6mo)</p>
                <p class="text-2xl font-bold text-green-600 mt-1">
                  ${{ totalIncome.toLocaleString() }}
                </p>
              </div>
              <div class="card bg-red-50">
                <p class="text-sm text-gray-600">Total Expenses (6mo)</p>
                <p class="text-2xl font-bold text-red-600 mt-1">
                  ${{ totalExpenses.toLocaleString() }}
                </p>
              </div>
              <div class="card bg-blue-50">
                <p class="text-sm text-gray-600">Net Cash Flow (6mo)</p>
                <p class="text-2xl font-bold text-blue-600 mt-1">
                  ${{ (totalIncome - totalExpenses).toLocaleString() }}
                </p>
              </div>
            </div>

            <div>
              <h3 class="text-lg font-semibold mb-4">ROI Breakdown</h3>
              <div class="space-y-3">
                <div class="flex justify-between items-center p-3 bg-gray-50 rounded">
                  <span class="text-gray-700">Annual Rental Income</span>
                  <span class="font-semibold">${{ ((property.monthly_rent || 0) * 12).toLocaleString() }}</span>
                </div>
                <div class="flex justify-between items-center p-3 bg-gray-50 rounded">
                  <span class="text-gray-700">Estimated Annual Expenses</span>
                  <span class="font-semibold text-red-600">-${{ estimatedAnnualExpenses.toLocaleString() }}</span>
                </div>
                <div class="flex justify-between items-center p-3 bg-green-50 rounded border-2 border-green-200">
                  <span class="text-gray-700 font-medium">Annual Net Income</span>
                  <span class="font-bold text-green-600">${{ annualNetIncome.toLocaleString() }}</span>
                </div>
                <div class="flex justify-between items-center p-3 bg-primary-50 rounded">
                  <span class="text-gray-700">Purchase Price</span>
                  <span class="font-semibold">${{ (property.purchase_price || 0).toLocaleString() }}</span>
                </div>
                <div class="flex justify-between items-center p-3 bg-primary-100 rounded border-2 border-primary-300">
                  <span class="text-gray-700 font-medium">ROI</span>
                  <span class="font-bold text-primary-600">{{ estimatedROI }}%</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Tenants Tab -->
          <div v-if="activeTab === 'tenants'" class="space-y-6">
            <div class="text-center py-12">
              <div class="text-6xl mb-4">👥</div>
              <h3 class="text-xl font-semibold mb-2">Tenant Management</h3>
              <p class="text-gray-600 mb-4">View and manage tenant information, lease agreements, and payment history.</p>
              <button class="btn btn-primary">
                <span class="mr-1">➕</span> Add Tenant
              </button>
            </div>
          </div>

          <!-- Maintenance Tab -->
          <div v-if="activeTab === 'maintenance'" class="space-y-6">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-lg font-semibold">Maintenance History</h3>
              <button class="btn btn-primary">
                <span class="mr-1">🔧</span> New Request
              </button>
            </div>
            <div class="text-center py-12 bg-gray-50 rounded-lg">
              <div class="text-6xl mb-4">🔧</div>
              <p class="text-gray-600">No maintenance requests found</p>
              <p class="text-sm text-gray-500 mt-2">Create a new request to get started</p>
            </div>
          </div>

          <!-- Documents Tab -->
          <div v-if="activeTab === 'documents'" class="space-y-6">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-lg font-semibold">Documents & Files</h3>
              <button class="btn btn-primary">
                <span class="mr-1">📤</span> Upload Document
              </button>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div class="card hover:shadow-lg transition-shadow cursor-pointer">
                <div class="text-center py-6">
                  <div class="text-4xl mb-2">📄</div>
                  <p class="font-medium">Lease Agreements</p>
                  <p class="text-sm text-gray-500">0 files</p>
                </div>
              </div>
              <div class="card hover:shadow-lg transition-shadow cursor-pointer">
                <div class="text-center py-6">
                  <div class="text-4xl mb-2">🔍</div>
                  <p class="font-medium">Inspections</p>
                  <p class="text-sm text-gray-500">0 files</p>
                </div>
              </div>
              <div class="card hover:shadow-lg transition-shadow cursor-pointer">
                <div class="text-center py-6">
                  <div class="text-4xl mb-2">🧾</div>
                  <p class="font-medium">Receipts</p>
                  <p class="text-sm text-gray-500">0 files</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Activity Tab -->
          <div v-if="activeTab === 'activity'" class="space-y-4">
            <h3 class="text-lg font-semibold mb-4">Activity Timeline</h3>
            <div class="space-y-4">
              <div class="flex gap-4 p-4 bg-gray-50 rounded-lg">
                <div class="flex-shrink-0 w-10 h-10 bg-green-100 rounded-full flex items-center justify-center">
                  <span class="text-xl">✅</span>
                </div>
                <div class="flex-1">
                  <p class="font-medium">Property created</p>
                  <p class="text-sm text-gray-600">{{ formatDate(property.created_at) }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { usePropertyStore } from '@/stores/propertyStore'
import { format } from 'date-fns'
import CashFlowChart from '@/components/CashFlowChart.vue'

const route = useRoute()
const propertyStore = usePropertyStore()

const property = computed(() => propertyStore.currentProperty)
const loading = computed(() => propertyStore.loading)

// Active tab state
const activeTab = ref('overview')

// Tab configuration
const tabs = [
  { id: 'overview', label: 'Overview', icon: '📋' },
  { id: 'financial', label: 'Financial', icon: '💰' },
  { id: 'tenants', label: 'Tenants', icon: '👥' },
  { id: 'maintenance', label: 'Maintenance', icon: '🔧' },
  { id: 'documents', label: 'Documents', icon: '📄' },
  { id: 'activity', label: 'Activity', icon: '📊' }
]

// Generate last 6 months labels
const last6Months = computed(() => {
  const months = []
  const now = new Date()
  for (let i = 5; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1)
    months.push(d.toLocaleDateString('en-US', { month: 'short' }))
  }
  return months
})

// Generate sample monthly income data
const monthlyIncome = computed(() => {
  const baseRent = property.value?.monthly_rent || 0
  return last6Months.value.map(() => baseRent + (Math.random() * 100 - 50))
})

// Generate sample monthly expenses data
const monthlyExpenses = computed(() => {
  const baseExpense = (property.value?.monthly_rent || 0) * 0.3
  return last6Months.value.map(() => baseExpense + (Math.random() * 200 - 100))
})

// Calculate totals
const totalIncome = computed(() => 
  monthlyIncome.value.reduce((sum, val) => sum + val, 0)
)

const totalExpenses = computed(() => 
  monthlyExpenses.value.reduce((sum, val) => sum + val, 0)
)

// Financial calculations
const estimatedAnnualExpenses = computed(() => {
  const monthlyRent = property.value?.monthly_rent || 0
  // Estimate 30% of rent as expenses (maintenance, taxes, insurance, etc.)
  return Math.round(monthlyRent * 12 * 0.3)
})

const annualNetIncome = computed(() => {
  const monthlyRent = property.value?.monthly_rent || 0
  return (monthlyRent * 12) - estimatedAnnualExpenses.value
})

const estimatedROI = computed(() => {
  const purchasePrice = property.value?.purchase_price || 0
  if (purchasePrice === 0) return '0.0'
  return ((annualNetIncome.value / purchasePrice) * 100).toFixed(1)
})

// Lifecycle
onMounted(() => {
  const id = Number(route.params.id)
  propertyStore.fetchProperty(id)
})

// Methods
function formatDate(dateString?: string): string {
  if (!dateString) return 'N/A'
  try {
    return format(new Date(dateString), 'MMM dd, yyyy')
  } catch {
    return 'N/A'
  }
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
