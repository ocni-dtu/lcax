import { AppShell, rem, useMatches } from '@mantine/core'
import { Outlet } from 'react-router'

import { ErrorBoundary, Header } from '@/components'

export const AppLayout = () => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xl: rem(100) })

  return (
    <AppShell header={{ height: headerHeight, offset: false }}>
      <AppShell.Header withBorder={true} pl='lg' bg='grey.0'>
        <ErrorBoundary>
          <Header height={headerHeight} />
        </ErrorBoundary>
      </AppShell.Header>
      <AppShell.Main pt={`calc(${headerHeight}`} pb='xl'>
        <ErrorBoundary>
          <Outlet />
        </ErrorBoundary>
      </AppShell.Main>
    </AppShell>
  )
}
