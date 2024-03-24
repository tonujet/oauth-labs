import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
  fetchLogin,
  fetchLoginAuth0,
  fetchRefresh,
  fetchRegister,
  fetchUserInfo
} from '@/service/auth'
import type { LoginDto, RegisterDto, Tokens, User } from '@/common/interface/auth'
import { useRoute, useRouter } from 'vue-router'
import { AuthTypes } from '@/common/enum/auth'
import { routes } from '@/common/enum/routes'

export const userStore = defineStore(
  'user',
  () => {
    const router = useRouter()

    const user = ref<User | null>()
    const tokens = ref<Tokens | null>()

    const loading = ref(false)
    const isError = ref(false)
    const errorMessage = ref('')
    const logged = ref(false)

    const auth_req = async <T>(
      token_req: (params: T) => Promise<Tokens>,
      params: T | (() => T)
    ): Promise<void> => {
      isError.value = false
      loading.value = true
      try {
        if (typeof params === 'function') params = (params as () => T)()
        tokens.value = await token_req(params)
        user.value = await fetchUserInfo(tokens.value.access_token)
        logged.value = true
        await router.push(routes.USER.HOME)
      } catch (e: unknown) {
        if (e instanceof Error) errorMessage.value = e.message
        isError.value = true
      } finally {
        loading.value = false
      }
    }
    const login = (dto: LoginDto) => auth_req(fetchLogin, dto)
    const register = (dto: RegisterDto) => auth_req(fetchRegister, dto)
    const loginAuth2 = () =>
      auth_req(fetchLoginAuth0, () => {
        const code = useRoute().query.code
        if (typeof code !== 'string') throw new Error('Wrong query during oauth validation')
        return code
      })

    const logout = async () => {
      if (!logged.value) return
      logged.value = false
      user.value = null
      tokens.value = null
      isError.value = false
      loading.value = false
      await router.push(routes.USER.HOME)
    }

    const refresh = async () => {
      if (!logged.value || !tokens.value || loading.value) return
      loading.value = true
      try {
        const expiration_time = tokens.value.expiration_time
        const isExpired = Date.now() - expiration_time > 0
        console.log(`Expiration time: ${expiration_time}`)
        console.log(`Current time: ${Date.now()}`)
        console.log(`Is expired?: ${isExpired}`)
        if (tokens.value && isExpired) {
          tokens.value = await fetchRefresh(tokens.value)
        }
      } catch (e) {
        if (e instanceof Error) errorMessage.value = e.message
        isError.value = true
        await logout()
        await router.push(routes.USER.LOGIN)
      } finally {
        loading.value = false
      }
    }

    const auth2TypesToFetchLogin = {
      [AuthTypes.default]: () => {
        throw new Error(`Can't be handled`)
      },
      [AuthTypes.auth0]: loginAuth2
    }

    return {
      login,
      refresh,
      logout,
      register,
      user,
      tokens,
      loading,
      isError,
      logged,
      auth2TypesToFetchLogin
    }
  },
  {
    persist: {
      paths: ['user', 'tokens', 'logged']
    }
  }
)
