import { NavLink, Stack } from '@mantine/core'
import { Link, useLocation } from 'react-router'

import { pages, type PagesProps } from '@/pages'

const NavElement = ({ label, items, slug }: PagesProps) => {
  const location = useLocation()

  if (items && items.length > 0) {
    return (
      <NavLink component={Link} to={slug || ''} label={label} defaultOpened childrenOffset='lg' fw='bold' py='xs'>
        {items.map((item) => (
          <NavElement {...item} key={item.label} />
        ))}
      </NavLink>
    )
  }
  return (
    <NavLink component={Link} to={slug || ''} label={label} defaultOpened py={0} active={slug === location.pathname} />
  )
}

export const Sidebar = () => {
  return (
    <Stack m='xl'>
      {pages.map((item) => (
        <NavElement {...item} key={item.label} />
      ))}
    </Stack>
  )
}
