import { Group } from '@mantine/core'
import { type ReactNode } from 'react'

import { GitHubLink, Logo, Search, GitHubDiscussions } from '@/components'

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
      <Group>
        <GitHubLink />
        <GitHubDiscussions />
      </Group>
    </Group>
  )
}
