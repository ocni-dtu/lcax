import { AppShell, rem, useMatches } from '@mantine/core'
import { useHeadroom } from '@mantine/hooks'
import { Outlet } from 'react-router'

import { ErrorBoundary, Header } from '@/components'

export const AppLayout = () => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xxl: rem(100) })
  const pinned = useHeadroom({ fixedAt: 120 })

  return (
    <AppShell header={{ height: headerHeight, collapsed: !pinned, offset: false }} withBorder={false}>
      <AppShell.Header withBorder={false} pl='lg' bg='grey.0'>
        <ErrorBoundary>
          <Header height={headerHeight} />
        </ErrorBoundary>
      </AppShell.Header>
      <AppShell.Main pt={headerHeight} pb='xl'>
        <ErrorBoundary>
          <Outlet />
        </ErrorBoundary>
      </AppShell.Main>
    </AppShell>
  )
}
