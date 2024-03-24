import { get_oauth2_endpoint } from '@/common/utils/oauth2'
import { AuthTypes } from '@/common/enum/auth'

const auth0_client_id = import.meta.env.VITE_AUTH0_CLIENT_ID
const auth0_auth_server = import.meta.env.VITE_AUTH0_AUTH_SERVER
const auth0_audience = import.meta.env.VITE_AUTH0_AUDIENCE

const URLS = {
  OAUTH2: {
    AUTH0: {
      CLIENT_ID: auth0_client_id,
      AUDIENCE: auth0_audience,
      AUTH: get_oauth2_endpoint({
        auth_server: auth0_auth_server,
        type: AuthTypes.auth0,
        client_id: auth0_client_id,
        audience: auth0_audience
      })
    }
  }
} as const

export { URLS }
