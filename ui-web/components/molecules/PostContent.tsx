import { IComment, IPost } from '@/core/api'
import { HTMLProps } from 'react'
import cx from 'classnames'
import Link from 'next/link'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import {
  IoBookmarkOutline,
  IoChatboxOutline,
  IoEyeOffOutline,
  IoFlagOutline,
} from 'react-icons/io5'
import { LazyImage } from '../quarks/LazyImage'
import Config from '@/core/config'
import { cdnUrl } from '@/core/utils'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=`

export interface PostContentProps extends HTMLProps<HTMLDivElement> {
  post: IPost
  commentsLoading: boolean
  commentsCount?: number
  comments: IComment[]
  hideOrbitInformation?: boolean
}

export default function PostContent({
  post,
  commentsLoading,
  commentsCount,
  comments,
  hideOrbitInformation,
  className,
  ...rest
}: PostContentProps) {
  return (
    <div className={cx('orbit-post-content', className)} {...rest}>
      <div className="orbit-post-content__info-bar">
        {!!post.orbit_id && !hideOrbitInformation && (
          <>
            <Link
              className="orbit-post-content__info-bar-origin"
              href={post.orbit_uri || '/'}
            >
              <img
                className="orbit-post-content__info-bar-icon"
                alt={post.orbit_shortcode?.toLowerCase()}
                src={cdnUrl(post.orbit_avatar_uri || transparentPixelUri)}
              />
              o/{post.orbit_name?.toLowerCase()}
            </Link>
            <div className="orbit-post-content__info-bar-author">
              Posted by&nbsp;
              <Link
                className="orbit-post-content__info-bar-author-origin"
                href={`/users/${post.user_handle}`}
              >
                u/{post.user_handle}
              </Link>
              &nbsp;
              {dayjs.utc(post.created_at).fromNow()}
            </div>
          </>
        )}
        {(!post.orbit_id || hideOrbitInformation) && (
          <>
            <Link
              className="orbit-post-content__info-bar-origin"
              href={`/users/${post.user_handle}`}
            >
              <img
                className="orbit-post-content__info-bar-icon"
                alt={post.user_handle}
                src={cdnUrl(post.user_avatar_url || transparentPixelUri)}
              />
              u/{post.user_handle}
            </Link>
            <div className="orbit-post-content__info-bar-author">
              Posted&nbsp;
              {dayjs.utc(post.created_at).fromNow()}
            </div>
          </>
        )}
      </div>
      {!!post.title && (
        <div className="orbit-post-content__title">{post.title}</div>
      )}
      <div
        className={cx('orbit-post-content__content', {
          'orbit-post-content__content--user-post': !post.orbit_id,
        })}
        dangerouslySetInnerHTML={{ __html: post.content_html }}
      />
      {!!post.attachments.length && (
        <div className="orbit-post-content__attachments">
          {post.attachments.map((attachment) => {
            let uri
            if (attachment.uri) {
              if (attachment.uri.startsWith('http')) {
                uri = attachment.uri
              } else {
                uri = `${Config.cdn}${attachment.uri}`
              }
            } else {
              uri = transparentPixelUri
            }

            return (
              <LazyImage
                className="orbit-post-content__attachment"
                key={attachment.attachment_id}
                src={uri}
                thumbnailSrc={uri}
                blurhash={attachment.blurhash}
                asLink
              />
            )
          })}
        </div>
      )}
      <div className="orbit-post-content__commands">
        <Link
          className="orbit-post-content__command"
          href={`/feed/${post.post_id}/new-comment`}
        >
          <IoChatboxOutline />
          {!!commentsCount && (
            <>
              {commentsCount}
              &nbsp;
            </>
          )}
          Comments
        </Link>
        <div className="orbit-post-content__command" role="button">
          <IoBookmarkOutline />
          Save
        </div>
        <div className="orbit-post-content__command" role="button">
          <IoEyeOffOutline />
          Hide
        </div>
        <Link
          className="orbit-post-content__command"
          href={`/feed/${post.post_id}/report`}
        >
          <IoFlagOutline />
          Report
        </Link>
      </div>
      {comments.length > 0 && (
        <div className="orbit-post-content__comments">
          {comments.map((comment) => (
            <div
              key={comment.comment_id}
              className="orbit-post-content__comment"
            >
              <div className="orbit-post-content__info-bar">
                <Link
                  className="orbit-post-content__info-bar-origin"
                  href={`/users/${comment.user_handle}`}
                >
                  <img
                    className="orbit-post-content__info-bar-icon"
                    alt={comment.user_handle}
                    src={cdnUrl(comment.user_avatar_url || transparentPixelUri)}
                  />
                  u/{comment.user_handle}
                </Link>
                <div className="orbit-post-content__info-bar-author">
                  Commented&nbsp;
                  {dayjs.utc(comment.created_at).fromNow()}
                </div>
              </div>
              <div
                className="orbit-post-content__content orbit-post-content__content--user-post"
                dangerouslySetInnerHTML={{ __html: comment.content_html }}
              />
              <div className="orbit-post-content__commands">
                <Link
                  className="orbit-post-content__command"
                  href={`/feed/${post.post_id}/comments/${comment.comment_id}/new-comment`}
                >
                  Reply
                </Link>
                <div className="orbit-post-content__command" role="button">
                  Save
                </div>
                <div className="orbit-post-content__command" role="button">
                  Hide
                </div>
                <Link
                  className="orbit-post-content__command"
                  href={`/feed/${post.post_id}/comments/${comment.comment_id}/report`}
                >
                  Report
                </Link>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
