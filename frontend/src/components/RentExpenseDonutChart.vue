<template>
  <div>
    <component :is="ApexChart" type="donut" :options="options" :series="series" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import ApexChart from 'vue3-apexcharts'

interface Props {
  series: number[]
  labels?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  labels: () => ['Annual Rent', 'Annual Expenses']
})

const options = computed(() => ({
  labels: props.labels,
  legend: { 
    position: 'bottom' as const,
    horizontalAlign: 'center' as const
  },
  dataLabels: { 
    enabled: true,
    formatter: function (val: number) {
      return val.toFixed(0) + '%'
    }
  },
  colors: ['#10b981', '#ef4444'],
  plotOptions: {
    pie: {
      donut: {
        size: '70%',
        labels: {
          show: true,
          total: {
            show: true,
            label: 'Total',
            formatter: function (w: any) {
              const total = w.globals.seriesTotals.reduce((a: number, b: number) => a + b, 0)
              return '$' + total.toLocaleString()
            }
          }
        }
      }
    }
  },
  tooltip: { 
    y: { 
      formatter: (val: number) => `$${val?.toLocaleString()}` 
    }
  }
}))
</script>
