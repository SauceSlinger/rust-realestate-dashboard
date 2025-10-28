<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Tenants</h1>
        <p class="text-gray-600 mt-1">Manage tenant relationships and leases</p>
      </div>
      <button @click="showAddModal = true" class="btn btn-primary whitespace-nowrap">
        + Add Tenant
      </button>
    </div>

    <!-- Analytics Cards -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div class="card text-center">
        <p class="text-sm text-gray-600">Total Tenants</p>
        <p class="text-2xl font-bold text-gray-900 mt-1">{{ filteredTenants.length }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600">Active Leases</p>
        <p class="text-2xl font-bold text-green-600 mt-1">{{ activeLeaseCount }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600">Expiring Soon</p>
        <p class="text-2xl font-bold text-yellow-600 mt-1">{{ expiringSoonCount }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600">Total Rent/Month</p>
        <p class="text-2xl font-bold text-primary-600 mt-1">${{ totalMonthlyRent.toLocaleString() }}</p>
      </div>
    </div>

    <!-- Search and Filters -->
    <div class="card">
      <div class="flex flex-col md:flex-row gap-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search tenants by name, email, or phone..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          />
        </div>
        <select v-model="filterStatus" class="px-4 py-2 border border-gray-300 rounded-lg">
          <option value="">All Statuses</option>
          <option value="active">Active</option>
          <option value="expiring-soon">Expiring Soon (60 days)</option>
          <option value="past-due">Past Due</option>
        </select>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-12">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      <p class="text-gray-500 mt-4">Loading tenants...</p>
    </div>

    <!-- Empty State -->
    <div v-else-if="tenants.length === 0" class="text-center py-12 card">
      <div class="text-6xl mb-4">👥</div>
      <h2 class="text-2xl font-bold text-gray-900 mb-2">No Tenants Yet</h2>
      <p class="text-gray-600 mb-6">Start by adding your first tenant to manage leases and payments.</p>
      <button @click="showAddModal = true" class="btn btn-primary">
        + Add Your First Tenant
      </button>
    </div>

    <!-- Tenant Grid -->
    <div v-else-if="filteredTenants.length === 0" class="text-center py-12 card">
      <div class="text-6xl mb-4">🔍</div>
      <p class="text-gray-500 mb-4">No tenants match your search</p>
      <button @click="clearSearch" class="btn btn-secondary">Clear Search</button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="tenant in filteredTenants"
        :key="tenant.id"
        class="card hover:shadow-xl transition-all duration-200 cursor-pointer"
        @click="selectTenant(tenant)"
      >
        <!-- Tenant Avatar and Status -->
        <div class="flex items-start justify-between mb-4">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-full bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center text-white text-xl font-bold">
              {{ getInitials(tenant) }}
            </div>
            <div>
              <h3 class="font-semibold text-gray-900">{{ tenant.first_name }} {{ tenant.last_name }}</h3>
              <p class="text-sm text-gray-600">{{ getPropertyTitle(tenant.property_id) }}</p>
            </div>
          </div>
          <span
            class="px-2 py-1 text-xs font-semibold rounded-full"
            :class="getStatusBadge(tenant)"
          >
            {{ getLeaseStatus(tenant) }}
          </span>
        </div>

        <!-- Contact Info -->
        <div class="space-y-2 text-sm mb-4">
          <p v-if="tenant.email" class="flex items-center gap-2 text-gray-600">
            <span>📧</span> {{ tenant.email }}
          </p>
          <p v-if="tenant.phone" class="flex items-center gap-2 text-gray-600">
            <span>📱</span> {{ tenant.phone }}
          </p>
        </div>

        <!-- Lease Info -->
        <div class="pt-4 border-t border-gray-200 space-y-2 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-600">Lease Period:</span>
            <span class="font-medium">{{ formatDate(tenant.lease_start) }} - {{ formatDate(tenant.lease_end) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Monthly Rent:</span>
            <span class="font-semibold text-primary-600">${{ tenant.monthly_rent.toLocaleString() }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Days Remaining:</span>
            <span :class="getDaysRemainingClass(tenant)">{{ getDaysRemaining(tenant) }} days</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Tenant Detail Modal -->
    <div
      v-if="selectedTenant"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
      @click.self="selectedTenant = null"
    >
      <div class="bg-white rounded-lg max-w-4xl w-full max-h-[90vh] overflow-y-auto">
        <!-- Modal Header -->
        <div class="sticky top-0 bg-white border-b border-gray-200 px-6 py-4 flex justify-between items-center">
          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-full bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center text-white text-xl font-bold">
              {{ getInitials(selectedTenant) }}
            </div>
            <div>
              <h2 class="text-2xl font-bold text-gray-900">
                {{ selectedTenant.first_name }} {{ selectedTenant.last_name }}
              </h2>
              <p class="text-gray-600">{{ getPropertyTitle(selectedTenant.property_id) }}</p>
            </div>
          </div>
          <button @click="selectedTenant = null" class="text-gray-400 hover:text-gray-600">
            <span class="text-2xl">×</span>
          </button>
        </div>

        <!-- Modal Content -->
        <div class="p-6 space-y-6">
          <!-- Contact Information -->
          <div>
            <h3 class="text-lg font-semibold mb-3">Contact Information</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
                <span class="text-2xl">📧</span>
                <div>
                  <p class="text-xs text-gray-500">Email</p>
                  <p class="font-medium">{{ selectedTenant.email || 'N/A' }}</p>
                </div>
              </div>
              <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
                <span class="text-2xl">📱</span>
                <div>
                  <p class="text-xs text-gray-500">Phone</p>
                  <p class="font-medium">{{ selectedTenant.phone || 'N/A' }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Lease Details -->
          <div>
            <h3 class="text-lg font-semibold mb-3">Lease Details</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="p-4 bg-gray-50 rounded-lg">
                <p class="text-sm text-gray-600 mb-1">Lease Start</p>
                <p class="text-lg font-semibold">{{ formatDate(selectedTenant.lease_start) }}</p>
              </div>
              <div class="p-4 bg-gray-50 rounded-lg">
                <p class="text-sm text-gray-600 mb-1">Lease End</p>
                <p class="text-lg font-semibold">{{ formatDate(selectedTenant.lease_end) }}</p>
              </div>
              <div class="p-4 bg-primary-50 rounded-lg">
                <p class="text-sm text-gray-600 mb-1">Monthly Rent</p>
                <p class="text-2xl font-bold text-primary-600">${{ selectedTenant.monthly_rent.toLocaleString() }}</p>
              </div>
              <div class="p-4 bg-blue-50 rounded-lg">
                <p class="text-sm text-gray-600 mb-1">Security Deposit</p>
                <p class="text-2xl font-bold text-blue-600">${{ (selectedTenant.deposit_amount || 0).toLocaleString() }}</p>
              </div>
            </div>
          </div>

          <!-- Payment History Chart -->
          <div>
            <h3 class="text-lg font-semibold mb-3">Payment History (Last 6 Months)</h3>
            <PaymentHistoryChart
              :months="last6Months"
              :payments="generatePaymentHistory(selectedTenant)"
              :expected="selectedTenant.monthly_rent"
            />
          </div>

          <!-- Payment Stats -->
          <div class="grid grid-cols-3 gap-4">
            <div class="card text-center bg-green-50">
              <p class="text-sm text-gray-600">On-Time %</p>
              <p class="text-2xl font-bold text-green-600">{{ calculateOnTimePercent(selectedTenant) }}%</p>
            </div>
            <div class="card text-center bg-blue-50">
              <p class="text-sm text-gray-600">Total Paid</p>
              <p class="text-2xl font-bold text-blue-600">${{ calculateTotalPaid(selectedTenant).toLocaleString() }}</p>
            </div>
            <div class="card text-center bg-purple-50">
              <p class="text-sm text-gray-600">Avg Payment</p>
              <p class="text-2xl font-bold text-purple-600">${{ calculateAvgPayment(selectedTenant).toLocaleString() }}</p>
            </div>
          </div>

          <!-- Notes -->
          <div v-if="selectedTenant.notes">
            <h3 class="text-lg font-semibold mb-3">Notes</h3>
            <p class="text-gray-600 p-4 bg-gray-50 rounded-lg">{{ selectedTenant.notes }}</p>
          </div>

          <!-- Action Buttons -->
          <div class="flex gap-3 pt-4 border-t">
            <button class="btn btn-primary flex-1">
              <span class="mr-1">✏️</span> Edit Tenant
            </button>
            <button class="btn btn-secondary flex-1">
              <span class="mr-1">📄</span> View Lease
            </button>
            <button class="btn btn-secondary">
              <span class="mr-1">💬</span> Contact
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Tenant Modal -->
    <div
      v-if="showAddModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
      @click.self="showAddModal = false"
    >
      <div class="bg-white rounded-lg p-6 max-w-md w-full">
        <h2 class="text-2xl font-bold mb-4">Add New Tenant</h2>
        <p class="text-gray-600 mb-4">Tenant creation form will be implemented in the next phase.</p>
        <div class="flex justify-end space-x-2">
          <button @click="showAddModal = false" class="btn btn-secondary">Cancel</button>
          <button class="btn btn-primary">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { useTenantStore } from '@/stores/tenantStore'
import { usePropertyStore } from '@/stores/propertyStore'
import { format, differenceInDays, parseISO } from 'date-fns'
import PaymentHistoryChart from '@/components/PaymentHistoryChart.vue'
import type { Tenant } from '@/types'

const tenantStore = useTenantStore()
const propertyStore = usePropertyStore()

// State
const searchQuery = ref('')
const filterStatus = ref('')
const selectedTenant = ref<Tenant | null>(null)
const showAddModal = ref(false)

// Computed
const tenants = computed(() => tenantStore.tenants)
const properties = computed(() => propertyStore.properties)
const loading = computed(() => tenantStore.loading)

// Filter tenants based on search and status
const filteredTenants = computed(() => {
  let result = tenants.value

  // Text search
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(t =>
      `${t.first_name} ${t.last_name}`.toLowerCase().includes(query) ||
      t.email?.toLowerCase().includes(query) ||
      t.phone?.toLowerCase().includes(query)
    )
  }

  // Status filter
  if (filterStatus.value === 'active') {
    result = result.filter(t => t.status === 'active')
  } else if (filterStatus.value === 'expiring-soon') {
    result = result.filter(t => {
      const days = getDaysRemaining(t)
      return days <= 60 && days > 0
    })
  } else if (filterStatus.value === 'past-due') {
    result = result.filter(t => getDaysRemaining(t) < 0)
  }

  return result
})

// Analytics
const activeLeaseCount = computed(() =>
  tenants.value.filter(t => t.status === 'active').length
)

const expiringSoonCount = computed(() =>
  tenants.value.filter(t => {
    const days = getDaysRemaining(t)
    return days <= 60 && days > 0
  }).length
)

const totalMonthlyRent = computed(() =>
  filteredTenants.value.reduce((sum, t) => sum + t.monthly_rent, 0)
)

// Generate last 6 months
const last6Months = computed(() => {
  const months = []
  const now = new Date()
  for (let i = 5; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1)
    months.push(d.toLocaleDateString('en-US', { month: 'short' }))
  }
  return months
})

// Lifecycle
onMounted(async () => {
  await Promise.all([
    tenantStore.fetchTenants(),
    propertyStore.fetchProperties()
  ])
})

// Methods
function getInitials(tenant: Tenant): string {
  return `${tenant.first_name[0]}${tenant.last_name[0]}`.toUpperCase()
}

function getPropertyTitle(propertyId: number): string {
  const property = properties.value.find(p => p.id === propertyId)
  return property?.title || 'Unknown Property'
}

function formatDate(dateString: string): string {
  try {
    return format(parseISO(dateString), 'MMM dd, yyyy')
  } catch {
    return 'N/A'
  }
}

function getDaysRemaining(tenant: Tenant): number {
  try {
    const leaseEnd = parseISO(tenant.lease_end)
    const today = new Date()
    return differenceInDays(leaseEnd, today)
  } catch {
    return 0
  }
}

function getDaysRemainingClass(tenant: Tenant): string {
  const days = getDaysRemaining(tenant)
  if (days < 0) return 'font-semibold text-red-600'
  if (days <= 30) return 'font-semibold text-orange-600'
  if (days <= 60) return 'font-semibold text-yellow-600'
  return 'font-medium text-gray-900'
}

function getLeaseStatus(tenant: Tenant): string {
  const days = getDaysRemaining(tenant)
  if (days < 0) return 'Expired'
  if (days <= 30) return 'Expires Soon'
  if (tenant.status === 'active') return 'Active'
  return tenant.status
}

function getStatusBadge(tenant: Tenant): string {
  const days = getDaysRemaining(tenant)
  if (days < 0) return 'bg-red-100 text-red-800'
  if (days <= 30) return 'bg-orange-100 text-orange-800'
  if (days <= 60) return 'bg-yellow-100 text-yellow-800'
  return 'bg-green-100 text-green-800'
}

function selectTenant(tenant: Tenant) {
  selectedTenant.value = tenant
}

function clearSearch() {
  searchQuery.value = ''
  filterStatus.value = ''
}

// Generate sample payment history for a tenant
function generatePaymentHistory(tenant: Tenant): number[] {
  const baseRent = tenant.monthly_rent
  // Generate realistic payment data (mostly on time with occasional variations)
  return last6Months.value.map(() => {
    const onTime = Math.random() > 0.15 // 85% on-time rate
    if (onTime) {
      return baseRent
    } else {
      // Late or partial payment
      return Math.random() > 0.5 ? baseRent * 0.8 : baseRent * 0.5
    }
  })
}

function calculateOnTimePercent(tenant: Tenant): number {
  const payments = generatePaymentHistory(tenant)
  const onTimeCount = payments.filter(p => p === tenant.monthly_rent).length
  return Math.round((onTimeCount / payments.length) * 100)
}

function calculateTotalPaid(tenant: Tenant): number {
  const payments = generatePaymentHistory(tenant)
  return Math.round(payments.reduce((sum, p) => sum + p, 0))
}

function calculateAvgPayment(tenant: Tenant): number {
  const payments = generatePaymentHistory(tenant)
  return Math.round(payments.reduce((sum, p) => sum + p, 0) / payments.length)
}
</script>
