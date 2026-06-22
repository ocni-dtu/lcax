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
import { useDisclosure } from '@mantine/hooks'
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
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xl: rem(100) })
  const [mobileOpened, { toggle: toggleMobile }] = useDisclosure()

  return (
    <>
      <Helmet key={title}>
        <title>{title} | LCAx Docs</title>
        <meta name='description' content={description} />
      </Helmet>
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
    </>
  )
}

const TableOfContents = () => {
  const { pathname } = useLocation()
  return (
    <Stack mih='100vh' w={{ base: '15rem' }} justify='start' m='xl'>
      <Text fw={700}>On this page</Text>
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
