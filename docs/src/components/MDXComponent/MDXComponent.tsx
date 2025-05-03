import { MDXRemote, MDXRemoteProps } from 'next-mdx-remote/rsc'
import { CodeHighlightTabs } from '@/components'
import { Image } from '@mantine/core'

interface ImageProps extends Record<string, unknown> {
  alt: string
}

const components = {
  CodeHighlightTabs,
  img: (props: ImageProps) => <Image {...props} alt={props.alt} h='auto' w={720} />,
}

export const MDXComponent = (props: MDXRemoteProps) => (
  <MDXRemote {...props} components={{ ...components, ...(props.components || {}) }} />
)
