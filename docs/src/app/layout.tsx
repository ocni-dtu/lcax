'use client'

import '@mantine/core/styles.css'
import '@mantine/code-highlight/styles.css'
import { ReactNode } from 'react'
import { ColorSchemeScript, mantineHtmlProps, MantineProvider } from '@mantine/core'
import { theme } from '@/components'

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang='en' {...mantineHtmlProps}>
      <head>
        <title>LCAx Docs</title>
        <meta charSet='UTF-8' />
        <link rel='icon' type='image/svg+xml' href='/lcax.svg' />
        <meta name='viewport' content='width=device-width, initial-scale=1.0' />
        {process.env.UMAMI_ID? <script defer src="https://umami.kongsgaard.eu/script.js" data-website-id={process.env.UMAMI_ID}></script>: undefined}
        <ColorSchemeScript />
      </head>
      <body>
        <MantineProvider theme={theme}>{children}</MantineProvider>
      </body>
    </html>
  )
}
