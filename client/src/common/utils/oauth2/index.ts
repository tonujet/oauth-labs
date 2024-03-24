import type { OAuth2Params } from '@/common/interface/auth'

const get_oauth2_endpoint = ({
  auth_server,
  client_id,
  type,
  audience,
  redirect_uri = 'http://localhost:5173/oauth2',
  scope = 'openid profile email offline_access read:current_user'
}: OAuth2Params) => {
  return `${auth_server}/authorize?client_id=${client_id}&redirect_uri=${redirect_uri}&response_type=code&scope=${scope}&state=${type}&audience=${audience}`
}

export { get_oauth2_endpoint }
