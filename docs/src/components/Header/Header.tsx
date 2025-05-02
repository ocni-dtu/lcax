import { Group } from '@mantine/core'
import { GitHubLink, Logo } from '@/components'

interface HeaderProps {
  height?: string
}

export const Header = ({ height }: HeaderProps) => {
  return (
    <Group h={height} justify='space-between' pr='xl'>
      <Logo height={height} />
      {/*<Search />*/}
      <GitHubLink />
    </Group>
  )
}
