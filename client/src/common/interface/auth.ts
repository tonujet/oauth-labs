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
}

interface User {
  email: string
  nickname: string
  name: string
  created_at: string
  updated_at: string
}

export type { LoginDto, RegisterDto, Tokens, User }
