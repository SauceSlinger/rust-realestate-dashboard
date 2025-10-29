<template>
  <div>
    <div v-if="series.length === 0" class="text-center py-8">
      <p class="text-gray-500">No performance data available</p>
    </div>
    <div v-else>
      <component :is="ApexChart" type="line" :options="options" :series="series" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import ApexChart from 'vue3-apexcharts'

interface Props {
  series: any[]
  categories?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  categories: () => []
})

const options = computed(() => ({
  chart: { 
    id: 'portfolio-performance', 
    toolbar: { show: true },
    animations: {
      enabled: true,
      easing: 'easeinout',
      speed: 800
    }
  },
  xaxis: { 
    categories: props.categories,
    labels: {
      rotate: -45,
      rotateAlways: false
    }
  },
  stroke: { 
    curve: 'smooth' as const,
    width: 3
  },
  markers: { 
    size: 4,
    hover: {
      size: 6
    }
  },
  colors: ['#3b82f6'],
  tooltip: { 
    y: { 
      formatter: (val: number) => `$${val?.toLocaleString()}` 
    }
  },
  grid: {
    borderColor: '#e5e7eb',
    strokeDashArray: 4
  }
}))
</script>
