import { createRouter, createWebHistory } from 'vue-router';
import HomeLayout from '../layouts/HomeLayout.vue';

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
          component: () => import('@/views/Dashboard.vue'),
        },
        {
          path: 'listing',
          name: 'listing',
          component: () => import('@/views/Listing.vue'),
        },
        {
          path: 'register-assembler',
          name: 'register-assembler',
          component: () => import('@/views/RegisterAssembler.vue'),
        },
        {
          path: 'assembler/:id',
          name: 'assembler-details',
          component: () => import('@/views/AssemblerDetails.vue'),
        },
        {
          path: 'register-assembly',
          name: 'register-assembly',
          component: () => import('@/views/RegisterAssembly.vue'),
        },
        {
          path: 'assembly-list',
          name: 'assembly-list',
          component: () => import('@/views/AssemblyList.vue'),
        },
        {
          path: 'assembly/:id',
          name: 'assembly-details',
          component: () => import('@/views/AssemblyDetails.vue'),
        },
      ],
    },
  ],
});

export default router;
