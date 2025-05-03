import { Alert, Center } from '@mantine/core'
import { IconAlertCircle } from '@tabler/icons-react'
import React from 'react'

interface ErrorMessageProps {
  error?: { message: string } | null
}
export const ErrorMessage = ({ error }: ErrorMessageProps) => {
  if (!error) {
    return null
  }
  return (
    <Center>
      <Alert icon={<IconAlertCircle size='1rem' />} title='Bummer!' color='red'>
        {error.message}
      </Alert>
    </Center>
  )
}
