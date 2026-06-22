import { Image } from '@mantine/core'
import { Link } from 'react-router'

import logo from '@/assets/lcax.svg'

interface LogoProps {
  height?: string
}

export const Logo = ({ height }: LogoProps) => {
  return (
    <Link to='/'>
      <Image src={logo} alt='logo' h={height} w='auto' />
    </Link>
  )
}
