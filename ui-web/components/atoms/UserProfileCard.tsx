import Link from 'next/link'
import { useAuth } from '@/components/organisms/AuthContext'
import cx from 'classnames'
import { useProfile } from '@/components/organisms/ProfileContext'
import { IoLogInOutline } from 'react-icons/io5'
import IconButton, { IconButtonIcon } from './IconButton'

export interface IUserButtonProps {
  className?: string
  active?: boolean
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function UserProfileCard({
  className,
  active,
}: IUserButtonProps) {
  const { authenticated } = useAuth()
  const { state: profileState } = useProfile()
  const { loading, profile } = profileState

  if (!authenticated) {
    return <></>
  }

  return (
    <div className={cx('chameleon-user-profile-card', className)}>
      <Link
        href={profile ? `/users/${profile.handle}` : ''}
        className="chameleon-user-profile-card__avatar"
        title={profile?.handle || 'You'}
      >
        {!loading && !!profile && (
          <img
            className="chameleon-user-profile-card__profile-image"
            src={profile.avatar_url || transparentPixelUri}
            alt={profile.handle || 'You'}
          />
        )}
        {(loading || !profile) && (
          <div className="chameleon-user-profile-card__profile-image" />
        )}
      </Link>
      <div className="chameleon-user-profile-card__details">
        {!loading && !!profile && (
          <Link
            className="chameleon-user-profile-card__profile-link"
            href={`/users/${profile.handle}`}
          >
            @{profile.handle}
          </Link>
        )}
        {(loading || !profile) && (
          <div className="chameleon-user-profile-card__profile-link">@</div>
        )}
        <Link
          className="chameleon-user-profile-card__profile-settings-link"
          href={`/settings/profile`}
        >
          Edit profile
        </Link>
      </div>
      <IconButton
        className="chameleon-user-profile-card__options-button"
        contentClassName="chameleon-user-profile-card__options-button-content"
        icon={IconButtonIcon.Options}
      />
    </div>
  )
}
