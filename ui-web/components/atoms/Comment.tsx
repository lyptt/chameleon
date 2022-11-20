import { HTMLAttributes } from 'react'
import classNames from './Comment.module.css'
import cx from 'classnames'
import { IComment } from '@/core/api'
import Link from 'next/link'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import dayjsLocalizedFormat from 'dayjs/plugin/localizedFormat'
import PlainButton from '../quarks/PlainButton'
import IconButton, { IconButtonIcon } from './IconButton'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)
dayjs.extend(dayjsLocalizedFormat)

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export interface ICommentProps extends HTMLAttributes<HTMLDivElement> {
  comment: IComment
  onProfileLinkClicked?: () => void
  onCommentLikeClicked?: (comment: IComment) => void
}

export default function Comment({
  comment,
  onProfileLinkClicked,
  onCommentLikeClicked,
  className,
  ...props
}: ICommentProps) {
  let relativeDate = ''
  if (
    dayjs.utc(comment.created_at).isBefore(dayjs.utc().subtract(1, 'month'))
  ) {
    relativeDate = dayjs.utc(comment.created_at).local().format('L')
  } else {
    relativeDate = dayjs().to(dayjs.utc(comment.created_at).local())
  }

  return (
    <div
      className={cx(className, 'chameleon-comment', classNames.comment)}
      {...props}
    >
      <Link
        href={`/users/${comment.user_fediverse_id}`}
        onClick={onProfileLinkClicked}
        className={cx(
          'chameleon-comment__avatar-image',
          classNames.avatarImage
        )}
      >
        <img
          src={comment.user_avatar_url || transparentPixelUri}
          alt={comment.user_handle}
        />
      </Link>
      <p
        className={cx(
          className,
          'chameleon-comment__content',
          classNames.content
        )}
      >
        <Link
          href={`/users/${comment.user_fediverse_id}`}
          onClick={onProfileLinkClicked}
          className={cx(
            'chameleon-comment__profile-link',
            classNames.profileLink
          )}
        >
          {comment.user_handle}
        </Link>
        <span dangerouslySetInnerHTML={{ __html: comment.content_html }} />
        <div
          className={cx(className, 'chameleon-comment__meta', classNames.meta)}
        >
          <span>{relativeDate}</span>
          {comment.likes === 1 && <strong>1 like</strong>}
          {comment.likes > 1 && <strong>{comment.likes} likes</strong>}
          <PlainButton thin faded>
            Reply
          </PlainButton>
        </div>
      </p>
      <div
        className={cx(
          className,
          'chameleon-comment__actions',
          classNames.actions
        )}
      >
        <IconButton
          icon={IconButtonIcon.Like}
          active={comment.liked}
          small
          onClick={() => onCommentLikeClicked?.(comment)}
        />
      </div>
    </div>
  )
}
