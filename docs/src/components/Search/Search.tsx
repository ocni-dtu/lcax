import { TextInput } from '@mantine/core'
import { IconSearch } from '@tabler/icons-react'

export const Search = () => {
  return <TextInput leftSection={<IconSearch />} placeholder='Search' />
}
