import {
  AppShell,
  rem,
  ScrollArea,
  Stack,
  TableOfContents as MantineTableOfContents,
  Text,
  useMatches,
  Burger,
  Container,
} from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { type ReactNode } from 'react'

import { Header, Sidebar, ErrorBoundary } from '@/components'

export const DocsLayout = ({ children }: { children: ReactNode }) => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xl: rem(100) })
  const [mobileOpened, { toggle: toggleMobile }] = useDisclosure()

  return (
    <AppShell
      header={{ height: headerHeight, offset: true }}
      navbar={{
        width: 300,
        breakpoint: 'sm',
        collapsed: { mobile: !mobileOpened, desktop: false },
      }}
      aside={{ width: 300, breakpoint: 'sm', collapsed: { desktop: false, mobile: true } }}
    >
      <AppShell.Header withBorder={true} pl='lg' bg='grey.0'>
        <ErrorBoundary>
          <Header
            MenuComponent={<Burger opened={mobileOpened} onClick={toggleMobile} hiddenFrom='sm' size='sm' />}
            height={headerHeight}
          />
        </ErrorBoundary>
      </AppShell.Header>
      <AppShell.Main pt={`calc(${headerHeight}`} pl={'md'} pr={'md'}>
        <ErrorBoundary>
          <Container>{children}</Container>
        </ErrorBoundary>
      </AppShell.Main>
      <AppShell.Navbar>
        <AppShell.Section grow component={ScrollArea}>
          <ErrorBoundary>
            <Sidebar />
          </ErrorBoundary>
        </AppShell.Section>
      </AppShell.Navbar>
      <AppShell.Aside>
        <AppShell.Section grow component={ScrollArea}>
          <ErrorBoundary>
            <TableOfContents />
          </ErrorBoundary>
        </AppShell.Section>
      </AppShell.Aside>
    </AppShell>
  )
}

const TableOfContents = () => (
  <Stack mih='100vh' w={{ base: '15rem' }} justify='start' m='xl'>
    <Text fw={700}>On this page</Text>
    <MantineTableOfContents
      variant='filled'
      size='sm'
      radius='sm'
      getControlProps={({ data }) => ({
        onClick: () => data.getNode().scrollIntoView(),
        children: data.value,
      })}
    />
  </Stack>
)
