import { Text, Title } from '@mantine/core'
import { useLocation } from 'react-router'

import { DocsLayout } from '@/components'
import { createSlugMap } from '@/lib'

import { referenceIndex } from './pages'

export const ReferencePage = () => {
  const location = useLocation()
  const slugMap = createSlugMap(referenceIndex.items)

  const { Content, title, description } = slugMap[location.pathname]

  return (
    <DocsLayout>
      <Title>{title}</Title>
      <Text size={'sm'} c='gray.5' mb='lg'>
        {description}
      </Text>
      <Content />
    </DocsLayout>
  )
}
