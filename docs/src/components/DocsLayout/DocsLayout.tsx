import {
  AppShell,
  Burger,
  Container,
  rem,
  ScrollArea,
  Stack,
  TableOfContents as MantineTableOfContents,
  Text,
  useMatches,
} from '@mantine/core'
import { useDisclosure, useHeadroom } from '@mantine/hooks'
import { type ReactNode } from 'react'
import { Helmet } from 'react-helmet-async'
import { useLocation } from 'react-router'

import { ErrorBoundary, Header, Sidebar } from '@/components'

export const DocsLayout = ({
  children,
  title,
  description,
}: {
  children: ReactNode
  title?: string
  description?: string
}) => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xxl: rem(100) })
  const pinned = useHeadroom({ fixedAt: 120 })
  const [mobileOpened, { toggle: toggleMobile }] = useDisclosure()
  const _title = `${title} | LCAx Docs`

  return (
    <>
      <Helmet key={title}>
        <title>{_title}</title>
        <meta name='description' content={description} />
      </Helmet>
      <AppShell
        header={{ height: headerHeight, collapsed: !pinned, offset: true }}
        navbar={{
          width: 300,
          breakpoint: 'sm',
          collapsed: { mobile: !mobileOpened, desktop: false },
        }}
        aside={{ width: 300, breakpoint: 'sm', collapsed: { desktop: false, mobile: true } }}
        withBorder={false}
      >
        <AppShell.Header withBorder={false} pl='lg' bg='grey.0'>
          <ErrorBoundary>
            <Header
              MenuComponent={<Burger opened={mobileOpened} onClick={toggleMobile} hiddenFrom='sm' size='sm' />}
              height={headerHeight}
            />
          </ErrorBoundary>
        </AppShell.Header>
        <AppShell.Main>
          <ErrorBoundary>
            <Container fluid bg='grey.0' p={0} mih='100%'>
              <Container py='xl'>{children}</Container>
            </Container>
          </ErrorBoundary>
        </AppShell.Main>
        <AppShell.Navbar withBorder={false} bg='grey.0'>
          <AppShell.Section grow component={ScrollArea}>
            <ErrorBoundary>
              <Sidebar />
            </ErrorBoundary>
          </AppShell.Section>
        </AppShell.Navbar>
        <AppShell.Aside withBorder={false} bg='grey.0'>
          <AppShell.Section grow component={ScrollArea}>
            <ErrorBoundary>
              <TableOfContents />
            </ErrorBoundary>
          </AppShell.Section>
        </AppShell.Aside>
      </AppShell>
    </>
  )
}

const TableOfContents = () => {
  const { pathname } = useLocation()
  return (
    <Stack mih='100vh' w={{ base: '15rem' }} justify='start' m='xl'>
      <Text fw={500}>On this page</Text>
      <MantineTableOfContents
        key={pathname}
        variant='filled'
        size='sm'
        radius='sm'
        getControlProps={({ data }) => {
          return {
            onClick: () => data.getNode().scrollIntoView(),
            children: data.value,
          }
        }}
      />
    </Stack>
  )
}
