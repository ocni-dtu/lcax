'use client'

import { NavLink, Stack } from '@mantine/core'
import Link from 'next/link'
import { usePathname } from 'next/navigation'

interface NavigationProps {
  label: string
  slug?: string
  items?: NavigationProps[]
}

const navigation: NavigationProps[] = [
  {
    label: 'Concept',
    items: [
      { label: 'Introduction', slug: '/concept/introduction' },
      { label: 'Why Rust', slug: '/concept/rust' },
      { label: "LCAx's Data Structure", slug: '/concept/data-structure' },
      { label: "LCAx's 4 Pillars", slug: '/concept/pillars' },
    ],
  },
  {
    label: 'Guides',
    items: [
      { label: 'Install LCAx', slug: '/guides/installation' },
      { label: 'Data Structure', slug: '/guides/data-structure' },
      {
        label: 'Conversion',
        items: [
          { label: 'ILCD', slug: '/guides/conversion/ilcd' },
          { label: 'LCAByg', slug: '/guides/conversion/lcabyg' },
          { label: 'Real-Time LCA', slug: '/guides/conversion/realtimelca' },
          { label: 'Custom Converter', slug: '/guides/conversion/custom' },
        ],
      },
      {
        label: 'Validation',
        items: [
          { label: 'Validate', slug: '/guides/validation/validate' },
          { label: 'Advanced', slug: '/guides/validation/advanced' },
        ],
      },
      {
        label: 'Calculation',
        items: [
          { label: 'Calculate', slug: '/guides/calculation/calculate' },
          { label: 'Results', slug: '/guides/calculation/results' },
        ],
      },
    ],
  },
  {
    label: 'Reference',
    items: [
      { label: 'Changelog', slug: '/reference/changelog' },
      { label: 'Migration', slug: '/reference/migration' },
      {
        label: 'JSON Schemas',
        items: [
          { label: 'LCAx', slug: '/reference/schemas/lcax' },
          { label: 'Validation', slug: '/reference/schemas/validation' },
        ],
      },
      {
        label: 'API',
        items: [
          { label: 'Python', slug: '/reference/api/python' },
          { label: 'TypeScript', slug: '/reference/api/javascript' },
          { label: 'Rust', slug: '/reference/api/rust' },
        ],
      },
    ],
  },
]

const NavElement = ({ label, items, slug }: NavigationProps) => {
  const pathname = usePathname()
  if (items && items.length > 0) {
    return (
      <NavLink component={Link} href={slug || ''} label={label} defaultOpened childrenOffset='lg' fw='bold' py='xs'>
        {items.map((item) => (
          <NavElement {...item} key={item.label} />
        ))}
      </NavLink>
    )
  }
  return <NavLink component={Link} href={slug || ''} label={label} defaultOpened py={0} active={slug === pathname} />
}

export const Sidebar = () => {
  return (
    <Stack m='xl'>
      {navigation.map((item) => (
        <NavElement {...item} key={item.label} />
      ))}
    </Stack>
  )
}
