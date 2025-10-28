<template>
  <div>
    <h1 class="text-3xl font-bold mb-6">Dashboard</h1>
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <div class="bg-white p-6 rounded-lg shadow">
        <h3 class="text-gray-500 text-sm font-medium">Total Properties</h3>
        <p class="text-3xl font-bold text-blue-600">{{ properties.length }}</p>
      </div>
      
      <div class="bg-white p-6 rounded-lg shadow">
        <h3 class="text-gray-500 text-sm font-medium">Active Tenants</h3>
        <p class="text-3xl font-bold text-green-600">{{ activeTenants }}</p>
      </div>
      
      <div class="bg-white p-6 rounded-lg shadow">
        <h3 class="text-gray-500 text-sm font-medium">Pending Reminders</h3>
        <p class="text-3xl font-bold text-orange-600">{{ pendingReminders }}</p>
      </div>
      
      <div class="bg-white p-6 rounded-lg shadow">
        <h3 class="text-gray-500 text-sm font-medium">Total Value</h3>
        <p class="text-3xl font-bold text-purple-600">${{ totalValue.toLocaleString() }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="bg-white p-6 rounded-lg shadow">
        <h2 class="text-xl font-bold mb-4">Recent Properties</h2>
        <div v-if="properties.length === 0" class="text-gray-500">No properties yet</div>
        <div v-else class="space-y-2">
          <div v-for="property in properties.slice(0, 5)" :key="property.id" 
               class="flex justify-between items-center py-2 border-b">
            <div>
              <p class="font-medium">{{ property.address }}</p>
              <p class="text-sm text-gray-500">{{ property.city }}, {{ property.state }}</p>
            </div>
            <span class="px-3 py-1 rounded-full text-sm"
                  :class="property.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'">
              {{ property.status }}
            </span>
          </div>
        </div>
      </div>

      <div class="bg-white p-6 rounded-lg shadow">
        <h2 class="text-xl font-bold mb-4">Upcoming Reminders</h2>
        <div v-if="reminders.length === 0" class="text-gray-500">No reminders</div>
        <div v-else class="space-y-2">
          <div v-for="reminder in reminders.slice(0, 5)" :key="reminder.id" 
               class="flex justify-between items-center py-2 border-b">
            <div>
              <p class="font-medium">{{ reminder.title }}</p>
              <p class="text-sm text-gray-500">{{ reminder.due_date }}</p>
            </div>
            <span class="px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800">
              {{ reminder.reminder_type }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { propertyApi, reminderApi, tenantApi, type Property, type Reminder, type Tenant } from '@/services/api'

const properties = ref<Property[]>([])
const reminders = ref<Reminder[]>([])
const tenants = ref<Tenant[]>([])

const activeTenants = computed(() => 
  tenants.value.filter(t => t.status === 'active').length
)

const pendingReminders = computed(() => 
  reminders.value.filter(r => !r.completed).length
)

const totalValue = computed(() => 
  properties.value.reduce((sum, p) => sum + p.current_value, 0)
)

onMounted(async () => {
  try {
    const [propsRes, remindersRes, tenantsRes] = await Promise.all([
      propertyApi.getAll(),
      reminderApi.getAll(),
      tenantApi.getAll()
    ])
    properties.value = propsRes.data
    reminders.value = remindersRes.data
    tenants.value = tenantsRes.data
  } catch (error) {
    console.error('Error loading dashboard data:', error)
  }
})
</script>
