import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '../views/DashboardView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView,
    },
    {
      path: '/properties',
      name: 'properties',
      component: () => import('../views/PropertiesView.vue'),
    },
    {
      path: '/reminders',
      name: 'reminders',
      component: () => import('../views/RemindersView.vue'),
    },
    {
      path: '/tenants',
      name: 'tenants',
      component: () => import('../views/TenantsView.vue'),
    },
    {
      path: '/market-data',
      name: 'market-data',
      component: () => import('../views/MarketDataView.vue'),
    },
  ],
})

export default router
