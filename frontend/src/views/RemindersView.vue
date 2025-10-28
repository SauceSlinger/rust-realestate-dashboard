<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Reminders & Calendar</h1>
      <button @click="showAddForm = true" 
              class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
        Add Reminder
      </button>
    </div>

    <!-- Add/Edit Form Modal -->
    <div v-if="showAddForm" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4">
        <h2 class="text-2xl font-bold mb-4">{{ editingReminder ? 'Edit' : 'Add' }} Reminder</h2>
        <form @submit.prevent="saveReminder" class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-1">Title</label>
            <input v-model="formData.title" type="text" required
                   class="w-full border rounded px-3 py-2">
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Description</label>
            <textarea v-model="formData.description" rows="3"
                      class="w-full border rounded px-3 py-2"></textarea>
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Due Date</label>
            <input v-model="formData.due_date" type="date" required
                   class="w-full border rounded px-3 py-2">
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Type</label>
            <select v-model="formData.reminder_type" required
                    class="w-full border rounded px-3 py-2">
              <option value="maintenance">Maintenance</option>
              <option value="inspection">Inspection</option>
              <option value="rent-due">Rent Due</option>
              <option value="lease-renewal">Lease Renewal</option>
              <option value="tax">Tax</option>
              <option value="other">Other</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium mb-1">Property (Optional)</label>
            <select v-model="formData.property_id"
                    class="w-full border rounded px-3 py-2">
              <option :value="undefined">None</option>
              <option v-for="property in properties" :key="property.id" :value="property.id">
                {{ property.address }}
              </option>
            </select>
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

    <!-- Reminders List -->
    <div v-if="reminders.length === 0" class="text-center py-12 text-gray-500">
      No reminders yet. Click "Add Reminder" to create one.
    </div>
    <div v-else class="space-y-4">
      <div v-for="reminder in reminders" :key="reminder.id"
           class="bg-white rounded-lg shadow p-6">
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <div class="flex items-center space-x-3 mb-2">
              <input type="checkbox" 
                     :checked="reminder.completed"
                     @change="toggleComplete(reminder)"
                     class="w-5 h-5">
              <h3 class="text-xl font-bold" :class="{ 'line-through text-gray-400': reminder.completed }">
                {{ reminder.title }}
              </h3>
              <span class="px-3 py-1 rounded-full text-sm"
                    :class="getReminderTypeColor(reminder.reminder_type)">
                {{ reminder.reminder_type }}
              </span>
            </div>
            <p v-if="reminder.description" class="text-gray-600 mb-2">{{ reminder.description }}</p>
            <div class="flex items-center space-x-4 text-sm text-gray-500">
              <span>📅 {{ reminder.due_date }}</span>
              <span v-if="reminder.property_id" class="text-blue-600">
                Property ID: {{ reminder.property_id }}
              </span>
            </div>
          </div>
          <div class="flex space-x-2 ml-4">
            <button @click="editReminder(reminder)"
                    class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700">
              Edit
            </button>
            <button @click="deleteReminder(reminder.id!)"
                    class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700">
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
import { reminderApi, propertyApi, type Reminder, type Property } from '@/services/api'

const reminders = ref<Reminder[]>([])
const properties = ref<Property[]>([])
const showAddForm = ref(false)
const editingReminder = ref<Reminder | null>(null)

const defaultFormData = {
  title: '',
  description: '',
  due_date: new Date().toISOString().split('T')[0],
  reminder_type: 'maintenance',
  property_id: undefined as number | undefined
}

const formData = ref({ ...defaultFormData })

const getReminderTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    maintenance: 'bg-yellow-100 text-yellow-800',
    inspection: 'bg-blue-100 text-blue-800',
    'rent-due': 'bg-green-100 text-green-800',
    'lease-renewal': 'bg-purple-100 text-purple-800',
    tax: 'bg-red-100 text-red-800',
    other: 'bg-gray-100 text-gray-800'
  }
  return colors[type] || colors.other
}

const loadReminders = async () => {
  try {
    const response = await reminderApi.getAll()
    reminders.value = response.data
  } catch (error) {
    console.error('Error loading reminders:', error)
  }
}

const loadProperties = async () => {
  try {
    const response = await propertyApi.getAll()
    properties.value = response.data
  } catch (error) {
    console.error('Error loading properties:', error)
  }
}

const saveReminder = async () => {
  try {
    if (editingReminder.value) {
      await reminderApi.update(editingReminder.value.id!, formData.value)
    } else {
      const data = {
        title: formData.value.title,
        description: formData.value.description,
        due_date: formData.value.due_date!,
        reminder_type: formData.value.reminder_type,
        property_id: formData.value.property_id
      }
      await reminderApi.create(data)
    }
    await loadReminders()
    closeForm()
  } catch (error) {
    console.error('Error saving reminder:', error)
  }
}

const editReminder = (reminder: Reminder) => {
  editingReminder.value = reminder
  formData.value = {
    title: reminder.title,
    description: reminder.description || '',
    due_date: reminder.due_date,
    reminder_type: reminder.reminder_type,
    property_id: reminder.property_id
  }
  showAddForm.value = true
}

const toggleComplete = async (reminder: Reminder) => {
  try {
    await reminderApi.update(reminder.id!, { completed: !reminder.completed })
    await loadReminders()
  } catch (error) {
    console.error('Error updating reminder:', error)
  }
}

const deleteReminder = async (id: number) => {
  if (confirm('Are you sure you want to delete this reminder?')) {
    try {
      await reminderApi.delete(id)
      await loadReminders()
    } catch (error) {
      console.error('Error deleting reminder:', error)
    }
  }
}

const closeForm = () => {
  showAddForm.value = false
  editingReminder.value = null
  formData.value = { ...defaultFormData }
}

onMounted(() => {
  loadReminders()
  loadProperties()
})
</script>
