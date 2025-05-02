/** @type {import('next').NextConfig} */
const nextConfig = {
    output: "export" as "export" | "standalone" | undefined,
    distDir: 'dist',
    // Configure `pageExtensions` to include markdown and MDX files
    pageExtensions: ['js', 'jsx', 'md', 'mdx', 'ts', 'tsx'],
    // Optionally, add any other Next.js config below
}