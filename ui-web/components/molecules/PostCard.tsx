import { IPost } from '@/core/api'
import Config from '@/core/config'
import cx from 'classnames'
import Link from 'next/link'
import IconButton, { IconButtonIcon } from '@/components/atoms/IconButton'
import classNames from './PostCard.module.css'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'
import { LazyImage } from '@/components/atoms/LazyImage'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

export interface IPostCardProps {
  className?: string
  post: IPost
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function PostCard({ className, post }: IPostCardProps) {
  return (
    <article className={cx('chameleon-post', className, classNames.post)}>
      <div className={cx('chameleon-post__masthead', classNames.masthead)}>
        <Link href={`/users/${post.user_fediverse_id}`}>
          <a className={cx('chameleon-post__avatar', classNames.avatar)}>
            <img
              className={cx(
                'chameleon-post__avatar-image',
                classNames.avatarImage
              )}
              src={post.user_avatar_url || transparentPixelUri}
              alt={post.user_handle}
            />
            <div
              className={cx(
                'chameleon-post__avatar-name',
                classNames.avatarName
              )}
            >
              {post.user_handle}
            </div>
          </a>
        </Link>
      </div>
      <LazyImage
        className={cx('chameleon-post__content', classNames.content)}
        blurhash={post.content_blurhash}
        srcSet={`${Config.cdn}/${post.content_image_uri_large} 2048w, ${Config.cdn}/${post.content_image_uri_medium} 1024w, ${Config.cdn}/${post.content_image_uri_small} 256w`}
        src={`${Config.cdn}/${post.content_image_uri_medium}`}
      />
      <div className={cx('chameleon-post__action-bar', classNames.actionBar)}>
        <div className={cx('chameleon-post__tools', classNames.tools)}>
          <IconButton icon={IconButtonIcon.Like} />
          <IconButton icon={IconButtonIcon.Message} />
          <IconButton icon={IconButtonIcon.Share} />
          <IconButton
            className={cx('chameleon-post__save', classNames.save)}
            icon={IconButtonIcon.Save}
          />
        </div>
        <p className={cx('chameleon-post__stats', classNames.stats)}>
          1337 likes
        </p>
        <p className={cx('chameleon-post__date', classNames.date)}>
          {dayjs.utc(post.created_at).fromNow()}
        </p>
      </div>
    </article>
  )
}
