// Free tier cache utility - minimizes API calls to once per week
export interface CacheEntry<T> {
  data: T
  timestamp: number
  expiresAt: number
}

const WEEK_IN_MS = 7 * 24 * 60 * 60 * 1000

export class FreeTierCache {
  private static getStorageKey(key: string): string {
    return `cache_${key}`
  }

  /**
   * Get cached data if valid, otherwise return null
   */
  static get<T>(key: string): T | null {
    try {
      const raw = localStorage.getItem(this.getStorageKey(key))
      if (!raw) return null

      const entry: CacheEntry<T> = JSON.parse(raw)
      const now = Date.now()

      // Check if cache is still valid
      if (now < entry.expiresAt) {
        return entry.data
      }

      // Cache expired, remove it
      this.remove(key)
      return null
    } catch (error) {
      console.error('Cache get error:', error)
      return null
    }
  }

  /**
   * Set cache entry with weekly expiration (default)
   * Can be overridden for testing
   */
  static set<T>(key: string, data: T, ttlMs: number = WEEK_IN_MS): void {
    try {
      const now = Date.now()
      const entry: CacheEntry<T> = {
        data,
        timestamp: now,
        expiresAt: now + ttlMs
      }

      localStorage.setItem(this.getStorageKey(key), JSON.stringify(entry))
    } catch (error) {
      console.error('Cache set error:', error)
    }
  }

  /**
   * Remove specific cache entry
   */
  static remove(key: string): void {
    try {
      localStorage.removeItem(this.getStorageKey(key))
    } catch (error) {
      console.error('Cache remove error:', error)
    }
  }

  /**
   * Check if we should fetch fresh data
   * Returns true if cache is missing or expired
   */
  static shouldFetch(key: string): boolean {
    return this.get(key) === null
  }

  /**
   * Get days until next Monday for API refresh scheduling
   */
  static daysUntilMonday(): number {
    const now = new Date()
    const dayOfWeek = now.getDay()
    // 0 = Sunday, 1 = Monday, etc.
    const daysUntilMonday = dayOfWeek === 0 ? 1 : dayOfWeek === 1 ? 7 : 8 - dayOfWeek
    return daysUntilMonday
  }

  /**
   * Check if today is Monday (API refresh day)
   */
  static isMonday(): boolean {
    return new Date().getDay() === 1
  }

  /**
   * Clear all cache entries (useful for debugging)
   */
  static clearAll(): void {
    try {
      const keys = Object.keys(localStorage)
      keys.forEach(key => {
        if (key.startsWith('cache_')) {
          localStorage.removeItem(key)
        }
      })
    } catch (error) {
      console.error('Cache clear error:', error)
    }
  }

  /**
   * Get cache metadata for debugging
   */
  static getMetadata(key: string): { age: number; expiresIn: number } | null {
    try {
      const raw = localStorage.getItem(this.getStorageKey(key))
      if (!raw) return null

      const entry: CacheEntry<any> = JSON.parse(raw)
      const now = Date.now()

      return {
        age: now - entry.timestamp,
        expiresIn: entry.expiresAt - now
      }
    } catch (error) {
      return null
    }
  }
}

// ROI Calculator persistence
const ROI_CACHE_KEY = 'roi_calculator_inputs'

export interface ROIInputs {
  purchasePrice: number | null
  currentValue: number | null
  annualRent: number | null
  annualExpenses: number | null
}

export function saveROIInputs(inputs: ROIInputs): void {
  try {
    localStorage.setItem(ROI_CACHE_KEY, JSON.stringify(inputs))
  } catch (error) {
    console.error('Failed to save ROI inputs:', error)
  }
}

export function loadROIInputs(): ROIInputs | null {
  try {
    const raw = localStorage.getItem(ROI_CACHE_KEY)
    if (!raw) return null
    return JSON.parse(raw)
  } catch (error) {
    console.error('Failed to load ROI inputs:', error)
    return null
  }
}
