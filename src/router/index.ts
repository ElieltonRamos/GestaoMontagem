import { createRouter, createWebHistory } from 'vue-router'
import HomeLayout from '../layouts/HomeLayout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: HomeLayout,
      children: [
        {
          path: '',
          name: 'dashboard',
          component: () => import('@/views/Dashboard.vue')
        },
        {
          path: 'listing',
          name: 'listing',
          component: () => import('@/views/Listing.vue')
        },
        {
          path: 'register-assembler',
          name: 'register-assembler',
          component: () => import('@/views/RegisterAssembler.vue')
        },
        {
          path: 'register-assembly',
          name: 'register-assembly',
          component: () => import('@/views/RegisterAssembly.vue')
        }
      ]
    }
  ]
})

export default router
