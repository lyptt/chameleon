import Link from 'next/link'
import { useAuth } from '../organisms/AuthContext'
import cx from 'classnames'
import { useProfile } from '../organisms/ProfileContext'
import { IoLogInOutline } from 'react-icons/io5'

export interface IUserButtonProps {
  className?: string
  active?: boolean
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function UserButton({ className, active }: IUserButtonProps) {
  const auth = useAuth()
  const { state: profileState } = useProfile()
  const { loading, profile } = profileState

  return (
    <>
      {auth.authenticated && (
        <Link
          href="/profile"
          className={cx(
            'chameleon-user-button',
            className,
            'chameleon-user-button--authenticated',
            { 'chameleon-user-button--active': active }
          )}
        >
          {!loading && !!profile && (
            <img
              className={cx('chameleon-user-button__profile-image')}
              src={profile.avatar_url || transparentPixelUri}
              alt={profile.handle || 'You'}
            />
          )}
          <span>Profile</span>
        </Link>
      )}
      {!auth.authenticated && (
        <a
          href="/api/oauth/authorize"
          className={cx('chameleon-user-button', className, {
            'chameleon-user-button--active': active,
          })}
        >
          <IoLogInOutline />
          <span>Sign in or create account</span>
        </a>
      )}
    </>
  )
}
