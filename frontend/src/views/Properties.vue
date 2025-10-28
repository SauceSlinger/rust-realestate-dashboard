<template>
  <div class="space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-3xl font-bold text-gray-900">Properties</h1>
      <button @click="showCreateModal = true" class="btn btn-primary">
        + Add Property
      </button>
    </div>

    <div v-if="loading" class="text-center py-8">
      <p class="text-gray-500">Loading...</p>
    </div>

    <div v-else-if="properties.length === 0" class="text-center py-12 card">
      <p class="text-gray-500 mb-4">No properties found</p>
      <button @click="showCreateModal = true" class="btn btn-primary">
        Add Your First Property
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="property in properties" :key="property.id" class="card hover:shadow-lg transition-shadow duration-200">
        <div class="flex justify-between items-start mb-4">
          <h3 class="text-lg font-bold text-gray-900">{{ property.title }}</h3>
          <span class="px-2 py-1 text-xs font-semibold rounded-full" :class="getStatusClass(property.status)">
            {{ property.status }}
          </span>
        </div>
        
        <div class="space-y-2 text-sm text-gray-600">
          <p>📍 {{ property.address }}, {{ property.city }}, {{ property.state }}</p>
          <p>🏢 {{ property.property_type }}</p>
          <p v-if="property.bedrooms">🛏️ {{ property.bedrooms }} bed, {{ property.bathrooms }} bath</p>
          <p v-if="property.square_feet">📐 {{ property.square_feet.toLocaleString() }} sq ft</p>
        </div>

        <div class="mt-4 pt-4 border-t border-gray-200">
          <div class="flex justify-between items-center">
            <div>
              <p class="text-xs text-gray-500">Monthly Rent</p>
              <p class="text-lg font-bold text-primary-600">
                ${{ property.monthly_rent?.toLocaleString() || 'N/A' }}
              </p>
            </div>
            <router-link :to="`/properties/${property.id}`" class="btn btn-secondary text-sm">
              View Details
            </router-link>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Property Modal (simplified) -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h2 class="text-2xl font-bold mb-4">Add New Property</h2>
        <p class="text-gray-600 mb-4">Create property form would go here</p>
        <div class="flex justify-end space-x-2">
          <button @click="showCreateModal = false" class="btn btn-secondary">Cancel</button>
          <button @click="handleCreate" class="btn btn-primary">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'

const propertyStore = usePropertyStore()
const showCreateModal = ref(false)

const properties = computed(() => propertyStore.properties)
const loading = computed(() => propertyStore.loading)

onMounted(() => {
  propertyStore.fetchProperties()
})

function getStatusClass(status: string): string {
  const statusClasses: Record<string, string> = {
    occupied: 'bg-green-100 text-green-800',
    vacant: 'bg-yellow-100 text-yellow-800',
    maintenance: 'bg-red-100 text-red-800'
  }
  return statusClasses[status] || 'bg-gray-100 text-gray-800'
}

function handleCreate() {
  // Form handling would go here
  showCreateModal.value = false
}
</script>
