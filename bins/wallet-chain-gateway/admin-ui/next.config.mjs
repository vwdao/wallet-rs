/** @type {import('next').NextConfig} */
const gateway = process.env.GATEWAY_URL || 'http://127.0.0.1:8080'

const nextConfig = {
  output: 'export',
  trailingSlash: true,
  basePath: '/admin',
  async rewrites() {
    return [
      { source: '/admin/login', destination: `${gateway}/admin/login`, basePath: false },
      { source: '/admin/endpoints', destination: `${gateway}/admin/endpoints`, basePath: false },
      { source: '/admin/endpoints/:path*', destination: `${gateway}/admin/endpoints/:path*`, basePath: false },
      { source: '/admin/networks', destination: `${gateway}/admin/networks`, basePath: false },
      { source: '/admin/keys', destination: `${gateway}/admin/keys`, basePath: false },
      { source: '/admin/keys/:path*', destination: `${gateway}/admin/keys/:path*`, basePath: false },
      { source: '/admin/stats', destination: `${gateway}/admin/stats`, basePath: false },
      { source: '/admin/stats/:path*', destination: `${gateway}/admin/stats/:path*`, basePath: false },
      { source: '/admin/settings', destination: `${gateway}/admin/settings`, basePath: false },
      { source: '/admin/settings/:path*', destination: `${gateway}/admin/settings/:path*`, basePath: false },
    ]
  },
}

export default nextConfig
