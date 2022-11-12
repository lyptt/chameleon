import { createContext, useContext } from 'react'
import cookie from 'cookie'

export interface ISession {
  access_token: string
  refresh_token: string
  token_type: string
  scope: string
  created_at: number
  expires_at: number
  refresh_expires_at: number
}

export interface IAuthContext {
  authenticated: boolean
  session?: ISession
}

export function buildAuthContext(cookieString: string): IAuthContext {
  const values = cookie.parse(cookieString)

  if ('chameleon-session' in values) {
    return {
      authenticated: true,
      session: JSON.parse(values['chameleon-session']),
    }
  }

  return { authenticated: false }
}

const AuthContext = createContext<IAuthContext>(undefined as any)

export function useAuth() {
  return useContext(AuthContext)
}

export default AuthContext
