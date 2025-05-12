'use client'

import { useRouter } from 'next/navigation'
import { IconSearch } from '@tabler/icons-react'
import { ActionIcon, rem, TextInput } from '@mantine/core'
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
      {/* @ts-expect-error non-sense */}
      <ActionIcon onClick={handleOnClick} variant='subtle' hiddenFrom='md'>
        <IconSearch />
      </ActionIcon>
      <TextInput
        leftSection={<IconSearch />}
        placeholder='Search'
        visibleFrom='md'
        wrapperProps={{ onClick: handleOnClick }}
      />
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
