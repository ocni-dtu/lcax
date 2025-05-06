import { AppShell, Group, rem, SimpleGrid, Stack, Text, useMatches } from '@mantine/core'
import { Header } from '@/components'
import { ReactNode } from 'react'
import { IconBook2, IconConfetti, IconMessages, IconPlayerPlay, IconSitemap } from '@tabler/icons-react'
import Link from 'next/link'

interface AppLayoutProps {
  children: ReactNode
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xl: rem(100) })
  const footerHeight = useMatches({ base: rem(75), lg: rem(100), xl: rem(175) })

  return (
    <AppShell header={{ height: headerHeight, offset: false }} footer={{ height: footerHeight }}>
      <AppShell.Header withBorder={true} pl='lg' bg='grey.0'>
        <Header height={headerHeight} />
      </AppShell.Header>
      <AppShell.Main pt={`calc(${headerHeight}`} pb={`calc(${footerHeight}`}>
        {children}
      </AppShell.Main>
      <AppShell.Footer bg='grey.0' p='lg'>
        <Footer />
      </AppShell.Footer>
    </AppShell>
  )
}

const Footer = () => {
  return (
    <SimpleGrid cols={{ base: 1, lg: 3 }}>
      <Stack>
        <Text fw={700}>Quick Links:</Text>
        <SimpleGrid cols={{ base: 1, lg: 3 }} spacing='xs'>
          <Group gap='xs' justify='start'>
            <IconPlayerPlay />
            <Link href='/guides/installation'>
              <Text c='black'>Install LCAx</Text>
            </Link>
          </Group>
          <Group gap='xs' justify='start'>
            <IconBook2 />
            <Link href='/concept/pillars'>
              <Text c='black'>Core Concepts</Text>
            </Link>
          </Group>
          <Group gap='xs' justify='start'>
            <IconSitemap />
            <Link href='/reference/schemas/lcax'>
              <Text c='black'>JSON Schema</Text>
            </Link>
          </Group>
        </SimpleGrid>
      </Stack>
      <Stack>
        <Text fw={700}>Latest Updates:</Text>
        <Group gap='xs' justify='start'>
          <IconConfetti />
          <Link href='/reference/changelog'>
            <Text c='black'>What&apos;s new in LCAx?</Text>
          </Link>
        </Group>
      </Stack>
      <Stack>
        <Text fw={700}>Need Help?</Text>
        <Group gap='xs' justify='start'>
          <IconMessages />
          <Link href='https://github.com/ocni-dtu/lcax/discussions'>
            <Text c='black'>GitHub Discussions</Text>
          </Link>
        </Group>
      </Stack>
    </SimpleGrid>
  )
}
