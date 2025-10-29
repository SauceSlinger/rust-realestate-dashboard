<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4 overflow-y-auto">
    <div class="bg-white rounded-lg p-6 max-w-2xl w-full my-8">
      <h2 class="text-2xl font-bold mb-4">{{ isEdit ? 'Edit Property' : 'Add New Property' }}</h2>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- Basic Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="md:col-span-2">
            <label class="block text-sm font-medium text-gray-700 mb-1">Property Title *</label>
            <input
              v-model="form.title"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., Sunset Villa"
            />
          </div>

          <div class="md:col-span-2">
            <label class="block text-sm font-medium text-gray-700 mb-1">Address *</label>
            <input
              v-model="form.address"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., 123 Main Street"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">City *</label>
            <input
              v-model="form.city"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., San Francisco"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">State *</label>
            <input
              v-model="form.state"
              type="text"
              required
              maxlength="2"
              class="input w-full"
              placeholder="e.g., CA"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">ZIP Code *</label>
            <input
              v-model="form.zip_code"
              type="text"
              required
              class="input w-full"
              placeholder="e.g., 94102"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Property Type *</label>
            <select v-model="form.property_type" required class="input w-full">
              <option value="">Select type...</option>
              <option value="single_family">Single Family</option>
              <option value="multi_family">Multi Family</option>
              <option value="condo">Condo</option>
              <option value="apartment">Apartment</option>
              <option value="townhouse">Townhouse</option>
              <option value="commercial">Commercial</option>
            </select>
          </div>
        </div>

        <!-- Property Details -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Bedrooms</label>
            <input
              v-model.number="form.bedrooms"
              type="number"
              min="0"
              class="input w-full"
              placeholder="e.g., 3"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Bathrooms</label>
            <input
              v-model.number="form.bathrooms"
              type="number"
              min="0"
              step="0.5"
              class="input w-full"
              placeholder="e.g., 2.5"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Square Feet</label>
            <input
              v-model.number="form.square_feet"
              type="number"
              min="0"
              class="input w-full"
              placeholder="e.g., 1500"
            />
          </div>
        </div>

        <!-- Financial Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Purchase Price</label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="form.purchase_price"
                type="number"
                min="0"
                class="input w-full pl-7"
                placeholder="e.g., 450000"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Current Value</label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="form.current_value"
                type="number"
                min="0"
                class="input w-full pl-7"
                placeholder="e.g., 500000"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Monthly Rent</label>
            <div class="relative">
              <span class="absolute left-3 top-2 text-gray-500">$</span>
              <input
                v-model.number="form.monthly_rent"
                type="number"
                min="0"
                class="input w-full pl-7"
                placeholder="e.g., 2500"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Status *</label>
            <select v-model="form.status" required class="input w-full">
              <option value="occupied">Occupied</option>
              <option value="vacant">Vacant</option>
              <option value="maintenance">Maintenance</option>
            </select>
          </div>
        </div>

        <!-- Notes -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Notes</label>
          <textarea
            v-model="form.notes"
            rows="3"
            class="input w-full"
            placeholder="Additional property information..."
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
            {{ submitting ? 'Saving...' : (isEdit ? 'Update Property' : 'Create Property') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import type { Property, CreateProperty } from '@/types'

interface Props {
  property?: Property | null
  isEdit?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  property: null,
  isEdit: false
})

const emit = defineEmits<{
  close: []
  submit: [data: CreateProperty]
}>()

const submitting = ref(false)
const errorMessage = ref('')

const form = ref<CreateProperty>({
  title: '',
  address: '',
  city: '',
  state: '',
  zip_code: '',
  property_type: '',
  bedrooms: undefined,
  bathrooms: undefined,
  square_feet: undefined,
  purchase_price: undefined,
  current_value: undefined,
  monthly_rent: undefined,
  status: 'vacant',
  notes: ''
})

// Populate form if editing
onMounted(() => {
  if (props.isEdit && props.property) {
    form.value = {
      title: props.property.title,
      address: props.property.address,
      city: props.property.city,
      state: props.property.state,
      zip_code: props.property.zip_code,
      property_type: props.property.property_type,
      bedrooms: props.property.bedrooms,
      bathrooms: props.property.bathrooms,
      square_feet: props.property.square_feet,
      purchase_price: props.property.purchase_price,
      current_value: props.property.current_value,
      monthly_rent: props.property.monthly_rent,
      status: props.property.status,
      notes: props.property.notes || ''
    }
  }
})

async function handleSubmit() {
  submitting.value = true
  errorMessage.value = ''
  
  try {
    emit('submit', form.value)
  } catch (error: any) {
    errorMessage.value = error.message || 'Failed to save property'
  } finally {
    submitting.value = false
  }
}
</script>
