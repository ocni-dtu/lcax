import { getDocsContent } from '@/lib'

export const baseUrl = `${process.env.DOMAIN?.startsWith('localhost') ? 'http://' : 'https://'}${process.env.DOMAIN}`

export default async function sitemap() {
  const docs = getDocsContent().map((post) => ({
    url: `${baseUrl}/${post.slug}`,
  }))

  return [baseUrl, ...docs]
}
