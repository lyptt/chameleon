import { IPost } from '@/core/api'
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

export interface IPostCardProps {
  className?: string
  post: IPost
  linkToPost?: boolean
  handlePostLiked?: (post: IPost) => void
  handlePostReplied?: (post: IPost) => void
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function PostCard({
  className,
  post,
  linkToPost = true,
  handlePostLiked,
  handlePostReplied,
}: IPostCardProps) {
  let relativeDate = ''
  if (dayjs.utc(post.created_at).isBefore(dayjs.utc().subtract(3, 'days'))) {
    relativeDate = dayjs.utc(post.created_at).local().format('L')
  } else {
    relativeDate = dayjs()
      .locale('en-mini')
      .to(dayjs.utc(post.created_at).local(), true)
  }

  const postUri =
    post.uri.indexOf('http') === 0
      ? post.uri
      : `${Config.fqdn}/users/${post.user_handle}/${post.uri}`

  const image = (
    <LazyImage
      className="chameleon-post__image"
      contentClassName="chameleon-post__image-content"
      blurhash={post.content_blurhash}
      srcSet={`${Config.cdn}/${post.content_image_uri_large} ${post.content_width_large}w, ${Config.cdn}/${post.content_image_uri_medium} ${post.content_width_medium}w, ${Config.cdn}/${post.content_image_uri_small} ${post.content_width_small}w`}
      src={`${Config.cdn}/${post.content_image_uri_medium}`}
    />
  )

  return (
    <article className={cx('chameleon-post', className)}>
      <div className="chameleon-post__masthead">
        <Link
          href={`/users/${post.user_fediverse_id}`}
          className="chameleon-post__avatar"
        >
          <img
            className="chameleon-post__avatar-image"
            src={post.user_avatar_url || transparentPixelUri}
            alt={post.user_handle}
          />
        </Link>
        <div className="chameleon-post__masthead-details">
          <Link
            href={`/users/${post.user_handle}`}
            className="chameleon-post__profile-name"
          >
            {post.user_handle}
          </Link>
          <div className="chameleon-post__profile-handle">
            {post.user_fediverse_id}
          </div>
        </div>
        <div className="chameleon-post__masthead-post-details">
          <div className="chameleon-post__masthead-post-details-visibility">
            <IoEarth className="chameleon-post__masthead-post-details-visibility-image" />
          </div>
          <div className="chameleon-post__masthead-post-details-date">
            {relativeDate}
          </div>
        </div>
      </div>
      {linkToPost && (
        <Link className="chameleon-post__image-link" href={postUri}>
          {image}
        </Link>
      )}
      {!linkToPost && image}
      {post.content_html.trim().length > 0 && (
        <>
          {linkToPost && (
            <Link
              className="chameleon-post__body"
              dangerouslySetInnerHTML={{ __html: post.content_html }}
              href={postUri}
            />
          )}
          {!linkToPost && (
            <div
              className="chameleon-post__body"
              dangerouslySetInnerHTML={{ __html: post.content_html }}
            />
          )}
        </>
      )}
      <div className="chameleon-post__action-bar">
        <IconButton
          className="chameleon-post__comments"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.Reply}
          onClick={() => handlePostReplied?.(post)}
        >
          {post.comments > 0 && (
            <span className="chameleon-post__comment-count">
              {post.comments}+
            </span>
          )}
        </IconButton>
        <IconButton
          className="chameleon-post__boost"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.Boost}
        />
        <IconButton
          className="chameleon-post__like"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.Like}
          active={post.liked}
          onClick={() => handlePostLiked?.(post)}
        />
        <IconButton
          className="chameleon-post__save"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.Save}
        />
        <IconButton
          className="chameleon-post__share"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.Share}
        />
        <IconButton
          className="chameleon-post__options"
          contentClassName="chameleon-post__action-icon"
          icon={IconButtonIcon.OptionsHorizontal}
        />
      </div>
    </article>
  )
}
