import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/sign-in',
      name: 'login',
      component: () => import('@/views/SignInView.vue')
    },
    {
      path: '/sign-up',
      name: 'register',
      component: () => import('@/views/SignUpView.vue')
    }
  ]
})

export default router
