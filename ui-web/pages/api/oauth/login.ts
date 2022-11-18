import Config from '@/core/config'
import type { NextApiRequest, NextApiResponse } from 'next'

export default function login(_req: NextApiRequest, res: NextApiResponse) {
  return res.redirect(
    `${Config.apiUri}/oauth/authorize?response_type=code&client_id=${Config.clientId}&redirect_uri=${Config.redirectUri}`
  )
}
