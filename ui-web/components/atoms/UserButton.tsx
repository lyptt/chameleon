import Link from 'next/link'
import { useAuth } from '../organisms/AuthContext'
import classNames from './UserButton.module.css'
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
        <Link href="/profile">
          <a
            className={cx(
              className,
              classNames.button,
              classNames.authenticated,
              { [classNames.active]: active }
            )}
          >
            {!loading && !!profile && (
              <img
                className={classNames.img}
                src={profile.avatar_url || transparentPixelUri}
                alt={profile.handle || 'You'}
              />
            )}
            <span>Profile</span>
          </a>
        </Link>
      )}
      {!auth.authenticated && (
        <a
          href="/api/oauth/authorize"
          className={cx(className, classNames.button, {
            [classNames.active]: active,
          })}
        >
          <IoLogInOutline />
          <span>Sign in or create account</span>
        </a>
      )}
    </>
  )
}
