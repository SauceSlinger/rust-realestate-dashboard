import { defineStore } from 'pinia'
import { ref } from 'vue'
import { eventService } from '@/services/api'
import type { CalendarEvent } from '@/types'

export const useEventStore = defineStore('event', () => {
  const events = ref<CalendarEvent[]>([])
  const currentEvent = ref<CalendarEvent | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchEvents() {
    loading.value = true
    error.value = null
    try {
      const response = await eventService.getAll()
      events.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch events'
      // Demo mode: generate sample events
      events.value = generateSampleEvents()
    } finally {
      loading.value = false
    }
  }

  async function fetchEvent(id: number) {
    loading.value = true
    error.value = null
    try {
      const response = await eventService.getById(id)
      currentEvent.value = response.data
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch event'
      // Demo mode: find from local array
      const event = events.value.find(ev => ev.id === id)
      if (event) {
        currentEvent.value = event
        return event
      }
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createEvent(data: Partial<CalendarEvent>) {
    loading.value = true
    error.value = null
    try {
      const response = await eventService.create(data)
      events.value.unshift(response.data)
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to create event'
      // Demo mode: create with mock ID
      const newEvent: CalendarEvent = {
        id: Math.max(...events.value.map(ev => ev.id), 0) + 1,
        ...data,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      } as CalendarEvent
      events.value.unshift(newEvent)
      return newEvent
    } finally {
      loading.value = false
    }
  }

  async function updateEvent(id: number, data: Partial<CalendarEvent>) {
    loading.value = true
    error.value = null
    try {
      const response = await eventService.update(id, data)
      const index = events.value.findIndex(ev => ev.id === id)
      if (index !== -1) {
        events.value[index] = response.data
      }
      if (currentEvent.value?.id === id) {
        currentEvent.value = response.data
      }
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to update event'
      // Demo mode: update locally
      const index = events.value.findIndex(ev => ev.id === id)
      if (index !== -1) {
        events.value[index] = {
          ...events.value[index],
          ...data,
          updated_at: new Date().toISOString()
        }
      }
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteEvent(id: number) {
    loading.value = true
    error.value = null
    try {
      await eventService.delete(id)
      events.value = events.value.filter(ev => ev.id !== id)
    } catch (e: any) {
      error.value = e.message || 'Failed to delete event'
      // Demo mode: delete locally
      events.value = events.value.filter(ev => ev.id !== id)
    } finally {
      loading.value = false
    }
  }

  function generateSampleEvents(): CalendarEvent[] {
    const now = new Date()
    const addDays = (days: number) => {
      const d = new Date(now)
      d.setDate(d.getDate() + days)
      return d.toISOString()
    }
    const addHours = (days: number, hours: number) => {
      const d = new Date(now)
      d.setDate(d.getDate() + days)
      d.setHours(hours, 0, 0, 0)
      return d.toISOString()
    }

    return [
      {
        id: 1,
        property_id: 1,
        title: 'Property Showing - Sunset Apartments',
        description: 'Meet with potential tenant Sarah Johnson',
        event_type: 'showing',
        start_time: addHours(2, 14),
        end_time: addHours(2, 15),
        reminder_minutes: 60,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      },
      {
        id: 2,
        property_id: 2,
        title: 'Annual Property Inspection',
        description: 'Full inspection of Oceanview Condo with city inspector',
        event_type: 'inspection',
        start_time: addHours(7, 10),
        end_time: addHours(7, 12),
        reminder_minutes: 1440,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      },
      {
        id: 3,
        property_id: 1,
        title: 'HVAC Maintenance',
        description: 'Scheduled maintenance for heating system',
        event_type: 'maintenance',
        start_time: addHours(5, 9),
        end_time: addHours(5, 11),
        reminder_minutes: 2880,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      },
      {
        id: 4,
        property_id: 3,
        title: 'Lease Signing - Downtown Loft',
        description: 'Sign new lease with Michael Brown',
        event_type: 'lease-signing',
        start_time: addHours(4, 15),
        end_time: addHours(4, 16),
        reminder_minutes: 60,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      },
      {
        id: 5,
        property_id: undefined,
        title: 'Portfolio Review Meeting',
        description: 'Quarterly review with financial advisor',
        event_type: 'meeting',
        start_time: addHours(14, 13),
        end_time: addHours(14, 14),
        reminder_minutes: 1440,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      },
      {
        id: 6,
        property_id: 2,
        title: 'Plumbing Repair',
        description: 'Fix leaky faucet in unit 2B',
        event_type: 'maintenance',
        start_time: addHours(1, 11),
        end_time: addHours(1, 12),
        reminder_minutes: 30,
        completed: false,
        created_at: now.toISOString(),
        updated_at: now.toISOString()
      }
    ]
  }

  return {
    events,
    currentEvent,
    loading,
    error,
    fetchEvents,
    fetchEvent,
    createEvent,
    updateEvent,
    deleteEvent,
    generateSampleEvents
  }
})
