import { getCode, getDocsContent, Metadata } from '@/lib'
import { DocsPage } from '@/components/DocsPage'

export const generateStaticParams = () => {
  return getDocsContent(['reference']).map((post) => ({ slug: post.slug }))
}

interface PageProps {
  params: Promise<{
    slug: string
  }>
}

const Page = async ({ params }: PageProps) => {
  const awaitedParams = await params
  const post = getDocsContent(['reference']).find((post) => post.slug === awaitedParams.slug)
  const scope = getCode()

  return (
    <DocsPage
      post={
        post
          ? {
              ...post,
              metadata: { title: 'Changelog', description: 'A reference of changes made to LCAx' } as Metadata,
            }
          : undefined
      }
      scope={scope}
    />
  )
}

export default Page
