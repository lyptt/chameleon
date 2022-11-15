const Config = {
  apiUri: process.env.NEXT_PUBLIC_API_URI,
  redirectUri: process.env.OAUTH_REDIRECT_URI,
  clientId: process.env.OAUTH_CLIENT_ID,
  clientSecret: process.env.OAUTH_CLIENT_SECRET,
  fqdn: process.env.NEXT_PUBLIC_FQDN,
  cdn: process.env.NEXT_PUBLIC_CDN,
}

export default Config
