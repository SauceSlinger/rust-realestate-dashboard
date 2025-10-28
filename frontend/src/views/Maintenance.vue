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
        <select v-model="filterCategory" class="input">
          <option value="">All Categories</option>
          <option value="plumbing">Plumbing</option>
          <option value="electrical">Electrical</option>
          <option value="hvac">HVAC</option>
          <option value="appliance">Appliance</option>
          <option value="structural">Structural</option>
          <option value="cosmetic">Cosmetic</option>
          <option value="other">Other</option>
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
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <!-- New Column -->
      <div class="bg-gray-50 rounded-lg p-4">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-900 flex items-center gap-2">
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
              <span>{{ getCategoryIcon(request.category) }} {{ request.category }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="mt-2 text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Assigned Column -->
      <div class="bg-gray-50 rounded-lg p-4">
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
            <p class="text-sm text-gray-600 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500 mb-2">
              <span>{{ getCategoryIcon(request.category) }} {{ request.category }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.assigned_to" class="mt-2 text-xs text-blue-600 flex items-center gap-1">
              👤 {{ request.assigned_to }}
            </div>
          </div>
        </div>
      </div>

      <!-- In Progress Column -->
      <div class="bg-gray-50 rounded-lg p-4">
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
              <span>{{ getCategoryIcon(request.category) }} {{ request.category }}</span>
              <span>{{ formatDate(request.created_at) }}</span>
            </div>
            <div class="text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.assigned_to" class="mt-2 text-xs text-blue-600 flex items-center gap-1">
              👤 {{ request.assigned_to }}
            </div>
            <div v-if="request.estimated_cost" class="mt-2 text-xs text-gray-700 font-medium">
              Est. Cost: ${{ request.estimated_cost.toLocaleString() }}
            </div>
          </div>
        </div>
      </div>

      <!-- Completed Column -->
      <div class="bg-gray-50 rounded-lg p-4">
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
            <p class="text-sm text-gray-600 mb-2">{{ request.description }}</p>
            <div class="flex items-center justify-between text-xs text-gray-500 mb-2">
              <span>{{ getCategoryIcon(request.category) }} {{ request.category }}</span>
              <span>{{ formatDate(request.completed_at!) }}</span>
            </div>
            <div class="text-xs text-gray-600">
              {{ getPropertyTitle(request.property_id) }}
            </div>
            <div v-if="request.actual_cost" class="mt-2 text-xs text-gray-900 font-medium">
              Cost: ${{ request.actual_cost.toLocaleString() }}
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

          <div class="grid grid-cols-2 gap-4">
            <div>
              <h3 class="font-medium text-gray-900 mb-1">Category</h3>
              <p class="text-gray-600">{{ getCategoryIcon(selectedRequest.category) }} {{ selectedRequest.category }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 mb-1">Status</h3>
              <p class="text-gray-600">{{ selectedRequest.status }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 mb-1">Property</h3>
              <p class="text-gray-600">{{ getPropertyTitle(selectedRequest.property_id) }}</p>
            </div>
            <div>
              <h3 class="font-medium text-gray-900 mb-1">Created</h3>
              <p class="text-gray-600">{{ formatDate(selectedRequest.created_at) }}</p>
            </div>
          </div>

          <div v-if="selectedRequest.assigned_to">
            <h3 class="font-medium text-gray-900 mb-1">Assigned To</h3>
            <p class="text-gray-600">{{ selectedRequest.assigned_to }}</p>
          </div>

          <div v-if="selectedRequest.estimated_cost || selectedRequest.actual_cost" class="grid grid-cols-2 gap-4">
            <div v-if="selectedRequest.estimated_cost">
              <h3 class="font-medium text-gray-900 mb-1">Estimated Cost</h3>
              <p class="text-gray-600">${{ selectedRequest.estimated_cost.toLocaleString() }}</p>
            </div>
            <div v-if="selectedRequest.actual_cost">
              <h3 class="font-medium text-gray-900 mb-1">Actual Cost</h3>
              <p class="text-gray-600">${{ selectedRequest.actual_cost.toLocaleString() }}</p>
            </div>
          </div>

          <div v-if="selectedRequest.notes">
            <h3 class="font-medium text-gray-900 mb-1">Notes</h3>
            <p class="text-gray-600">{{ selectedRequest.notes }}</p>
          </div>

          <div class="pt-4 border-t">
            <h3 class="font-medium text-gray-900 mb-3">Before/After Photos</h3>
            <div class="grid grid-cols-2 gap-4">
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
          <button class="btn btn-primary">Edit Request</button>
        </div>
      </div>
    </div>

    <!-- Add Request Modal -->
    <div
      v-if="showAddRequestModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
      @click.self="showAddRequestModal = false"
    >
      <div class="bg-white rounded-lg p-6 max-w-md w-full">
        <h2 class="text-2xl font-bold mb-4">New Maintenance Request</h2>
        <p class="text-gray-600 mb-4">Request creation form will be implemented in the next phase.</p>
        <div class="flex justify-end space-x-2">
          <button @click="showAddRequestModal = false" class="btn btn-secondary">Cancel</button>
          <button class="btn btn-primary">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format } from 'date-fns'
import { usePropertyStore } from '@/stores/propertyStore'

const propertyStore = usePropertyStore()

interface MaintenanceRequest {
  id: number
  title: string
  description: string
  category: string
  priority: string
  status: string
  property_id: number
  assigned_to: string | null
  estimated_cost: number | null
  actual_cost: number | null
  created_at: string
  completed_at: string | null
  notes: string | null
}

const showAddRequestModal = ref(false)
const selectedRequest = ref<MaintenanceRequest | null>(null)
const filterPriority = ref('')
const filterCategory = ref('')
const filterProperty = ref<number | ''>('')

const requests = ref<MaintenanceRequest[]>([])

const generateSampleRequests = () => {
  const today = new Date()
  const sampleRequests: MaintenanceRequest[] = [
    // New requests
    {
      id: 1,
      title: 'Leaking Kitchen Faucet',
      description: 'Kitchen sink faucet has a constant drip causing water waste',
      category: 'plumbing',
      priority: 'high',
      status: 'new',
      property_id: 1,
      assigned_to: null,
      estimated_cost: null,
      actual_cost: null,
      created_at: format(today, 'yyyy-MM-dd'),
      completed_at: null,
      notes: null
    },
    {
      id: 2,
      title: 'Broken Window Lock',
      description: 'Bedroom window lock is broken, security concern',
      category: 'structural',
      priority: 'urgent',
      status: 'new',
      property_id: 2,
      assigned_to: null,
      estimated_cost: null,
      actual_cost: null,
      created_at: format(today, 'yyyy-MM-dd'),
      completed_at: null,
      notes: null
    },
    {
      id: 3,
      title: 'Paint Touch-up Needed',
      description: 'Hallway walls need paint touch-up after tenant move-out',
      category: 'cosmetic',
      priority: 'low',
      status: 'new',
      property_id: 3,
      assigned_to: null,
      estimated_cost: null,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 86400000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: null
    },
    // Assigned requests
    {
      id: 4,
      title: 'HVAC Annual Service',
      description: 'Scheduled annual HVAC maintenance and filter replacement',
      category: 'hvac',
      priority: 'medium',
      status: 'assigned',
      property_id: 1,
      assigned_to: 'Mike Johnson - HVAC Pro',
      estimated_cost: 250,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 172800000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: 'Scheduled for next Tuesday'
    },
    {
      id: 5,
      title: 'Replace Dishwasher',
      description: 'Dishwasher motor failed, needs replacement',
      category: 'appliance',
      priority: 'high',
      status: 'assigned',
      property_id: 4,
      assigned_to: 'Sarah Lee - Appliance Repair',
      estimated_cost: 650,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 259200000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: 'Tenant has been provided with temporary unit'
    },
    // In Progress requests
    {
      id: 6,
      title: 'Electrical Outlet Repair',
      description: 'Living room outlet not working, possible wiring issue',
      category: 'electrical',
      priority: 'urgent',
      status: 'in_progress',
      property_id: 2,
      assigned_to: 'Tom Davis - Electric Solutions',
      estimated_cost: 180,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 345600000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: 'Electrician on-site, diagnosing issue'
    },
    {
      id: 7,
      title: 'Garage Door Spring',
      description: 'Garage door spring broken, door won\'t open',
      category: 'structural',
      priority: 'high',
      status: 'in_progress',
      property_id: 5,
      assigned_to: 'Bob Wilson - Garage Services',
      estimated_cost: 320,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 432000000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: 'Parts ordered, installation scheduled'
    },
    {
      id: 8,
      title: 'Landscaping Maintenance',
      description: 'Quarterly landscaping and lawn care',
      category: 'other',
      priority: 'low',
      status: 'in_progress',
      property_id: 3,
      assigned_to: 'Green Thumb Landscaping',
      estimated_cost: 150,
      actual_cost: null,
      created_at: format(new Date(today.getTime() - 518400000), 'yyyy-MM-dd'),
      completed_at: null,
      notes: 'In progress, completing this week'
    },
    // Completed requests
    {
      id: 9,
      title: 'Toilet Repair',
      description: 'Running toilet causing high water bill',
      category: 'plumbing',
      priority: 'medium',
      status: 'completed',
      property_id: 1,
      assigned_to: 'Jack Smith - Plumbing Plus',
      estimated_cost: 120,
      actual_cost: 135,
      created_at: format(new Date(today.getTime() - 604800000), 'yyyy-MM-dd'),
      completed_at: format(new Date(today.getTime() - 86400000), 'yyyy-MM-dd'),
      notes: 'Replaced flapper valve and fill tube'
    },
    {
      id: 10,
      title: 'Smoke Detector Battery',
      description: 'Replace batteries in all smoke detectors',
      category: 'other',
      priority: 'medium',
      status: 'completed',
      property_id: 2,
      assigned_to: 'Property Manager',
      estimated_cost: 30,
      actual_cost: 25,
      created_at: format(new Date(today.getTime() - 691200000), 'yyyy-MM-dd'),
      completed_at: format(new Date(today.getTime() - 604800000), 'yyyy-MM-dd'),
      notes: 'All units tested and working'
    },
    {
      id: 11,
      title: 'Gutter Cleaning',
      description: 'Clean gutters and downspouts before winter',
      category: 'structural',
      priority: 'low',
      status: 'completed',
      property_id: 4,
      assigned_to: 'Clean Sweep Services',
      estimated_cost: 200,
      actual_cost: 200,
      created_at: format(new Date(today.getTime() - 777600000), 'yyyy-MM-dd'),
      completed_at: format(new Date(today.getTime() - 691200000), 'yyyy-MM-dd'),
      notes: 'All gutters cleaned and inspected'
    },
    {
      id: 12,
      title: 'Air Filter Replacement',
      description: 'Replace HVAC air filters in all units',
      category: 'hvac',
      priority: 'low',
      status: 'completed',
      property_id: 5,
      assigned_to: 'Property Manager',
      estimated_cost: 50,
      actual_cost: 45,
      created_at: format(new Date(today.getTime() - 864000000), 'yyyy-MM-dd'),
      completed_at: format(new Date(today.getTime() - 777600000), 'yyyy-MM-dd'),
      notes: 'Scheduled for next quarter'
    }
  ]
  requests.value = sampleRequests
}

const filteredRequests = computed(() => {
  return requests.value.filter(request => {
    if (filterPriority.value && request.priority !== filterPriority.value) return false
    if (filterCategory.value && request.category !== filterCategory.value) return false
    if (filterProperty.value && request.property_id !== filterProperty.value) return false
    return true
  })
})

const newRequests = computed(() => filteredRequests.value.filter(r => r.status === 'new'))
const assignedRequests = computed(() => filteredRequests.value.filter(r => r.status === 'assigned'))
const inProgressRequestsList = computed(() => filteredRequests.value.filter(r => r.status === 'in_progress'))
const completedRequestsList = computed(() => filteredRequests.value.filter(r => r.status === 'completed'))

const openRequests = computed(() => newRequests.value.length)
const inProgressRequests = computed(() => assignedRequests.value.length + inProgressRequestsList.value.length)
const completedRequests = computed(() => completedRequestsList.value.length)

const avgCompletionTime = computed(() => {
  const completed = requests.value.filter(r => r.status === 'completed' && r.completed_at)
  if (completed.length === 0) return 0
  const totalDays = completed.reduce((sum, r) => {
    const created = new Date(r.created_at)
    const completedDate = new Date(r.completed_at!)
    const days = Math.floor((completedDate.getTime() - created.getTime()) / (1000 * 60 * 60 * 24))
    return sum + days
  }, 0)
  return Math.round(totalDays / completed.length)
})

const totalCost = computed(() => {
  return requests.value.reduce((sum, r) => sum + (r.actual_cost || 0), 0)
})

const getCategoryIcon = (category: string) => {
  const icons: Record<string, string> = {
    plumbing: '🚰',
    electrical: '⚡',
    hvac: '❄️',
    appliance: '🔌',
    structural: '🏗️',
    cosmetic: '🎨',
    other: '🔧'
  }
  return icons[category] || '🔧'
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

const viewRequestDetails = (request: MaintenanceRequest) => {
  selectedRequest.value = request
}

onMounted(() => {
  generateSampleRequests()
})
</script>
