import { HTMLProps } from 'react'
import cx from 'classnames'
import AnchorLink from '@/components/quarks/AnchorLink'
import { useAuth } from '@/components/organisms/AuthContext'
import Button from '@/components/atoms/Button'

export interface WelcomeCardProps extends HTMLProps<HTMLDivElement> {
  hideActions?: boolean
}

export default function WelcomeCard({
  className,
  hideActions,
  ...rest
}: WelcomeCardProps) {
  const { authenticated } = useAuth()

  return (
    <div className={cx('orbit-welcome-card', className)} {...rest}>
      <div className="orbit-welcome-card__banner">
        <img
          src="/images/logo.svg"
          alt="Orbit mascot"
          className="orbit-welcome-card__banner-logo"
          draggable="false"
        />
      </div>
      <div className="orbit-welcome-card__content">
        <div className="orbit-welcome-card__content-title">
          orbit.gaming welcomes you
        </div>
        {authenticated && (
          <>
            <p className="orbit-welcome-card__content-headline">
              This is your home instance.
            </p>
            <p className="orbit-welcome-card__content-subheadline">
              Please be mindful of the{' '}
              <AnchorLink href="/about">instance rules</AnchorLink> when posting
              content or interacting with members of this community.
            </p>
            {!hideActions && (
              <>
                <Button
                  className="orbit-welcome-card__content-new-post"
                  href="/new-post"
                >
                  Post Something
                </Button>
                <Button
                  className="orbit-welcome-card__content-new-orbit"
                  href="/new-orbit"
                  variant="outline"
                >
                  Start an Orbit
                </Button>
              </>
            )}
          </>
        )}
        {!authenticated && (
          <>
            <p className="orbit-welcome-card__content-headline">
              You can read content on this community without creating an
              account. You&apos;ll need an account to post content or interact
              with members of this community.
            </p>

            <Button
              className="orbit-welcome-card__content-sign-in"
              href="/api/oauth/login"
            >
              Sign in
            </Button>
            <Button
              className="orbit-welcome-card__content-new-account"
              href="/api/oauth/register"
              variant="outline"
            >
              Create an account
            </Button>
          </>
        )}
      </div>
    </div>
  )
}
