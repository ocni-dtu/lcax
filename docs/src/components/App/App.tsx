import { CodeHighlightAdapterProvider, createShikiAdapter } from '@mantine/code-highlight'
import { MantineProvider } from '@mantine/core'
import { Helmet, HelmetProvider } from 'react-helmet-async'
import { BrowserRouter } from 'react-router'
import '@mantine/core/styles.css'
import '@mantine/spotlight/styles.css'
import '@mantine/code-highlight/styles.css'

import { theme, AppRouter } from '@/components'

async function loadShiki() {
  const { createHighlighter } = await import('shiki')
  return await createHighlighter({
    langs: ['ts', 'json', 'rs', 'bash', 'py', 'yaml'],
    themes: [],
  })
}

const shikiAdapter = createShikiAdapter(loadShiki)

const umamiId = import.meta.env.VITE_UMAMI_ID

export const App = () => {
  return (
    <HelmetProvider>
      <MantineProvider theme={theme}>
        <CodeHighlightAdapterProvider adapter={shikiAdapter}>
          <BrowserRouter>
            {umamiId && (
              <Helmet>
                <script defer src='https://umami.kongsgaard.eu/script.js' data-website-id={umamiId} />
              </Helmet>
            )}
            <AppRouter />
          </BrowserRouter>
        </CodeHighlightAdapterProvider>
      </MantineProvider>
    </HelmetProvider>
  )
}
