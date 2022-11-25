import { HTMLProps } from 'react'
import { IProfile, IProfileStats } from '@/core/api'
import cx from 'classnames'
import Button from '@/components/quarks/Button'
import { ISession } from '@/components/organisms/AuthContext'
import pluralize from 'pluralize'
import Link from 'next/link'

export interface IUserProfileTileProps extends HTMLProps<HTMLDivElement> {
  session?: ISession
  profile: IProfile
  stats?: IProfileStats
  postCount?: number
  feedAvailable?: boolean
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function UserProfileTile({
  suppressHydrationWarning,
  session,
  profile,
  stats,
  feedAvailable,
  className,
  postCount,
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
        {session && stats && !stats.user_is_you && (
          <Button className="chameleon-user-profile-tile__follow-button" bold>
            {stats.following_user && <>Unfollow</>}
            {!stats.following_user && <>Follow</>}
          </Button>
        )}
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
      <div className="chameleon-user-profile-tile__row chameleon-user-profile-tile__links-row">
        {!!profile.url_1 && !!profile.url_1_title && (
          <div className="chameleon-user-profile-tile__link-row">
            <div className="chameleon-user-profile-tile__link-title">
              {profile.url_1_title}
            </div>
            <Link
              href={profile.url_1}
              className="chameleon-user-profile-tile__link-description"
            >
              {profile.url_1.split('://')[1]}
            </Link>
          </div>
        )}
        {!!profile.url_2 && !!profile.url_2_title && (
          <div className="chameleon-user-profile-tile__link-row">
            <div className="chameleon-user-profile-tile__link-title">
              {profile.url_2_title}
            </div>
            <Link
              href={profile.url_2}
              className="chameleon-user-profile-tile__link-description"
            >
              {profile.url_2.split('://')[1]}
            </Link>
          </div>
        )}
        {!!profile.url_3 && !!profile.url_3_title && (
          <div className="chameleon-user-profile-tile__link-row">
            <div className="chameleon-user-profile-tile__link-title">
              {profile.url_3_title}
            </div>
            <Link
              href={profile.url_3}
              className="chameleon-user-profile-tile__link-description"
            >
              {profile.url_3.split('://')[1]}
            </Link>
          </div>
        )}
        {!!profile.url_4 && !!profile.url_4_title && (
          <div className="chameleon-user-profile-tile__link-row">
            <div className="chameleon-user-profile-tile__link-title">
              {profile.url_4_title}
            </div>
            <Link
              href={profile.url_4}
              className="chameleon-user-profile-tile__link-description"
            >
              {profile.url_4.split('://')[1]}
            </Link>
          </div>
        )}
        {!!profile.url_5 && !!profile.url_5_title && (
          <div className="chameleon-user-profile-tile__link-row">
            <div className="chameleon-user-profile-tile__link-title">
              {profile.url_5_title}
            </div>
            <Link
              href={profile.url_5}
              className="chameleon-user-profile-tile__link-description"
            >
              {profile.url_5.split('://')[1]}
            </Link>
          </div>
        )}
      </div>
      {!!stats && (
        <div className="chameleon-user-profile-tile__row chameleon-user-profile-tile__stats-row">
          <div className="chameleon-user-profile-tile__stats-row-item">
            <span className="chameleon-user-profile-tile__stats-row-item-stat">
              {postCount ? postCount : '-'}
            </span>{' '}
            {pluralize('Post', postCount || 0)}
          </div>
          <div className="chameleon-user-profile-tile__stats-row-item">
            <span className="chameleon-user-profile-tile__stats-row-item-stat">
              {stats.following_count}
            </span>{' '}
            Following
          </div>
          <div className="chameleon-user-profile-tile__stats-row-item">
            <span className="chameleon-user-profile-tile__stats-row-item-stat">
              {stats.followers_count}
            </span>{' '}
            {pluralize('Follower', stats.followers_count)}
          </div>
        </div>
      )}
    </div>
  )
}
