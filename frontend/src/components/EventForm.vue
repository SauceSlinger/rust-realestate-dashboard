<template>
  <div
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    @click.self="emit('close')"
  >
    <div class="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <h2 class="text-2xl font-bold mb-6">{{ isEdit ? 'Edit Event' : 'Create New Event' }}</h2>

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
            placeholder="e.g., Property Showing, Inspection"
          />
        </div>

        <!-- Event Type -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Event Type <span class="text-red-500">*</span>
          </label>
          <select
            v-model="formData.event_type"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          >
            <option value="">Select type...</option>
            <option value="showing">Showing</option>
            <option value="inspection">Inspection</option>
            <option value="maintenance">Maintenance</option>
            <option value="meeting">Meeting</option>
            <option value="lease-signing">Lease Signing</option>
            <option value="other">Other</option>
          </select>
        </div>

        <!-- Property -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Property
          </label>
          <select
            v-model="formData.property_id"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          >
            <option :value="null">No property (general event)</option>
            <option v-for="property in properties" :key="property.id" :value="property.id">
              {{ property.title }}
            </option>
          </select>
        </div>

        <!-- Date and Time Row -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Start Time -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              Start Date & Time <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.start_time"
              type="datetime-local"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            />
          </div>

          <!-- End Time -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">
              End Date & Time <span class="text-red-500">*</span>
            </label>
            <input
              v-model="formData.end_time"
              type="datetime-local"
              required
              class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            />
          </div>
        </div>

        <!-- Reminder -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Reminder
          </label>
          <select
            v-model="formData.reminder_minutes"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
          >
            <option :value="null">No reminder</option>
            <option :value="15">15 minutes before</option>
            <option :value="30">30 minutes before</option>
            <option :value="60">1 hour before</option>
            <option :value="1440">1 day before</option>
            <option :value="2880">2 days before</option>
          </select>
        </div>

        <!-- Description -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Description
          </label>
          <textarea
            v-model="formData.description"
            rows="4"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            placeholder="Event details, notes, attendees..."
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
            {{ isEdit ? 'Update Event' : 'Create Event' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { usePropertyStore } from '@/stores/propertyStore'
import type { CalendarEvent } from '@/types'

interface Props {
  event?: CalendarEvent
  isEdit?: boolean
  defaultStart?: string
  defaultEnd?: string
}

const props = withDefaults(defineProps<Props>(), {
  isEdit: false
})

const emit = defineEmits<{
  close: []
  submit: [data: Partial<CalendarEvent>]
}>()

const propertyStore = usePropertyStore()
const properties = ref(propertyStore.properties)
const errorMessage = ref('')

const formData = reactive<Partial<CalendarEvent>>({
  title: props.event?.title || '',
  description: props.event?.description || '',
  event_type: props.event?.event_type || '',
  property_id: props.event?.property_id || undefined,
  start_time: props.event?.start_time || props.defaultStart || '',
  end_time: props.event?.end_time || props.defaultEnd || '',
  reminder_minutes: props.event?.reminder_minutes || undefined,
  completed: props.event?.completed || false
})

onMounted(async () => {
  if (properties.value.length === 0) {
    await propertyStore.fetchProperties()
    properties.value = propertyStore.properties
  }
})

function formatDateTimeLocal(isoString: string): string {
  // Convert ISO string to datetime-local format (YYYY-MM-DDTHH:MM)
  const date = new Date(isoString)
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const hours = String(date.getHours()).padStart(2, '0')
  const minutes = String(date.getMinutes()).padStart(2, '0')
  return `${year}-${month}-${day}T${hours}:${minutes}`
}

function handleSubmit() {
  errorMessage.value = ''

  // Validate end time is after start time
  if (formData.start_time && formData.end_time) {
    const start = new Date(formData.start_time)
    const end = new Date(formData.end_time)
    if (end <= start) {
      errorMessage.value = 'End time must be after start time'
      return
    }
  }

  // Convert datetime-local to ISO strings
  const submitData = {
    ...formData,
    start_time: formData.start_time ? new Date(formData.start_time).toISOString() : '',
    end_time: formData.end_time ? new Date(formData.end_time).toISOString() : ''
  }

  emit('submit', submitData)
}
</script>
