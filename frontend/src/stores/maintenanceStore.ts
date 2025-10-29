import { defineStore } from 'pinia'
import { ref } from 'vue'
import { maintenanceService } from '@/services/api'
import type { MaintenanceRecord } from '@/types'

export const useMaintenanceStore = defineStore('maintenance', () => {
  const requests = ref<MaintenanceRecord[]>([])
  const currentRequest = ref<MaintenanceRecord | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchRequests() {
    loading.value = true
    error.value = null
    try {
      const response = await maintenanceService.getAll()
      requests.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch maintenance requests'
      // Demo mode: generate sample requests
      requests.value = generateSampleRequests()
    } finally {
      loading.value = false
    }
  }

  async function createRequest(data: Partial<MaintenanceRecord>) {
    loading.value = true
    error.value = null
    try {
      const response = await maintenanceService.create(data)
      requests.value.unshift(response.data)
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to create maintenance request'
      // Demo mode: create with mock ID
      const newRequest: MaintenanceRecord = {
        id: Math.max(...requests.value.map(r => r.id), 0) + 1,
        ...data,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      } as MaintenanceRecord
      requests.value.unshift(newRequest)
      return newRequest
    } finally {
      loading.value = false
    }
  }

  async function updateRequest(id: number, data: Partial<MaintenanceRecord>) {
    loading.value = true
    error.value = null
    try {
      const response = await maintenanceService.update(id, data)
      const index = requests.value.findIndex(r => r.id === id)
      if (index !== -1) {
        requests.value[index] = response.data
      }
      if (currentRequest.value?.id === id) {
        currentRequest.value = response.data
      }
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to update maintenance request'
      // Demo mode: update locally
      const index = requests.value.findIndex(r => r.id === id)
      if (index !== -1) {
        requests.value[index] = {
          ...requests.value[index],
          ...data,
          updated_at: new Date().toISOString()
        }
      }
      throw e
    } finally {
      loading.value = false
    }
  }

  // Note: API doesn't have delete endpoint, but we'll provide it for demo mode
  async function deleteRequest(id: number) {
    loading.value = true
    error.value = null
    try {
      // No backend delete endpoint, so this will always fail in production
      // await maintenanceService.delete(id) // Does not exist
      throw new Error('Delete not supported by backend')
    } catch (e: any) {
      error.value = e.message || 'Failed to delete maintenance request'
      // Demo mode: delete locally
      requests.value = requests.value.filter(r => r.id !== id)
    } finally {
      loading.value = false
    }
  }

  function generateSampleRequests(): MaintenanceRecord[] {
    const now = new Date()
    const addDays = (days: number) => {
      const d = new Date(now)
      d.setDate(d.getDate() + days)
      return d.toISOString().split('T')[0]
    }
    const subtractDays = (days: number) => {
      const d = new Date(now)
      d.setDate(d.getDate() - days)
      return d.toISOString().split('T')[0]
    }

    return [
      {
        id: 1,
        property_id: 1,
        title: 'Leaking Kitchen Faucet',
        description: 'Kitchen sink faucet has been dripping constantly. Needs replacement or repair.',
        priority: 'medium',
        status: 'pending',
        cost: 150,
        scheduled_date: addDays(2),
        contractor: 'ABC Plumbing',
        notes: 'Tenant reports water waste is significant',
        created_at: subtractDays(3),
        updated_at: subtractDays(3)
      },
      {
        id: 2,
        property_id: 2,
        title: 'HVAC System Not Cooling',
        description: 'Air conditioning unit not producing cold air. Filter changed but issue persists.',
        priority: 'urgent',
        status: 'in-progress',
        cost: 450,
        scheduled_date: addDays(1),
        contractor: 'CoolAir HVAC Services',
        notes: 'Technician scheduled for tomorrow morning',
        created_at: subtractDays(2),
        updated_at: subtractDays(1)
      },
      {
        id: 3,
        property_id: 1,
        title: 'Broken Window Pane',
        description: 'Living room window has a crack. Needs replacement before winter.',
        priority: 'high',
        status: 'pending',
        cost: 250,
        scheduled_date: addDays(5),
        contractor: 'Crystal Clear Windows',
        created_at: subtractDays(5),
        updated_at: subtractDays(5)
      },
      {
        id: 4,
        property_id: 3,
        title: 'Garage Door Opener Repair',
        description: 'Garage door opener stopped working. Remote batteries replaced but no improvement.',
        priority: 'low',
        status: 'completed',
        cost: 120,
        scheduled_date: subtractDays(1),
        contractor: 'Door Masters',
        notes: 'Motor needed replacement. Completed successfully.',
        created_at: subtractDays(10),
        updated_at: subtractDays(1)
      },
      {
        id: 5,
        property_id: 2,
        title: 'Clogged Bathroom Drain',
        description: 'Master bathroom shower drain is slow. Water pools during showers.',
        priority: 'medium',
        status: 'in-progress',
        cost: 85,
        scheduled_date: addDays(0),
        contractor: 'ABC Plumbing',
        notes: 'Same day service requested',
        created_at: subtractDays(1),
        updated_at: subtractDays(1)
      },
      {
        id: 6,
        property_id: 1,
        title: 'Painting Exterior Trim',
        description: 'Front porch trim needs repainting. Wood is starting to show weather damage.',
        priority: 'low',
        status: 'pending',
        cost: 300,
        scheduled_date: addDays(14),
        contractor: "Pro Painters",
        notes: 'Scheduled for next month when weather improves',
        created_at: subtractDays(7),
        updated_at: subtractDays(7)
      },
      {
        id: 7,
        property_id: 3,
        title: 'Replace Smoke Detector Batteries',
        description: 'Annual smoke detector battery replacement for all units.',
        priority: 'medium',
        status: 'completed',
        cost: 30,
        scheduled_date: subtractDays(3),
        contractor: undefined,
        notes: 'Completed by property manager',
        created_at: subtractDays(4),
        updated_at: subtractDays(3)
      },
      {
        id: 8,
        property_id: 2,
        title: 'Roof Leak Inspection',
        description: 'Tenant reports water stain on ceiling after recent heavy rain.',
        priority: 'urgent',
        status: 'pending',
        cost: 500,
        scheduled_date: addDays(1),
        contractor: 'A+ Roofing',
        notes: 'Emergency inspection needed ASAP',
        created_at: subtractDays(1),
        updated_at: subtractDays(1)
      }
    ]
  }

  return {
    requests,
    currentRequest,
    loading,
    error,
    fetchRequests,
    createRequest,
    updateRequest,
    deleteRequest,
    generateSampleRequests
  }
})
