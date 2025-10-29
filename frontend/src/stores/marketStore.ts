import { defineStore } from 'pinia'
import { ref } from 'vue'
import { marketService } from '@/services/api'
import type { MarketAnalytics, TrendData, TrendPoint } from '@/types'
import { FreeTierCache } from '@/utils/freeTierCache'

const ANALYTICS_CACHE_KEY = 'market_analytics'
const TRENDS_CACHE_KEY = 'market_trends'

export const useMarketStore = defineStore('market', () => {
  const analytics = ref<MarketAnalytics | null>(null)
  const trends = ref<TrendData[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const lastFetchDate = ref<string | null>(null)

  // Generate sample analytics data for demo mode
  function generateSampleAnalytics(): MarketAnalytics {
    return {
      total_properties: 8,
      total_value: 2450000,
      average_rent: 2100,
      occupancy_rate: 87.5,
      market_trends: []
    }
  }

  // Generate sample trend data with realistic market fluctuations
  function generateSampleTrends(): TrendData[] {
    const today = new Date()
    const locations = [
      { name: 'San Francisco, CA', basePrice: 1200000 },
      { name: 'Austin, TX', basePrice: 450000 },
      { name: 'Seattle, WA', basePrice: 850000 }
    ]

    return locations.map(location => {
      const timeSeries: TrendPoint[] = []
      let currentPrice = location.basePrice

      // Generate 12 months of data with realistic fluctuations
      for (let i = 11; i >= 0; i--) {
        const date = new Date(today)
        date.setMonth(date.getMonth() - i)
        
        // Add realistic market fluctuation (-2% to +3% per month)
        const fluctuation = (Math.random() * 0.05 - 0.02)
        currentPrice = currentPrice * (1 + fluctuation)

        timeSeries.push({
          date: date.toISOString().split('T')[0],
          median_price: Math.round(currentPrice),
          inventory_count: Math.floor(Math.random() * 200 + 150)
        })
      }

      return {
        location: location.name,
        time_series: timeSeries
      }
    })
  }

  async function fetchAnalytics(forceRefresh = false) {
    // Check cache first (free tier optimization)
    if (!forceRefresh) {
      const cached = FreeTierCache.get<MarketAnalytics>(ANALYTICS_CACHE_KEY)
      if (cached) {
        analytics.value = cached
        return
      }
    }

    loading.value = true
    error.value = null
    try {
      // Try to fetch from backend
      const response = await marketService.getAnalytics()
      analytics.value = response.data
      
      // Cache the result
      FreeTierCache.set(ANALYTICS_CACHE_KEY, response.data)
      lastFetchDate.value = new Date().toISOString()
    } catch (e: any) {
      // Fallback to sample data if backend unavailable (free tier friendly)
      console.warn('Backend unavailable, using sample data:', e.message)
      analytics.value = generateSampleAnalytics()
      
      // Cache sample data for the week
      FreeTierCache.set(ANALYTICS_CACHE_KEY, analytics.value)
      error.value = null // Don't show error in demo mode
    } finally {
      loading.value = false
    }
  }

  async function fetchTrends(forceRefresh = false) {
    // Check cache first (free tier optimization)
    if (!forceRefresh) {
      const cached = FreeTierCache.get<TrendData[]>(TRENDS_CACHE_KEY)
      if (cached) {
        trends.value = cached
        return
      }
    }

    loading.value = true
    error.value = null
    try {
      // Try to fetch from backend
      const response = await marketService.getTrends()
      trends.value = response.data
      
      // Cache the result
      FreeTierCache.set(TRENDS_CACHE_KEY, response.data)
      lastFetchDate.value = new Date().toISOString()
    } catch (e: any) {
      // Fallback to sample data if backend unavailable (free tier friendly)
      console.warn('Backend unavailable, using sample data:', e.message)
      trends.value = generateSampleTrends()
      
      // Cache sample data for the week
      FreeTierCache.set(TRENDS_CACHE_KEY, trends.value)
      error.value = null // Don't show error in demo mode
    } finally {
      loading.value = false
    }
  }

  async function triggerScrape() {
    loading.value = true
    error.value = null
    try {
      await marketService.triggerScrape()
      
      // Clear cache to force fresh data on next fetch
      FreeTierCache.remove(ANALYTICS_CACHE_KEY)
      FreeTierCache.remove(TRENDS_CACHE_KEY)
    } catch (e: any) {
      // In demo mode, just regenerate sample data
      console.warn('Backend unavailable for scrape, regenerating sample data')
      
      // Clear cache and regenerate
      FreeTierCache.remove(ANALYTICS_CACHE_KEY)
      FreeTierCache.remove(TRENDS_CACHE_KEY)
      
      analytics.value = generateSampleAnalytics()
      trends.value = generateSampleTrends()
      
      FreeTierCache.set(ANALYTICS_CACHE_KEY, analytics.value)
      FreeTierCache.set(TRENDS_CACHE_KEY, trends.value)
      
      error.value = null // Don't show error in demo mode
    } finally {
      loading.value = false
    }
  }

  // Check if it's Monday and auto-refresh if needed
  function checkAndRefresh() {
    if (FreeTierCache.isMonday() && FreeTierCache.shouldFetch(ANALYTICS_CACHE_KEY)) {
      console.log('Monday refresh: fetching fresh market data')
      fetchAnalytics(true)
      fetchTrends(true)
    }
  }

  return {
    analytics,
    trends,
    loading,
    error,
    lastFetchDate,
    fetchAnalytics,
    fetchTrends,
    triggerScrape,
    checkAndRefresh,
    generateSampleAnalytics,
    generateSampleTrends
  }
})
