import cx from 'classnames'
import { HTMLProps } from 'react'
import Link from 'next/link'
import { useAuth } from '@/components/organisms/AuthContext'
import Button from '@/components/atoms/Button'
import { useProfile } from '@/components/organisms/ProfileContext'
import { IProfile } from '@/core/api'
import { cdnUrl } from '@/core/utils'

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface UserButtonProps extends HTMLProps<HTMLDivElement> {
  specificProfile?: boolean
  profile?: IProfile
}

export default function UserButton({
  specificProfile,
  profile,
  className,
  ...rest
}: UserButtonProps) {
  const { authenticated } = useAuth()
  const { state } = useProfile()
  const { profile: ownProfile } = state

  if (specificProfile) {
    return (
      <Link legacyBehavior href={profile ? `/users/${profile.handle}` : '#'}>
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
                src={cdnUrl(profile.avatar_url || transparentPixelUri)}
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
          {!!ownProfile && (
            <>
              <img
                className="orbit-user-button__avatar"
                src={cdnUrl(ownProfile.avatar_url || transparentPixelUri)}
                alt={ownProfile.handle}
              />
              <div className="orbit-user-button__details">
                <div className="orbit-user-button__details-handle">
                  {ownProfile.handle}
                </div>
                <div className="orbit-user-button__details-full-handle">
                  {ownProfile.fediverse_id}
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
