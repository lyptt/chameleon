import cx from 'classnames'
import { HTMLProps } from 'react'
import { useAuth } from '@/components/organisms/AuthContext'
import Button from '../atoms/Button'

export default function UserButton({
  className,
  ...rest
}: HTMLProps<HTMLDivElement>) {
  const { authenticated } = useAuth()

  if (authenticated) {
    return <></>
  }

  return <Button {...(rest as any)}>Sign in</Button>
}
