import { defineStore } from 'pinia'
import { ref } from 'vue'
import { propertyService } from '@/services/api'
import type { Property, CreateProperty } from '@/types'

export const usePropertyStore = defineStore('property', () => {
  const properties = ref<Property[]>([])
  const currentProperty = ref<Property | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchProperties() {
    loading.value = true
    error.value = null
    try {
      const response = await propertyService.getAll()
      properties.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch properties'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function fetchProperty(id: number) {
    loading.value = true
    error.value = null
    try {
      const response = await propertyService.getById(id)
      currentProperty.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch property'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createProperty(data: CreateProperty) {
    loading.value = true
    error.value = null
    try {
      const response = await propertyService.create(data)
      properties.value.unshift(response.data)
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to create property'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function updateProperty(id: number, data: Partial<CreateProperty>) {
    loading.value = true
    error.value = null
    try {
      const response = await propertyService.update(id, data)
      const index = properties.value.findIndex(p => p.id === id)
      if (index !== -1) {
        properties.value[index] = response.data
      }
      if (currentProperty.value?.id === id) {
        currentProperty.value = response.data
      }
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to update property'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteProperty(id: number) {
    loading.value = true
    error.value = null
    try {
      await propertyService.delete(id)
      properties.value = properties.value.filter(p => p.id !== id)
    } catch (e: any) {
      error.value = e.message || 'Failed to delete property'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    properties,
    currentProperty,
    loading,
    error,
    fetchProperties,
    fetchProperty,
    createProperty,
    updateProperty,
    deleteProperty
  }
})
