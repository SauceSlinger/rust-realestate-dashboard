<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Tenants</h1>
      <button @click="showAddForm = true" 
              class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
        Add Tenant
      </button>
    </div>

    <!-- Add/Edit Form Modal -->
    <div v-if="showAddForm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-screen overflow-y-auto">
        <h2 class="text-2xl font-bold mb-4">{{ editingTenant ? 'Edit' : 'Add' }} Tenant</h2>
        <form @submit.prevent="saveTenant" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium mb-1">Property</label>
              <select v-model.number="formData.property_id" required
                      class="w-full border rounded px-3 py-2">
                <option v-for="property in properties" :key="property.id" :value="property.id">
                  {{ property.address }}
                </option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Name</label>
              <input v-model="formData.name" type="text" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Email</label>
              <input v-model="formData.email" type="email" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Phone</label>
              <input v-model="formData.phone" type="tel" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Lease Start</label>
              <input v-model="formData.lease_start" type="date" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Lease End</label>
              <input v-model="formData.lease_end" type="date" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Monthly Rent</label>
              <input v-model.number="formData.monthly_rent" type="number" step="0.01" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Deposit</label>
              <input v-model.number="formData.deposit" type="number" step="0.01" required
                     class="w-full border rounded px-3 py-2">
            </div>
            <div>
              <label class="block text-sm font-medium mb-1">Status</label>
              <select v-model="formData.status" required
                      class="w-full border rounded px-3 py-2">
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="pending">Pending</option>
              </select>
            </div>
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

    <!-- Tenants Grid -->
    <div v-if="tenants.length === 0" class="text-center py-12 text-gray-500">
      No tenants yet. Click "Add Tenant" to get started.
    </div>
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="tenant in tenants" :key="tenant.id"
           class="bg-white rounded-lg shadow-lg p-6">
        <div class="flex justify-between items-start mb-4">
          <h3 class="text-xl font-bold">{{ tenant.name }}</h3>
          <span class="px-3 py-1 rounded-full text-sm"
                :class="tenant.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'">
            {{ tenant.status }}
          </span>
        </div>
        
        <div class="space-y-2 text-sm">
          <div class="flex items-center space-x-2">
            <span class="text-gray-600">📧</span>
            <span>{{ tenant.email }}</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="text-gray-600">📱</span>
            <span>{{ tenant.phone }}</span>
          </div>
          <div class="flex items-center space-x-2">
            <span class="text-gray-600">🏠</span>
            <span>Property ID: {{ tenant.property_id }}</span>
          </div>
          <div class="flex justify-between border-t pt-2 mt-2">
            <span class="text-gray-600">Monthly Rent:</span>
            <span class="font-medium">${{ tenant.monthly_rent.toLocaleString() }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Deposit:</span>
            <span class="font-medium">${{ tenant.deposit.toLocaleString() }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Lease:</span>
            <span class="font-medium text-sm">{{ tenant.lease_start }} to {{ tenant.lease_end }}</span>
          </div>
        </div>

        <div class="mt-4 flex space-x-2">
          <button @click="editTenant(tenant)"
                  class="flex-1 px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">
            Edit
          </button>
          <button @click="deleteTenant(tenant.id!)"
                  class="flex-1 px-3 py-2 bg-red-600 text-white rounded hover:bg-red-700">
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { tenantApi, propertyApi, type Tenant, type Property } from '@/services/api'

const tenants = ref<Tenant[]>([])
const properties = ref<Property[]>([])
const showAddForm = ref(false)
const editingTenant = ref<Tenant | null>(null)

const defaultFormData = {
  property_id: 0,
  name: '',
  email: '',
  phone: '',
  lease_start: new Date().toISOString().split('T')[0],
  lease_end: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
  monthly_rent: 0,
  deposit: 0,
  status: 'active'
}

const formData = ref({ ...defaultFormData })

const loadTenants = async () => {
  try {
    const response = await tenantApi.getAll()
    tenants.value = response.data
  } catch (error) {
    console.error('Error loading tenants:', error)
  }
}

const loadProperties = async () => {
  try {
    const response = await propertyApi.getAll()
    properties.value = response.data
    if (properties.value.length > 0 && !formData.value.property_id) {
      formData.value.property_id = properties.value[0]?.id || 0
    }
  } catch (error) {
    console.error('Error loading properties:', error)
  }
}

const saveTenant = async () => {
  try {
    if (editingTenant.value) {
      await tenantApi.update(editingTenant.value.id!, formData.value)
    } else {
      const data = {
        property_id: formData.value.property_id,
        name: formData.value.name,
        email: formData.value.email,
        phone: formData.value.phone,
        lease_start: formData.value.lease_start!,
        lease_end: formData.value.lease_end!,
        monthly_rent: formData.value.monthly_rent,
        deposit: formData.value.deposit,
        status: formData.value.status
      }
      await tenantApi.create(data)
    }
    await loadTenants()
    closeForm()
  } catch (error) {
    console.error('Error saving tenant:', error)
  }
}

const editTenant = (tenant: Tenant) => {
  editingTenant.value = tenant
  formData.value = { ...tenant }
  showAddForm.value = true
}

const deleteTenant = async (id: number) => {
  if (confirm('Are you sure you want to delete this tenant?')) {
    try {
      await tenantApi.delete(id)
      await loadTenants()
    } catch (error) {
      console.error('Error deleting tenant:', error)
    }
  }
}

const closeForm = () => {
  showAddForm.value = false
  editingTenant.value = null
  formData.value = { ...defaultFormData }
}

onMounted(() => {
  loadTenants()
  loadProperties()
})
</script>
