import { ENV } from '@/common/enum/env'

const AuthEndpoints = {
  LOGIN: `${ENV.API}/auth/login`,
  REFRESH: `${ENV.API}/auth/refresh`,
  REGISTER: `${ENV.API}/auth/register`
} as const

const UserEnpoints = {
  INFO: `${ENV.API}/user/info`
} as const

export { AuthEndpoints, UserEnpoints }
