<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend)

interface Props {
  months: string[]
  payments: number[]
  expected: number
}

const props = defineProps<Props>()

const chartData = computed(() => ({
  labels: props.months,
  datasets: [
    {
      label: 'Payments Received',
      data: props.payments,
      borderColor: 'rgb(34, 197, 94)',
      backgroundColor: 'rgba(34, 197, 94, 0.1)',
      tension: 0.4,
      pointRadius: 4,
      pointHoverRadius: 6
    },
    {
      label: 'Expected Payment',
      data: Array(props.months.length).fill(props.expected),
      borderColor: 'rgb(156, 163, 175)',
      backgroundColor: 'transparent',
      borderDash: [5, 5],
      tension: 0,
      pointRadius: 0
    }
  ]
}))

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'top' as const
    },
    tooltip: {
      callbacks: {
        label: function(context: any) {
          let label = context.dataset.label || ''
          if (label) {
            label += ': '
          }
          if (context.parsed.y !== null) {
            label += new Intl.NumberFormat('en-US', {
              style: 'currency',
              currency: 'USD'
            }).format(context.parsed.y)
          }
          return label
        }
      }
    }
  },
  scales: {
    y: {
      beginAtZero: true,
      ticks: {
        callback: function(value: any) {
          return '$' + value.toLocaleString()
        }
      }
    }
  }
}
</script>

<template>
  <div class="h-64">
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>
