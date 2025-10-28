<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Calendar & Events</h1>
        <p class="text-gray-600 mt-1">Schedule and manage property events</p>
      </div>
      <button @click="showAddEventModal = true" class="btn btn-primary whitespace-nowrap">
        + Add Event
      </button>
    </div>

    <!-- Event Type Filters -->
    <div class="card">
      <div class="flex flex-wrap gap-3">
        <button
          v-for="category in eventCategories"
          :key="category.type"
          @click="toggleCategory(category.type)"
          :class="[
            'px-4 py-2 rounded-lg font-medium transition-all',
            visibleCategories.includes(category.type)
              ? `${category.bgColor} ${category.textColor} shadow-md`
              : 'bg-gray-100 text-gray-400 hover:bg-gray-200'
          ]"
        >
          <span class="mr-2">{{ category.icon }}</span>
          {{ category.label }}
        </button>
      </div>
    </div>

    <!-- Calendar -->
    <div class="card">
      <FullCalendar :options="calendarOptions" />
    </div>

    <!-- Upcoming Events Sidebar -->
    <div class="card">
      <h2 class="text-xl font-bold text-gray-900 mb-4">Upcoming Events (Next 7 Days)</h2>
      <div v-if="upcomingEvents.length === 0" class="text-center py-8">
        <p class="text-gray-500">No upcoming events</p>
      </div>
      <div v-else class="space-y-3">
        <div
          v-for="event in upcomingEvents"
          :key="event.id"
          class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors"
        >
          <div
            :class="[
              'flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center text-xl',
              getCategoryConfig(event.event_type).bgColor
            ]"
          >
            {{ getCategoryConfig(event.event_type).icon }}
          </div>
          <div class="flex-1 min-w-0">
            <p class="font-medium text-gray-900">{{ event.title }}</p>
            <p class="text-sm text-gray-600">{{ event.description }}</p>
            <p class="text-xs text-gray-500 mt-1">
              {{ formatEventDate(event.event_date) }}
              <span v-if="event.property_id" class="ml-2">
                • {{ getPropertyTitle(event.property_id) }}
              </span>
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Event Modal -->
    <div
      v-if="showAddEventModal"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
      @click.self="showAddEventModal = false"
    >
      <div class="bg-white rounded-lg p-6 max-w-md w-full">
        <h2 class="text-2xl font-bold mb-4">Add New Event</h2>
        <p class="text-gray-600 mb-4">Event creation form will be implemented in the next phase.</p>
        <div class="flex justify-end space-x-2">
          <button @click="showAddEventModal = false" class="btn btn-secondary">Cancel</button>
          <button class="btn btn-primary">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format, addDays, isBefore, isAfter } from 'date-fns'
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import timeGridPlugin from '@fullcalendar/timegrid'
import interactionPlugin from '@fullcalendar/interaction'
import type { CalendarOptions, EventClickArg } from '@fullcalendar/core'
import { usePropertyStore } from '@/stores/propertyStore'

const propertyStore = usePropertyStore()

interface CalendarEvent {
  id: number
  title: string
  description: string
  event_date: string
  event_type: string
  property_id: number | null
  status: string
  created_at: string
}

const showAddEventModal = ref(false)
const visibleCategories = ref<string[]>(['maintenance', 'rent', 'inspection', 'renewal', 'showing', 'meeting'])

const eventCategories = [
  { type: 'maintenance', label: 'Maintenance', icon: '🔧', bgColor: 'bg-orange-100', textColor: 'text-orange-700', calendarColor: '#fb923c' },
  { type: 'rent', label: 'Rent Due', icon: '💰', bgColor: 'bg-green-100', textColor: 'text-green-700', calendarColor: '#4ade80' },
  { type: 'inspection', label: 'Inspections', icon: '🔍', bgColor: 'bg-blue-100', textColor: 'text-blue-700', calendarColor: '#60a5fa' },
  { type: 'renewal', label: 'Renewals', icon: '📝', bgColor: 'bg-purple-100', textColor: 'text-purple-700', calendarColor: '#c084fc' },
  { type: 'showing', label: 'Showings', icon: '🏠', bgColor: 'bg-yellow-100', textColor: 'text-yellow-700', calendarColor: '#fbbf24' },
  { type: 'meeting', label: 'Meetings', icon: '📞', bgColor: 'bg-pink-100', textColor: 'text-pink-700', calendarColor: '#f472b6' }
]

// Sample events
const events = ref<CalendarEvent[]>([])

const generateSampleEvents = () => {
  const today = new Date()
  const sampleEvents: CalendarEvent[] = [
    // Maintenance events
    {
      id: 1,
      title: 'HVAC Inspection',
      description: 'Annual HVAC system inspection and filter replacement',
      event_date: format(addDays(today, 2), 'yyyy-MM-dd'),
      event_type: 'maintenance',
      property_id: 1,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    {
      id: 2,
      title: 'Fix Leaking Faucet',
      description: 'Kitchen faucet repair at Sunset Villa',
      event_date: format(addDays(today, 5), 'yyyy-MM-dd'),
      event_type: 'maintenance',
      property_id: 2,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    // Rent due dates
    {
      id: 3,
      title: 'Rent Due',
      description: 'Monthly rent payment due for all properties',
      event_date: format(new Date(today.getFullYear(), today.getMonth() + 1, 1), 'yyyy-MM-dd'),
      event_type: 'rent',
      property_id: null,
      status: 'upcoming',
      created_at: format(today, 'yyyy-MM-dd')
    },
    // Inspections
    {
      id: 4,
      title: 'Property Inspection',
      description: 'Quarterly property condition inspection',
      event_date: format(addDays(today, 7), 'yyyy-MM-dd'),
      event_type: 'inspection',
      property_id: 1,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    {
      id: 5,
      title: 'Move-In Inspection',
      description: 'Pre-tenant move-in inspection documentation',
      event_date: format(addDays(today, 12), 'yyyy-MM-dd'),
      event_type: 'inspection',
      property_id: 3,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    // Renewals
    {
      id: 6,
      title: 'Lease Renewal Discussion',
      description: 'Meet with tenant to discuss lease renewal options',
      event_date: format(addDays(today, 15), 'yyyy-MM-dd'),
      event_type: 'renewal',
      property_id: 1,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    // Showings
    {
      id: 7,
      title: 'Property Showing',
      description: 'Show property to prospective tenants',
      event_date: format(addDays(today, 3), 'yyyy-MM-dd'),
      event_type: 'showing',
      property_id: 4,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    {
      id: 8,
      title: 'Virtual Tour',
      description: 'Online property tour for remote prospects',
      event_date: format(addDays(today, 6), 'yyyy-MM-dd'),
      event_type: 'showing',
      property_id: 5,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    // Meetings
    {
      id: 9,
      title: 'Vendor Meeting',
      description: 'Discuss landscaping services for Q2',
      event_date: format(addDays(today, 4), 'yyyy-MM-dd'),
      event_type: 'meeting',
      property_id: null,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    },
    {
      id: 10,
      title: 'Insurance Review',
      description: 'Annual property insurance policy review',
      event_date: format(addDays(today, 20), 'yyyy-MM-dd'),
      event_type: 'meeting',
      property_id: null,
      status: 'scheduled',
      created_at: format(today, 'yyyy-MM-dd')
    }
  ]
  events.value = sampleEvents
}

const getCategoryConfig = (type: string) => {
  return eventCategories.find(cat => cat.type === type) || eventCategories[0]
}

const toggleCategory = (type: string) => {
  const index = visibleCategories.value.indexOf(type)
  if (index > -1) {
    visibleCategories.value.splice(index, 1)
  } else {
    visibleCategories.value.push(type)
  }
}

const filteredEvents = computed(() => {
  return events.value.filter(event => visibleCategories.value.includes(event.event_type))
})

const calendarEvents = computed(() => {
  return filteredEvents.value.map(event => {
    const config = getCategoryConfig(event.event_type)
    return {
      id: event.id.toString(),
      title: event.title,
      date: event.event_date,
      backgroundColor: config.calendarColor,
      borderColor: config.calendarColor,
      extendedProps: {
        description: event.description,
        type: event.event_type,
        propertyId: event.property_id
      }
    }
  })
})

const upcomingEvents = computed(() => {
  const sevenDaysFromNow = addDays(new Date(), 7)
  return filteredEvents.value
    .filter(event => {
      const eventDate = new Date(event.event_date)
      return isAfter(eventDate, new Date()) && isBefore(eventDate, sevenDaysFromNow)
    })
    .sort((a, b) => new Date(a.event_date).getTime() - new Date(b.event_date).getTime())
    .slice(0, 5)
})

const formatEventDate = (dateStr: string) => {
  return format(new Date(dateStr), 'MMM dd, yyyy')
}

const getPropertyTitle = (propertyId: number) => {
  const property = propertyStore.properties.find(p => p.id === propertyId)
  return property ? property.title : 'All Properties'
}

const handleEventClick = (clickInfo: EventClickArg) => {
  alert(`Event: ${clickInfo.event.title}\n${clickInfo.event.extendedProps.description}`)
}

const calendarOptions: CalendarOptions = {
  plugins: [dayGridPlugin, timeGridPlugin, interactionPlugin],
  initialView: 'dayGridMonth',
  headerToolbar: {
    left: 'prev,next today',
    center: 'title',
    right: 'dayGridMonth,timeGridWeek,timeGridDay'
  },
  events: calendarEvents.value,
  editable: true,
  selectable: true,
  selectMirror: true,
  dayMaxEvents: true,
  weekends: true,
  eventClick: handleEventClick,
  height: 'auto',
  contentHeight: 600
}

onMounted(() => {
  generateSampleEvents()
})
</script>
