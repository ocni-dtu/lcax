declare module '*.mdx' {
  import { type ReactNode } from 'react'
  export const frontmatter: any
  export default function (props: any): ReactNode
}

declare module '*.md' {
  import { type ReactNode } from 'react'
  export const frontmatter: any
  export default function (props: any): ReactNode
}
