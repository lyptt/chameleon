import cx from 'classnames'
import Modal from 'react-modal'
import ScrollLock from 'react-scrolllock'
import { useEffect, useState, KeyboardEvent } from 'react'
import {
  postActionAddPostComment,
  postActionDismissPost,
  postActionLoadComments,
  postActionUpdateCommentLiked,
  postActionUpdateLiked,
  usePost,
} from '@/components/organisms/PostContext'
import Comment from '@/components/atoms/Comment'
import IconButton, { IconButtonIcon } from '@/components/atoms/IconButton'
import { LazyImage } from '@/components/atoms/LazyImage'
import Config from '@/core/config'
import Link from 'next/link'
import PlainButton from '@/components/quarks/PlainButton'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import { useAuth } from '@/components/organisms/AuthContext'
import { IComment, IPost } from '@/core/api'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

export interface IPostModal {
  className?: string
  onClose?: () => void
  onPostLiked?: (post: IPost) => void
  onCommentSubmitted?: (
    post: IPost,
    comment: string,
    skipWebRequest?: boolean
  ) => void
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function PostModal({
  className,
  onClose,
  onPostLiked,
  onCommentSubmitted,
}: IPostModal) {
  const { state, dispatch } = usePost()
  const { session } = useAuth()
  const { post, loading, loadingFailed, initialCommentLoadComplete, comments } =
    state
  const [opened, setOpened] = useState(false)
  const open = !!post || loading || loadingFailed
  const [comment, setComment] = useState('')

  useEffect(() => {
    if (!open) {
      return
    }

    setOpened(true)
    if (!initialCommentLoadComplete && !!post) {
      postActionLoadComments(post.post_id, 0, session?.access_token, dispatch)
    }
  }, [open, dispatch, initialCommentLoadComplete, post, session])

  const handleClose = () => {
    postActionDismissPost(dispatch)
    onClose?.()
    setOpened(false)
  }

  const handlePostLiked = () => {
    if (!post) {
      return
    }

    onPostLiked?.(post)
    postActionUpdateLiked(!post.liked, dispatch)
  }

  const handlePostClicked = () => {
    if (!post) {
      return
    }

    if (!comment.length) {
      return
    }

    onCommentSubmitted?.(post, comment, true)
    postActionAddPostComment(
      comment,
      post.post_id,
      session?.access_token,
      dispatch
    )
    setComment('')
  }

  const handleCommentLikeClicked = (comment: IComment) => {
    postActionUpdateCommentLiked(
      comment.liked ?? false,
      comment.comment_id,
      comment.post_id,
      session?.access_token,
      dispatch
    )
  }

  const handleKeyUp = (event: KeyboardEvent<HTMLInputElement>) => {
    if (!post) {
      return
    }

    if (event.key === 'Enter') {
      event.preventDefault()
      handlePostClicked()
    } else {
      setComment((event.target as any).value || '')
    }
  }

  return (
    <Modal
      isOpen={open}
      onRequestClose={handleClose}
      className={cx('chameleon-modal-post', className)}
      overlayClassName="chameleon-modal-post--overlay"
      overlayElement={(props, contentEl) => (
        <div {...props}>
          <>
            {contentEl}{' '}
            <div className={cx('chameleon-modal-post__close')}>
              <IconButton icon={IconButtonIcon.Close} title="Close" />
            </div>
          </>
        </div>
      )}
      contentLabel="View Post"
    >
      <ScrollLock>
        <div className={cx('chameleon-modal-post__content')}>
          {post && (
            <LazyImage
              className={cx('chameleon-modal-post__image')}
              srcSet={`${Config.cdn}/${post.content_image_uri_large} ${post.content_width_large}w, ${Config.cdn}/${post.content_image_uri_medium} ${post.content_width_medium}w, ${Config.cdn}/${post.content_image_uri_small} ${post.content_width_small}w`}
              src={`${Config.cdn}/${post.content_image_uri_medium}`}
            />
          )}
          {!post && <div className={cx('chameleon-modal-post__image')}></div>}
          <div className={cx('chameleon-modal-post__details')}>
            {!!post && (
              <>
                <div className={cx('chameleon-modal-post__masthead')}>
                  <Link
                    href={`/users/${post.user_fediverse_id}`}
                    onClick={handleClose}
                    className={cx('chameleon-modal-post__avatar')}
                  >
                    <img
                      className={cx('chameleon-modal-post__avatar-image')}
                      src={post.user_avatar_url || transparentPixelUri}
                      alt={post.user_handle}
                    />
                    <div className={cx('chameleon-modal-post__avatar-name')}>
                      {post.user_handle}
                    </div>
                  </Link>
                </div>
                <div className={cx('chameleon-modal-post__comments')}>
                  {comments.map((comment) => (
                    <Comment
                      key={comment.comment_id}
                      comment={comment}
                      onProfileLinkClicked={handleClose}
                      onCommentLikeClicked={handleCommentLikeClicked}
                    />
                  ))}
                </div>
                <div className={cx('chameleon-modal-post__action-bar')}>
                  <div className={cx('chameleon-modal-post__tools')}>
                    <IconButton
                      icon={IconButtonIcon.Like}
                      active={post.liked}
                      onClick={handlePostLiked}
                    />
                    <IconButton icon={IconButtonIcon.Message} />
                    <IconButton icon={IconButtonIcon.Share} />
                    <IconButton
                      className={cx('chameleon-modal-post__save')}
                      icon={IconButtonIcon.Save}
                    />
                  </div>
                  <p className={cx('chameleon-modal-post__stats')}>
                    {post.likes === 0 && (
                      <>
                        <span
                          className={cx(
                            'chameleon-modal-post__stats--thin-text'
                          )}
                        >
                          Be the first to
                        </span>{' '}
                        <a
                          href=""
                          className={cx('chameleon-modal-post__stats--cta')}
                          onClick={(e) => {
                            e.preventDefault()
                            handlePostLiked()
                          }}
                        >
                          like this
                        </a>
                      </>
                    )}
                    {post.likes === 1 && '1 like'}
                    {post.likes > 1 && `${post.likes} likes`}
                  </p>
                  <p className={cx('chameleon-modal-post__date')}>
                    {dayjs.utc(post.created_at).fromNow()}
                  </p>
                  <div className={cx('chameleon-modal-post__comment-bar')}>
                    <input
                      id={`post-comment-${post.post_id}`}
                      className={cx('chameleon-modal-post__comment-bar-field')}
                      placeholder="Add a comment..."
                      onKeyUp={handleKeyUp}
                      value={comment}
                      onChange={(e) => setComment(e.target.value)}
                    />
                    <PlainButton
                      brand
                      className={cx(
                        'chameleon-modal-post__comment-bar-post-button'
                      )}
                      onClick={handlePostClicked}
                      disabled={!comment.length}
                    >
                      Post!
                    </PlainButton>
                  </div>
                </div>
              </>
            )}
          </div>
        </div>
      </ScrollLock>
    </Modal>
  )
}
