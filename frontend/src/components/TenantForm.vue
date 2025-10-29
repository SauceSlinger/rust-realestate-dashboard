<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4 overflow-y-auto">
    <div class="bg-white rounded-lg p-6 max-w-2xl w-full my-8">
      <h2 class="text-2xl font-bold mb-4">{{ isEdit ? 'Edit Tenant' : 'Add New Tenant' }}</h2>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Personal Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">First Name *</label>
            <input
              v-model="form.first_name"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., John"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Last Name *</label>
            <input
              v-model="form.last_name"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., Smith"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
            <input
              v-model="form.email"
              type="email"
              class="input w-full"
              placeholder="e.g., john@example.com"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Phone</label>
            <input
              v-model="form.phone"
              type="tel"
              class="input w-full"
              placeholder="e.g., (555) 123-4567"
            />
          </div>
        </div>

        <!-- Property Assignment -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Property *</label>
          <select v-model.number="form.property_id" required class="input w-full">
            <option value="">Select property...</option>
            <option v-for="property in properties" :key="property.id" :value="property.id">
              {{ property.title }} - {{ property.address }}
            </option>
          </select>
        </div>

        <!-- Lease Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Lease Start *</label>
            <input
              v-model="form.lease_start"
              type="date"
              required
              class="input w-full"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Lease End *</label>
            <input
              v-model="form.lease_end"
              type="date"
              required
              class="input w-full"
            />
          </div>
        </div>

        <!-- Financial Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Monthly Rent *</label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="form.monthly_rent"
                type="number"
                min="0"
                required
                class="input w-full pl-7"
                placeholder="e.g., 2500"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Security Deposit</label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="form.deposit_amount"
                type="number"
                min="0"
                class="input w-full pl-7"
                placeholder="e.g., 2500"
              />
            </div>
          </div>
        </div>

        <!-- Status -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Status *</label>
          <select v-model="form.status" required class="input w-full">
            <option value="active">Active</option>
            <option value="pending">Pending</option>
            <option value="past">Past</option>
          </select>
        </div>

        <!-- Notes -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Notes</label>
          <textarea
            v-model="form.notes"
            rows="3"
            class="input w-full"
            placeholder="Additional tenant information..."
          ></textarea>
        </div>

        <!-- Error Message -->
        <div v-if="errorMessage" class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
          {{ errorMessage }}
        </div>

        <!-- Actions -->
        <div class="flex justify-end space-x-2 pt-4 border-t">
          <button type="button" @click="$emit('close')" class="btn btn-secondary" :disabled="submitting">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? 'Saving...' : (isEdit ? 'Update Tenant' : 'Add Tenant') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import type { Tenant, Property } from '@/types'

interface Props {
  tenant?: Tenant | null
  isEdit?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  tenant: null,
  isEdit: false
})

const emit = defineEmits<{
  close: []
  submit: [data: Partial<Tenant>]
}>()

const propertyStore = usePropertyStore()
const properties = ref<Property[]>([])
const submitting = ref(false)
const errorMessage = ref('')

const form = ref<Partial<Tenant>>({
  property_id: 0,
  first_name: '',
  last_name: '',
  email: '',
  phone: '',
  lease_start: '',
  lease_end: '',
  monthly_rent: 0,
  deposit_amount: 0,
  status: 'active',
  notes: ''
})

onMounted(async () => {
  // Load properties for dropdown
  await propertyStore.fetchProperties()
  properties.value = propertyStore.properties

  // Populate form if editing
  if (props.isEdit && props.tenant) {
    form.value = {
      property_id: props.tenant.property_id,
      first_name: props.tenant.first_name,
      last_name: props.tenant.last_name,
      email: props.tenant.email,
      phone: props.tenant.phone,
      lease_start: props.tenant.lease_start,
      lease_end: props.tenant.lease_end,
      monthly_rent: props.tenant.monthly_rent,
      deposit_amount: props.tenant.deposit_amount,
      status: props.tenant.status,
      notes: props.tenant.notes || ''
    }
  }
})

async function handleSubmit() {
  submitting.value = true
  errorMessage.value = ''
  
  try {
    emit('submit', form.value)
  } catch (error: any) {
    errorMessage.value = error.message || 'Failed to save tenant'
  } finally {
    submitting.value = false
  }
}
</script>
