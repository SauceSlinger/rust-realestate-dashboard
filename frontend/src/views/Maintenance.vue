<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Maintenance Requests</h1>
        <p class="text-gray-600 mt-1">Track and manage property maintenance</p>
      </div>
      <button @click="showAddRequestModal = true" class="btn btn-primary whitespace-nowrap">
        + New Request
      </button>
    </div>

    <!-- KPI Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
      <div class="card text-center">
        <p class="text-sm text-gray-600 mb-1">Open Requests</p>
        <p class="text-3xl font-bold text-orange-600">{{ openRequests }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 mb-1">In Progress</p>
        <p class="text-3xl font-bold text-blue-600">{{ inProgressRequests }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 mb-1">Completed</p>
        <p class="text-3xl font-bold text-green-600">{{ completedRequests }}</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 mb-1">Avg Time</p>
        <p class="text-3xl font-bold text-gray-900">{{ avgCompletionTime }}d</p>
      </div>
      <div class="card text-center">
        <p class="text-sm text-gray-600 mb-1">Total Cost</p>
        <p class="text-3xl font-bold text-gray-900">${{ totalCost.toLocaleString() }}</p>
      </div>
    </div>

    <!-- Filters -->
    <div class="card">
      <div class="flex flex-wrap gap-3">
        <select v-model="filterPriority" class="input">
          <option value="">All Priorities</option>
          <option value="urgent">Urgent</option>
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
        </select>
        <select v-model="filterProperty" class="input">
          <option value="">All Properties</option>
          <option v-for="property in propertyStore.properties" :key="property.id" :value="property.id">
            {{ property.title }}
          </option>
        </select>
      </div>
    </div>

    <!-- Kanban Board -->
    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4">
      <!-- New Column -->
      <div class="bg-gray-50 dark:bg-slate-800 rounded-lg p-4 min-w-[280px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-900 dark:text-slate-100 flex items-center gap-2">
            <span class="w-3 h-3 bg-red-500 rounded-full"></span>
            New
          </h3>
          <span class="bg-red-100 text-red-700 px-2 py-1 rounded-full text-sm font-medium">
            {{ newRequests.length }}
          </span>
        </div>
        <div class="space-y-3">
          <div
            v-for="request in newRequests"
            :key="request.id"
            @click="viewRequestDetails(request)"
            class="bg-white rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer border-l-4"
            :class="getPriorityBorderClass(request.priority)"
          >
            <div class="flex items-start justify-between mb-2">
              <h4 class="font-medium text-gray-900">{{ request.title }}</h4>
              <span :class="getPriorityBadgeClass(request.priority)">
                {{ request.priority }}
              </span>
            </div>
            <p class="text-sm text-gray-600 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500">
              <span>{{ getCategoryIcon(request.priority) }} {{ request.priority }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="mt-2 text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Assigned Column -->
      <div class="bg-gray-50 dark:bg-slate-800 rounded-lg p-4 min-w-[280px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-900 flex items-center gap-2">
            <span class="w-3 h-3 bg-yellow-500 rounded-full"></span>
            Assigned
          </h3>
          <span class="bg-yellow-100 text-yellow-700 px-2 py-1 rounded-full text-sm font-medium">
            {{ assignedRequests.length }}
          </span>
        </div>
        <div class="space-y-3">
          <div
            v-for="request in assignedRequests"
            :key="request.id"
            @click="viewRequestDetails(request)"
            class="bg-white rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer border-l-4"
            :class="getPriorityBorderClass(request.priority)"
          >
            <div class="flex items-start justify-between mb-2">
              <h4 class="font-medium text-gray-900">{{ request.title }}</h4>
              <span :class="getPriorityBadgeClass(request.priority)">
                {{ request.priority }}
              </span>
            </div>
            <p class="text-sm text-gray-600 dark:text-slate-400 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500 dark:text-slate-500 mb-2">
              <span>{{ getCategoryIcon(request.priority) }} {{ request.priority }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="text-xs text-gray-600 dark:text-slate-400">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.contractor" class="mt-2 text-xs text-blue-600 dark:text-blue-400 flex items-center gap-1">
              👤 {{ request.contractor }}
            </div>
          </div>
        </div>
      </div>

      <!-- In Progress Column -->
      <div class="bg-gray-50 dark:bg-slate-800 rounded-lg p-4 min-w-[280px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-900 flex items-center gap-2">
            <span class="w-3 h-3 bg-blue-500 rounded-full"></span>
            In Progress
          </h3>
          <span class="bg-blue-100 text-blue-700 px-2 py-1 rounded-full text-sm font-medium">
            {{ inProgressRequestsList.length }}
          </span>
        </div>
        <div class="space-y-3">
          <div
            v-for="request in inProgressRequestsList"
            :key="request.id"
            @click="viewRequestDetails(request)"
            class="bg-white rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer border-l-4"
            :class="getPriorityBorderClass(request.priority)"
          >
            <div class="flex items-start justify-between mb-2">
              <h4 class="font-medium text-gray-900">{{ request.title }}</h4>
              <span :class="getPriorityBadgeClass(request.priority)">
                {{ request.priority }}
              </span>
            </div>
            <p class="text-sm text-gray-600 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500 mb-2">
              <span>{{ getCategoryIcon(request.priority) }} {{ request.priority }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.contractor" class="mt-2 text-xs text-blue-600 dark:text-blue-400 flex items-center gap-1">
              👤 {{ request.contractor }}
            </div>
            <div v-if="request.cost" class="mt-2 text-xs text-gray-700 dark:text-slate-300 font-medium">
              Est. Cost: ${{ request.cost.toLocaleString() }}
            </div>
          </div>
        </div>
      </div>

      <!-- Completed Column -->
      <div class="bg-gray-50 dark:bg-slate-800 rounded-lg p-4 min-w-[280px]">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-900 flex items-center gap-2">
            <span class="w-3 h-3 bg-green-500 rounded-full"></span>
            Completed
          </h3>
          <span class="bg-green-100 text-green-700 px-2 py-1 rounded-full text-sm font-medium">
            {{ completedRequestsList.length }}
          </span>
        </div>
        <div class="space-y-3">
          <div
            v-for="request in completedRequestsList"
            :key="request.id"
            @click="viewRequestDetails(request)"
            class="bg-white rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer border-l-4 border-green-500"
          >
            <div class="flex items-start justify-between mb-2">
              <h4 class="font-medium text-gray-900">{{ request.title }}</h4>
              <span class="px-2 py-1 bg-green-100 text-green-700 rounded-full text-xs font-medium">
                ✓
              </span>
            </div>
            <p class="text-sm text-gray-600 dark:text-slate-400 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500 dark:text-slate-500 mb-2">
              <span>{{ getCategoryIcon(request.priority) }} {{ request.priority }}</span>
              <span>{{ formatDate(request.completed_date!) }}</span>
            </div>
            <div class="text-xs text-gray-600 dark:text-slate-400">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.cost" class="mt-2 text-xs text-gray-900 dark:text-slate-100 font-medium">
              Cost: ${{ request.cost.toLocaleString() }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Request Detail Modal -->
    <div
      v-if="selectedRequest"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
      @click.self="selectedRequest = null"
    >
      <div class="bg-white rounded-lg p-6 max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        <div class="flex justify-between items-start mb-4">
          <div>
            <h2 class="text-2xl font-bold mb-2">{{ selectedRequest.title }}</h2>
            <span :class="getPriorityBadgeClass(selectedRequest.priority)">
              {{ selectedRequest.priority }} priority
            </span>
          </div>
          <button @click="selectedRequest = null" class="text-gray-500 hover:text-gray-700">
            ✕
          </button>
        </div>

        <div class="space-y-4">
          <div>
            <h3 class="font-medium text-gray-900 mb-2">Description</h3>
            <p class="text-gray-600">{{ selectedRequest.description }}</p>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Priority</h3>
              <p class="text-gray-600 dark:text-slate-400">{{ getCategoryIcon(selectedRequest.priority) }} {{ selectedRequest.priority }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Status</h3>
              <p class="text-gray-600 dark:text-slate-400">{{ selectedRequest.status }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Property</h3>
              <p class="text-gray-600 dark:text-slate-400">{{ getPropertyTitle(selectedRequest.property_id) }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Created</h3>
              <p class="text-gray-600 dark:text-slate-400">{{ formatDate(selectedRequest.created_at) }}</p>
            </div>
          </div>

          <div v-if="selectedRequest.contractor">
            <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Contractor</h3>
            <p class="text-gray-600 dark:text-slate-400">{{ selectedRequest.contractor }}</p>
          </div>

          <div v-if="selectedRequest.cost" class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Cost</h3>
              <p class="text-gray-600 dark:text-slate-400">${{ selectedRequest.cost.toLocaleString() }}</p>
            </div>
            <div v-if="selectedRequest.scheduled_date">
              <h3 class="font-medium text-gray-900 dark:text-slate-100 mb-1">Scheduled Date</h3>
              <p class="text-gray-600 dark:text-slate-400">{{ formatDate(selectedRequest.scheduled_date) }}</p>
            </div>
          </div>

          <div v-if="selectedRequest.notes">
            <h3 class="font-medium text-gray-900 mb-1">Notes</h3>
            <p class="text-gray-600">{{ selectedRequest.notes }}</p>
          </div>

          <div class="pt-4 border-t">
            <h3 class="font-medium text-gray-900 mb-3">Before/After Photos</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div class="aspect-video bg-gradient-to-br from-gray-100 to-gray-200 rounded-lg flex items-center justify-center">
                <span class="text-gray-400">Before Photo</span>
              </div>
              <div class="aspect-video bg-gradient-to-br from-green-100 to-green-200 rounded-lg flex items-center justify-center">
                <span class="text-gray-400">After Photo</span>
              </div>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <button @click="selectedRequest = null" class="btn btn-secondary">Close</button>
          <button @click="openEditModal(selectedRequest)" class="btn btn-primary">Edit Request</button>
          <button @click="handleDelete(selectedRequest.id)" class="btn btn-danger">Delete</button>
        </div>
      </div>
    </div>

    <!-- Add Request Modal -->
    <MaintenanceForm
      v-if="showAddRequestModal"
      @close="closeModal"
      @submit="handleSubmit"
    />

    <!-- Edit Request Modal -->
    <MaintenanceForm
      v-if="showEditRequestModal && editingRequest"
      :request="editingRequest"
      :is-edit="true"
      @close="closeModal"
      @submit="handleSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format } from 'date-fns'
import { usePropertyStore } from '@/stores/propertyStore'
import { useMaintenanceStore } from '@/stores/maintenanceStore'
import MaintenanceForm from '@/components/MaintenanceForm.vue'
import type { MaintenanceRecord } from '@/types'

const propertyStore = usePropertyStore()
const maintenanceStore = useMaintenanceStore()

const showAddRequestModal = ref(false)
const showEditRequestModal = ref(false)
const selectedRequest = ref<MaintenanceRecord | null>(null)
const editingRequest = ref<MaintenanceRecord | null>(null)
const filterPriority = ref('')
const filterProperty = ref<number | ''>('')

const requests = computed(() => maintenanceStore.requests)

const filteredRequests = computed(() => {
  return requests.value.filter(request => {
    if (filterPriority.value && request.priority !== filterPriority.value) return false
    // Note: category field removed, filtering by priority only
    if (filterProperty.value && request.property_id !== filterProperty.value) return false
    return true
  })
})

const newRequests = computed(() => filteredRequests.value.filter(r => r.status === 'new' || r.status === 'pending'))
const assignedRequests = computed(() => filteredRequests.value.filter(r => r.status === 'assigned' || r.status === 'scheduled'))
const inProgressRequestsList = computed(() => filteredRequests.value.filter(r => r.status === 'in_progress'))
const completedRequestsList = computed(() => filteredRequests.value.filter(r => r.status === 'completed'))

const openRequests = computed(() => newRequests.value.length)
const inProgressRequests = computed(() => assignedRequests.value.length + inProgressRequestsList.value.length)
const completedRequests = computed(() => completedRequestsList.value.length)

const avgCompletionTime = computed(() => {
  const completed = requests.value.filter(r => r.status === 'completed' && r.completed_date)
  if (completed.length === 0) return 0
  const totalDays = completed.reduce((sum, r) => {
    const created = new Date(r.created_at)
    const completedDate = new Date(r.completed_date!)
    const days = Math.floor((completedDate.getTime() - created.getTime()) / (1000 * 60 * 60 * 24))
    return sum + days
  }, 0)
  return Math.round(totalDays / completed.length)
})

const totalCost = computed(() => {
  return requests.value.reduce((sum, r) => sum + (r.cost || 0), 0)
})

const getCategoryIcon = (priority: string) => {
  // Map priority to icons for visual clarity
  const icons: Record<string, string> = {
    urgent: '🚨',
    high: '⚠️',
    medium: '🔧',
    low: '📋'
  }
  return icons[priority] || '🔧'
}

const getPriorityBadgeClass = (priority: string) => {
  const classes: Record<string, string> = {
    urgent: 'px-2 py-1 bg-red-100 text-red-700 rounded-full text-xs font-medium',
    high: 'px-2 py-1 bg-orange-100 text-orange-700 rounded-full text-xs font-medium',
    medium: 'px-2 py-1 bg-yellow-100 text-yellow-700 rounded-full text-xs font-medium',
    low: 'px-2 py-1 bg-green-100 text-green-700 rounded-full text-xs font-medium'
  }
  return classes[priority] || classes.medium
}

const getPriorityBorderClass = (priority: string) => {
  const classes: Record<string, string> = {
    urgent: 'border-red-500',
    high: 'border-orange-500',
    medium: 'border-yellow-500',
    low: 'border-green-500'
  }
  return classes[priority] || classes.medium
}

const formatDate = (dateStr: string) => {
  return format(new Date(dateStr), 'MMM dd, yyyy')
}

const getPropertyTitle = (propertyId: number) => {
  const property = propertyStore.properties.find(p => p.id === propertyId)
  return property ? property.title : 'Unknown Property'
}

const viewRequestDetails = (request: MaintenanceRecord) => {
  selectedRequest.value = request
}

async function handleSubmit(requestData: Partial<MaintenanceRecord>) {
  try {
    if (editingRequest.value) {
      await maintenanceStore.updateRequest(editingRequest.value.id, requestData)
    } else {
      await maintenanceStore.createRequest(requestData)
    }
    closeModal()
  } catch (error) {
    console.error('Failed to save maintenance request:', error)
  }
}

function openEditModal(request: MaintenanceRecord) {
  editingRequest.value = request
  showEditRequestModal.value = true
  selectedRequest.value = null
}

async function handleDelete(id: number) {
  if (!confirm('Are you sure you want to delete this request? This action cannot be undone.')) {
    return
  }
  try {
    await maintenanceStore.deleteRequest(id)
    selectedRequest.value = null
  } catch (error) {
    console.error('Failed to delete request:', error)
  }
}

function closeModal() {
  showAddRequestModal.value = false
  showEditRequestModal.value = false
  editingRequest.value = null
}

onMounted(async () => {
  await Promise.all([
    maintenanceStore.fetchRequests(),
    propertyStore.fetchProperties()
  ])
})
</script>
