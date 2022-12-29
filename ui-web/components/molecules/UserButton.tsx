import cx from 'classnames'
import { HTMLProps } from 'react'
import Link from 'next/link'
import { useAuth } from '@/components/organisms/AuthContext'
import Button from '@/components/atoms/Button'
import { useProfile } from '@/components/organisms/ProfileContext'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export default function UserButton({
  className,
  ...rest
}: HTMLProps<HTMLDivElement>) {
  const { authenticated } = useAuth()
  const { state } = useProfile()
  const { profile } = state

  if (authenticated) {
    return (
      <Link legacyBehavior href="/profile">
        <a
          className={cx(
            'orbit-user-button',
            'orbit-user-button--authenticated',
            className
          )}
        >
          {!!profile && (
            <>
              <img
                className="orbit-user-button__avatar"
                src={profile.avatar_url || transparentPixelUri}
                alt={profile.handle}
              />
              <div className="orbit-user-button__details">
                <div className="orbit-user-button__details-handle">
                  {profile.handle}
                </div>
                <div className="orbit-user-button__details-full-handle">
                  {profile.fediverse_id}
                </div>
              </div>
            </>
          )}
        </a>
      </Link>
    )
  }

  return (
    <Button
      className={cx(
        'orbit-user-button',
        'orbit-user-button--unauthenticated',
        className
      )}
      href="/api/oauth/login"
      {...(rest as any)}
    >
      Sign in
    </Button>
  )
}
