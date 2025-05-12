'use client'

import { useRouter } from 'next/navigation'
import { IconSearch } from '@tabler/icons-react'
import { rem, TextInput } from '@mantine/core'
import { Spotlight, spotlight } from '@mantine/spotlight'
import searchIndex from '@public/searchIndex.json'

export const Search = () => {
  const router = useRouter()

  const actions = searchIndex.map((page) => ({
    ...page,
    id: page.slug,
    onClick: () => router.push(page.slug),
  }))

  const handleOnClick = (e: MouseEvent) => {
    e.preventDefault()
    spotlight.open()
  }

  return (
    <>
      <TextInput leftSection={<IconSearch />} placeholder='Search' wrapperProps={{ onClick: handleOnClick }} />
      <Spotlight
        shortcut={['mod + K', 'mod + P', '/']}
        actions={actions}
        tagsToIgnore={[]}
        highlightQuery
        clearQueryOnClose
        radius='md'
        limit={7}
        nothingFound='Nothing found...'
        searchProps={{
          leftSection: <IconSearch style={{ width: rem(20), height: rem(20) }} />,
          placeholder: 'Search documentation...',
        }}
      />
    </>
  )
}
