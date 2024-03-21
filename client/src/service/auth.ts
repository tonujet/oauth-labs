import { AuthEndpoints, UserEnpoints } from '@/common/enum/endpoints'
import type { Tokens, LoginDto, User, RegisterDto } from '@/common/interface/auth'

const fetch_login = (dto: LoginDto): Promise<Tokens> => {
  return fetch(AuthEndpoints.LOGIN, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dto)
  }).then((res) => res.json())
}

const fetch_register = (dto: RegisterDto): Promise<Tokens> => {
  return fetch(AuthEndpoints.REGISTER, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dto)
  }).then((res) => res.json())
}

const fetch_refresh = (tokens: Tokens): Promise<Tokens> => {
  return fetch(AuthEndpoints.REFRESH, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(tokens)
  }).then((res) => res.json())
}

const fetch_user_info = (access_token: string): Promise<User> => {
  return fetch(UserEnpoints.INFO, {
    method: 'GET',
    headers: { Authorization: `Bearer ${access_token}` }
  }).then((res) => res.json())
}

export { fetch_login, fetch_register, fetch_refresh, fetch_user_info }
