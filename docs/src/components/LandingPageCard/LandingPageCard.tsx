import { Card, Group, Text, Title } from '@mantine/core'
import { ReactNode } from 'react'

interface CardProps {
  title: string
  description: string
  icon: ReactNode
}

export const LandingPageCard = ({ title, icon, description }: CardProps) => {
  return (
    <Card shadow="sm" padding="lg" radius="xl" withBorder bg="grey.2">
      <Group mb='md'>{icon}<Title order={3}>{title}</Title></Group>
      <Text>
        {description}
      </Text>

    </Card>
  )
}
