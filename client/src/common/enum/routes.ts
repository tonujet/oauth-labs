const routes = {
  USER: {
    HOME: '/',
    LOGIN: '/sign-in',
    REGISTER: '/sign-up'
  },
  OAUTH2: {
    AUTH0: {
      REDIRECT: '/redirect/oauth2/auth0'
    },
    LOADING: '/oauth2'
  }
} as const

export { routes }
