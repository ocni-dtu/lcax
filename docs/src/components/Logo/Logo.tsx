import NextImage from 'next/image'
import { Image } from '@mantine/core'
import Link from 'next/link'
import logo from '../../../public/lcax.svg'

interface LogoProps {
  height?: string
}

export const Logo = ({ height }: LogoProps) => {
  return (
    <Link href='/'>
      <Image component={NextImage} src={logo} alt='logo' h={height} w='auto' />
    </Link>
  )
}
