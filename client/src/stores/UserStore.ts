import { defineStore } from 'pinia'
import { ref } from 'vue'
import { fetch_login, fetch_refresh, fetch_register, fetch_user_info } from '@/service/auth'
import type { LoginDto, RegisterDto, Tokens, User } from '@/common/interface/auth'
import { useRouter } from 'vue-router'

export const userStore = defineStore(
  'user',
  () => {
    const router = useRouter()

    const user = ref<User | null>()
    const tokens = ref<Tokens | null>()

    const loading = ref(false)
    const is_error = ref(false)
    const logged = ref(false)

    const login = async (dto: LoginDto) => {
      is_error.value = false
      loading.value = true

      try {
        tokens.value = await fetch_login(dto)
        user.value = await fetch_user_info(tokens.value.access_token)
        logged.value = true
        await router.push('/')
      } catch (e) {
        is_error.value = true
      } finally {
        loading.value = false
      }
    }

    const register = async (dto: RegisterDto) => {
      loading.value = true
      is_error.value = false

      try {
        tokens.value = await fetch_register(dto)
        user.value = await fetch_user_info(tokens.value.access_token)
        logged.value = true
        await router.push('/')
      } catch (e) {
        is_error.value = true
      } finally {
        loading.value = false
      }
    }

    const isTokenExpired = () => {
      if (!tokens.value) throw Error('There isn any token')
      const decodedToken = JSON.parse(atob(tokens.value.access_token.split('.')[1]))
      const expirationTime = decodedToken.exp
      const expirationDate = new Date(expirationTime * 1000)
      const formattedExpirationDate = expirationDate.toLocaleString()
      console.log('Token expires at:', formattedExpirationDate)

      if (expirationTime) {
        const currentTime = Math.floor(Date.now() / 1000)
        return expirationTime < currentTime
      } else {
        return false
      }
    }

    const logout = () => {
      logged.value = false
      user.value = null
      tokens.value = null
      is_error.value = false
      loading.value = false
      router.push('/')
    }

    const refresh = async () => {
      if (!logged.value) return
      try {
        const is_expired = isTokenExpired()
        if (tokens.value && is_expired) {
          tokens.value = await fetch_refresh(tokens.value)
        }
      } catch (e) {
        logout()
        await router.push('/sign-in')
      }
    }

    return { login, refresh, logout, register, user, tokens, loading, is_error, logged }
  },
  {
    persist: {
      paths: ['user', 'tokens', 'logged']
    }
  }
)
