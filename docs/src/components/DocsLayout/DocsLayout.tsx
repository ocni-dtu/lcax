import { ReactNode } from 'react'
import { Group, Stack, TableOfContents, Text } from '@mantine/core'
import { Sidebar } from '@/components'

export const DocsLayout = ({ children }: { children: ReactNode }) => {
  return (
    <Group align='start'>
      <Sidebar />
      {children}
      <Stack mih='100vh' miw={{ base: '15rem' }} justify='start' m='xl'>
        <Text fw={700}>On this page</Text>
        <TableOfContents
          variant='filled'
          size='sm'
          radius='sm'
          getControlProps={({ data }) => ({
            onClick: () => data.getNode().scrollIntoView(),
            children: data.value,
          })}
        />
      </Stack>
    </Group>
  )
}
