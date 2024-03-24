import { AuthTypes } from '@/common/enum/auth'

interface LoginDto {
  login: string
  password: string
}

interface RegisterDto {
  email: string
  password: string
  username: string
}

interface Tokens {
  access_token: string
  refresh_token: string
  expiration_time: number
}

interface User {
  email: string
  nickname: string
  name: string
  created_at: string
  updated_at: string
}

interface OAuth2Params {
  auth_server: string
  client_id: string
  type: keyof typeof AuthTypes
  audience: string
  redirect_uri?: string
  scope?: string
}

export type { LoginDto, RegisterDto, Tokens, User, OAuth2Params }
