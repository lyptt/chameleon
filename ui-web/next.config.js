/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
  sassOptions: {
    additionalData: !!process.env.SHOW_BREAKPOINTS
      ? `$mq-show-breakpoints: (mobile, tablet, desktop, wide);`
      : '',
  },
}

module.exports = nextConfig
