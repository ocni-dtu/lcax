import { type MouseEvent } from 'react'
import { ActionIcon, rem, TextInput } from '@mantine/core'
import { Spotlight, spotlight } from '@mantine/spotlight'
import { IconSearch } from '@tabler/icons-react'
import { useNavigate } from 'react-router'

import searchIndex from '@/assets/searchIndex.json'

export const Search = () => {
  const navigate = useNavigate()
  const actions = searchIndex.map((page) => ({
    ...page,
    id: page.slug,
    onClick: () => navigate(page.slug),
  }))

  const handleOnClick = (e: MouseEvent) => {
    e.preventDefault()
    spotlight.open()
  }

  return (
    <>
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
