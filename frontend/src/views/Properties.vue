<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-slate-100">Properties</h1>
        <p class="text-gray-600 dark:text-slate-400 mt-1">Manage your real estate portfolio</p>
      </div>
      <button @click="showCreateModal = true" class="btn btn-primary whitespace-nowrap">
        + Add Property
      </button>
    </div>

    <!-- Analytics Summary Bar -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div class="card text-center">
        <p class="text-sm text-gray-600 dark:text-slate-400">Total Properties</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-slate-100 mt-1">{{ filteredProperties.length }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 dark:text-slate-400">Avg Price</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-slate-100 mt-1">${{ formatNumber(averagePrice) }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 dark:text-slate-400">Avg Rent</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-slate-100 mt-1">${{ formatNumber(averageRent) }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 dark:text-slate-400">Total Sq Ft</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-slate-100 mt-1">{{ formatNumber(totalSquareFeet) }}</p>
      </div>
    </div>

    <!-- Filters and Search -->
    <div class="card space-y-4">
      <!-- Search Bar -->
      <div class="flex flex-col md:flex-row gap-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search properties by title, address, or city..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-slate-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100"
          />
        </div>
        <button
          @click="showFilters = !showFilters"
          class="btn btn-secondary whitespace-nowrap flex items-center gap-2"
        >
          <span>🔍</span>
          {{ showFilters ? 'Hide Filters' : 'Show Filters' }}
          <span v-if="activeFilterCount > 0" class="bg-primary-600 text-white rounded-full px-2 py-0.5 text-xs">
            {{ activeFilterCount }}
          </span>
        </button>
      </div>

      <!-- Advanced Filters -->
      <div v-if="showFilters" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 pt-4 border-t border-gray-200 dark:border-slate-700">
        <!-- Status Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-slate-300 mb-1">Status</label>
          <select v-model="filters.status" class="w-full px-3 py-2 border border-gray-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100">
            <option value="">All Statuses</option>
            <option value="occupied">Occupied</option>
            <option value="vacant">Vacant</option>
            <option value="maintenance">Maintenance</option>
          </select>
        </div>

        <!-- Property Type Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-slate-300 mb-1">Type</label>
          <select v-model="filters.propertyType" class="w-full px-3 py-2 border border-gray-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100">
            <option value="">All Types</option>
            <option value="single-family">Single Family</option>
            <option value="multi-family">Multi Family</option>
            <option value="apartment">Apartment</option>
            <option value="condo">Condo</option>
            <option value="commercial">Commercial</option>
          </select>
        </div>

        <!-- Bedrooms Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-slate-300 mb-1">Bedrooms</label>
          <select v-model="filters.bedrooms" class="w-full px-3 py-2 border border-gray-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100">
            <option value="">Any</option>
            <option value="1">1+</option>
            <option value="2">2+</option>
            <option value="3">3+</option>
            <option value="4">4+</option>
          </select>
        </div>

        <!-- City Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-slate-300 mb-1">City</label>
          <select v-model="filters.city" class="w-full px-3 py-2 border border-gray-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100">
            <option value="">All Cities</option>
            <option v-for="city in uniqueCities" :key="city" :value="city">{{ city }}</option>
          </select>
        </div>

        <!-- Clear Filters Button -->
        <div class="md:col-span-2 lg:col-span-4 flex justify-end">
          <button @click="clearFilters" class="text-sm text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 font-medium">
            Clear All Filters
          </button>
        </div>
      </div>
    </div>

    <!-- View Toggle and Sort -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <!-- View Toggle -->
      <div class="flex gap-2 bg-gray-100 dark:bg-slate-800 rounded-lg p-1">
        <button
          @click="viewMode = 'grid'"
          :class="[
            'px-4 py-2 rounded-md transition-colors',
            viewMode === 'grid' ? 'bg-white dark:bg-slate-700 shadow text-primary-600 dark:text-primary-400' : 'text-gray-600 dark:text-slate-400 hover:text-gray-900 dark:hover:text-slate-200'
          ]"
        >
          <span class="text-lg">▦</span> Grid
        </button>
        <button
          @click="viewMode = 'list'"
          :class="[
            'px-4 py-2 rounded-md transition-colors',
            viewMode === 'list' ? 'bg-white dark:bg-slate-700 shadow text-primary-600 dark:text-primary-400' : 'text-gray-600 dark:text-slate-400 hover:text-gray-900 dark:hover:text-slate-200'
          ]"
        >
          <span class="text-lg">☰</span> List
        </button>
      </div>

      <!-- Sort Options -->
      <div class="flex items-center gap-2">
        <label class="text-sm text-gray-600 dark:text-slate-400">Sort by:</label>
        <select v-model="sortBy" class="px-3 py-2 border border-gray-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-gray-900 dark:text-slate-100">
          <option value="recent">Recently Added</option>
          <option value="price-high">Price: High to Low</option>
          <option value="price-low">Price: Low to High</option>
          <option value="rent-high">Rent: High to Low</option>
          <option value="rent-low">Rent: Low to High</option>
          <option value="sqft-high">Sq Ft: High to Low</option>
        </select>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="text-center py-12">
      <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 dark:border-primary-400"></div>
      <p class="text-gray-500 dark:text-slate-400 mt-4">Loading properties...</p>
    </div>

    <!-- No Properties -->
    <div v-else-if="properties.length === 0" class="text-center py-12 card">
      <div class="text-6xl mb-4">🏠</div>
      <p class="text-gray-500 dark:text-slate-400 mb-4">No properties found</p>
      <button @click="showCreateModal = true" class="btn btn-primary">
        Add Your First Property
      </button>
    </div>

    <!-- No Filtered Results -->
    <div v-else-if="filteredProperties.length === 0" class="text-center py-12 card">
      <div class="text-6xl mb-4">🔍</div>
      <p class="text-gray-500 dark:text-slate-400 mb-4">No properties match your filters</p>
      <button @click="clearFilters" class="btn btn-secondary">
        Clear Filters
      </button>
    </div>

    <!-- Grid View -->
    <div v-else-if="viewMode === 'grid'" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="property in sortedProperties"
        :key="property.id"
        class="card hover:shadow-xl transition-all duration-200 group"
      >
        <!-- Property Image Placeholder -->
        <div class="relative h-48 bg-gradient-to-br from-primary-100 to-primary-200 rounded-lg mb-4 overflow-hidden">
          <div class="absolute inset-0 flex items-center justify-center text-6xl opacity-30">
            🏠
          </div>
          <div class="absolute top-2 right-2">
            <span class="px-3 py-1 text-xs font-semibold rounded-full shadow-lg" :class="getStatusClass(property.status)">
              {{ property.status }}
            </span>
          </div>
          <div class="absolute bottom-2 left-2">
            <span class="px-3 py-1 text-xs font-bold bg-white/90 backdrop-blur rounded-full shadow">
              {{ property.property_type }}
            </span>
          </div>
        </div>

        <!-- Property Info -->
        <div class="space-y-3">
          <h3 class="text-lg font-bold text-gray-900 dark:text-slate-100 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
            {{ property.title }}
          </h3>
          
          <div class="space-y-1 text-sm text-gray-600 dark:text-slate-400">
            <p class="flex items-center gap-1">
              <span>📍</span>
              {{ property.address }}, {{ property.city }}, {{ property.state }}
            </p>
            <div class="flex items-center gap-4">
              <p v-if="property.bedrooms" class="flex items-center gap-1">
                <span>🛏️</span> {{ property.bedrooms }} bed
              </p>
              <p v-if="property.bathrooms" class="flex items-center gap-1">
                <span>🚿</span> {{ property.bathrooms }} bath
              </p>
              <p v-if="property.square_feet" class="flex items-center gap-1">
                <span>📐</span> {{ property.square_feet.toLocaleString() }} ft²
              </p>
            </div>
          </div>

          <!-- Price and Action -->
          <div class="pt-3 border-t border-gray-200 dark:border-slate-700 flex justify-between items-center gap-2">
            <div class="flex-1">
              <p class="text-xs text-gray-500 dark:text-slate-400">Monthly Rent</p>
              <p class="text-xl font-bold text-primary-600 dark:text-primary-400">
                ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
              </p>
            </div>
            <div class="flex gap-2">
              <button @click="openEditModal(property)" class="p-2 text-blue-600 hover:bg-blue-50 rounded transition-colors" title="Edit">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button @click="handleDelete(property)" class="p-2 text-red-600 hover:bg-red-50 rounded transition-colors" title="Delete">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
              <router-link
                :to="`/properties/${property.id}`"
                class="btn btn-primary text-sm"
              >
                View →
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- List View -->
    <div v-else class="card overflow-hidden">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-slate-700">
          <thead class="bg-gray-50 dark:bg-slate-800">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Property</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Location</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Type</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Beds/Baths</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Sq Ft</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Status</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Rent</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-slate-400 uppercase tracking-wider">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white dark:bg-slate-900 divide-y divide-gray-200 dark:divide-slate-700">
            <tr v-for="property in sortedProperties" :key="property.id" class="hover:bg-gray-50 dark:hover:bg-slate-800 transition-colors">
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="font-medium text-gray-900 dark:text-slate-100">{{ property.title }}</div>
                <div class="text-sm text-gray-500 dark:text-slate-400">{{ property.address }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">
                {{ property.city }}, {{ property.state }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">
                {{ property.property_type }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">
                {{ property.bedrooms || '-' }} / {{ property.bathrooms || '-' }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-slate-300">
                {{ property.square_feet?.toLocaleString() || '-' }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full" :class="getStatusClass(property.status)">
                  {{ property.status }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm font-semibold text-primary-600 dark:text-primary-400">
                ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm">
                <div class="flex items-center gap-2">
                  <button @click="openEditModal(property)" class="text-blue-600 hover:text-blue-800 font-medium" title="Edit">
                    ✏️
                  </button>
                  <button @click="handleDelete(property)" class="text-red-600 hover:text-red-800 font-medium" title="Delete">
                    🗑️
                  </button>
                  <router-link :to="`/properties/${property.id}`" class="text-primary-600 hover:text-primary-800 font-medium">
                    View Details →
                  </router-link>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create/Edit Property Modal -->
    <PropertyForm
      v-if="showCreateModal || showEditModal"
      :property="editingProperty"
      :is-edit="showEditModal"
      @close="closeModal"
      @submit="handleSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import PropertyForm from '@/components/PropertyForm.vue'
import type { Property, CreateProperty } from '@/types'

const propertyStore = usePropertyStore()

// Reactive state
const showCreateModal = ref(false)
const showEditModal = ref(false)
const editingProperty = ref<Property | null>(null)
const showFilters = ref(false)
const viewMode = ref<'grid' | 'list'>('grid')
const searchQuery = ref('')
const sortBy = ref('recent')

const filters = ref({
  status: '',
  propertyType: '',
  bedrooms: '',
  city: ''
})

// Computed properties
const properties = computed(() => propertyStore.properties)
const loading = computed(() => propertyStore.loading)

// Get unique cities for filter dropdown
const uniqueCities = computed(() => {
  const cities = properties.value.map(p => p.city)
  return [...new Set(cities)].sort()
})

// Filter properties based on search and filters
const filteredProperties = computed(() => {
  let result = properties.value

  // Text search
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(p =>
      p.title.toLowerCase().includes(query) ||
      p.address.toLowerCase().includes(query) ||
      p.city.toLowerCase().includes(query)
    )
  }

  // Status filter
  if (filters.value.status) {
    result = result.filter(p => p.status === filters.value.status)
  }

  // Property type filter
  if (filters.value.propertyType) {
    result = result.filter(p => p.property_type === filters.value.propertyType)
  }

  // Bedrooms filter
  if (filters.value.bedrooms) {
    const minBedrooms = parseInt(filters.value.bedrooms)
    result = result.filter(p => (p.bedrooms || 0) >= minBedrooms)
  }

  // City filter
  if (filters.value.city) {
    result = result.filter(p => p.city === filters.value.city)
  }

  return result
})

// Sort properties
const sortedProperties = computed(() => {
  const sorted = [...filteredProperties.value]

  switch (sortBy.value) {
    case 'price-high':
      return sorted.sort((a, b) => (b.purchase_price || 0) - (a.purchase_price || 0))
    case 'price-low':
      return sorted.sort((a, b) => (a.purchase_price || 0) - (b.purchase_price || 0))
    case 'rent-high':
      return sorted.sort((a, b) => (b.monthly_rent || 0) - (a.monthly_rent || 0))
    case 'rent-low':
      return sorted.sort((a, b) => (a.monthly_rent || 0) - (b.monthly_rent || 0))
    case 'sqft-high':
      return sorted.sort((a, b) => (b.square_feet || 0) - (a.square_feet || 0))
    case 'recent':
    default:
      return sorted.sort((a, b) => b.id - a.id)
  }
})

// Count active filters
const activeFilterCount = computed(() => {
  let count = 0
  if (filters.value.status) count++
  if (filters.value.propertyType) count++
  if (filters.value.bedrooms) count++
  if (filters.value.city) count++
  return count
})

// Analytics computed properties
const averagePrice = computed(() => {
  if (filteredProperties.value.length === 0) return 0
  const total = filteredProperties.value.reduce((sum, p) => sum + (p.purchase_price || 0), 0)
  return Math.round(total / filteredProperties.value.length)
})

const averageRent = computed(() => {
  if (filteredProperties.value.length === 0) return 0
  const total = filteredProperties.value.reduce((sum, p) => sum + (p.monthly_rent || 0), 0)
  return Math.round(total / filteredProperties.value.length)
})

const totalSquareFeet = computed(() => {
  return filteredProperties.value.reduce((sum, p) => sum + (p.square_feet || 0), 0)
})

// Lifecycle
onMounted(() => {
  propertyStore.fetchProperties()
})

// Methods
function clearFilters() {
  filters.value = {
    status: '',
    propertyType: '',
    bedrooms: '',
    city: ''
  }
  searchQuery.value = ''
}

function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US').format(num)
}

function getStatusClass(status: string): string {
  const statusClasses: Record<string, string> = {
    occupied: 'bg-green-100 text-green-800',
    vacant: 'bg-yellow-100 text-yellow-800',
    maintenance: 'bg-red-100 text-red-800'
  }
  return statusClasses[status] || 'bg-gray-100 text-gray-800'
}

function openEditModal(property: Property) {
  editingProperty.value = property
  showEditModal.value = true
}

function closeModal() {
  showCreateModal.value = false
  showEditModal.value = false
  editingProperty.value = null
}

async function handleSubmit(data: CreateProperty) {
  try {
    if (showEditModal.value && editingProperty.value) {
      await propertyStore.updateProperty(editingProperty.value.id, data)
    } else {
      await propertyStore.createProperty(data)
    }
    closeModal()
    await propertyStore.fetchProperties()
  } catch (error) {
    console.error('Failed to save property:', error)
  }
}

async function handleDelete(property: Property) {
  if (confirm(`Are you sure you want to delete "${property.title}"? This action cannot be undone.`)) {
    try {
      await propertyStore.deleteProperty(property.id)
      await propertyStore.fetchProperties()
    } catch (error) {
      console.error('Failed to delete property:', error)
      alert('Failed to delete property. Please try again.')
    }
  }
}
</script>
