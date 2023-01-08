import Config from '@/core/config'
import type { NextApiRequest, NextApiResponse } from 'next'
import fetch from 'node-fetch'
import cookie from 'cookie'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'

import { ISession } from '@/components/organisms/AuthContext'

dayjs.extend(dayjsUtc)

export default async function authorize(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (!('code' in req.query)) {
    // This shouldn't happen, if it does then try to authenticate again
    return res.redirect(
      `${Config.apiUri}/oauth/authorize?response_type=code&client_id=${Config.clientId}&redirect_uri=${Config.redirectUri}`
    )
  }

  const authorizationCode = req.query.code as string
  const controller = new AbortController()
  const timeout = setTimeout(() => {
    controller.abort()
  }, 150)

  try {
    const response = await fetch(`${Config.apiUri}/oauth/token`, {
      method: 'POST',
      signal: controller.signal,
      headers: {
        'content-type': 'application/x-www-form-urlencoded',
      },
      body: `grant_type=authorization_code&client_id=${Config.clientId}&client_secret=${Config.clientSecret}&redirect_uri=${Config.redirectUri}&code=${authorizationCode}`,
    })

    const data = (await response.json()) as ISession

    return res
      .setHeader(
        'Set-Cookie',
        cookie.serialize('orbit-session', JSON.stringify(data), {
          path: '/',
          expires: dayjs.unix(data.expires_at).toDate(),
          domain: new URL(Config.fqdn!).hostname,
        })
      )
      .redirect(Config.fqdn || '/')
  } catch (error) {
    return res.redirect(Config.fqdn || '/')
  } finally {
    clearTimeout(timeout)
  }
}
