import { Text, Title } from '@mantine/core'
import { useLocation } from 'react-router'

import { DocsLayout } from '@/components'
import { createSlugMap } from '@/lib'

import { conceptIndex } from './pages'

export const ConceptPage = () => {
  const location = useLocation()
  const slugMap = createSlugMap(conceptIndex.items)

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
