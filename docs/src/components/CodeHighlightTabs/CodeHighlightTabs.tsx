import { CodeHighlightTabs as MantineCodeHighlight } from '@mantine/code-highlight'
import {
  IconBrandPython,
  IconBrandTypescript,
  IconBrandRust,
  IconJson,
  IconFileText,
  IconTerminal2,
} from '@tabler/icons-react'

const getFileIcon = (fileName: string) => {
  if (fileName.endsWith('.ts') || fileName.endsWith('.tsx')) {
    return <IconBrandTypescript size={18} />
  }

  if (fileName.endsWith('.py')) {
    return <IconBrandPython size={18} />
  }

  if (fileName.endsWith('.rs')) {
    return <IconBrandRust size={18} />
  }

  if (fileName.endsWith('.json')) {
    return <IconJson size={18} />
  }

  if (fileName.endsWith('.yaml')) {
    return <IconFileText size={18} />
  }

  return <IconTerminal2 size={18} />
}

export const CodeHighlightTabs = (props: any) => {
  return (
    <MantineCodeHighlight
      {...props}
      withExpandButton={true}
      defaultExpanded={props.expanded || false}
      getFileIcon={getFileIcon}
    />
  )
}
