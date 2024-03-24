import { AuthEndpoints, UserEndpoints } from '@/common/enum/endpoints'
import type { LoginDto, RegisterDto, Tokens, User } from '@/common/interface/auth'
import { fetchApi } from '@/service'

const fetchLogin = (dto: LoginDto): Promise<Tokens> => {
  return fetchApi(AuthEndpoints.LOGIN, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dto)
  }).then((res) => res.json())
}

const fetchLoginAuth0 = (code: string): Promise<Tokens> => {
  return fetchApi(AuthEndpoints.LOGIN_AUTH0(code), {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  }).then((res) => res.json())
}

const fetchRegister = (dto: RegisterDto): Promise<Tokens> => {
  return fetchApi(AuthEndpoints.REGISTER, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dto)
  }).then((res) => res.json())
}

const fetchRefresh = (tokens: Tokens): Promise<Tokens> => {
  return fetchApi(AuthEndpoints.REFRESH, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(tokens)
  }).then((res) => res.json())
}

const fetchUserInfo = (access_token: string): Promise<User> => {
  return fetchApi(UserEndpoints.INFO, {
    method: 'GET',
    headers: { Authorization: `Bearer ${access_token}` }
  }).then((res) => res.json())
}

export { fetchLogin, fetchRegister, fetchRefresh, fetchUserInfo, fetchLoginAuth0 }
