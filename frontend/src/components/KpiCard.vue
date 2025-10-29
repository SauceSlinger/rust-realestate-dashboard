<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  title: string
  value: string | number
  trend?: number
  icon?: string
  color?: 'primary' | 'success' | 'warning' | 'danger'
}

const props = withDefaults(defineProps<Props>(), {
  color: 'primary'
})

const colorClasses = computed(() => {
  const colors = {
    primary: 'text-blue-600',
    success: 'text-green-600',
    warning: 'text-yellow-600',
    danger: 'text-red-600'
  }
  return colors[props.color]
})

const trendClass = computed(() => {
  if (!props.trend) return ''
  return props.trend > 0 ? 'text-green-600' : 'text-red-600'
})

const trendIcon = computed(() => {
  if (!props.trend) return ''
  return props.trend > 0 ? '↗' : '↘'
})
</script>

<template>
  <div class="card hover:shadow-lg transition-shadow duration-200">
    <div class="flex items-start justify-between">
      <div class="flex-1">
        <p class="text-sm font-medium text-gray-600 dark:text-slate-400">{{ title }}</p>
        <p class="text-3xl font-bold mt-2" :class="colorClasses">
          {{ value }}
        </p>
        <p v-if="trend !== undefined" class="text-sm mt-2 font-medium" :class="trendClass">
          {{ trendIcon }} {{ Math.abs(trend) }}%
          <span class="text-gray-500 dark:text-slate-400 font-normal">vs last month</span>
        </p>
      </div>
      <div v-if="icon" class="text-4xl opacity-20">
        {{ icon }}
      </div>
    </div>
  </div>
</template>
