import { IPost } from '@/core/api'
import { HTMLProps } from 'react'
import cx from 'classnames'
import Link from 'next/link'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface PostCardProps extends HTMLProps<HTMLAnchorElement> {
  post: IPost
}

export default function PostCard({ post, className, ...rest }: PostCardProps) {
  return (
    <Link legacyBehavior href={post.uri}>
      <a className={cx('orbit-post-card', className)} {...rest}>
        <div className="orbit-post-card__info-bar">
          {!!post.orbit_id && (
            <>
              <Link legacyBehavior href={post.orbit_uri || '/'}>
                <a className="orbit-post-card__info-bar-origin">
                  <img
                    className="orbit-post-card__info-bar-icon"
                    alt={post.orbit_shortcode?.toLowerCase()}
                    src={post.orbit_avatar_uri || transparentPixelUri}
                  />
                  o/{post.orbit_name?.toLowerCase()}
                </a>
              </Link>
              <div className="orbit-post-card__info-bar-author">
                Posted by&nbsp;
                <Link legacyBehavior href={`/users/${post.user_handle}`}>
                  <a className="orbit-post-card__info-bar-author-origin">
                    u/{post.user_handle}
                  </a>
                </Link>
                &nbsp;
                {dayjs.utc(post.created_at).fromNow()}
              </div>
            </>
          )}
          {!post.orbit_id && (
            <>
              <Link legacyBehavior href={`/users/${post.user_handle}`}>
                <a className="orbit-post-card__info-bar-origin">
                  <img
                    className="orbit-post-card__info-bar-icon"
                    alt={post.user_handle}
                    src={post.user_avatar_url || transparentPixelUri}
                  />
                  u/{post.user_handle}
                </a>
              </Link>
              <div className="orbit-post-card__info-bar-author">
                Posted&nbsp;
                {dayjs.utc(post.created_at).fromNow()}
              </div>
            </>
          )}
        </div>
        {!!post.title && (
          <div className="orbit-post-card__title">{post.title}</div>
        )}
        <div
          className={cx('orbit-post-card__content', {
            'orbit-post-card__content--user-post': !post.orbit_id,
          })}
          dangerouslySetInnerHTML={{ __html: post.content_html }}
        />
      </a>
    </Link>
  )
}
