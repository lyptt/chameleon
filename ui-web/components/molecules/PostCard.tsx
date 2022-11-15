import { IPost } from '@/core/api'
import Config from '@/core/config'
import cx from 'classnames'
import Link from 'next/link'
import IconButton, { IconButtonIcon } from '../atoms/IconButton'
import classNames from './PostCard.module.css'
import dayjs from 'dayjs'
import dayjsUtc from 'dayjs/plugin/utc'
import dayjsRelative from 'dayjs/plugin/relativeTime'

dayjs.extend(dayjsUtc)
dayjs.extend(dayjsRelative)

export interface IPostCardProps {
  className?: string
  post: IPost
}

const transparentPixelUri = `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==`

export default function PostCard({ className, post }: IPostCardProps) {
  return (
    <article className={cx(className, classNames.post)}>
      <div className={classNames.masthead}>
        <Link href={`/users/${post.user_fediverse_id}`}>
          <a className={classNames.avatar}>
            <img
              className={classNames.avatarImage}
              src={post.user_avatar_url || transparentPixelUri}
              alt={post.user_handle}
            />
            <div className={classNames.avatarName}>{post.user_handle}</div>
          </a>
        </Link>
      </div>
      <img
        className={classNames.content}
        src={`${Config.cdn}/${post.content_image_uri_large}`}
      />
      <div className={classNames.actionBar}>
        <div className={classNames.tools}>
          <IconButton icon={IconButtonIcon.Like} />
          <IconButton icon={IconButtonIcon.Message} />
          <IconButton icon={IconButtonIcon.Share} />
          <IconButton className={classNames.save} icon={IconButtonIcon.Save} />
        </div>
        <p className={classNames.stats}>1337 likes</p>
        <p className={classNames.date}>
          {dayjs.utc(post.created_at).fromNow()}
        </p>
      </div>
    </article>
  )
}
