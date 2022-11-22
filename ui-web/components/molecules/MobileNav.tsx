import { HTMLAttributes } from 'react'
import cx from 'classnames'
import Link from 'next/link'
import { useAuth } from '@/components/organisms/AuthContext'
import Button from '@/components/quarks/Button'
import PlainButton from '@/components/quarks/PlainButton'

export default function MobileNav({
  className,
}: HTMLAttributes<HTMLDivElement>) {
  const { session } = useAuth()

  return (
    <>
      <nav className={cx('chameleon-mobile-nav', className)}>
        <Link className="chameleon-mobile-nav__title-link" href="/">
          <h1 className="chameleon-mobile-nav__title">Chameleon</h1>
        </Link>
        {!session && (
          <>
            <Button
              href="/api/oauth/authorize"
              className="chameleon-mobile-nav__login-button"
              bold
            >
              Sign in
            </Button>
            <PlainButton
              href="/api/oauth/authorize"
              className="chameleon-mobile__register-button"
              brand
            >
              Create account
            </PlainButton>
          </>
        )}
      </nav>
    </>
  )
}
