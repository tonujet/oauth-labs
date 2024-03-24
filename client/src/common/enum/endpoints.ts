const AuthEndpoints = {
  LOGIN: `/auth/login`,
  REFRESH: `/auth/refresh`,
  REGISTER: `/auth/register`,
  LOGIN_AUTH0: (code: string) => `/auth/login/auth0/${code}`
} as const

const UserEndpoints = {
  INFO: `/user/info`
} as const

export { AuthEndpoints, UserEndpoints }
