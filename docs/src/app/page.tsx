'use client'

import { Button, Container, Divider, Group, List, SimpleGrid, Stack, Text, Title } from '@mantine/core'
import Link from 'next/link'
import { LandingPageCard } from '@/components'
import {
  IconBellQuestion, IconBook2,
  IconClipboardData,
  IconConfetti,
  IconCpu,
  IconFileCode,
  IconListCheck,
  IconMessages,
  IconPlayerPlay,
  IconSitemap,
  IconSteam,
  IconTransform,
} from '@tabler/icons-react'

export default function Home() {
  return (
    <Container fluid mih="100vh" bg="grey.0">
      <Stack h="100%" justify="space-around">
        <Stack justify="center" align="center" mih="70vh">
          <Title>Build Sustainability Tools with LCAx!</Title>
          <Text size="xl">Open-source multi-language toolkit for building modern LCA applications</Text>
          <Button c="black" mt="xl" component={Link} href="/concept/introduction" w={'fit-content'} size="xl">
            Get started
          </Button>
        </Stack>
        <Features />
      </Stack>
      <Footer />
    </Container>
  )
}

const Features = () => {
  const features = [
    {
      title: 'Common Format',
      description: 'A unified data format that bridges different LCA tools and standards, enabling seamless data exchange and interoperability.',
      icon: <IconFileCode />,
    },
    {
      title: 'Data Conversion',
      description: 'Seamlessly convert between LCAbyg, ILCD, and other industry-standard formats with built-in data transformation.',
      icon: <IconTransform />,
    },
    {
      title: 'Impact Analysis',
      description: 'Comprehensive LCA impact calculations with support for multiple assessment methods and normalization.',
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
      <SimpleGrid cols={3} mt="xl" spacing='xl'>
        {features.map(feature => <LandingPageCard {...feature} key={feature.title} />)}
      </SimpleGrid>
    </Container>
  )
}

const Footer = () => {
  return (
    <Stack mt='xl' pb='xl'>
      <Divider />
    <Group justify='space-evenly'>
      <Stack>
        <Text fw={700}>Quick Links:</Text>
        <Group>
          <Group gap='xs' justify='start'><IconPlayerPlay /><Link href="/guides/installation"><Text c='black'>Install LCAx</Text></Link></Group>
          <Group gap='xs' justify='start'><IconBook2 /><Link href="/concept/pillars"><Text c='black'>Core Concepts</Text></Link></Group>
          <Group gap='xs' justify='start'><IconSitemap /><Link href="/reference/schemas/lcax"><Text c='black'>JSON Schema</Text></Link></Group>
        </Group>
      </Stack>
      <Stack>
        <Text fw={700}>Latest Updates:</Text>
        <Group gap='xs' justify='start'><IconConfetti /><Link href="/reference/changelog"><Text c='black'>What's new in LCAx?</Text></Link></Group>
      </Stack>
      <Stack>
        <Text fw={700}>Need Help?</Text>
        <Group gap='xs' justify='start'><IconMessages /><Link href="https://github.com/ocni-dtu/lcax/discussions"><Text c='black'>GitHub Discussions</Text></Link></Group>
      </Stack>
    </Group>
    </Stack>
  )
}
