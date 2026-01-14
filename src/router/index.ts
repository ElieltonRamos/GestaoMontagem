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
          path: 'listagem',
          name: 'listagem',
          component: () => import('@/views/Listagem.vue'),
        },
        {
          path: 'cadastrar-montador',
          name: 'cadastrar-montador',
          component: () => import('@/views/CadastrarMontador.vue')
        },
        {
          path: 'registrar-montagem',
          name: 'registrar-montagem',
          component: () => import('@/views/RegistrarMontagem.vue')
        }
      ],
    },
  ],
});

export default router;
