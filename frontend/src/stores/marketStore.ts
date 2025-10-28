import { defineStore } from 'pinia'
import { ref } from 'vue'
import { marketService } from '@/services/api'
import type { MarketAnalytics, TrendData } from '@/types'

export const useMarketStore = defineStore('market', () => {
  const analytics = ref<MarketAnalytics | null>(null)
  const trends = ref<TrendData[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchAnalytics() {
    loading.value = true
    error.value = null
    try {
      const response = await marketService.getAnalytics()
      analytics.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch analytics'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function fetchTrends() {
    loading.value = true
    error.value = null
    try {
      const response = await marketService.getTrends()
      trends.value = response.data
    } catch (e: any) {
      error.value = e.message || 'Failed to fetch trends'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function triggerScrape() {
    loading.value = true
    error.value = null
    try {
      await marketService.triggerScrape()
    } catch (e: any) {
      error.value = e.message || 'Failed to trigger scrape'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    analytics,
    trends,
    loading,
    error,
    fetchAnalytics,
    fetchTrends,
    triggerScrape
  }
})
