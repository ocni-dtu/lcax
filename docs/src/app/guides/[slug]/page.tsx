import { getCode, getDocsContent } from '@/lib'
import { DocsPage } from '@/components/DocsPage'

export const generateStaticParams = () => {
  return getDocsContent(['guides']).map((post) => ({ slug: post.slug }))
}

interface PageProps {
  params: Promise<{
    slug: string
  }>
}

const Page = async ({ params }: PageProps) => {
  const awaitedParams = await params
  const post = getDocsContent(['guides']).find((post) => post.slug === awaitedParams.slug)
  const scope = getCode()

  return <DocsPage post={post} scope={scope} />
}

export default Page
