<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
      <div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-slate-100 dark:text-slate-100">Calendar & Events</h1>
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
        <p class="text-gray-500 dark:text-slate-400">No upcoming events</p>
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
            <p class="font-medium text-gray-900 dark:text-slate-100 dark:text-slate-100">{{ event.title }}</p>
            <p class="text-sm text-gray-600 dark:text-slate-400">{{ event.description }}</p>
            <p class="text-xs text-gray-500 mt-1">
              {{ formatEventDate(event.start_time) }}
              <span v-if="event.property_id" class="ml-2">
                • {{ getPropertyTitle(event.property_id) }}
              </span>
            </p>
          </div>
          <div class="flex gap-1">
            <button @click="openEditModal(event)" class="p-1 hover:bg-gray-200 rounded">
              ✏️
            </button>
            <button @click="handleDelete(event.id)" class="p-1 hover:bg-red-100 rounded text-red-600">
              🗑️
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Event Modal -->
    <EventForm
      v-if="showAddEventModal"
      :default-start="selectedDateRange?.start"
      :default-end="selectedDateRange?.end"
      @close="closeModal"
      @submit="handleSubmit"
    />

    <!-- Edit Event Modal -->
    <EventForm
      v-if="showEditEventModal && editingEvent"
      :event="editingEvent"
      :is-edit="true"
      @close="closeModal"
      @submit="handleSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { format, addDays, isBefore, isAfter, parseISO } from 'date-fns'
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import timeGridPlugin from '@fullcalendar/timegrid'
import interactionPlugin from '@fullcalendar/interaction'
import type { CalendarOptions, EventClickArg, DateSelectArg } from '@fullcalendar/core'
import { usePropertyStore } from '@/stores/propertyStore'
import { useEventStore } from '@/stores/eventStore'
import EventForm from '@/components/EventForm.vue'
import type { CalendarEvent } from '@/types'

const propertyStore = usePropertyStore()
const eventStore = useEventStore()

const showAddEventModal = ref(false)
const showEditEventModal = ref(false)
const editingEvent = ref<CalendarEvent | null>(null)
const selectedDateRange = ref<{ start: string; end: string } | null>(null)
const visibleCategories = ref<string[]>(['maintenance', 'inspection', 'showing', 'lease-signing', 'meeting'])

const eventCategories = [
  { type: 'maintenance', label: 'Maintenance', icon: '🔧', bgColor: 'bg-orange-100', textColor: 'text-orange-700', calendarColor: '#fb923c' },
  { type: 'inspection', label: 'Inspections', icon: '🔍', bgColor: 'bg-blue-100', textColor: 'text-blue-700', calendarColor: '#60a5fa' },
  { type: 'showing', label: 'Showings', icon: '🏠', bgColor: 'bg-yellow-100', textColor: 'text-yellow-700', calendarColor: '#fbbf24' },
  { type: 'lease-signing', label: 'Lease Signing', icon: '📝', bgColor: 'bg-purple-100', textColor: 'text-purple-700', calendarColor: '#c084fc' },
  { type: 'meeting', label: 'Meetings', icon: '📞', bgColor: 'bg-pink-100', textColor: 'text-pink-700', calendarColor: '#f472b6' },
  { type: 'other', label: 'Other', icon: '📅', bgColor: 'bg-gray-100', textColor: 'text-gray-700', calendarColor: '#9ca3af' }
]

const events = computed(() => eventStore.events)

const getCategoryConfig = (type: string) => {
  return eventCategories.find(cat => cat.type === type) || eventCategories[5]
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
      start: event.start_time,
      end: event.end_time,
      backgroundColor: config.calendarColor,
      borderColor: config.calendarColor,
      extendedProps: {
        description: event.description,
        type: event.event_type,
        propertyId: event.property_id,
        fullEvent: event
      }
    }
  })
})

const upcomingEvents = computed(() => {
  const sevenDaysFromNow = addDays(new Date(), 7)
  return filteredEvents.value
    .filter(event => {
      const eventDate = parseISO(event.start_time)
      return isAfter(eventDate, new Date()) && isBefore(eventDate, sevenDaysFromNow)
    })
    .sort((a, b) => parseISO(a.start_time).getTime() - parseISO(b.start_time).getTime())
    .slice(0, 5)
})

const formatEventDate = (dateStr: string) => {
  return format(parseISO(dateStr), 'MMM dd, yyyy h:mm a')
}

const getPropertyTitle = (propertyId: number | null) => {
  if (!propertyId) return 'General Event'
  const property = propertyStore.properties.find(p => p.id === propertyId)
  return property ? property.title : 'Unknown Property'
}

const handleEventClick = (clickInfo: EventClickArg) => {
  const event = clickInfo.event.extendedProps.fullEvent as CalendarEvent
  openEditModal(event)
}

const handleDateSelect = (selectInfo: DateSelectArg) => {
  selectedDateRange.value = {
    start: selectInfo.startStr,
    end: selectInfo.endStr
  }
  showAddEventModal.value = true
}

async function handleSubmit(eventData: Partial<CalendarEvent>) {
  try {
    if (editingEvent.value) {
      await eventStore.updateEvent(editingEvent.value.id, eventData)
    } else {
      await eventStore.createEvent(eventData)
    }
    closeModal()
  } catch (error) {
    console.error('Failed to save event:', error)
  }
}

function openEditModal(event: CalendarEvent) {
  editingEvent.value = event
  showEditEventModal.value = true
}

async function handleDelete(id: number) {
  if (!confirm('Are you sure you want to delete this event?')) {
    return
  }
  try {
    await eventStore.deleteEvent(id)
    closeModal()
  } catch (error) {
    console.error('Failed to delete event:', error)
  }
}

function closeModal() {
  showAddEventModal.value = false
  showEditEventModal.value = false
  editingEvent.value = null
  selectedDateRange.value = null
}

const calendarOptions = computed<CalendarOptions>(() => ({
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
  select: handleDateSelect,
  height: 'auto',
  contentHeight: 600
}))

onMounted(async () => {
  await Promise.all([
    eventStore.fetchEvents(),
    propertyStore.fetchProperties()
  ])
})
</script>
