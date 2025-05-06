'use client'

import { Button, Container, SimpleGrid, Stack, Text, Title } from '@mantine/core'
import Link from 'next/link'
import { AppLayout, LandingPageCard } from '@/components'
import { IconClipboardData, IconCpu, IconFileCode, IconListCheck, IconSteam, IconTransform } from '@tabler/icons-react'

export default function Home() {
  return (
    <AppLayout>
      <Container fluid mih='100vh' bg='grey.0'>
        <Stack h='100%' justify='space-around'>
          <Hero />
          <Features />
        </Stack>
      </Container>
    </AppLayout>
  )
}

const Hero = () => (
  <Stack justify='center' align='center' mih='70vh' px='lg'>
    <Title>Build Sustainability Tools with LCAx!</Title>
    <Text size='xl'>Open-source multi-language toolkit for building modern LCA applications</Text>
    <Button c='black' mt='xl' component={Link} href='/concept/introduction' w={'fit-content'} size='xl'>
      Get started
    </Button>
  </Stack>
)
const Features = () => {
  const features = [
    {
      title: 'Common Format',
      description:
        'A unified data format that bridges different LCA tools and standards, enabling seamless data exchange and interoperability.',
      icon: <IconFileCode />,
    },
    {
      title: 'Data Conversion',
      description:
        'Seamlessly convert between LCAbyg, ILCD, and other industry-standard formats with built-in data transformation.',
      icon: <IconTransform />,
    },
    {
      title: 'Impact Analysis',
      description:
        'Comprehensive LCA impact calculations with support for multiple assessment methods and normalization.',
      icon: <IconCpu />,
    },
    {
      title: 'Data Validation',
      description: 'Ensure data quality with robust validation tools and customizable rule sets.',
      icon: <IconListCheck />,
    },
    {
      title: 'EPD Integration',
      description: 'Full support for EPDs with comprehensive product and transport data management.',
      icon: <IconClipboardData />,
    },
    {
      title: 'Cross-Platform',
      description: 'Use LCAx in your preferred programming language with consistent APIs and data structures.',
      icon: <IconSteam />,
    },
  ]

  return (
    <Container mih='70vh'>
      <Title order={2}>Features</Title>
      <SimpleGrid cols={{ base: 1, lg: 3 }} mt='xl' spacing='xl'>
        {features.map((feature) => (
          <LandingPageCard {...feature} key={feature.title} />
        ))}
      </SimpleGrid>
    </Container>
  )
}
