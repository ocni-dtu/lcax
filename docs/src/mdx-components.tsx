import type { MDXComponents } from 'mdx/types'
import NextImage, { ImageProps } from 'next/image'
import { Image } from '@mantine/core'

export function useMDXComponents(components: MDXComponents): MDXComponents {
  return {
    img: (props) => <Image component={NextImage} w='100%' h='auto' {...(props as ImageProps)} alt={props.alt} />,
    ...components,
  }
}
