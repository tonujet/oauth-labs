import { ENV } from '@/common/enum/env'
import { userStore } from '@/stores/userStore'

const fetchApi = async (input: string, init?: RequestInit): Promise<Response> => {
  const { refresh } = userStore()
  await refresh()
  const endpoint = `${ENV.API}${input}`
  return await fetch(endpoint, init)
}

export { fetchApi }
