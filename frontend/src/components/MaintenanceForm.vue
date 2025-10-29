<template>
  <div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    @click.self="emit('close')"
  >
    <div class="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <h2 class="text-2xl font-bold mb-6">{{ isEdit ? 'Edit Maintenance Request' : 'Create Maintenance Request' }}</h2>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Title -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Title <span class="text-red-500">*</span>
          </label>
          <input
            v-model="formData.title"
            type="text"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="e.g., Fix leaking faucet, HVAC repair"
          />
        </div>

        <!-- Property -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Property <span class="text-red-500">*</span>
          </label>
          <select
            v-model="formData.property_id"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          >
            <option value="">Select property...</option>
            <option v-for="property in properties" :key="property.id" :value="property.id">
              {{ property.title }}
            </option>
          </select>
        </div>

        <!-- Priority and Status Row -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Priority -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Priority <span class="text-red-500">*</span>
            </label>
            <select
              v-model="formData.priority"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            >
              <option value="">Select priority...</option>
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="urgent">Urgent</option>
            </select>
          </div>

          <!-- Status -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Status <span class="text-red-500">*</span>
            </label>
            <select
              v-model="formData.status"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            >
              <option value="">Select status...</option>
              <option value="pending">Pending</option>
              <option value="in-progress">In Progress</option>
              <option value="completed">Completed</option>
            </select>
          </div>
        </div>

        <!-- Cost and Scheduled Date Row -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Cost -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Estimated Cost
            </label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="formData.cost"
                type="number"
                min="0"
                step="0.01"
                class="w-full pl-7 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                placeholder="0.00"
              />
            </div>
          </div>

          <!-- Scheduled Date -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Scheduled Date
            </label>
            <input
              v-model="formData.scheduled_date"
              type="date"
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            />
          </div>
        </div>

        <!-- Contractor -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Contractor
          </label>
          <input
            v-model="formData.contractor"
            type="text"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Contractor name or company"
          />
        </div>

        <!-- Description -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Description
          </label>
          <textarea
            v-model="formData.description"
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Detailed description of the issue..."
          ></textarea>
        </div>

        <!-- Notes -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Notes
          </label>
          <textarea
            v-model="formData.notes"
            rows="3"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Additional notes, updates, or special instructions..."
          ></textarea>
        </div>

        <!-- Error Message -->
        <div v-if="errorMessage" class="p-3 bg-red-50 border border-red-200 rounded-lg text-red-700 text-sm">
          {{ errorMessage }}
        </div>

        <!-- Action Buttons -->
        <div class="flex justify-end space-x-3 pt-4 border-t">
          <button
            type="button"
            @click="emit('close')"
            class="btn btn-secondary"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="btn btn-primary"
          >
            {{ isEdit ? 'Update Request' : 'Create Request' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import type { MaintenanceRecord } from '@/types'

interface Props {
  request?: MaintenanceRecord
  isEdit?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isEdit: false
})

const emit = defineEmits<{
  close: []
  submit: [data: Partial<MaintenanceRecord>]
}>()

const propertyStore = usePropertyStore()
const properties = ref(propertyStore.properties)
const errorMessage = ref('')

const formData = reactive<Partial<MaintenanceRecord>>({
  property_id: props.request?.property_id || undefined,
  title: props.request?.title || '',
  description: props.request?.description || '',
  priority: props.request?.priority || '',
  status: props.request?.status || 'pending',
  cost: props.request?.cost || undefined,
  scheduled_date: props.request?.scheduled_date || '',
  contractor: props.request?.contractor || '',
  notes: props.request?.notes || ''
})

onMounted(async () => {
  if (properties.value.length === 0) {
    await propertyStore.fetchProperties()
    properties.value = propertyStore.properties
  }
})

function handleSubmit() {
  errorMessage.value = ''

  // Basic validation
  if (!formData.property_id) {
    errorMessage.value = 'Please select a property'
    return
  }
  if (!formData.title) {
    errorMessage.value = 'Please enter a title'
    return
  }
  if (!formData.priority) {
    errorMessage.value = 'Please select a priority level'
    return
  }
  if (!formData.status) {
    errorMessage.value = 'Please select a status'
    return
  }

  emit('submit', formData)
}
</script>
