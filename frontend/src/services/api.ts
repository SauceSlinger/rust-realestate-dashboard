import axios from 'axios'
import type { Property, CreateProperty, Tenant, CalendarEvent, MaintenanceRecord, MarketAnalytics, TrendData } from '@/types'

const API_BASE_URL = (() => {
  // Prefer build-time environment if set to a non-default value
  const env = import.meta.env.VITE_API_BASE_URL
  if (env && env !== '/api') return env

  // Runtime fallback: when the site is served from GitHub Pages, use the Render backend
  if (typeof window !== 'undefined') {
    const host = window.location.hostname
    if (host === 'sauceslinger.github.io') {
      return 'https://rust-realestate-backend.onrender.com/api'
    }
  }

  // Default to same-origin /api for local development (proxied in vite)
  return '/api'
})()

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json'
  }
})

// Properties
export const propertyService = {
  getAll: () => api.get<Property[]>('/properties'),
  getById: (id: number) => api.get<Property>(`/properties/${id}`),
  create: (data: CreateProperty) => api.post<Property>('/properties', data),
  update: (id: number, data: Partial<CreateProperty>) => api.put<Property>(`/properties/${id}`, data),
  delete: (id: number) => api.delete(`/properties/${id}`)
}

// Tenants
export const tenantService = {
  getAll: () => api.get<Tenant[]>('/tenants'),
  getById: (id: number) => api.get<Tenant>(`/tenants/${id}`),
  create: (data: Partial<Tenant>) => api.post<Tenant>('/tenants', data),
  update: (id: number, data: Partial<Tenant>) => api.put<Tenant>(`/tenants/${id}`, data),
  delete: (id: number) => api.delete(`/tenants/${id}`)
}

// Events
export const eventService = {
  getAll: () => api.get<CalendarEvent[]>('/events'),
  getById: (id: number) => api.get<CalendarEvent>(`/events/${id}`),
  create: (data: Partial<CalendarEvent>) => api.post<CalendarEvent>('/events', data),
  update: (id: number, data: Partial<CalendarEvent>) => api.put<CalendarEvent>(`/events/${id}`, data),
  delete: (id: number) => api.delete(`/events/${id}`)
}

// Maintenance
export const maintenanceService = {
  getAll: () => api.get<MaintenanceRecord[]>('/maintenance'),
  create: (data: Partial<MaintenanceRecord>) => api.post<MaintenanceRecord>('/maintenance', data),
  update: (id: number, data: Partial<MaintenanceRecord>) => api.put<MaintenanceRecord>(`/maintenance/${id}`, data)
}

// Market Data
export const marketService = {
  getTrends: () => api.get<TrendData[]>('/market/trends'),
  getAnalytics: () => api.get<MarketAnalytics>('/market/analytics'),
  triggerScrape: () => api.post('/market/scrape')
}

export default api
