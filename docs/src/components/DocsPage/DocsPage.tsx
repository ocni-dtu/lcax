import { notFound } from 'next/navigation'
import { Container, Title } from '@mantine/core'
import { baseUrl } from '@/app/sitemap'
import { MDXComponent } from '@/components'
import { Metadata } from '@/lib'

interface DocsPageProps {
  post:
    | {
        metadata: Metadata | null
        slug: string
        content: string
      }
    | undefined
  scope?: Record<string, string>
}

export const DocsPage = ({ post, scope }: DocsPageProps) => {
  if (!post) {
    notFound()
  }
  return (
    <Container my='xl' size='sm'>
      <section>
        <script
          type='application/ld+json'
          suppressHydrationWarning
          dangerouslySetInnerHTML={{
            __html: JSON.stringify({
              '@context': 'https://schema.org',
              '@type': 'BlogPosting',
              headline: post.metadata?.title || 'LCAx Docs',
              description: post.metadata?.description || '',
              url: `${baseUrl}/guides/${post.slug}`,
              author: {
                '@type': 'Person',
                name: 'Christian Kongsgaard',
              },
            }),
          }}
        />
        <Title>{post.metadata?.title || 'LCAx Docs'}</Title>
        <article className='prose'>
          <Container size='sm'>
            <MDXComponent source={post.content} options={{ scope }} />
          </Container>
        </article>
      </section>
    </Container>
  )
}
