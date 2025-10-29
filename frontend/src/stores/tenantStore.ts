import { defineStore } from 'pinia'
import { ref } from 'vue'
import { tenantService } from '@/services/api'
import type { Tenant } from '@/types'

export const useTenantStore = defineStore('tenant', () => {
  const tenants = ref<Tenant[]>([])
  const currentTenant = ref<Tenant | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchTenants() {
    loading.value = true
    error.value = null
    try {
      const response = await tenantService.getAll()
      tenants.value = response.data
    } catch (e) {
      error.value = 'Failed to fetch tenants'
      console.error(e)
      // Set sample data for demo
      tenants.value = generateSampleTenants()
    } finally {
      loading.value = false
    }
  }

  async function fetchTenant(id: number) {
    loading.value = true
    error.value = null
    try {
      const response = await tenantService.getById(id)
      currentTenant.value = response.data
    } catch (e) {
      error.value = 'Failed to fetch tenant'
      console.error(e)
      // Try to find in local array
      currentTenant.value = tenants.value.find(t => t.id === id) || null
    } finally {
      loading.value = false
    }
  }

  function generateSampleTenants(): Tenant[] {
    const today = new Date()
    const futureDate = (months: number) => {
      const d = new Date(today)
      d.setMonth(d.getMonth() + months)
      return d.toISOString().split('T')[0]
    }
    const pastDate = (months: number) => {
      const d = new Date(today)
      d.setMonth(d.getMonth() - months)
      return d.toISOString().split('T')[0]
    }

    return [
      {
        id: 1,
        property_id: 1,
        first_name: 'John',
        last_name: 'Smith',
        email: 'john.smith@email.com',
        phone: '(555) 123-4567',
        lease_start: pastDate(6),
        lease_end: futureDate(6),
        monthly_rent: 1800,
        deposit_amount: 3600,
        status: 'active',
        notes: 'Excellent tenant, always pays on time',
        created_at: pastDate(6),
        updated_at: new Date().toISOString()
      },
      {
        id: 2,
        property_id: 2,
        first_name: 'Sarah',
        last_name: 'Johnson',
        email: 'sarah.j@email.com',
        phone: '(555) 234-5678',
        lease_start: pastDate(12),
        lease_end: futureDate(1),
        monthly_rent: 2200,
        deposit_amount: 4400,
        status: 'active',
        notes: 'Long-term tenant, lease renewal pending',
        created_at: pastDate(12),
        updated_at: new Date().toISOString()
      },
      {
        id: 3,
        property_id: 3,
        first_name: 'Michael',
        last_name: 'Brown',
        email: 'mbrown@email.com',
        phone: '(555) 345-6789',
        lease_start: pastDate(8),
        lease_end: futureDate(4),
        monthly_rent: 1950,
        deposit_amount: 3900,
        status: 'active',
        created_at: pastDate(8),
        updated_at: new Date().toISOString()
      },
      {
        id: 4,
        property_id: 1,
        first_name: 'Emily',
        last_name: 'Davis',
        email: 'emily.davis@email.com',
        phone: '(555) 456-7890',
        lease_start: pastDate(3),
        lease_end: futureDate(9),
        monthly_rent: 1750,
        deposit_amount: 3500,
        status: 'active',
        notes: 'New tenant, settling in well',
        created_at: pastDate(3),
        updated_at: new Date().toISOString()
      },
      {
        id: 5,
        property_id: 2,
        first_name: 'David',
        last_name: 'Wilson',
        email: 'dwilson@email.com',
        phone: '(555) 567-8901',
        lease_start: pastDate(18),
        lease_end: pastDate(2),
        monthly_rent: 2100,
        deposit_amount: 4200,
        status: 'inactive',
        notes: 'Lease expired, moved out',
        created_at: pastDate(18),
        updated_at: new Date().toISOString()
      }
    ]
  }

  async function createTenant(data: Partial<Tenant>) {
    loading.value = true
    error.value = null
    try {
      const response = await tenantService.create(data)
      tenants.value.unshift(response.data)
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to create tenant'
      // Demo mode: create with mock ID
      const newTenant: Tenant = {
        id: Math.max(...tenants.value.map(t => t.id)) + 1,
        ...data,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      } as Tenant
      tenants.value.unshift(newTenant)
      return newTenant
    } finally {
      loading.value = false
    }
  }

  async function updateTenant(id: number, data: Partial<Tenant>) {
    loading.value = true
    error.value = null
    try {
      const response = await tenantService.update(id, data)
      const index = tenants.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tenants.value[index] = response.data
      }
      if (currentTenant.value?.id === id) {
        currentTenant.value = response.data
      }
      return response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to update tenant'
      // Demo mode: update locally
      const index = tenants.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tenants.value[index] = {
          ...tenants.value[index],
          ...data,
          updated_at: new Date().toISOString()
        }
      }
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteTenant(id: number) {
    loading.value = true
    error.value = null
    try {
      await tenantService.delete(id)
      tenants.value = tenants.value.filter(t => t.id !== id)
    } catch (e: any) {
      error.value = e.message || 'Failed to delete tenant'
      // Demo mode: delete locally
      tenants.value = tenants.value.filter(t => t.id !== id)
    } finally {
      loading.value = false
    }
  }

  return {
    tenants,
    currentTenant,
    loading,
    error,
    fetchTenants,
    fetchTenant,
    createTenant,
    updateTenant,
    deleteTenant,
    generateSampleTenants
  }
})
