import { getCode, getDocsContent } from '@/lib'
import { DocsPage } from '@/components/DocsPage'

export const generateStaticParams = () => {
  return getDocsContent(['reference', 'schemas']).map((post) => ({ slug: post.slug }))
}

interface PageProps {
  params: Promise<{
    slug: string
  }>
}

const Page = async ({ params }: PageProps) => {
  const awaitedParams = await params
  const post = getDocsContent(['reference', 'schemas']).find((post) => post.slug === awaitedParams.slug)
  const scope = getCode()

  return <DocsPage post={post} scope={scope} />
}

export default Page
