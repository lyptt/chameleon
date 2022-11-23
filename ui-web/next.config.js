/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
  sassOptions: {
    additionalData: !!process.env.SHOW_BREAKPOINTS
      ? `$mq-show-breakpoints: (mobileMini, mobile, tablet, desktop, wide);`
      : '',
  },

  experimental: {
    scrollRestoration: true,
  },
}

module.exports = nextConfig
