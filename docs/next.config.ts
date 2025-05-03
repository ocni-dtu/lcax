import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
    output: "export" as "export" | "standalone" | undefined,
    distDir: 'dist',
    // Configure `pageExtensions` to include markdown and MDX files
    // pageExtensions: ['js', 'jsx', 'md', 'mdx', 'ts', 'tsx'],
    // Optionally, add any other Next.js config below
}

export default nextConfig