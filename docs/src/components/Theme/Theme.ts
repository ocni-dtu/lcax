'use client'

import { Container, createTheme, Divider, rem, Title } from '@mantine/core'
import '@fontsource/inter-tight/400.css'
import '@fontsource/inter-tight/500.css'
import '@fontsource/inter-tight/600.css'
import '@fontsource/inter-tight/700.css'
import '@fontsource/jetbrains-mono'
import classes from './theme.module.css'

const CONTAINER_SIZES: Record<string, number> = {
  xs: 540,
  sm: 720,
  md: 960,
  lg: 1140,
  xl: 1320,
  xxl: 1560,
}

const fontFamily = 'Inter Tight, sans-serif'

const fontSizes = {
  xs: rem(12),
  sm: rem(14),
  md: rem(16),
  lg: rem(18),
  xl: rem(20),
}

export const theme = createTheme({
  defaultRadius: 'xl',
  colors: {
    yellow: [
      '#fff9e2',
      '#fbf2cf',
      '#f4e3a4',
      '#eed474',
      '#e8c547',
      '#e5be32',
      '#e4ba22',
      '#caa312',
      '#b49107',
      '#9b7d00',
    ],
    grey: [
      '#f5f5f5',
      '#e7e7e7',
      '#d9d9d9',
      '#b2b2b2',
      '#9a9a9a',
      '#8b8b8b',
      '#848484',
      '#717171',
      '#656565',
      '#575757',
    ],
    indigo: [
      '#f1f2f9',
      '#e0e2eb',
      '#bdc2d9',
      '#97a0c7',
      '#7883b8',
      '#6470af',
      '#5967ac',
      '#4a5797',
      '#414d87',
      '#354278',
    ],
  },
  primaryShade: 4,
  primaryColor: 'yellow',
  black: '#000',
  white: '#fff',
  fontFamily,
  fontFamilyMonospace: 'JetBrains Mono',
  fontSizes,
  headings: {
    fontFamily,
    fontWeight: '500',
  },
  components: {
    Divider: Divider.extend({
      defaultProps: { color: 'black' },
    }),
    Title: Title.extend({
      classNames: {
        root: classes.heading,
      },
    }),
    Container: Container.extend({
      vars: (_, { size, fluid }) => ({
        root: {
          '--container-size': fluid
            ? '100%'
            : size !== undefined && size in CONTAINER_SIZES
              ? rem(CONTAINER_SIZES[size])
              : rem(size),
        },
      }),
    }),
  },
})
