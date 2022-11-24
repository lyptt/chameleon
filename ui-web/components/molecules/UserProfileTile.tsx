import { HTMLProps } from 'react'
import { IProfile } from '@/core/api'
import cx from 'classnames'
import Button from '../quarks/Button'

export interface IUserProfileTileProps extends HTMLProps<HTMLDivElement> {
  profile: IProfile
  feedAvailable?: boolean
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function UserProfileTile({
  profile,
  feedAvailable,
  className,
  ...props
}: IUserProfileTileProps) {
  return (
    <div
      className={cx(
        'chameleon-user-profile-tile',
        { 'chameleon-user-profile-tile--cutoff': feedAvailable },
        className
      )}
      {...props}
    >
      <div className="chameleon-user-profile-tile__row chameleon-user-profile-tile__header-row">
        <img
          src={profile.avatar_url || transparentPixelUri}
          alt={profile.handle || 'unknown'}
          className="chameleon-user-profile-tile__photo"
          draggable="false"
        />
        <Button className="chameleon-user-profile-tile__follow-button" bold>
          Follow
        </Button>
      </div>
      <div className="chameleon-user-profile-tile__row chameleon-user-profile-tile__info-row">
        <div className="chameleon-user-profile-tile__name">
          {profile.handle}
        </div>
        <div className="chameleon-user-profile-tile__handle">
          {profile.fediverse_id}
        </div>
      </div>
      {!!profile.intro_html && (
        <div
          className="chameleon-user-profile-tile__row chameleon-user-profile-tile__intro-row"
          dangerouslySetInnerHTML={{ __html: profile.intro_html }}
        />
      )}
    </div>
  )
}
