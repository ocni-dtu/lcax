import { AppShell, rem, useMatches } from '@mantine/core'
import { Header } from '@/components'
import { ReactNode } from 'react'

interface AppLayoutProps {
  children: ReactNode
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  const headerHeight = useMatches({ base: rem(50), lg: rem(65), xl: rem(100) })

  return (
    <AppShell header={{ height: headerHeight, offset: false }}>
      <AppShell.Header withBorder={true} pl='lg' bg='grey.0'>
        <Header height={headerHeight} />
      </AppShell.Header>
      <AppShell.Main pt={`calc(${headerHeight}`}>{children}</AppShell.Main>
    </AppShell>
  )
}
