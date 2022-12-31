import { IPost } from '@/core/api'
import { HTMLProps } from 'react'
import cx from 'classnames'
import Link from 'next/link'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import { LazyImage } from '../quarks/LazyImage'
import Config from '@/core/config'
import { cdnUrl } from '@/core/utils'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface PostCardProps extends HTMLProps<HTMLAnchorElement> {
  post: IPost
  hideOrbitInformation?: boolean
}

export default function PostCard({
  post,
  hideOrbitInformation,
  className,
  ...rest
}: PostCardProps) {
  const post_content_empty = post.content_md.trim().length === 0
  let firstAttachmentUri: string | undefined
  if (post.attachments.length > 0) {
    const attachment = post.attachments[0]
    if (attachment.uri) {
      if (attachment.uri.startsWith('http')) {
        firstAttachmentUri = attachment.uri
      } else {
        firstAttachmentUri = `${Config.cdn}${attachment.uri}`
      }
    } else {
      firstAttachmentUri = transparentPixelUri
    }
  }

  return (
    <Link legacyBehavior href={post.uri}>
      <a
        className={cx('orbit-post-card', className, {
          'orbit-post-card--image-post':
            !!post.attachments.length && post_content_empty,
        })}
        {...rest}
      >
        <div className="orbit-post-card__info-bar">
          {!!post.orbit_id && !hideOrbitInformation && (
            <>
              <Link legacyBehavior href={post.orbit_uri || '/'}>
                <a className="orbit-post-card__info-bar-origin">
                  <img
                    className="orbit-post-card__info-bar-icon"
                    alt={post.orbit_shortcode?.toLowerCase()}
                    src={cdnUrl(post.orbit_avatar_uri || transparentPixelUri)}
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
          {(!post.orbit_id || hideOrbitInformation) && (
            <>
              <Link legacyBehavior href={`/users/${post.user_handle}`}>
                <a className="orbit-post-card__info-bar-origin">
                  <img
                    className="orbit-post-card__info-bar-icon"
                    alt={post.user_handle}
                    src={cdnUrl(post.user_avatar_url || transparentPixelUri)}
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
        {!post_content_empty && (
          <div
            className={cx('orbit-post-card__content', {
              'orbit-post-card__content--user-post': !post.orbit_id,
            })}
            dangerouslySetInnerHTML={{ __html: post.content_html }}
          />
        )}
        {!!firstAttachmentUri && post_content_empty && (
          <LazyImage
            className={cx('orbit-post-card__image-content', {
              'orbit-post-card__image-content--user-post': !post.orbit_id,
            })}
            src={firstAttachmentUri}
            thumbnailSrc={firstAttachmentUri}
            blurhash={post.attachments[0].blurhash}
            alt={post.title}
          />
        )}
      </a>
    </Link>
  )
}
