import { Button, Center, Title } from '@mantine/core'
import { type ReactNode, type ErrorInfo } from 'react'
import { ErrorBoundary as ExternalErrorBoundary, useErrorBoundary } from 'react-error-boundary'

import { ErrorMessage } from '@/components'

interface Props {
  children: ReactNode
}

export const ErrorBoundary = ({ children }: Props) => {
  const logError = (error: unknown, info: ErrorInfo) => {
    console.error(error, info)
  }

  return (
    <ExternalErrorBoundary FallbackComponent={Fallback} onError={logError}>
      {children}
    </ExternalErrorBoundary>
  )
}

const Fallback = ({ error }: { error: any }) => {
  const { resetBoundary } = useErrorBoundary()

  return (
    <div style={{ margin: '1em' }}>
      <ErrorMessage error={error} />
      <br />
      <Center>
        <Title order={3}>Try going back or refresh the page</Title>
        <Button onClick={resetBoundary}>Reset</Button>
      </Center>
    </div>
  )
}
