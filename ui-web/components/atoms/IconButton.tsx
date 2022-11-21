import cx from 'classnames'
import {
  IoHeartOutline,
  IoHeart,
  IoChatbubbleOutline,
  IoChatbubble,
  IoPaperPlaneOutline,
  IoPaperPlane,
  IoBookmarkOutline,
  IoBookmark,
  IoClose,
  IoStarOutline,
  IoStar,
  IoEllipsisVertical,
} from 'react-icons/io5'
import dayjs from 'dayjs'

export enum IconButtonIcon {
  Like,
  Message,
  Share,
  Save,
  Close,
  Options,
}

export interface IIconButtonProps {
  className?: string
  contentClassName?: string
  href?: string
  onClick?: () => void
  icon: IconButtonIcon
  title?: string
  active?: boolean
  small?: boolean
}

function determineStyleClassName(icon: IconButtonIcon): string {
  switch (icon) {
    case IconButtonIcon.Like:
      return 'chameleon-icon-button--like'
    case IconButtonIcon.Message:
      return 'chameleon-icon-button--message'
    case IconButtonIcon.Share:
      return 'chameleon-icon-button--share'
    case IconButtonIcon.Save:
      return 'chameleon-icon-button--save'
    case IconButtonIcon.Close:
      return 'chameleon-icon-button--close'
    case IconButtonIcon.Options:
      return 'chameleon-icon-button--options'
  }
}

export default function IconButton({
  className,
  contentClassName,
  onClick,
  icon,
  title,
  active,
  small,
}: IIconButtonProps) {
  const date = dayjs()
  const isLoveDay = date.date() === 14 && date.month() === 1
  return (
    <button
      className={cx(
        'chameleon-icon-button',

        className,
        determineStyleClassName(icon),
        {
          'chameleon-icon-button--active': active,
          'chameleon-icon-button--small': small,
          'chameleon-icon-button--love-is-in-the-air': isLoveDay,
        }
      )}
      title={title}
      onClick={onClick}
    >
      {!active && icon === IconButtonIcon.Like && isLoveDay && (
        <IoHeartOutline
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {active && icon === IconButtonIcon.Like && isLoveDay && (
        <IoHeart
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {!active && icon === IconButtonIcon.Like && !isLoveDay && (
        <IoStarOutline
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {active && icon === IconButtonIcon.Like && !isLoveDay && (
        <IoStar
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {!active && icon === IconButtonIcon.Message && (
        <IoChatbubbleOutline
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {active && icon === IconButtonIcon.Message && (
        <IoChatbubble
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {!active && icon === IconButtonIcon.Share && (
        <IoPaperPlaneOutline
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {active && icon === IconButtonIcon.Share && (
        <IoPaperPlane
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {!active && icon === IconButtonIcon.Save && (
        <IoBookmarkOutline
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {active && icon === IconButtonIcon.Save && (
        <IoBookmark
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Close && (
        <IoClose
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
      {icon === IconButtonIcon.Options && (
        <IoEllipsisVertical
          className={cx('chameleon-icon-button__content', contentClassName)}
        />
      )}
    </button>
  )
}
