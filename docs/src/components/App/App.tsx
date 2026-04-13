import { CodeHighlightAdapterProvider, createShikiAdapter } from '@mantine/code-highlight'
import { MantineProvider } from '@mantine/core'
import { BrowserRouter } from 'react-router'
import '@mantine/core/styles.css'
import '@mantine/spotlight/styles.css'
import '@mantine/code-highlight/styles.css'
import { theme, AppRouter } from '@/components'

async function loadShiki() {
  const { createHighlighter } = await import('shiki')
  return await createHighlighter({
    langs: ['ts', 'json', 'rs', 'bash', 'py', 'yaml'],
    // You can load supported themes here
    themes: [],
  })
}

const shikiAdapter = createShikiAdapter(loadShiki)

export const App = () => {
  return (
    <MantineProvider theme={theme}>
      <CodeHighlightAdapterProvider adapter={shikiAdapter}>
        <BrowserRouter>
          <AppRouter />
        </BrowserRouter>
      </CodeHighlightAdapterProvider>
    </MantineProvider>
  )
}
