import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import { routes } from '@/common/enum/routes'
import { userStore } from '@/stores/userStore'
import { storeToRefs } from 'pinia'
import { URLS } from '@/common/enum/urls'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: routes.USER.HOME,
      name: 'home',
      component: HomeView
    },
    {
      path: routes.USER.LOGIN,
      name: 'login',
      component: () => import('@/views/SignInView.vue'),
      meta: {
        guestOnly: true
      }
    },
    {
      path: routes.USER.REGISTER,
      name: 'register',
      component: () => import('@/views/SignUpView.vue'),
      meta: {
        guestOnly: true
      }
    },
    {
      path: routes.OAUTH2.LOADING,
      name: 'oauth2',
      component: () => import('@/views/OAuth2View.vue'),
      meta: {
        guestOnly: true
      }
    },
    {
      path: routes.OAUTH2.AUTH0.REDIRECT,
      name: 'auth0',
      component: {},
      meta: {
        guestOnly: true,
        RedirectExternalUrl: URLS.OAUTH2.AUTH0.AUTH
      }
    }
  ]
})

router.beforeEach(async (to, from, next) => {
  const { logged, isError } = storeToRefs(userStore())
  isError.value = false
  if (to.matched.some((record) => record.meta.guestOnly)) {
    if (logged.value) {
      next(routes.USER.HOME)
      return
    }
  }

  if (to.matched.some((record) => record.meta.RedirectExternalUrl)) {
    const url: string = to.meta.RedirectExternalUrl as string
    window.location.replace(url)
    return
  }

  next()
})

export default router
