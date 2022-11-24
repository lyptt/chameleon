import { IComment, IPost } from '@/core/api'
import Config from '@/core/config'
import cx from 'classnames'
import Link from 'next/link'
import IconButton, { IconButtonIcon } from '@/components/atoms/IconButton'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import dayjsLocalizedFormat from 'dayjs/plugin/localizedFormat'
import { LazyImage } from '@/components/atoms/LazyImage'
import { IoEarth } from 'react-icons/io5'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)
dayjs.extend(dayjsLocalizedFormat)

const localeObject = {
  relativeTime: {
    // relative time format strings, keep %s %d as the same
    future: 'in %s', // e.g. in 2 hours, %s been replaced with 2hours
    past: '%s ago',
    s: 's',
    m: 'min',
    mm: '%dm',
    h: 'h',
    hh: '%dh',
    d: 'd',
    dd: '%dd',
    M: 'm',
    MM: '%dm',
    y: 'y',
    yy: '%dy',
  },
}

dayjs.locale('en-mini', localeObject)
dayjs.locale('en')

export interface ICommentProps {
  backUri?: string
  className?: string
  comment: IComment
  handleCommentLiked?: (comment: IComment) => void
  handleCommentReplied?: (comment: IComment) => void
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function Comment({
  className,
  comment,
  handleCommentLiked,
  handleCommentReplied,
  backUri,
}: ICommentProps) {
  let relativeDate = ''
  if (dayjs.utc(comment.created_at).isBefore(dayjs.utc().subtract(3, 'days'))) {
    relativeDate = dayjs.utc(comment.created_at).local().format('L')
  } else {
    relativeDate = dayjs()
      .locale('en-mini')
      .to(dayjs.utc(comment.created_at).local(), true)
  }

  return (
    <article className={cx('chameleon-comment', className)}>
      <div className="chameleon-comment__masthead">
        <Link
          href={`/users/${comment.user_handle}${
            backUri ? `?from=${backUri}` : ''
          }`}
          className="chameleon-comment__avatar"
        >
          <img
            className="chameleon-comment__avatar-image"
            src={comment.user_avatar_url || transparentPixelUri}
            alt={comment.user_handle}
          />
        </Link>
        <div className="chameleon-comment__masthead-details">
          <Link
            href={`/users/${comment.user_handle}${
              backUri ? `?from=${backUri}` : ''
            }`}
            className="chameleon-comment__profile-name"
          >
            {comment.user_handle}
          </Link>
          <div className="chameleon-comment__profile-handle">
            {comment.user_fediverse_id}
          </div>
        </div>
        <div className="chameleon-comment__masthead-comment-details">
          <div className="chameleon-comment__masthead-comment-details-visibility">
            <IoEarth className="chameleon-comment__masthead-comment-details-visibility-image" />
          </div>
          <div className="chameleon-comment__masthead-comment-details-date">
            {relativeDate}
          </div>
        </div>
      </div>
      {comment.content_html.trim().length > 0 && (
        <div
          className="chameleon-comment__body"
          dangerouslySetInnerHTML={{ __html: comment.content_html }}
        />
      )}
      <div className="chameleon-comment__action-bar">
        <IconButton
          className="chameleon-comment__comments"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.Reply}
          onClick={() => handleCommentReplied?.(comment)}
        />
        <IconButton
          className="chameleon-comment__boost"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.Boost}
        />
        <IconButton
          className="chameleon-comment__like"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.Like}
          active={comment.liked}
          onClick={() => handleCommentLiked?.(comment)}
        />
        <IconButton
          className="chameleon-comment__save"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.Save}
        />
        <IconButton
          className="chameleon-comment__share"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.Share}
        />
        <IconButton
          className="chameleon-comment__options"
          contentClassName="chameleon-comment__action-icon"
          icon={IconButtonIcon.OptionsHorizontal}
        />
      </div>
    </article>
  )
}
