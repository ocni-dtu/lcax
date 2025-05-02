'use client'

import '@mantine/core/styles.css'
import '@mantine/code-highlight/styles.css'
import { ReactNode } from 'react'
import { ColorSchemeScript, MantineProvider, mantineHtmlProps } from '@mantine/core'
import { AppLayout, theme } from '@/components'

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang='en' {...mantineHtmlProps}>
      <head>
        <title>LCAx Docs</title>
        <meta charSet='UTF-8' />
        <link rel='icon' type='image/svg+xml' href='/lcax.svg' />
        <meta name='viewport' content='width=device-width, initial-scale=1.0' />
        <ColorSchemeScript />
      </head>
      <body>
        <MantineProvider theme={theme}>
          <AppLayout>{children}</AppLayout>
        </MantineProvider>
      </body>
    </html>
  )
}
