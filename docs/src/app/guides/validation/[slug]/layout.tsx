'use client'

import { ReactNode } from 'react'
import { DocsLayout } from '@/components'

export default function Layout({ children }: { children: ReactNode }) {
  return <DocsLayout>{children}</DocsLayout>
}
