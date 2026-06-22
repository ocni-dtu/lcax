import { IconBrandGithub, IconMessages } from '@tabler/icons-react'
import { Link } from 'react-router'

export const GitHubLink = () => {
  return (
    <Link to='https://github.com/ocni-dtu/lcax'>
      <IconBrandGithub color='black' />
    </Link>
  )
}

export const GitHubDiscussions = () => (
  <Link to='https://github.com/ocni-dtu/lcax/discussions'>
    <IconMessages color='black' />
  </Link>
)
