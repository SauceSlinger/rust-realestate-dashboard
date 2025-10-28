<template>
  <div class="space-y-6">
    <h1 class="text-3xl font-bold text-gray-900">Property Details</h1>
    <div v-if="loading" class="text-center py-8">
      <p class="text-gray-500">Loading...</p>
    </div>
    <div v-else-if="property" class="card">
      <h2 class="text-2xl font-bold">{{ property.title }}</h2>
      <p class="text-gray-600 mt-2">{{ property.address }}, {{ property.city }}, {{ property.state }} {{ property.zip_code }}</p>
      <div class="mt-4">
        <p><strong>Type:</strong> {{ property.property_type }}</p>
        <p><strong>Status:</strong> {{ property.status }}</p>
        <p v-if="property.monthly_rent"><strong>Monthly Rent:</strong> ${{ property.monthly_rent.toLocaleString() }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { usePropertyStore } from '@/stores/propertyStore'

const route = useRoute()
const propertyStore = usePropertyStore()

const property = computed(() => propertyStore.currentProperty)
const loading = computed(() => propertyStore.loading)

onMounted(() => {
  const id = Number(route.params.id)
  propertyStore.fetchProperty(id)
})
</script>
