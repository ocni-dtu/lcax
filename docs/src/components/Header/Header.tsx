import { Group } from '@mantine/core'
import { GitHubLink, Logo, Search } from '@/components'
import { ReactNode } from 'react'

interface HeaderProps {
  height?: string
  MenuComponent?: ReactNode
}

export const Header = ({ height, MenuComponent }: HeaderProps) => {
  return (
    <Group h={height} justify='space-between' pr='xl'>
      {MenuComponent}
      <Logo height={height} />
      <Search />
      <GitHubLink />
    </Group>
  )
}
