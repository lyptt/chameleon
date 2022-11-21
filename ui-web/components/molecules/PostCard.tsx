import { IPost } from '@/core/api'
import Config from '@/core/config'
import cx from 'classnames'
import Link from 'next/link'
import IconButton, { IconButtonIcon } from '@/components/atoms/IconButton'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import { LazyImage } from '@/components/atoms/LazyImage'
import PlainButton from '@/components/quarks/PlainButton'
import { KeyboardEvent, useState } from 'react'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

export interface IPostCardProps {
  className?: string
  post: IPost
  handlePostLiked?: (post: IPost) => void
  handleCommentSubmitted?: (post: IPost, comment: string) => void
  handlePostExpanded?: (post: IPost) => void
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function PostCard({
  className,
  post,
  handlePostLiked,
  handleCommentSubmitted,
  handlePostExpanded,
}: IPostCardProps) {
  const [comment, setComment] = useState('')

  const handleKeyUp = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      if (!comment.length) {
        return
      }

      event.preventDefault()
      handleCommentSubmitted?.(post, comment)
      setComment('')
    } else {
      setComment((event.target as any).value || '')
    }
  }

  const handlePostClicked = () => handleCommentSubmitted?.(post, comment)

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
          <div className="chameleon-post__avatar-name">{post.user_handle}</div>
        </Link>
      </div>
      <LazyImage
        className="chameleon-post__content"
        blurhash={post.content_blurhash}
        srcSet={`${Config.cdn}/${post.content_image_uri_large} ${post.content_width_large}w, ${Config.cdn}/${post.content_image_uri_medium} ${post.content_width_medium}w, ${Config.cdn}/${post.content_image_uri_small} ${post.content_width_small}w`}
        src={`${Config.cdn}/${post.content_image_uri_medium}`}
      />
      <div className="chameleon-post__action-bar">
        <div className="chameleon-post__tools">
          <IconButton
            icon={IconButtonIcon.Like}
            active={post.liked}
            onClick={() => handlePostLiked?.(post)}
          />
          <IconButton icon={IconButtonIcon.Message} />
          <IconButton icon={IconButtonIcon.Share} />
          <IconButton
            className="chameleon-post__save"
            icon={IconButtonIcon.Save}
          />
        </div>
        <p className="chameleon-post__stats">
          {post.likes === 0 && (
            <>
              <span className="chameleon-post__stats--thin-text">
                Be the first to
              </span>{' '}
              <a
                href=""
                className="chameleon-post__stats--cta"
                onClick={(e) => {
                  e.preventDefault()
                  handlePostLiked?.(post)
                }}
              >
                like this
              </a>
            </>
          )}
          {post.likes === 1 && '1 like'}
          {post.likes > 1 && `${post.likes} likes`}
        </p>
        {post.comments > 0 && (
          <button
            className="chameleon-post__comments"
            onClick={() => handlePostExpanded?.(post)}
          >
            {post.comments === 1 && <>View comments</>}
            {post.comments > 1 && <>View all {post.comments} comments</>}
          </button>
        )}
        <p className="chameleon-post__date">
          {dayjs.utc(post.created_at).fromNow()}
        </p>
        <div className="chameleon-post__comment-bar">
          <input
            id={`post-comment-${post.post_id}`}
            className="chameleon-post__comment-bar-field"
            placeholder="Add a comment..."
            onKeyUp={handleKeyUp}
            value={comment}
            onChange={(e) => setComment(e.target.value)}
          />
          <PlainButton
            brand
            className="chameleon-post__comment-bar-post-button"
            onClick={handlePostClicked}
            disabled={!comment.length}
          >
            Post!
          </PlainButton>
        </div>
      </div>
    </article>
  )
}
