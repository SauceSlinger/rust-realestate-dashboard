import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export interface Property {
  id?: number;
  address: string;
  city: string;
  state: string;
  zip_code: string;
  property_type: string;
  bedrooms: number;
  bathrooms: number;
  square_feet: number;
  purchase_price: number;
  current_value: number;
  status: string;
  notes?: string;
  created_at?: string;
  updated_at?: string;
}

export interface Reminder {
  id?: number;
  property_id?: number;
  title: string;
  description?: string;
  due_date: string;
  completed: boolean;
  reminder_type: string;
  created_at?: string;
}

export interface MarketData {
  id?: number;
  location: string;
  median_price: number;
  average_price_per_sqft: number;
  inventory_count: number;
  days_on_market: number;
  data_date: string;
  source: string;
  created_at?: string;
}

export interface Tenant {
  id?: number;
  property_id: number;
  name: string;
  email: string;
  phone: string;
  lease_start: string;
  lease_end: string;
  monthly_rent: number;
  deposit: number;
  status: string;
  created_at?: string;
}

export interface TrendPoint {
  date: string;
  median_price: number;
  average_price_per_sqft: number;
}

export interface MarketTrend {
  location: string;
  trend_data: TrendPoint[];
}

// Property API
export const propertyApi = {
  getAll: () => api.get<Property[]>('/properties'),
  getById: (id: number) => api.get<Property>(`/properties/${id}`),
  create: (property: Omit<Property, 'id' | 'created_at' | 'updated_at'>) => 
    api.post<number>('/properties', property),
  update: (id: number, property: Partial<Property>) => 
    api.put(`/properties/${id}`, property),
  delete: (id: number) => api.delete(`/properties/${id}`),
};

// Reminder API
export const reminderApi = {
  getAll: () => api.get<Reminder[]>('/reminders'),
  create: (reminder: Omit<Reminder, 'id' | 'completed' | 'created_at'>) => 
    api.post<number>('/reminders', reminder),
  update: (id: number, reminder: Partial<Reminder>) => 
    api.put(`/reminders/${id}`, reminder),
  delete: (id: number) => api.delete(`/reminders/${id}`),
};

// Market Data API
export const marketDataApi = {
  getAll: (location?: string) => 
    api.get<MarketData[]>('/market-data', { params: { location } }),
  create: (data: Omit<MarketData, 'id' | 'created_at'>) => 
    api.post<number>('/market-data', data),
  getTrends: (location?: string) => 
    api.get<MarketTrend[]>('/market-data/trends', { params: { location } }),
};

// Scraper API
export const scraperApi = {
  scrapeMarketData: () => api.post('/scrape'),
};

// Tenant API
export const tenantApi = {
  getAll: (property_id?: number) => 
    api.get<Tenant[]>('/tenants', { params: { property_id } }),
  create: (tenant: Omit<Tenant, 'id' | 'created_at'>) => 
    api.post<number>('/tenants', tenant),
  update: (id: number, tenant: Partial<Tenant>) => 
    api.put(`/tenants/${id}`, tenant),
  delete: (id: number) => api.delete(`/tenants/${id}`),
};

export default api;
