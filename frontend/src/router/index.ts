import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/views/Dashboard.vue'
import Properties from '@/views/Properties.vue'
import PropertyDetail from '@/views/PropertyDetail.vue'
import Tenants from '@/views/Tenants.vue'
import Calendar from '@/views/Calendar.vue'
import Maintenance from '@/views/Maintenance.vue'
import MarketInsights from '@/views/MarketInsights.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: Dashboard
    },
    {
      path: '/properties',
      name: 'properties',
      component: Properties
    },
    {
      path: '/properties/:id',
      name: 'property-detail',
      component: PropertyDetail
    },
    {
      path: '/tenants',
      name: 'tenants',
      component: Tenants
    },
    {
      path: '/calendar',
      name: 'calendar',
      component: Calendar
    },
    {
      path: '/maintenance',
      name: 'maintenance',
      component: Maintenance
    },
    {
      path: '/market',
      name: 'market',
      component: MarketInsights
    }
  ]
})

export default router
