export interface Property {
  id: number
  title: string
  address: string
  city: string
  state: string
  zip_code: string
  property_type: string
  bedrooms?: number
  bathrooms?: number
  square_feet?: number
  purchase_price?: number
  current_value?: number
  monthly_rent?: number
  status: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface CreateProperty {
  title: string
  address: string
  city: string
  state: string
  zip_code: string
  property_type: string
  bedrooms?: number
  bathrooms?: number
  square_feet?: number
  purchase_price?: number
  current_value?: number
  monthly_rent?: number
  status: string
  notes?: string
}

export interface Tenant {
  id: number
  property_id: number
  first_name: string
  last_name: string
  email?: string
  phone?: string
  lease_start: string
  lease_end: string
  monthly_rent: number
  deposit_amount?: number
  status: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface CalendarEvent {
  id: number
  title: string
  description?: string
  event_type: string
  property_id?: number
  start_time: string
  end_time?: string
  reminder_minutes?: number
  completed: boolean
  created_at: string
  updated_at: string
}

export interface MaintenanceRecord {
  id: number
  property_id: number
  title: string
  description?: string
  priority: string
  status: string
  cost?: number
  scheduled_date?: string
  completed_date?: string
  contractor?: string
  notes?: string
  created_at: string
  updated_at: string
}

export interface MarketData {
  id: number
  location: string
  median_price?: number
  average_price?: number
  inventory_count?: number
  days_on_market?: number
  price_change_percent?: number
  data_source: string
  recorded_date: string
  created_at: string
}

export interface TrendPoint {
  date: string
  median_price?: number
  inventory_count?: number
}

export interface TrendData {
  location: string
  time_series: TrendPoint[]
}

export interface MarketAnalytics {
  total_properties: number
  total_value: number
  average_rent: number
  occupancy_rate: number
  market_trends: TrendData[]
}
