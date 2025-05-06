import { ReactNode } from 'react'
import {
  AppShell,
  rem,
  ScrollArea,
  Stack,
  TableOfContents as MantineTableOfContents,
  Text,
  useMatches,
  Burger,
} from '@mantine/core'
import { Header, Sidebar } from '@/components'
import { useDisclosure } from '@mantine/hooks'

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
        <Header
          MenuComponent={<Burger opened={mobileOpened} onClick={toggleMobile} hiddenFrom='sm' size='sm' />}
          height={headerHeight}
        />
      </AppShell.Header>
      <AppShell.Main pt={`calc(${headerHeight}`}>{children}</AppShell.Main>
      <AppShell.Navbar>
        <AppShell.Section grow component={ScrollArea}>
          <Sidebar />
        </AppShell.Section>
      </AppShell.Navbar>
      <AppShell.Aside>
        <AppShell.Section grow component={ScrollArea}>
          <TableOfContents />
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
