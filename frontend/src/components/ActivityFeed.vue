<script setup lang="ts">
import { computed } from 'vue'
import { formatDistanceToNow } from 'date-fns'

interface Activity {
  id: number
  type: 'payment' | 'maintenance' | 'tenant' | 'property'
  title: string
  description: string
  timestamp: Date
  icon: string
}

interface Props {
  activities: Activity[]
  maxItems?: number
}

const props = withDefaults(defineProps<Props>(), {
  maxItems: 10
})

const displayActivities = computed(() => 
  props.activities.slice(0, props.maxItems)
)

function getActivityColor(type: string): string {
  const colors: Record<string, string> = {
    payment: 'bg-green-100 text-green-800',
    maintenance: 'bg-red-100 text-red-800',
    tenant: 'bg-blue-100 text-blue-800',
    property: 'bg-purple-100 text-purple-800'
  }
  return colors[type] || 'bg-gray-100 text-gray-800'
}

function formatTime(date: Date): string {
  return formatDistanceToNow(date, { addSuffix: true })
}
</script>

<template>
  <div class="card">
    <h2 class="text-xl font-bold text-gray-900 dark:text-slate-100 mb-4">Recent Activity</h2>
    
    <div v-if="activities.length === 0" class="text-center py-8">
      <p class="text-gray-500 dark:text-slate-400">No recent activity</p>
    </div>
    
    <div v-else class="space-y-4">
      <div
        v-for="activity in displayActivities"
        :key="activity.id"
        class="flex items-start space-x-3 p-3 rounded-lg hover:bg-gray-50 dark:hover:bg-slate-800 transition-colors"
      >
        <div 
          class="flex-shrink-0 w-10 h-10 rounded-full flex items-center justify-center text-xl"
          :class="getActivityColor(activity.type)"
        >
          {{ activity.icon }}
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-gray-900 dark:text-slate-100">
            {{ activity.title }}
          </p>
          <p class="text-sm text-gray-600 dark:text-slate-400 mt-1">
            {{ activity.description }}
          </p>
          <p class="text-xs text-gray-500 dark:text-slate-500 mt-1">
            {{ formatTime(activity.timestamp) }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
