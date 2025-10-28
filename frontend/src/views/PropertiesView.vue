<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Properties</h1>
      <button @click="showAddForm = true" 
              class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
        Add Property
      </button>
    </div>

    <!-- Add/Edit Form Modal -->
    <div v-if="showAddForm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-screen overflow-y-auto">
        <h2 class="text-2xl font-bold mb-4">{{ editingProperty ? 'Edit' : 'Add' }} Property</h2>
        <form @submit.prevent="saveProperty" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium mb-1">Address</label>
              <input v-model="formData.address" type="text" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">City</label>
              <input v-model="formData.city" type="text" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">State</label>
              <input v-model="formData.state" type="text" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Zip Code</label>
              <input v-model="formData.zip_code" type="text" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Property Type</label>
              <select v-model="formData.property_type" required
                      class="w-full border rounded px-3 py-2">
                <option value="single-family">Single Family</option>
                <option value="multi-family">Multi Family</option>
                <option value="condo">Condo</option>
                <option value="townhouse">Townhouse</option>
                <option value="commercial">Commercial</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Status</label>
              <select v-model="formData.status" required
                      class="w-full border rounded px-3 py-2">
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="sold">Sold</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Bedrooms</label>
              <input v-model.number="formData.bedrooms" type="number" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Bathrooms</label>
              <input v-model.number="formData.bathrooms" type="number" step="0.5" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Square Feet</label>
              <input v-model.number="formData.square_feet" type="number" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Purchase Price</label>
              <input v-model.number="formData.purchase_price" type="number" step="0.01" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Current Value</label>
              <input v-model.number="formData.current_value" type="number" step="0.01" required
                     class="w-full border rounded px-3 py-2">
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Notes</label>
            <textarea v-model="formData.notes" rows="3"
                      class="w-full border rounded px-3 py-2"></textarea>
          </div>
          <div class="flex justify-end space-x-2">
            <button type="button" @click="closeForm"
                    class="px-4 py-2 border rounded hover:bg-gray-100">
              Cancel
            </button>
            <button type="submit"
                    class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
              Save
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Properties Grid -->
    <div v-if="properties.length === 0" class="text-center py-12 text-gray-500">
      No properties yet. Click "Add Property" to get started.
    </div>
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="property in properties" :key="property.id"
           class="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div>
              <h3 class="text-xl font-bold">{{ property.address }}</h3>
              <p class="text-gray-600">{{ property.city }}, {{ property.state }} {{ property.zip_code }}</p>
            </div>
            <span class="px-3 py-1 rounded-full text-sm"
                  :class="property.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'">
              {{ property.status }}
            </span>
          </div>
          
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600">Type:</span>
              <span class="font-medium">{{ property.property_type }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Beds/Baths:</span>
              <span class="font-medium">{{ property.bedrooms }} / {{ property.bathrooms }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Sq Ft:</span>
              <span class="font-medium">{{ property.square_feet.toLocaleString() }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Purchase Price:</span>
              <span class="font-medium">${{ property.purchase_price.toLocaleString() }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Current Value:</span>
              <span class="font-medium text-green-600">${{ property.current_value.toLocaleString() }}</span>
            </div>
          </div>

          <div class="mt-4 flex space-x-2">
            <button @click="editProperty(property)"
                    class="flex-1 px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
              Edit
            </button>
            <button @click="deleteProperty(property.id!)"
                    class="flex-1 px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700">
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { propertyApi, type Property } from '@/services/api'

const properties = ref<Property[]>([])
const showAddForm = ref(false)
const editingProperty = ref<Property | null>(null)

const defaultFormData = {
  address: '',
  city: '',
  state: '',
  zip_code: '',
  property_type: 'single-family',
  bedrooms: 0,
  bathrooms: 0,
  square_feet: 0,
  purchase_price: 0,
  current_value: 0,
  status: 'active',
  notes: ''
}

const formData = ref({ ...defaultFormData })

const loadProperties = async () => {
  try {
    const response = await propertyApi.getAll()
    properties.value = response.data
  } catch (error) {
    console.error('Error loading properties:', error)
  }
}

const saveProperty = async () => {
  try {
    if (editingProperty.value) {
      await propertyApi.update(editingProperty.value.id!, formData.value)
    } else {
      await propertyApi.create(formData.value)
    }
    await loadProperties()
    closeForm()
  } catch (error) {
    console.error('Error saving property:', error)
  }
}

const editProperty = (property: Property) => {
  editingProperty.value = property
  formData.value = {
    address: property.address,
    city: property.city,
    state: property.state,
    zip_code: property.zip_code,
    property_type: property.property_type,
    bedrooms: property.bedrooms,
    bathrooms: property.bathrooms,
    square_feet: property.square_feet,
    purchase_price: property.purchase_price,
    current_value: property.current_value,
    status: property.status,
    notes: property.notes || ''
  }
  showAddForm.value = true
}

const deleteProperty = async (id: number) => {
  if (confirm('Are you sure you want to delete this property?')) {
    try {
      await propertyApi.delete(id)
      await loadProperties()
    } catch (error) {
      console.error('Error deleting property:', error)
    }
  }
}

const closeForm = () => {
  showAddForm.value = false
  editingProperty.value = null
  formData.value = { ...defaultFormData }
}

onMounted(loadProperties)
</script>
