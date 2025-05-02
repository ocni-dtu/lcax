import { getDocsContent } from '@/lib'
import { DocsPage } from '@/components/DocsPage'

export const generateStaticParams = () => {
  const posts = getDocsContent(['concept'])

  return posts.map((post) => ({
    slug: post.slug,
  }))
}

interface PageProps {
  params: Promise<{
    slug: string
  }>
}

const Page = async ({ params }: PageProps) => {
  const awaitedParams = await params
  const post = getDocsContent(['concept']).find((post) => post.slug === awaitedParams.slug)

  return <DocsPage post={post} />
}

export default Page
